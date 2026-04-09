use serde::Serialize;
use serde_json::json;
use std::sync::atomic::Ordering;
use std::time::Instant;

use super::error::AppError;
use super::processor::PacketProcessor;
use super::{support::read_state, support::write_state, AppState};
use crate::application::services::capture_pipeline_commit::{
    compute_capture_pipeline_state, CapturePipelineDependencies, CapturePipelineSource,
};
use crate::application::use_cases::capture as use_case;
use gm_capture::PcapReader;

#[derive(Serialize)]
pub struct ImportResult {
    pub file_count: usize,
    pub packet_count: usize,
    pub connection_count: usize,
    pub asset_count: usize,
    pub protocols_detected: Vec<String>,
    pub duration_ms: u64,
    pub per_file: Vec<FileImportResult>,
}

pub use use_case::ImportFileResult as FileImportResult;

#[derive(Serialize)]
pub struct StopCaptureResult {
    pub packets_captured: u64,
    pub bytes_captured: u64,
    pub elapsed_seconds: f64,
    pub pcap_saved: bool,
    pub pcap_path: Option<String>,
    pub packets_saved: usize,
}

#[derive(Serialize)]
pub struct CaptureStatusInfo {
    pub is_running: bool,
    pub is_paused: bool,
    pub packets_captured: u64,
    pub bytes_captured: u64,
    pub elapsed_seconds: f64,
}

pub async fn cancel_import(state: &AppState) -> Result<(), AppError> {
    state.import_cancelled.store(true, Ordering::SeqCst);
    log::info!("PCAP import cancellation requested");
    Ok(())
}

pub fn import_pcap_files(paths: Vec<String>, state: &AppState) -> Result<ImportResult, AppError> {
    let start = Instant::now();

    let reader = PcapReader::new();
    let mut processor = PacketProcessor::new();

    let import_outcome = use_case::run_import(
        &paths,
        start,
        |path| reader.read_file(path).map_err(|e| e.to_string()),
        |packet| processor.process_packet(packet),
        |file_index, file_count, filename, packet_count, started| {
            emit_import_progress(
                state,
                file_index,
                file_count,
                filename,
                packet_count,
                started,
            )
        },
    )?;
    let per_file_results = import_outcome.per_file_results;
    let total_packet_count = import_outcome.total_packet_count;

    let (connection_count, asset_count, protocols_detected) =
        compute_and_apply_import_state(processor, &per_file_results, state)?;
    let duration_ms = start.elapsed().as_millis() as u64;

    Ok(ImportResult {
        file_count: paths.len(),
        packet_count: total_packet_count,
        connection_count,
        asset_count,
        protocols_detected,
        duration_ms,
        per_file: per_file_results,
    })
}

fn emit_import_progress(
    state: &AppState,
    file_index: usize,
    file_count: usize,
    filename: &str,
    packet_count: usize,
    start: Instant,
) {
    if let Some(tx) = &state.event_tx {
        let progress = (file_index + 1) as f64 / file_count as f64 * 100.0;
        let _ = tx.send((
            "import_progress".to_string(),
            json!({
                "current_file": filename,
                "file_index": file_index,
                "file_count": file_count,
                "packets_processed": packet_count,
                "bytes_processed": 0,
                "file_size": 0,
                "progress_percent": progress,
                "elapsed_secs": start.elapsed().as_secs_f64(),
            }),
        ));
    }
}

fn compute_and_apply_import_state(
    mut processor: PacketProcessor,
    per_file_results: &[FileImportResult],
    state: &AppState,
) -> Result<(usize, usize, Vec<String>), AppError> {
    let imported_files: Vec<String> = per_file_results
        .iter()
        .filter(|f| f.status == "ok")
        .map(|f| f.filename.clone())
        .collect();

    let existing_imported_files = read_state(&state.capture, "capture")
        .map_err(AppError::state_lock)?
        .imported_files
        .clone();
    let inv = read_state(&state.inventory, "inventory").map_err(AppError::state_lock)?;
    let sigs = read_state(&state.signatures, "signatures").map_err(AppError::state_lock)?;

    let deps = CapturePipelineDependencies {
        signature_engine: &sigs.signature_engine,
        oui_lookup: &inv.oui_lookup,
        geoip_lookup: &inv.geoip_lookup,
    };
    let mut source = PacketProcessorCaptureSource {
        processor: &mut processor,
    };
    let (update, result) = compute_capture_pipeline_state(
        &mut source,
        &deps,
        &existing_imported_files,
        &imported_files,
    );

    drop(sigs);
    drop(inv);

    {
        let mut cap = write_state(&state.capture, "capture").map_err(AppError::state_lock)?;
        cap.topology = update.topology;
        cap.connections = update.connections;
        cap.packet_summaries = update.packet_summaries;
        cap.redundancy_protocols = update.redundancy_protocols;
        cap.imported_files = update.imported_files;
    }
    {
        let mut inv = write_state(&state.inventory, "inventory").map_err(AppError::state_lock)?;
        inv.assets = update.assets;
        inv.deep_parse_info = update.deep_parse_info;
    }
    {
        let mut analysis =
            write_state(&state.analysis, "analysis").map_err(AppError::state_lock)?;
        analysis.connection_stats = update.connection_stats;
        analysis.pattern_anomalies = update.pattern_anomalies;
    }

    Ok((
        result.connection_count,
        result.asset_count,
        result.protocols_detected,
    ))
}

struct PacketProcessorCaptureSource<'a> {
    processor: &'a mut PacketProcessor,
}

impl CapturePipelineSource for PacketProcessorCaptureSource<'_> {
    fn build_deep_parse_info(&self) -> std::collections::HashMap<String, super::DeepParseInfo> {
        self.processor.build_deep_parse_info()
    }

    fn build_assets(
        &self,
        signature_engine: &gm_signatures::SignatureEngine,
        deep_parse_info: &std::collections::HashMap<String, super::DeepParseInfo>,
        oui_lookup: &gm_db::OuiLookup,
        geoip_lookup: &gm_db::GeoIpLookup,
    ) -> (
        Vec<super::AssetInfo>,
        std::collections::HashMap<String, Vec<super::AssetSignatureMatch>>,
    ) {
        self.processor
            .build_assets(signature_engine, deep_parse_info, oui_lookup, geoip_lookup)
    }

    fn topology_snapshot(&self) -> gm_topology::TopologyGraph {
        self.processor.topo_builder.snapshot()
    }

    fn get_connections(&mut self) -> Vec<super::ConnectionInfo> {
        self.processor.get_connections()
    }

    fn get_packet_summaries(&self) -> std::collections::HashMap<String, Vec<super::PacketSummary>> {
        self.processor.get_packet_summaries()
    }

    fn build_pattern_results(
        &mut self,
    ) -> (
        Vec<gm_analysis::ConnectionStats>,
        Vec<gm_analysis::PatternAnomaly>,
    ) {
        self.processor.build_pattern_results()
    }

    fn build_redundancy_info(&self) -> Vec<gm_parsers::RedundancyInfo> {
        self.processor.build_redundancy_info()
    }

    fn get_protocols_detected(&self) -> Vec<String> {
        self.processor.get_protocols_detected()
    }
}

pub async fn stop_capture(
    save_path: Option<String>,
    state: &AppState,
) -> Result<StopCaptureResult, AppError> {
    let (mut capture, processing_thread) = {
        let mut cap = write_state(&state.capture, "capture").map_err(AppError::state_lock)?;
        let capture = cap.live_capture.take();
        let processing = cap.processing_thread.take();
        (capture, processing)
    };

    let Some(ref mut handle) = capture else {
        return Err(AppError::no_capture_running());
    };

    let stats = handle.stats();
    handle
        .stop()
        .map_err(|e| AppError::external_process(e.to_string()))?;

    if let Some(pt) = processing_thread {
        let _ = pt.join();
    }

    let (pcap_saved, pcap_path, packets_saved) = if let Some(ref path) = save_path {
        let count = handle
            .save_to_pcap(path)
            .map_err(|e| AppError::IoError(e.to_string()))?;
        log::info!("Saved {} packets to {}", count, path);
        (true, Some(path.clone()), count)
    } else {
        (false, None, 0)
    };

    log::info!(
        "Live capture stopped: {} packets, {} bytes, {:.1}s",
        stats.packets_captured,
        stats.bytes_captured,
        stats.elapsed_seconds
    );

    Ok(StopCaptureResult {
        packets_captured: stats.packets_captured,
        bytes_captured: stats.bytes_captured,
        elapsed_seconds: stats.elapsed_seconds,
        pcap_saved,
        pcap_path,
        packets_saved,
    })
}

pub async fn pause_capture(state: &AppState) -> Result<(), AppError> {
    let cap = read_state(&state.capture, "capture").map_err(AppError::state_lock)?;
    if let Some(ref handle) = cap.live_capture {
        handle.pause();
        log::info!("Live capture paused");
        Ok(())
    } else {
        Err(AppError::no_capture_running())
    }
}

pub async fn resume_capture(state: &AppState) -> Result<(), AppError> {
    let cap = read_state(&state.capture, "capture").map_err(AppError::state_lock)?;
    if let Some(ref handle) = cap.live_capture {
        handle.resume();
        log::info!("Live capture resumed");
        Ok(())
    } else {
        Err(AppError::no_capture_running())
    }
}

pub async fn get_capture_status(state: &AppState) -> Result<CaptureStatusInfo, AppError> {
    let cap = read_state(&state.capture, "capture").map_err(AppError::state_lock)?;
    if let Some(ref handle) = cap.live_capture {
        let stats = handle.stats();
        Ok(CaptureStatusInfo {
            is_running: handle.is_running(),
            is_paused: handle.is_paused(),
            packets_captured: stats.packets_captured,
            bytes_captured: stats.bytes_captured,
            elapsed_seconds: stats.elapsed_seconds,
        })
    } else {
        Ok(CaptureStatusInfo {
            is_running: false,
            is_paused: false,
            packets_captured: 0,
            bytes_captured: 0,
            elapsed_seconds: 0.0,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::application::use_cases::capture as use_case;

    use super::FileImportResult;

    #[test]
    fn filename_from_path_returns_basename_when_present() {
        let filename = use_case::filename_from_path("/tmp/captures/demo.pcap");
        assert_eq!(filename, "demo.pcap");
    }

    #[test]
    fn filename_from_path_returns_input_when_no_basename_found() {
        let filename = use_case::filename_from_path("capture.pcap");
        assert_eq!(filename, "capture.pcap");
    }

    #[test]
    fn total_packet_count_sums_all_files() {
        let files = vec![
            FileImportResult {
                filename: "a.pcap".to_string(),
                packet_count: 3,
                status: "ok".to_string(),
            },
            FileImportResult {
                filename: "b.pcap".to_string(),
                packet_count: 7,
                status: "ok".to_string(),
            },
        ];

        assert_eq!(use_case::total_packet_count(&files), 10);
    }

    #[test]
    fn ensure_successful_import_fails_when_no_packets_and_no_ok_files() {
        let files = vec![FileImportResult {
            filename: "broken.pcap".to_string(),
            packet_count: 0,
            status: "error: parse failed".to_string(),
        }];

        let result = use_case::ensure_successful_import(0, &files);
        assert!(result.is_err());
    }

    #[test]
    fn ensure_successful_import_succeeds_when_any_file_is_ok() {
        let files = vec![FileImportResult {
            filename: "ok.pcap".to_string(),
            packet_count: 0,
            status: "ok".to_string(),
        }];

        let result = use_case::ensure_successful_import(0, &files);
        assert!(result.is_ok());
    }
}

use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::sync::atomic::Ordering;
use std::time::Instant;

use super::processor::PacketProcessor;
use super::{support::read_state, support::write_state, AppState};
use gm_capture::PcapReader;
use gm_topology::TopologyGraph;

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

#[derive(Serialize)]
pub struct FileImportResult {
    pub filename: String,
    pub packet_count: usize,
    pub status: String,
}

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

pub async fn cancel_import(state: &AppState) -> Result<(), String> {
    state.import_cancelled.store(true, Ordering::SeqCst);
    log::info!("PCAP import cancellation requested");
    Ok(())
}

pub fn import_pcap_files(paths: Vec<String>, state: &AppState) -> Result<ImportResult, String> {
    let start = Instant::now();
    let (processor, per_file_results) = process_input_files(&paths, state, start)?;
    let total_packet_count = total_packet_count(&per_file_results);
    ensure_successful_import(total_packet_count, &per_file_results)?;
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

fn process_input_files(
    paths: &[String],
    state: &AppState,
    start: Instant,
) -> Result<(PacketProcessor, Vec<FileImportResult>), String> {
    let reader = PcapReader::new();
    let mut processor = PacketProcessor::new();
    let mut per_file_results: Vec<FileImportResult> = Vec::new();

    for (file_index, path) in paths.iter().enumerate() {
        let filename = filename_from_path(path);
        match reader.read_file(path) {
            Ok(packets) => {
                for packet in &packets {
                    processor.process_packet(packet);
                }
                per_file_results.push(FileImportResult {
                    filename: filename.clone(),
                    packet_count: packets.len(),
                    status: "ok".to_string(),
                });
                emit_import_progress(
                    state,
                    file_index,
                    paths.len(),
                    &filename,
                    packets.len(),
                    start,
                );
            }
            Err(e) => {
                per_file_results.push(FileImportResult {
                    filename,
                    packet_count: 0,
                    status: format!("error: {}", e),
                });
            }
        }
    }

    Ok((processor, per_file_results))
}

fn filename_from_path(path: &str) -> String {
    std::path::Path::new(path)
        .file_name()
        .map(|f| f.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string())
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

fn total_packet_count(per_file_results: &[FileImportResult]) -> usize {
    per_file_results.iter().map(|r| r.packet_count).sum()
}

fn ensure_successful_import(
    total_packet_count: usize,
    per_file_results: &[FileImportResult],
) -> Result<(), String> {
    if total_packet_count == 0 && !per_file_results.iter().any(|r| r.status == "ok") {
        return Err("No packets could be parsed from the provided files".to_string());
    }
    Ok(())
}

fn compute_and_apply_import_state(
    mut processor: PacketProcessor,
    per_file_results: &[FileImportResult],
    state: &AppState,
) -> Result<(usize, usize, Vec<String>), String> {
    let deep_parse_info = processor.build_deep_parse_info();
    let (assets, sig_results) = {
        let sigs = state.signatures.read().map_err(|e| e.to_string())?;
        let inv = state.inventory.read().map_err(|e| e.to_string())?;
        processor.build_assets(
            &sigs.signature_engine,
            &deep_parse_info,
            &inv.oui_lookup,
            &inv.geoip_lookup,
        )
    };

    let mut topology = processor.topo_builder.snapshot();
    enrich_topology_with_signatures(&mut topology, &sig_results);

    let connection_list = processor.get_connections();
    let packet_summaries = processor.get_packet_summaries();
    let (connection_stats, pattern_anomalies) = processor.build_pattern_results();
    let redundancy_protocols = processor.build_redundancy_info();
    let protocols_detected = processor.get_protocols_detected();

    let asset_count = assets.len();
    let connection_count = connection_list.len();
    let imported_files: Vec<String> = per_file_results
        .iter()
        .filter(|f| f.status == "ok")
        .map(|f| f.filename.clone())
        .collect();

    {
        let mut cap = state.capture.write().map_err(|e| e.to_string())?;
        cap.topology = topology;
        cap.connections = connection_list;
        cap.packet_summaries = packet_summaries;
        cap.redundancy_protocols = redundancy_protocols;
        cap.imported_files.extend(imported_files);
        cap.imported_files.sort();
        cap.imported_files.dedup();
    }
    {
        let mut inv = state.inventory.write().map_err(|e| e.to_string())?;
        inv.assets = assets;
        inv.deep_parse_info = deep_parse_info;
    }
    {
        let mut analysis = state.analysis.write().map_err(|e| e.to_string())?;
        analysis.connection_stats = connection_stats;
        analysis.pattern_anomalies = pattern_anomalies;
    }

    Ok((connection_count, asset_count, protocols_detected))
}

fn enrich_topology_with_signatures(
    topology: &mut TopologyGraph,
    sig_results: &HashMap<String, Vec<super::AssetSignatureMatch>>,
) {
    for node in &mut topology.nodes {
        if let Some(sig_matches) = sig_results.get(&node.ip_address) {
            if let Some(best) = sig_matches.first() {
                if let Some(ref v) = best.vendor {
                    node.vendor = Some(v.clone());
                }
                if let Some(ref dt) = best.device_type {
                    if best.confidence >= 3 {
                        node.device_type = dt.clone();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ensure_successful_import, filename_from_path, total_packet_count, FileImportResult,
    };

    #[test]
    fn filename_from_path_returns_basename_when_present() {
        let filename = filename_from_path("/tmp/captures/demo.pcap");
        assert_eq!(filename, "demo.pcap");
    }

    #[test]
    fn filename_from_path_returns_input_when_no_basename_found() {
        let filename = filename_from_path("capture.pcap");
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

        assert_eq!(total_packet_count(&files), 10);
    }

    #[test]
    fn ensure_successful_import_fails_when_no_packets_and_no_ok_files() {
        let files = vec![FileImportResult {
            filename: "broken.pcap".to_string(),
            packet_count: 0,
            status: "error: parse failed".to_string(),
        }];

        let result = ensure_successful_import(0, &files);
        assert!(result.is_err());
    }

    #[test]
    fn ensure_successful_import_succeeds_when_any_file_is_ok() {
        let files = vec![FileImportResult {
            filename: "ok.pcap".to_string(),
            packet_count: 0,
            status: "ok".to_string(),
        }];

        let result = ensure_successful_import(0, &files);
        assert!(result.is_ok());
    }
}

pub async fn stop_capture(
    save_path: Option<String>,
    state: &AppState,
) -> Result<StopCaptureResult, String> {
    let (mut capture, processing_thread) = {
        let mut cap = write_state(&state.capture, "capture")?;
        let capture = cap.live_capture.take();
        let processing = cap.processing_thread.take();
        (capture, processing)
    };

    let Some(ref mut handle) = capture else {
        return Err("No capture is running.".to_string());
    };

    let stats = handle.stats();
    handle.stop().map_err(|e| e.to_string())?;

    if let Some(pt) = processing_thread {
        let _ = pt.join();
    }

    let (pcap_saved, pcap_path, packets_saved) = if let Some(ref path) = save_path {
        let count = handle.save_to_pcap(path).map_err(|e| e.to_string())?;
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

pub async fn pause_capture(state: &AppState) -> Result<(), String> {
    let cap = read_state(&state.capture, "capture")?;
    if let Some(ref handle) = cap.live_capture {
        handle.pause();
        log::info!("Live capture paused");
        Ok(())
    } else {
        Err("No capture is running.".to_string())
    }
}

pub async fn resume_capture(state: &AppState) -> Result<(), String> {
    let cap = read_state(&state.capture, "capture")?;
    if let Some(ref handle) = cap.live_capture {
        handle.resume();
        log::info!("Live capture resumed");
        Ok(())
    } else {
        Err("No capture is running.".to_string())
    }
}

pub async fn get_capture_status(state: &AppState) -> Result<CaptureStatusInfo, String> {
    let cap = read_state(&state.capture, "capture")?;
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

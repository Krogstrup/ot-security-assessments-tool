use super::web_support::ApiError;
use super::SharedState;
use crate::application::services::capture_pipeline_commit::{
    compute_capture_pipeline_state, CapturePipelineDependencies, CapturePipelineSource,
};
use crate::commands::processor::PacketProcessor;
use crate::commands::support::{read_state, write_state};
use axum::Json;
use gm_capture::{LiveCaptureConfig, ParsedPacket};
use gm_types::LIVE_CAPTURE_BATCH_SIZE;
use serde::Serialize;
use serde_json::{json, Value};
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

struct PacketProcessorCaptureSource<'a> {
    processor: &'a mut PacketProcessor,
}

impl CapturePipelineSource for PacketProcessorCaptureSource<'_> {
    fn build_deep_parse_info(
        &self,
    ) -> std::collections::HashMap<String, crate::commands::DeepParseInfo> {
        self.processor.build_deep_parse_info()
    }

    fn build_assets(
        &self,
        signature_engine: &gm_signatures::SignatureEngine,
        deep_parse_info: &std::collections::HashMap<String, crate::commands::DeepParseInfo>,
        oui_lookup: &gm_db::OuiLookup,
        geoip_lookup: &gm_db::GeoIpLookup,
    ) -> (
        Vec<crate::commands::AssetInfo>,
        std::collections::HashMap<String, Vec<crate::commands::AssetSignatureMatch>>,
    ) {
        self.processor
            .build_assets(signature_engine, deep_parse_info, oui_lookup, geoip_lookup)
    }

    fn topology_snapshot(&self) -> gm_topology::TopologyGraph {
        self.processor.topo_builder.snapshot()
    }

    fn get_connections(&mut self) -> Vec<crate::commands::ConnectionInfo> {
        self.processor.get_connections()
    }

    fn get_packet_summaries(
        &self,
    ) -> std::collections::HashMap<String, Vec<crate::commands::PacketSummary>> {
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

pub(crate) fn to_json<T: Serialize>(value: T) -> Result<Json<Value>, ApiError> {
    serde_json::to_value(value)
        .map(Json)
        .map_err(|e| ApiError::internal(e.to_string()))
}

fn spawn_processing_thread_headless(
    rx: mpsc::Receiver<ParsedPacket>,
    state: SharedState,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut processor = PacketProcessor::new();
        let mut batch: Vec<ParsedPacket> = Vec::new();
        let mut last_flush = Instant::now();
        let flush_interval = Duration::from_millis(250);

        loop {
            match rx.recv_timeout(Duration::from_millis(50)) {
                Ok(packet) => {
                    batch.push(packet);
                    if batch.len() >= LIVE_CAPTURE_BATCH_SIZE
                        || last_flush.elapsed() >= flush_interval
                    {
                        flush_batch_headless(&state, &mut processor, &mut batch);
                        last_flush = Instant::now();
                    }
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    if !batch.is_empty() && last_flush.elapsed() >= flush_interval {
                        flush_batch_headless(&state, &mut processor, &mut batch);
                        last_flush = Instant::now();
                    }
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    if !batch.is_empty() {
                        flush_batch_headless(&state, &mut processor, &mut batch);
                    }
                    break;
                }
            }
        }
    })
}

fn flush_batch_headless(
    state: &SharedState,
    processor: &mut PacketProcessor,
    batch: &mut Vec<ParsedPacket>,
) {
    for packet in batch.drain(..) {
        processor.process_packet(&packet);
    }

    let existing_imported_files = match read_state(&state.capture, "capture") {
        Ok(capture) => capture.imported_files.clone(),
        Err(err) => {
            log::error!("capture flush read error: {}", err);
            return;
        }
    };
    let inventory = match read_state(&state.inventory, "inventory") {
        Ok(inventory) => inventory,
        Err(err) => {
            log::error!("capture flush inventory lock error: {}", err);
            return;
        }
    };
    let signatures = match read_state(&state.signatures, "signatures") {
        Ok(signatures) => signatures,
        Err(err) => {
            log::error!("capture flush signatures lock error: {}", err);
            return;
        }
    };
    let deps = CapturePipelineDependencies {
        signature_engine: &signatures.signature_engine,
        oui_lookup: &inventory.oui_lookup,
        geoip_lookup: &inventory.geoip_lookup,
    };
    let mut source = PacketProcessorCaptureSource { processor };
    let (update, _) =
        compute_capture_pipeline_state(&mut source, &deps, &existing_imported_files, &[]);
    drop(signatures);
    drop(inventory);

    if let Err(err) = (|| -> Result<(), String> {
        {
            let mut capture = write_state(&state.capture, "capture")?;
            capture.topology = update.topology;
            capture.connections = update.connections;
            capture.packet_summaries = update.packet_summaries;
            capture.redundancy_protocols = update.redundancy_protocols;
            capture.imported_files = update.imported_files;
        }
        {
            let mut inventory = write_state(&state.inventory, "inventory")?;
            inventory.assets = update.assets;
            inventory.deep_parse_info = update.deep_parse_info;
        }
        {
            let mut analysis = write_state(&state.analysis, "analysis")?;
            analysis.connection_stats = update.connection_stats;
            analysis.pattern_anomalies = update.pattern_anomalies;
        }
        Ok(())
    })() {
        log::error!("capture flush commit error: {}", err);
        return;
    }

    // Emit capture_stats event for SSE subscribers
    if let Some(tx) = &state.event_tx {
        let asset_count = state
            .inventory
            .read()
            .map(|inv| inv.assets.len())
            .unwrap_or(0);
        let connection_count = state
            .capture
            .read()
            .map(|cap| cap.connections.len())
            .unwrap_or(0);
        let _ = tx.send((
            "capture_stats".to_string(),
            json!({
                "packets_captured": 0,
                "packets_per_second": 0,
                "bytes_captured": 0,
                "active_connections": connection_count,
                "asset_count": asset_count,
                "elapsed_seconds": 0.0,
            }),
        ));
    }
}

pub(crate) async fn start_capture_headless(
    state: SharedState,
    interface_name: String,
    bpf_filter: Option<String>,
) -> Result<(), ApiError> {
    {
        let cap = state
            .capture
            .read()
            .map_err(|e| ApiError::internal(e.to_string()))?;
        if cap.live_capture.is_some() {
            return Err(ApiError::bad_request(
                "A capture is already running. Stop it first.",
            ));
        }
    }

    let config = LiveCaptureConfig {
        interface_name: interface_name.clone(),
        bpf_filter: bpf_filter.clone(),
        promiscuous: true,
        ring_buffer_size: 1_000_000,
        snaplen: 65_535,
    };

    let (handle, rx) = gm_capture::LiveCaptureHandle::start(config)
        .map_err(|e| ApiError::bad_request(e.to_string()))?;
    let processing = spawn_processing_thread_headless(rx, state.clone());

    let mut cap = state
        .capture
        .write()
        .map_err(|e| ApiError::internal(e.to_string()))?;
    cap.live_capture = Some(handle);
    cap.processing_thread = Some(processing);

    log::info!(
        "Headless live capture started on {} (filter: {:?})",
        interface_name,
        bpf_filter
    );
    Ok(())
}

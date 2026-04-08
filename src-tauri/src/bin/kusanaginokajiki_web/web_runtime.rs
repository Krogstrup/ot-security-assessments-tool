use super::web_support::ApiError;
use super::SharedState;
use crate::commands::processor::PacketProcessor;
use axum::Json;
use gm_capture::{LiveCaptureConfig, ParsedPacket};
use serde::Serialize;
use serde_json::{json, Value};
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

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
                    if batch.len() >= 500 || last_flush.elapsed() >= flush_interval {
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

    let deep_parse_info = processor.build_deep_parse_info();
    let (assets, sig_results) = {
        let sigs = match state.signatures.read() {
            Ok(v) => v,
            Err(e) => {
                log::error!("capture flush signatures lock error: {}", e);
                return;
            }
        };
        let inv = match state.inventory.read() {
            Ok(v) => v,
            Err(e) => {
                log::error!("capture flush inventory lock error: {}", e);
                return;
            }
        };
        processor.build_assets(
            &sigs.signature_engine,
            &deep_parse_info,
            &inv.oui_lookup,
            &inv.geoip_lookup,
        )
    };

    let mut topology = processor.topo_builder.snapshot();
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

    let connections = processor.get_connections();
    let packet_summaries = processor.get_packet_summaries();
    let (connection_stats, pattern_anomalies) = processor.build_pattern_results();
    let redundancy_protocols = processor.build_redundancy_info();

    if let Ok(mut cap) = state.capture.write() {
        cap.topology = topology;
        cap.connections = connections;
        cap.packet_summaries = packet_summaries;
        cap.redundancy_protocols = redundancy_protocols;
    }
    if let Ok(mut inv) = state.inventory.write() {
        inv.assets = assets;
        inv.deep_parse_info = deep_parse_info;
    }
    if let Ok(mut analysis) = state.analysis.write() {
        analysis.connection_stats = connection_stats;
        analysis.pattern_anomalies = pattern_anomalies;
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

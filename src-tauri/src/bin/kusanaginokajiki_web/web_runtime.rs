use super::web_support::ApiError;
use super::SharedState;
use crate::application::services::capture_pipeline_commit::commit_capture_pipeline_state;
use crate::commands::processor::PacketProcessor;
use axum::Json;
use gm_constants::LIVE_CAPTURE_BATCH_SIZE;
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
                    if batch.len() >= LIVE_CAPTURE_BATCH_SIZE || last_flush.elapsed() >= flush_interval {
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

    if let Err(err) = commit_capture_pipeline_state(state.as_ref(), processor, &[]) {
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

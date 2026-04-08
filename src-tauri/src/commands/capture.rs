use serde::Serialize;
use std::sync::atomic::Ordering;

use super::AppState;

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

pub async fn stop_capture(
    save_path: Option<String>,
    state: &AppState,
) -> Result<StopCaptureResult, String> {
    let (mut capture, processing_thread) = {
        let mut cap = state.capture.write().map_err(|e| e.to_string())?;
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
    let cap = state.capture.read().map_err(|e| e.to_string())?;
    if let Some(ref handle) = cap.live_capture {
        handle.pause();
        log::info!("Live capture paused");
        Ok(())
    } else {
        Err("No capture is running.".to_string())
    }
}

pub async fn resume_capture(state: &AppState) -> Result<(), String> {
    let cap = state.capture.read().map_err(|e| e.to_string())?;
    if let Some(ref handle) = cap.live_capture {
        handle.resume();
        log::info!("Live capture resumed");
        Ok(())
    } else {
        Err("No capture is running.".to_string())
    }
}

pub async fn get_capture_status(state: &AppState) -> Result<CaptureStatusInfo, String> {
    let cap = state.capture.read().map_err(|e| e.to_string())?;
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

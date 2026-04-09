//! Wireshark command adapters.

use crate::application::use_cases::wireshark as use_case;

use super::{error::AppError, support::read_state, AppState};

pub use use_case::{FrameRow, WiresharkInfo};

pub async fn detect_wireshark() -> Result<WiresharkInfo, AppError> {
    Ok(use_case::detect_wireshark())
}

pub async fn open_in_wireshark(connection_id: String, state: &AppState) -> Result<(), AppError> {
    let wireshark_path = use_case::find_wireshark_binary().ok_or_else(|| {
        AppError::external_process(
            "Wireshark not found. Install Wireshark and ensure it's in your PATH.",
        )
    })?;

    let (filter, pcap_files) = {
        let capture = read_state(&state.capture, "capture").map_err(AppError::state_lock)?;
        let conn = capture
            .connections
            .iter()
            .find(|c| c.id == connection_id)
            .ok_or_else(|| {
                AppError::invalid_input(format!("Connection {} not found", connection_id))
            })?;

        let filter = use_case::build_display_filter(conn);
        let pcap_files = conn.origin_files.clone();
        (filter, pcap_files)
    };

    let mut args: Vec<String> = Vec::new();
    args.push("-Y".to_string());
    args.push(filter);

    for file in &pcap_files {
        if !file.starts_with('[') {
            args.push("-r".to_string());
            args.push(file.clone());
            break;
        }
    }

    let status = std::process::Command::new(&wireshark_path)
        .args(&args)
        .spawn()
        .map_err(|e| AppError::external_process(format!("Failed to launch Wireshark: {}", e)))?;

    log::info!(
        "Launched Wireshark (PID: {:?}) with filter for connection {}",
        status.id(),
        connection_id
    );
    Ok(())
}

pub async fn open_wireshark_for_node(ip_address: String) -> Result<(), AppError> {
    let wireshark_path = use_case::find_wireshark_binary().ok_or_else(|| {
        AppError::external_process(
            "Wireshark not found. Install Wireshark and ensure it's in your PATH.",
        )
    })?;

    let filter = format!("ip.addr == {}", ip_address);
    let args = vec!["-Y".to_string(), filter];

    let status = std::process::Command::new(&wireshark_path)
        .args(&args)
        .spawn()
        .map_err(|e| AppError::external_process(format!("Failed to launch Wireshark: {}", e)))?;

    log::info!(
        "Launched Wireshark (PID: {:?}) for node {}",
        status.id(),
        ip_address
    );
    Ok(())
}

pub async fn get_connection_frames(
    connection_id: String,
    state: &AppState,
) -> Result<Vec<FrameRow>, AppError> {
    let capture = read_state(&state.capture, "capture").map_err(AppError::state_lock)?;
    let packets = capture
        .packet_summaries
        .get(&connection_id)
        .cloned()
        .unwrap_or_default();
    Ok(use_case::build_frame_rows(&packets))
}

pub async fn export_frames_csv(
    connection_id: String,
    state: &AppState,
) -> Result<String, AppError> {
    let capture = read_state(&state.capture, "capture").map_err(AppError::state_lock)?;
    let packets = capture
        .packet_summaries
        .get(&connection_id)
        .cloned()
        .unwrap_or_default();
    Ok(use_case::build_frames_csv(&packets))
}

pub async fn save_frames_csv(
    connection_id: String,
    output_path: String,
    state: &AppState,
) -> Result<(), AppError> {
    let csv = export_frames_csv(connection_id, state).await?;
    std::fs::write(&output_path, csv)
        .map_err(|e| AppError::IoError(format!("Failed to write CSV: {}", e)))?;
    Ok(())
}

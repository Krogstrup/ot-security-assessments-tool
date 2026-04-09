//! Adapter layer for export/reporting use-cases.

use gm_analysis::AllowlistEntry;

use crate::application::use_cases::export as use_case;

use super::{
    error::AppError,
    support::{mutex_state, read_state, write_text_file},
    AppState,
};

pub use use_case::file_exports::FilteredPcapResult;
pub use use_case::ReportConfigInput;

pub async fn export_assets_csv(output_path: String, state: &AppState) -> Result<String, AppError> {
    let inventory = read_state(&state.inventory, "inventory").map_err(AppError::state_lock)?;
    let csv = use_case::export_assets_csv(&inventory.assets).map_err(AppError::invalid_input)?;
    write_text_file(std::path::Path::new(&output_path), &csv).map_err(AppError::IoError)?;
    Ok(output_path)
}

pub async fn export_connections_csv(
    output_path: String,
    state: &AppState,
) -> Result<String, AppError> {
    let capture = read_state(&state.capture, "capture").map_err(AppError::state_lock)?;
    let csv =
        use_case::export_connections_csv(&capture.connections).map_err(AppError::invalid_input)?;
    write_text_file(std::path::Path::new(&output_path), &csv).map_err(AppError::IoError)?;
    Ok(output_path)
}

pub async fn export_topology_json(
    output_path: String,
    state: &AppState,
) -> Result<String, AppError> {
    let capture = read_state(&state.capture, "capture").map_err(AppError::state_lock)?;
    let inventory = read_state(&state.inventory, "inventory").map_err(AppError::state_lock)?;
    let session = mutex_state(&state.session, "session").map_err(AppError::state_lock)?;
    let json = use_case::export_topology_json(
        &inventory.assets,
        &capture.connections,
        session.current_session_name.as_deref(),
    )
    .map_err(AppError::invalid_input)?;
    write_text_file(std::path::Path::new(&output_path), &json).map_err(AppError::IoError)?;
    Ok(output_path)
}

pub async fn export_assets_json(output_path: String, state: &AppState) -> Result<String, AppError> {
    let inventory = read_state(&state.inventory, "inventory").map_err(AppError::state_lock)?;
    let json = use_case::export_assets_json(&inventory.assets).map_err(AppError::invalid_input)?;
    write_text_file(std::path::Path::new(&output_path), &json).map_err(AppError::IoError)?;
    Ok(output_path)
}

pub async fn generate_pdf_report(
    config: ReportConfigInput,
    output_path: String,
    state: &AppState,
) -> Result<String, AppError> {
    let capture = read_state(&state.capture, "capture").map_err(AppError::state_lock)?;
    let inventory = read_state(&state.inventory, "inventory").map_err(AppError::state_lock)?;
    let session = mutex_state(&state.session, "session").map_err(AppError::state_lock)?;
    use_case::generate_pdf_report(
        config,
        output_path,
        &inventory.assets,
        &capture.connections,
        session.current_session_name.as_deref(),
    )
    .map_err(AppError::invalid_input)
}

pub async fn export_sbom(
    format: String,
    output_path: String,
    state: &AppState,
) -> Result<String, AppError> {
    let inventory = read_state(&state.inventory, "inventory").map_err(AppError::state_lock)?;
    let content =
        use_case::export_sbom(format, &inventory.assets).map_err(AppError::invalid_input)?;
    write_text_file(std::path::Path::new(&output_path), &content).map_err(AppError::IoError)?;
    Ok(output_path)
}

pub async fn export_stix_bundle(output_path: String, state: &AppState) -> Result<String, AppError> {
    let capture = read_state(&state.capture, "capture").map_err(AppError::state_lock)?;
    let inventory = read_state(&state.inventory, "inventory").map_err(AppError::state_lock)?;
    let json = use_case::export_stix_bundle(&inventory.assets, &capture.connections)
        .map_err(AppError::invalid_input)?;
    write_text_file(std::path::Path::new(&output_path), &json).map_err(AppError::IoError)?;
    Ok(output_path)
}

pub async fn save_topology_image(
    image_data: String,
    output_path: String,
) -> Result<String, AppError> {
    use_case::save_topology_image(image_data, output_path).map_err(AppError::invalid_input)
}

pub async fn export_filtered_pcap(
    filter_ips: Vec<String>,
    filter_ports: Vec<u16>,
    output_path: String,
    state: &AppState,
) -> Result<FilteredPcapResult, AppError> {
    let input_paths = read_state(&state.capture, "capture")
        .map_err(AppError::state_lock)?
        .imported_files
        .clone();
    use_case::export_filtered_pcap(&input_paths, filter_ips, filter_ports, output_path)
        .map_err(AppError::invalid_input)
}

pub async fn generate_communication_allowlist(
    state: &AppState,
) -> Result<Vec<AllowlistEntry>, AppError> {
    let capture = read_state(&state.capture, "capture").map_err(AppError::state_lock)?;
    let inventory = read_state(&state.inventory, "inventory").map_err(AppError::state_lock)?;
    let analysis = read_state(&state.analysis, "analysis").map_err(AppError::state_lock)?;
    Ok(use_case::generate_communication_allowlist(
        &inventory.assets,
        &capture.connections,
        &analysis.connection_stats,
    ))
}

pub async fn export_allowlist_csv(
    output_path: String,
    state: &AppState,
) -> Result<String, AppError> {
    let entries = generate_communication_allowlist(state).await?;
    let csv = use_case::export_allowlist_csv(&entries);
    write_text_file(std::path::Path::new(&output_path), &csv).map_err(AppError::IoError)?;
    Ok(output_path)
}

pub async fn export_firewall_rules(
    output_path: String,
    state: &AppState,
) -> Result<String, AppError> {
    let entries = generate_communication_allowlist(state).await?;
    let rules = use_case::export_firewall_rules(&entries);
    write_text_file(std::path::Path::new(&output_path), &rules).map_err(AppError::IoError)?;
    Ok(output_path)
}

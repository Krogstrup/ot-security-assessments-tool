//! Adapter layer for export/reporting use-cases.

use gm_analysis::AllowlistEntry;

use crate::application::use_cases::export as use_case;

use super::{
    support::{mutex_state, read_state, write_text_file},
    AppState,
};

pub use use_case::ReportConfigInput;
pub use use_case::file_exports::FilteredPcapResult;

pub async fn export_assets_csv(output_path: String, state: &AppState) -> Result<String, String> {
    let inventory = read_state(&state.inventory, "inventory")?;
    let csv = use_case::export_assets_csv(&inventory.assets)?;
    write_text_file(std::path::Path::new(&output_path), &csv)?;
    Ok(output_path)
}

pub async fn export_connections_csv(
    output_path: String,
    state: &AppState,
) -> Result<String, String> {
    let capture = read_state(&state.capture, "capture")?;
    let csv = use_case::export_connections_csv(&capture.connections)?;
    write_text_file(std::path::Path::new(&output_path), &csv)?;
    Ok(output_path)
}

pub async fn export_topology_json(
    output_path: String,
    state: &AppState,
) -> Result<String, String> {
    let capture = read_state(&state.capture, "capture")?;
    let inventory = read_state(&state.inventory, "inventory")?;
    let session = mutex_state(&state.session, "session")?;
    let json = use_case::export_topology_json(
        &inventory.assets,
        &capture.connections,
        session.current_session_name.as_deref(),
    )?;
    write_text_file(std::path::Path::new(&output_path), &json)?;
    Ok(output_path)
}

pub async fn export_assets_json(output_path: String, state: &AppState) -> Result<String, String> {
    let inventory = read_state(&state.inventory, "inventory")?;
    let json = use_case::export_assets_json(&inventory.assets)?;
    write_text_file(std::path::Path::new(&output_path), &json)?;
    Ok(output_path)
}

pub async fn generate_pdf_report(
    config: ReportConfigInput,
    output_path: String,
    state: &AppState,
) -> Result<String, String> {
    let capture = read_state(&state.capture, "capture")?;
    let inventory = read_state(&state.inventory, "inventory")?;
    let session = mutex_state(&state.session, "session")?;
    use_case::generate_pdf_report(
        config,
        output_path,
        &inventory.assets,
        &capture.connections,
        session.current_session_name.as_deref(),
    )
}

pub async fn export_sbom(
    format: String,
    output_path: String,
    state: &AppState,
) -> Result<String, String> {
    let inventory = read_state(&state.inventory, "inventory")?;
    let content = use_case::export_sbom(format, &inventory.assets)?;
    write_text_file(std::path::Path::new(&output_path), &content)?;
    Ok(output_path)
}

pub async fn export_stix_bundle(
    output_path: String,
    state: &AppState,
) -> Result<String, String> {
    let capture = read_state(&state.capture, "capture")?;
    let inventory = read_state(&state.inventory, "inventory")?;
    let json = use_case::export_stix_bundle(&inventory.assets, &capture.connections)?;
    write_text_file(std::path::Path::new(&output_path), &json)?;
    Ok(output_path)
}

pub async fn save_topology_image(
    image_data: String,
    output_path: String,
) -> Result<String, String> {
    use_case::save_topology_image(image_data, output_path)
}

pub async fn export_filtered_pcap(
    filter_ips: Vec<String>,
    filter_ports: Vec<u16>,
    output_path: String,
    state: &AppState,
) -> Result<FilteredPcapResult, String> {
    let input_paths = read_state(&state.capture, "capture")?
        .imported_files
        .clone();
    use_case::export_filtered_pcap(&input_paths, filter_ips, filter_ports, output_path)
}

pub async fn generate_communication_allowlist(
    state: &AppState,
) -> Result<Vec<AllowlistEntry>, String> {
    let capture = read_state(&state.capture, "capture")?;
    let inventory = read_state(&state.inventory, "inventory")?;
    let analysis = read_state(&state.analysis, "analysis")?;
    Ok(use_case::generate_communication_allowlist(
        &inventory.assets,
        &capture.connections,
        &analysis.connection_stats,
    ))
}

pub async fn export_allowlist_csv(
    output_path: String,
    state: &AppState,
) -> Result<String, String> {
    let entries = generate_communication_allowlist(state).await?;
    let csv = use_case::export_allowlist_csv(&entries);
    write_text_file(std::path::Path::new(&output_path), &csv)?;
    Ok(output_path)
}

pub async fn export_firewall_rules(
    output_path: String,
    state: &AppState,
) -> Result<String, String> {
    let entries = generate_communication_allowlist(state).await?;
    let rules = use_case::export_firewall_rules(&entries);
    write_text_file(std::path::Path::new(&output_path), &rules)?;
    Ok(output_path)
}

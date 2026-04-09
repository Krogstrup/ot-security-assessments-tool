//! File export commands: CSV, JSON, PDF, SBOM, STIX, filtered PCAP, image.

use std::path::Path;

use gm_report::{ExportFinding, ReportConfig};
use serde::Deserialize;

use crate::commands::{
    support::{mutex_state, read_state, write_bytes_file, write_text_file},
    AppState,
};

use super::{
    base64::base64_decode,
    report_builders::{
        build_report_data, compute_protocol_stats, state_assets_to_export,
        state_connections_to_export,
    },
};

// ─── Public types ─────────────────────────────────────────────────────────────

/// Report configuration from the frontend.
#[derive(Debug, Deserialize)]
pub struct ReportConfigInput {
    pub assessor_name: String,
    pub client_name: String,
    pub assessment_date: Option<String>,
    pub title: Option<String>,
    pub include_executive_summary: bool,
    pub include_asset_inventory: bool,
    pub include_protocol_analysis: bool,
    pub include_findings: bool,
    pub include_recommendations: bool,
}

/// Result of a filtered PCAP export operation.
#[derive(serde::Serialize)]
pub struct FilteredPcapResult {
    pub output_path: String,
    pub packets_written: u64,
    pub source_files: usize,
}

// ─── CSV exports ──────────────────────────────────────────────────────────────

pub async fn export_assets_csv(output_path: String, state: &AppState) -> Result<String, String> {
    let inv = read_state(&state.inventory, "inventory")?;
    let assets = state_assets_to_export(&inv);
    let csv = gm_report::csv_export::assets_to_csv(&assets).map_err(|e| e.to_string())?;
    gm_report::csv_export::write_csv_file(&output_path, &csv).map_err(|e| e.to_string())?;
    log::info!("Exported {} assets to CSV: {}", assets.len(), output_path);
    Ok(output_path)
}

pub async fn export_connections_csv(
    output_path: String,
    state: &AppState,
) -> Result<String, String> {
    let cap = read_state(&state.capture, "capture")?;
    let connections = state_connections_to_export(&cap);
    let csv =
        gm_report::csv_export::connections_to_csv(&connections).map_err(|e| e.to_string())?;
    gm_report::csv_export::write_csv_file(&output_path, &csv).map_err(|e| e.to_string())?;
    log::info!(
        "Exported {} connections to CSV: {}",
        connections.len(),
        output_path
    );
    Ok(output_path)
}

// ─── JSON exports ─────────────────────────────────────────────────────────────

pub async fn export_topology_json(
    output_path: String,
    state: &AppState,
) -> Result<String, String> {
    let capture = read_state(&state.capture, "capture")?;
    let inventory = read_state(&state.inventory, "inventory")?;
    let session = mutex_state(&state.session, "session")?;

    let assets = state_assets_to_export(&inventory);
    let connections = state_connections_to_export(&capture);
    let stats = compute_protocol_stats(&capture);
    let session_name = session.current_session_name.as_deref();

    let json =
        gm_report::json_export::topology_to_json(&assets, &connections, &stats, session_name)
            .map_err(|e| e.to_string())?;
    gm_report::json_export::write_json_file(&output_path, &json).map_err(|e| e.to_string())?;
    log::info!("Exported topology JSON to: {}", output_path);
    Ok(output_path)
}

pub async fn export_assets_json(output_path: String, state: &AppState) -> Result<String, String> {
    let inv = read_state(&state.inventory, "inventory")?;
    let assets = state_assets_to_export(&inv);
    let json = gm_report::json_export::assets_to_json(&assets).map_err(|e| e.to_string())?;
    gm_report::json_export::write_json_file(&output_path, &json).map_err(|e| e.to_string())?;
    log::info!("Exported {} assets to JSON: {}", assets.len(), output_path);
    Ok(output_path)
}

// ─── PDF report ───────────────────────────────────────────────────────────────

pub async fn generate_pdf_report(
    config: ReportConfigInput,
    output_path: String,
    state: &AppState,
) -> Result<String, String> {
    let capture = read_state(&state.capture, "capture")?;
    let inventory = read_state(&state.inventory, "inventory")?;
    let session_name = mutex_state(&state.session, "session")?
        .current_session_name
        .clone();

    let data = build_report_data(&capture, &inventory, session_name.as_deref());

    let report_config = ReportConfig {
        assessor_name: config.assessor_name,
        client_name: config.client_name,
        assessment_date: config
            .assessment_date
            .unwrap_or_else(|| chrono::Utc::now().format("%Y-%m-%d").to_string()),
        title: config.title,
        include_executive_summary: config.include_executive_summary,
        include_asset_inventory: config.include_asset_inventory,
        include_protocol_analysis: config.include_protocol_analysis,
        include_findings: config.include_findings,
        include_recommendations: config.include_recommendations,
    };

    gm_report::pdf::generate_report(&report_config, &data, &output_path)
        .map_err(|e| e.to_string())?;

    log::info!("Generated PDF report: {}", output_path);
    Ok(output_path)
}

// ─── SBOM export ──────────────────────────────────────────────────────────────

pub async fn export_sbom(
    format: String,
    output_path: String,
    state: &AppState,
) -> Result<String, String> {
    let inv = read_state(&state.inventory, "inventory")?;
    let assets = state_assets_to_export(&inv);
    let entries = gm_report::sbom::assets_to_sbom(&assets);

    let content = match format.as_str() {
        "csv" => gm_report::sbom::sbom_to_csv(&entries).map_err(|e| e.to_string())?,
        "json" => gm_report::sbom::sbom_to_json(&entries).map_err(|e| e.to_string())?,
        _ => {
            return Err(format!(
                "Unsupported SBOM format: {}. Use 'csv' or 'json'.",
                format
            ))
        }
    };

    write_text_file(Path::new(&output_path), &content)?;
    log::info!(
        "Exported SBOM ({}) with {} entries to: {}",
        format,
        entries.len(),
        output_path
    );
    Ok(output_path)
}

// ─── STIX export ─────────────────────────────────────────────────────────────

pub async fn export_stix_bundle(
    output_path: String,
    state: &AppState,
) -> Result<String, String> {
    let capture = read_state(&state.capture, "capture")?;
    let inventory = read_state(&state.inventory, "inventory")?;

    let assets = state_assets_to_export(&inventory);
    let connections = state_connections_to_export(&capture);
    let findings: Vec<ExportFinding> = Vec::new();

    let json = gm_report::stix::generate_stix_bundle(&assets, &connections, &findings)
        .map_err(|e| e.to_string())?;
    write_text_file(Path::new(&output_path), &json)?;

    log::info!("Exported STIX 2.1 bundle to: {}", output_path);
    Ok(output_path)
}

// ─── Filtered PCAP export ────────────────────────────────────────────────────

pub async fn export_filtered_pcap(
    filter_ips: Vec<String>,
    filter_ports: Vec<u16>,
    output_path: String,
    state: &AppState,
) -> Result<FilteredPcapResult, String> {
    let input_paths = read_state(&state.capture, "capture")?
        .imported_files
        .clone();

    let source_files = input_paths.iter().filter(|p| !p.starts_with('[')).count();

    if source_files == 0 {
        return Err("No PCAP files have been imported to export from.".to_string());
    }

    let packets_written =
        gm_capture::filter_export_pcap(&input_paths, &filter_ips, &filter_ports, &output_path)
            .map_err(|e| e.to_string())?;

    log::info!(
        "Filtered PCAP export: {} packets from {} files → {}",
        packets_written,
        source_files,
        output_path
    );

    Ok(FilteredPcapResult {
        output_path,
        packets_written,
        source_files,
    })
}

// ─── Topology image export ───────────────────────────────────────────────────

pub async fn save_topology_image(
    image_data: String,
    output_path: String,
) -> Result<String, String> {
    if let Some(base64_data) = image_data.strip_prefix("data:image/png;base64,") {
        let bytes =
            base64_decode(base64_data).map_err(|e| format!("Invalid base64 data: {}", e))?;
        write_bytes_file(Path::new(&output_path), &bytes)?;
    } else if image_data.starts_with("<?xml") || image_data.starts_with("<svg") {
        write_text_file(Path::new(&output_path), &image_data)?;
    } else {
        let bytes =
            base64_decode(&image_data).map_err(|e| format!("Invalid image data: {}", e))?;
        write_bytes_file(Path::new(&output_path), &bytes)?;
    }

    log::info!("Saved topology image to: {}", output_path);
    Ok(output_path)
}

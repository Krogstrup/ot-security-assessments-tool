//! Export rendering use-cases: CSV, JSON, PDF, SBOM, STIX, filtered PCAP, image.

use std::path::Path;

use gm_report::{ExportFinding, ReportConfig};
use gm_types::{AssetInfo, ConnectionInfo};
use serde::Deserialize;

use super::{
    base64::base64_decode,
    report_builders::{assets_to_export, build_report_data, compute_protocol_stats, connections_to_export},
};

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

pub fn export_assets_csv(assets: &[AssetInfo]) -> Result<String, String> {
    let assets = assets_to_export(assets);
    gm_report::csv_export::assets_to_csv(&assets).map_err(|e| e.to_string())
}

pub fn export_connections_csv(connections: &[ConnectionInfo]) -> Result<String, String> {
    let connections = connections_to_export(connections);
    gm_report::csv_export::connections_to_csv(&connections).map_err(|e| e.to_string())
}

pub fn export_topology_json(
    assets: &[AssetInfo],
    connections: &[ConnectionInfo],
    session_name: Option<&str>,
) -> Result<String, String> {
    let assets = assets_to_export(assets);
    let export_connections = connections_to_export(connections);
    let stats = compute_protocol_stats(connections);
    gm_report::json_export::topology_to_json(&assets, &export_connections, &stats, session_name)
        .map_err(|e| e.to_string())
}

pub fn export_assets_json(assets: &[AssetInfo]) -> Result<String, String> {
    let assets = assets_to_export(assets);
    gm_report::json_export::assets_to_json(&assets).map_err(|e| e.to_string())
}

pub fn generate_pdf_report(
    config: ReportConfigInput,
    output_path: String,
    assets: &[AssetInfo],
    connections: &[ConnectionInfo],
    session_name: Option<&str>,
) -> Result<String, String> {
    let data = build_report_data(assets, connections, session_name);

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

    Ok(output_path)
}

pub fn export_sbom(format: String, assets: &[AssetInfo]) -> Result<String, String> {
    let assets = assets_to_export(assets);
    let entries = gm_report::sbom::assets_to_sbom(&assets);

    match format.as_str() {
        "csv" => gm_report::sbom::sbom_to_csv(&entries).map_err(|e| e.to_string()),
        "json" => gm_report::sbom::sbom_to_json(&entries).map_err(|e| e.to_string()),
        _ => Err(format!(
            "Unsupported SBOM format: {}. Use 'csv' or 'json'.",
            format
        )),
    }
}

pub fn export_stix_bundle(
    assets: &[AssetInfo],
    connections: &[ConnectionInfo],
) -> Result<String, String> {
    let assets = assets_to_export(assets);
    let connections = connections_to_export(connections);
    let findings: Vec<ExportFinding> = Vec::new();
    gm_report::stix::generate_stix_bundle(&assets, &connections, &findings).map_err(|e| e.to_string())
}

pub fn export_filtered_pcap(
    input_paths: &[String],
    filter_ips: Vec<String>,
    filter_ports: Vec<u16>,
    output_path: String,
) -> Result<FilteredPcapResult, String> {
    let source_files = input_paths.iter().filter(|p| !p.starts_with('[')).count();
    if source_files == 0 {
        return Err("No PCAP files have been imported to export from.".to_string());
    }

    let packets_written =
        gm_capture::filter_export_pcap(input_paths, &filter_ips, &filter_ports, &output_path)
            .map_err(|e| e.to_string())?;

    Ok(FilteredPcapResult {
        output_path,
        packets_written,
        source_files,
    })
}

pub fn save_topology_image(image_data: String, output_path: String) -> Result<String, String> {
    if let Some(base64_data) = image_data.strip_prefix("data:image/png;base64,") {
        let bytes =
            base64_decode(base64_data).map_err(|e| format!("Invalid base64 data: {}", e))?;
        std::fs::write(Path::new(&output_path), bytes).map_err(|e| e.to_string())?;
    } else if image_data.starts_with("<?xml") || image_data.starts_with("<svg") {
        std::fs::write(Path::new(&output_path), image_data).map_err(|e| e.to_string())?;
    } else {
        let bytes =
            base64_decode(&image_data).map_err(|e| format!("Invalid image data: {}", e))?;
        std::fs::write(Path::new(&output_path), bytes).map_err(|e| e.to_string())?;
    }

    Ok(output_path)
}

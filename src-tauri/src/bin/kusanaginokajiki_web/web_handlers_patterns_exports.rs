use axum::extract::{Path, State};
use axum::Json;
use serde_json::{json, Value};

use super::web_requests::{
    ExportEnforcementConfigRequest, ExportFilteredPcapRequest, ExportSbomRequest,
    GeneratePdfReportRequest, OutputPathRequest, SaveSettingsRequest, SaveTopologyImageRequest,
};
use super::web_runtime::to_json;
use super::web_support::{resolve_export_output_path, ApiError};
use super::SharedState;
use crate::commands;

pub(super) async fn get_settings_handler() -> Result<Json<Value>, ApiError> {
    to_json(commands::system::get_settings().map_err(ApiError::bad_request)?)
}

pub(super) async fn save_settings_handler(
    Json(body): Json<SaveSettingsRequest>,
) -> Result<Json<Value>, ApiError> {
    commands::system::save_settings(body.settings).map_err(ApiError::bad_request)?;
    Ok(Json(json!({})))
}

pub(super) async fn list_plugins_handler() -> Result<Json<Value>, ApiError> {
    to_json(commands::system::list_plugins().map_err(ApiError::bad_request)?)
}

pub(super) async fn get_connection_stats_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::patterns::get_connection_stats(state.as_ref()).map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn get_pattern_anomalies_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::patterns::get_pattern_anomalies(state.as_ref()).map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn get_redundancy_protocols_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::patterns::get_redundancy_protocols(state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn get_correlated_alerts_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::correlation::get_correlated_alerts(state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn get_alerts_for_ip_handler(
    Path(ip): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::correlation::get_alerts_for_ip(ip, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn clear_alerts_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    commands::correlation::clear_alerts(state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    Ok(Json(json!({})))
}

pub(super) async fn export_assets_csv_handler(
    State(state): State<SharedState>,
    Json(body): Json<OutputPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let output_path = resolve_export_output_path(&body.output_path, "assets.csv")?;
    to_json(
        commands::export::export_assets_csv(output_path, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn export_connections_csv_handler(
    State(state): State<SharedState>,
    Json(body): Json<OutputPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let output_path = resolve_export_output_path(&body.output_path, "connections.csv")?;
    to_json(
        commands::export::export_connections_csv(output_path, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn export_topology_json_handler(
    State(state): State<SharedState>,
    Json(body): Json<OutputPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let output_path = resolve_export_output_path(&body.output_path, "topology.json")?;
    to_json(
        commands::export::export_topology_json(output_path, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn export_assets_json_handler(
    State(state): State<SharedState>,
    Json(body): Json<OutputPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let output_path = resolve_export_output_path(&body.output_path, "assets.json")?;
    to_json(
        commands::export::export_assets_json(output_path, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn generate_pdf_report_handler(
    State(state): State<SharedState>,
    Json(body): Json<GeneratePdfReportRequest>,
) -> Result<Json<Value>, ApiError> {
    let output_path = resolve_export_output_path(&body.output_path, "assessment_report.pdf")?;
    to_json(
        commands::export::generate_pdf_report(body.config, output_path, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn export_sbom_handler(
    State(state): State<SharedState>,
    Json(body): Json<ExportSbomRequest>,
) -> Result<Json<Value>, ApiError> {
    let fallback_name = if body.format == "csv" {
        "sbom.csv"
    } else {
        "sbom.json"
    };
    let output_path = resolve_export_output_path(&body.output_path, fallback_name)?;
    to_json(
        commands::export::export_sbom(body.format, output_path, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn export_stix_bundle_handler(
    State(state): State<SharedState>,
    Json(body): Json<OutputPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let output_path = resolve_export_output_path(&body.output_path, "stix_bundle.json")?;
    to_json(
        commands::export::export_stix_bundle(output_path, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn save_topology_image_handler(
    Json(body): Json<SaveTopologyImageRequest>,
) -> Result<Json<Value>, ApiError> {
    let output_path = resolve_export_output_path(&body.output_path, "topology.png")?;
    to_json(
        commands::export::save_topology_image(body.image_data, output_path)
            .await
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn export_filtered_pcap_handler(
    State(state): State<SharedState>,
    Json(body): Json<ExportFilteredPcapRequest>,
) -> Result<Json<Value>, ApiError> {
    let output_path = resolve_export_output_path(&body.output_path, "filtered.pcap")?;
    to_json(
        commands::export::export_filtered_pcap(
            body.filter_ips,
            body.filter_ports,
            output_path,
            state.as_ref(),
        )
        .await
        .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn generate_communication_allowlist_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::export::generate_communication_allowlist(state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn export_allowlist_csv_handler(
    State(state): State<SharedState>,
    Json(body): Json<OutputPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let output_path = resolve_export_output_path(&body.output_path, "allowlist.csv")?;
    to_json(
        commands::export::export_allowlist_csv(output_path, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn export_firewall_rules_handler(
    State(state): State<SharedState>,
    Json(body): Json<OutputPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let output_path = resolve_export_output_path(&body.output_path, "firewall_rules.txt")?;
    to_json(
        commands::export::export_firewall_rules(output_path, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn run_segmentation_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::segmentation::run_segmentation(state.as_ref()).map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn export_enforcement_config_handler(
    State(state): State<SharedState>,
    Json(body): Json<ExportEnforcementConfigRequest>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::segmentation::export_enforcement_config(body.format, state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

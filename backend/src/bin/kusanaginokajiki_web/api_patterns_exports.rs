use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

use super::web_api_paths::signatures_patterns_correlation as spc_path;
use super::web_api_paths::system_exports_segmentation as ses_path;
use super::web_runtime::to_json;
use super::web_support::{resolve_export_output_path, ApiError};
use super::SharedState;
use crate::commands;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TestSignatureRequest {
    yaml: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SaveSettingsRequest {
    settings: commands::system::UserSettings,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OutputPathRequest {
    output_path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeneratePdfReportRequest {
    config: commands::export::ReportConfigInput,
    output_path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExportSbomRequest {
    format: String,
    output_path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SaveTopologyImageRequest {
    image_data: String,
    output_path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExportFilteredPcapRequest {
    filter_ips: Vec<String>,
    filter_ports: Vec<u16>,
    output_path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExportEnforcementConfigRequest {
    format: String,
}

pub(super) fn add_routes(router: Router<SharedState>) -> Router<SharedState> {
    router
        .route(spc_path::V1_SIGNATURES, get(get_signatures))
        .route(spc_path::V1_SIGNATURES_RELOAD, post(reload_signatures))
        .route(spc_path::V1_SIGNATURES_TEST, post(test_signature))
        .route(
            spc_path::V1_PATTERNS_CONNECTION_STATS,
            get(get_connection_stats),
        )
        .route(spc_path::V1_PATTERNS_ANOMALIES, get(get_pattern_anomalies))
        .route(
            spc_path::V1_PATTERNS_REDUNDANCY_PROTOCOLS,
            get(get_redundancy_protocols),
        )
        .route(
            spc_path::V1_CORRELATION_ALERTS,
            get(get_correlated_alerts).delete(clear_alerts),
        )
        .route(
            spc_path::V1_CORRELATION_ALERTS_BY_IP,
            get(get_alerts_for_ip),
        )
        .route(
            ses_path::V1_SYSTEM_SETTINGS,
            get(get_settings).put(save_settings),
        )
        .route(ses_path::V1_SYSTEM_PLUGINS, get(list_plugins))
        .route(ses_path::V1_EXPORTS_ASSETS_CSV, post(export_assets_csv))
        .route(
            ses_path::V1_EXPORTS_CONNECTIONS_CSV,
            post(export_connections_csv),
        )
        .route(
            ses_path::V1_EXPORTS_TOPOLOGY_JSON,
            post(export_topology_json),
        )
        .route(ses_path::V1_EXPORTS_ASSETS_JSON, post(export_assets_json))
        .route(ses_path::V1_EXPORTS_REPORT_PDF, post(generate_pdf_report))
        .route(ses_path::V1_EXPORTS_SBOM, post(export_sbom))
        .route(ses_path::V1_EXPORTS_STIX, post(export_stix_bundle))
        .route(
            ses_path::V1_EXPORTS_TOPOLOGY_IMAGE,
            post(save_topology_image),
        )
        .route(
            ses_path::V1_EXPORTS_PCAP_FILTERED,
            post(export_filtered_pcap),
        )
        .route(
            ses_path::V1_EXPORTS_ALLOWLIST,
            get(generate_communication_allowlist),
        )
        .route(
            ses_path::V1_EXPORTS_ALLOWLIST_CSV,
            post(export_allowlist_csv),
        )
        .route(
            ses_path::V1_EXPORTS_FIREWALL_RULES,
            post(export_firewall_rules),
        )
        .route(ses_path::V1_SEGMENTATION_RUN, post(run_segmentation))
        .route(
            ses_path::V1_SEGMENTATION_ENFORCEMENT_CONFIG,
            post(export_enforcement_config),
        )
}

async fn get_signatures(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    to_json(commands::signatures::get_signatures(state.as_ref()).map_err(ApiError::bad_request)?)
}

async fn reload_signatures(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    to_json(commands::signatures::reload_signatures(state.as_ref()).map_err(ApiError::bad_request)?)
}

async fn test_signature(
    State(state): State<SharedState>,
    Json(body): Json<TestSignatureRequest>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::signatures::test_signature(body.yaml, state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

async fn get_settings() -> Result<Json<Value>, ApiError> {
    to_json(commands::system::get_settings().map_err(ApiError::bad_request)?)
}

async fn save_settings(Json(body): Json<SaveSettingsRequest>) -> Result<Json<Value>, ApiError> {
    commands::system::save_settings(body.settings).map_err(ApiError::bad_request)?;
    Ok(Json(json!({})))
}

async fn list_plugins() -> Result<Json<Value>, ApiError> {
    to_json(commands::system::list_plugins().map_err(ApiError::bad_request)?)
}

async fn get_connection_stats(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    to_json(commands::analysis::get_connection_stats(state.as_ref()).map_err(ApiError::from)?)
}

async fn get_pattern_anomalies(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    to_json(commands::analysis::get_pattern_anomalies(state.as_ref()).map_err(ApiError::from)?)
}

async fn get_redundancy_protocols(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(commands::analysis::get_redundancy_protocols(state.as_ref()).map_err(ApiError::from)?)
}

async fn get_correlated_alerts(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::correlation::get_correlated_alerts(state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

async fn get_alerts_for_ip(
    Path(ip): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::correlation::get_alerts_for_ip(ip, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

async fn clear_alerts(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    commands::correlation::clear_alerts(state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    Ok(Json(json!({})))
}

async fn export_assets_csv(
    State(state): State<SharedState>,
    Json(body): Json<OutputPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let output_path = resolve_export_output_path(&body.output_path, "assets.csv")?;
    to_json(
        commands::export::export_assets_csv(output_path, state.as_ref())
            .await
            .map_err(ApiError::from)?,
    )
}

async fn export_connections_csv(
    State(state): State<SharedState>,
    Json(body): Json<OutputPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let output_path = resolve_export_output_path(&body.output_path, "connections.csv")?;
    to_json(
        commands::export::export_connections_csv(output_path, state.as_ref())
            .await
            .map_err(ApiError::from)?,
    )
}

async fn export_topology_json(
    State(state): State<SharedState>,
    Json(body): Json<OutputPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let output_path = resolve_export_output_path(&body.output_path, "topology.json")?;
    to_json(
        commands::export::export_topology_json(output_path, state.as_ref())
            .await
            .map_err(ApiError::from)?,
    )
}

async fn export_assets_json(
    State(state): State<SharedState>,
    Json(body): Json<OutputPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let output_path = resolve_export_output_path(&body.output_path, "assets.json")?;
    to_json(
        commands::export::export_assets_json(output_path, state.as_ref())
            .await
            .map_err(ApiError::from)?,
    )
}

async fn generate_pdf_report(
    State(state): State<SharedState>,
    Json(body): Json<GeneratePdfReportRequest>,
) -> Result<Json<Value>, ApiError> {
    let output_path = resolve_export_output_path(&body.output_path, "assessment_report.pdf")?;
    to_json(
        commands::export::generate_pdf_report(body.config, output_path, state.as_ref())
            .await
            .map_err(ApiError::from)?,
    )
}

async fn export_sbom(
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
            .map_err(ApiError::from)?,
    )
}

async fn export_stix_bundle(
    State(state): State<SharedState>,
    Json(body): Json<OutputPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let output_path = resolve_export_output_path(&body.output_path, "stix_bundle.json")?;
    to_json(
        commands::export::export_stix_bundle(output_path, state.as_ref())
            .await
            .map_err(ApiError::from)?,
    )
}

async fn save_topology_image(
    Json(body): Json<SaveTopologyImageRequest>,
) -> Result<Json<Value>, ApiError> {
    let output_path = resolve_export_output_path(&body.output_path, "topology.png")?;
    to_json(
        commands::export::save_topology_image(body.image_data, output_path)
            .await
            .map_err(ApiError::from)?,
    )
}

async fn export_filtered_pcap(
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
        .map_err(ApiError::from)?,
    )
}

async fn generate_communication_allowlist(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::export::generate_communication_allowlist(state.as_ref())
            .await
            .map_err(ApiError::from)?,
    )
}

async fn export_allowlist_csv(
    State(state): State<SharedState>,
    Json(body): Json<OutputPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let output_path = resolve_export_output_path(&body.output_path, "allowlist.csv")?;
    to_json(
        commands::export::export_allowlist_csv(output_path, state.as_ref())
            .await
            .map_err(ApiError::from)?,
    )
}

async fn export_firewall_rules(
    State(state): State<SharedState>,
    Json(body): Json<OutputPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let output_path = resolve_export_output_path(&body.output_path, "firewall_rules.txt")?;
    to_json(
        commands::export::export_firewall_rules(output_path, state.as_ref())
            .await
            .map_err(ApiError::from)?,
    )
}

async fn run_segmentation(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    to_json(commands::segmentation::run_segmentation(state.as_ref()).map_err(ApiError::from)?)
}

async fn export_enforcement_config(
    State(state): State<SharedState>,
    Json(body): Json<ExportEnforcementConfigRequest>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::segmentation::export_enforcement_config(body.format, state.as_ref())
            .map_err(ApiError::from)?,
    )
}

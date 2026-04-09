use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

use super::web_api_paths::ingest_wireshark as ingest_wireshark_path;
use super::web_api_paths::physical as physical_path;
use super::web_runtime::to_json;
use super::web_support::{
    resolve_export_output_path, resolve_import_input_path, ApiError, ImportKind,
};
use super::SharedState;
use crate::commands;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImportPathRequest {
    path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImportZeekRequest {
    paths: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImportSwitchPathRequest {
    path: String,
    switch_hostname: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConnectionIdRequest {
    connection_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IpAddressRequest {
    ip_address: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SaveFramesCsvRequest {
    output_path: String,
}

pub(super) fn add_routes(router: Router<SharedState>) -> Router<SharedState> {
    router
        .route(
            physical_path::V1_PHYSICAL_TOPOLOGY,
            get(get_physical_topology).delete(clear_physical_topology),
        )
        .route(
            physical_path::V1_PHYSICAL_CISCO_CONFIG,
            post(import_cisco_config),
        )
        .route(physical_path::V1_PHYSICAL_MAC_TABLE, post(import_mac_table))
        .route(
            physical_path::V1_PHYSICAL_CDP_NEIGHBORS,
            post(import_cdp_neighbors),
        )
        .route(physical_path::V1_PHYSICAL_ARP_TABLE, post(import_arp_table))
        .route(
            physical_path::V1_PHYSICAL_NETWORK_CONFIG,
            post(import_network_config),
        )
        .route(
            physical_path::V1_PHYSICAL_MAC_TABLE_AUTO,
            post(import_mac_table_auto),
        )
        .route(
            physical_path::V1_PHYSICAL_NEIGHBOR_TABLE,
            post(import_neighbor_table),
        )
        .route(
            physical_path::V1_PHYSICAL_INFERENCE_RUN,
            post(run_topology_inference),
        )
        .route(
            physical_path::V1_PHYSICAL_INFERENCE,
            get(get_inferred_topology),
        )
        .route(ingest_wireshark_path::V1_INGEST_ZEEK, post(import_zeek_logs))
        .route(
            ingest_wireshark_path::V1_INGEST_SURICATA,
            post(import_suricata_eve),
        )
        .route(ingest_wireshark_path::V1_INGEST_NMAP, post(import_nmap_xml))
        .route(
            ingest_wireshark_path::V1_INGEST_MASSCAN,
            post(import_masscan_json),
        )
        .route(
            ingest_wireshark_path::V1_INGEST_WAZUH,
            post(import_wazuh_alerts),
        )
        .route(
            ingest_wireshark_path::V1_INGEST_SINEMA,
            post(import_sinema_csv),
        )
        .route(ingest_wireshark_path::V1_INGEST_TIA, post(import_tia_xml))
        .route(
            ingest_wireshark_path::V1_INGEST_ZEEK_DEVICE_EVENTS_BY_IP,
            get(get_device_zeek_events),
        )
        .route(ingest_wireshark_path::V1_WIRESHARK_INFO, get(detect_wireshark))
        .route(
            ingest_wireshark_path::V1_WIRESHARK_OPEN_CONNECTION,
            post(open_in_wireshark),
        )
        .route(
            ingest_wireshark_path::V1_WIRESHARK_OPEN_NODE,
            post(open_wireshark_for_node),
        )
        .route(
            ingest_wireshark_path::V1_WIRESHARK_FRAMES_BY_CONNECTION_ID,
            get(get_connection_frames),
        )
        .route(
            ingest_wireshark_path::V1_WIRESHARK_FRAMES_CSV_BY_CONNECTION_ID,
            get(export_frames_csv).post(save_frames_csv),
        )
}

async fn import_cisco_config(
    State(state): State<SharedState>,
    Json(body): Json<ImportPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let path = resolve_import_input_path(&body.path, ImportKind::PhysicalConfig)?;
    to_json(
        commands::physical::import_cisco_config(path, state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

async fn import_mac_table(
    State(state): State<SharedState>,
    Json(body): Json<ImportSwitchPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let path = resolve_import_input_path(&body.path, ImportKind::PhysicalMac)?;
    to_json(
        commands::physical::import_mac_table(path, body.switch_hostname, state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

async fn import_cdp_neighbors(
    State(state): State<SharedState>,
    Json(body): Json<ImportSwitchPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let path = resolve_import_input_path(&body.path, ImportKind::PhysicalNeighbor)?;
    to_json(
        commands::physical::import_cdp_neighbors(path, body.switch_hostname, state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

async fn import_arp_table(
    State(state): State<SharedState>,
    Json(body): Json<ImportPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let path = resolve_import_input_path(&body.path, ImportKind::PhysicalArp)?;
    to_json(
        commands::physical::import_arp_table(path, state.as_ref()).map_err(ApiError::bad_request)?,
    )
}

async fn get_physical_topology(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::physical::get_physical_topology(state.as_ref()).map_err(ApiError::bad_request)?,
    )
}

async fn clear_physical_topology(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    commands::physical::clear_physical_topology(state.as_ref()).map_err(ApiError::bad_request)?;
    Ok(Json(json!({})))
}

async fn import_network_config(
    State(state): State<SharedState>,
    Json(body): Json<ImportPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let path = resolve_import_input_path(&body.path, ImportKind::PhysicalConfig)?;
    to_json(
        commands::physical::import_network_config(path, state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

async fn import_mac_table_auto(
    State(state): State<SharedState>,
    Json(body): Json<ImportSwitchPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let path = resolve_import_input_path(&body.path, ImportKind::PhysicalMac)?;
    to_json(
        commands::physical::import_mac_table_auto(path, body.switch_hostname, state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

async fn import_neighbor_table(
    State(state): State<SharedState>,
    Json(body): Json<ImportSwitchPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let path = resolve_import_input_path(&body.path, ImportKind::PhysicalNeighbor)?;
    to_json(
        commands::physical::import_neighbor_table(path, body.switch_hostname, state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

async fn run_topology_inference(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::physical::run_topology_inference(state.as_ref()).map_err(ApiError::bad_request)?,
    )
}

async fn get_inferred_topology(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::physical::get_inferred_topology(state.as_ref()).map_err(ApiError::bad_request)?,
    )
}

async fn import_zeek_logs(
    State(state): State<SharedState>,
    Json(body): Json<ImportZeekRequest>,
) -> Result<Json<Value>, ApiError> {
    let paths = body
        .paths
        .iter()
        .map(|path| resolve_import_input_path(path, ImportKind::Zeek))
        .collect::<Result<Vec<_>, _>>()?;
    to_json(
        commands::ingest::import_zeek_logs(paths, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

async fn import_suricata_eve(
    State(state): State<SharedState>,
    Json(body): Json<ImportPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let path = resolve_import_input_path(&body.path, ImportKind::Suricata)?;
    to_json(
        commands::ingest::import_suricata_eve(path, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

async fn import_nmap_xml(
    State(state): State<SharedState>,
    Json(body): Json<ImportPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let path = resolve_import_input_path(&body.path, ImportKind::Nmap)?;
    to_json(
        commands::ingest::import_nmap_xml(path, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

async fn import_masscan_json(
    State(state): State<SharedState>,
    Json(body): Json<ImportPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let path = resolve_import_input_path(&body.path, ImportKind::Masscan)?;
    to_json(
        commands::ingest::import_masscan_json(path, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

async fn import_wazuh_alerts(
    State(state): State<SharedState>,
    Json(body): Json<ImportPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let path = resolve_import_input_path(&body.path, ImportKind::Wazuh)?;
    to_json(
        commands::ingest::import_wazuh_alerts(path, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

async fn import_sinema_csv(
    State(state): State<SharedState>,
    Json(body): Json<ImportPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let path = resolve_import_input_path(&body.path, ImportKind::Sinema)?;
    to_json(
        commands::ingest::import_sinema_csv(path, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

async fn import_tia_xml(
    State(state): State<SharedState>,
    Json(body): Json<ImportPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let path = resolve_import_input_path(&body.path, ImportKind::Tia)?;
    to_json(
        commands::ingest::import_tia_xml(path, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

async fn get_device_zeek_events(
    Path(device_ip): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::ingest::get_device_zeek_events(device_ip, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

async fn detect_wireshark() -> Result<Json<Value>, ApiError> {
    to_json(
        commands::wireshark::detect_wireshark()
            .await
            .map_err(ApiError::bad_request)?,
    )
}

async fn open_in_wireshark(
    State(state): State<SharedState>,
    Json(body): Json<ConnectionIdRequest>,
) -> Result<Json<Value>, ApiError> {
    commands::wireshark::open_in_wireshark(body.connection_id, state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    Ok(Json(json!({})))
}

async fn open_wireshark_for_node(
    Json(body): Json<IpAddressRequest>,
) -> Result<Json<Value>, ApiError> {
    commands::wireshark::open_wireshark_for_node(body.ip_address)
        .await
        .map_err(ApiError::bad_request)?;
    Ok(Json(json!({})))
}

async fn get_connection_frames(
    Path(connection_id): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::wireshark::get_connection_frames(connection_id, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

async fn export_frames_csv(
    Path(connection_id): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::wireshark::export_frames_csv(connection_id, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

async fn save_frames_csv(
    Path(connection_id): Path<String>,
    State(state): State<SharedState>,
    Json(body): Json<SaveFramesCsvRequest>,
) -> Result<Json<Value>, ApiError> {
    let output_path = resolve_export_output_path(&body.output_path, "frames.csv")?;
    commands::wireshark::save_frames_csv(connection_id, output_path, state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    Ok(Json(json!({})))
}

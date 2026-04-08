use axum::extract::{Path, State};
use axum::Json;
use serde_json::{json, Value};

use super::web_requests::{
    ConnectionIdRequest, ImportPathRequest, ImportSwitchPathRequest, ImportZeekRequest,
    IpAddressRequest, SaveFramesCsvRequest,
};
use super::web_runtime::to_json;
use super::web_support::{
    resolve_export_output_path, resolve_import_input_path, ApiError, ImportKind,
};
use super::SharedState;
use crate::commands;

pub(super) async fn import_cisco_config_handler(
    State(state): State<SharedState>,
    Json(body): Json<ImportPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let path = resolve_import_input_path(&body.path, ImportKind::PhysicalConfig)?;
    to_json(
        commands::physical::import_cisco_config(path, state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn import_mac_table_handler(
    State(state): State<SharedState>,
    Json(body): Json<ImportSwitchPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let path = resolve_import_input_path(&body.path, ImportKind::PhysicalMac)?;
    to_json(
        commands::physical::import_mac_table(path, body.switch_hostname, state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn import_cdp_neighbors_handler(
    State(state): State<SharedState>,
    Json(body): Json<ImportSwitchPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let path = resolve_import_input_path(&body.path, ImportKind::PhysicalNeighbor)?;
    to_json(
        commands::physical::import_cdp_neighbors(path, body.switch_hostname, state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn import_arp_table_handler(
    State(state): State<SharedState>,
    Json(body): Json<ImportPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let path = resolve_import_input_path(&body.path, ImportKind::PhysicalArp)?;
    to_json(
        commands::physical::import_arp_table(path, state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn get_physical_topology_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::physical::get_physical_topology(state.as_ref()).map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn clear_physical_topology_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    commands::physical::clear_physical_topology(state.as_ref()).map_err(ApiError::bad_request)?;
    Ok(Json(json!({})))
}

pub(super) async fn import_network_config_handler(
    State(state): State<SharedState>,
    Json(body): Json<ImportPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let path = resolve_import_input_path(&body.path, ImportKind::PhysicalConfig)?;
    to_json(
        commands::physical::import_network_config(path, state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn import_mac_table_auto_handler(
    State(state): State<SharedState>,
    Json(body): Json<ImportSwitchPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let path = resolve_import_input_path(&body.path, ImportKind::PhysicalMac)?;
    to_json(
        commands::physical::import_mac_table_auto(path, body.switch_hostname, state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn import_neighbor_table_handler(
    State(state): State<SharedState>,
    Json(body): Json<ImportSwitchPathRequest>,
) -> Result<Json<Value>, ApiError> {
    let path = resolve_import_input_path(&body.path, ImportKind::PhysicalNeighbor)?;
    to_json(
        commands::physical::import_neighbor_table(path, body.switch_hostname, state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn run_topology_inference_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::physical::run_topology_inference(state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn get_inferred_topology_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::physical::get_inferred_topology(state.as_ref()).map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn import_zeek_logs_handler(
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

pub(super) async fn import_suricata_eve_handler(
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

pub(super) async fn import_nmap_xml_handler(
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

pub(super) async fn import_masscan_json_handler(
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

pub(super) async fn import_wazuh_alerts_handler(
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

pub(super) async fn import_sinema_csv_handler(
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

pub(super) async fn import_tia_xml_handler(
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

pub(super) async fn get_device_zeek_events_handler(
    Path(device_ip): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::ingest::get_device_zeek_events(device_ip, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn detect_wireshark_handler() -> Result<Json<Value>, ApiError> {
    to_json(
        commands::wireshark::detect_wireshark()
            .await
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn open_in_wireshark_handler(
    State(state): State<SharedState>,
    Json(body): Json<ConnectionIdRequest>,
) -> Result<Json<Value>, ApiError> {
    commands::wireshark::open_in_wireshark(body.connection_id, state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    Ok(Json(json!({})))
}

pub(super) async fn open_wireshark_for_node_handler(
    Json(body): Json<IpAddressRequest>,
) -> Result<Json<Value>, ApiError> {
    commands::wireshark::open_wireshark_for_node(body.ip_address)
        .await
        .map_err(ApiError::bad_request)?;
    Ok(Json(json!({})))
}

pub(super) async fn get_connection_frames_handler(
    Path(connection_id): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::wireshark::get_connection_frames(connection_id, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn export_frames_csv_handler(
    Path(connection_id): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::wireshark::export_frames_csv(connection_id, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn save_frames_csv_handler(
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

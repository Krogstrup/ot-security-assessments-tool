use axum::extract::{Path, Query, State};
use axum::Json;
use serde_json::{json, Value};

use super::web_requests::{
    AssetPagingQuery, BulkUpdateAssetsRequest, ConnectionPagingQuery, ProtocolStatsQuery,
    StartCaptureRequest, StopCaptureRequest, TestSignatureRequest, UpdateAssetRequest,
};
use super::web_runtime::{start_capture_headless, to_json};
use super::web_support::{resolve_export_output_path, ApiError};
use super::SharedState;
use crate::commands;
use crate::commands::data::{AssetPage, ConnectionPage, DataCounts};
use crate::commands::ProtocolStatInfo;
use gm_topology::TopologyGraph;

pub(super) async fn get_topology(
    State(state): State<SharedState>,
) -> Result<Json<TopologyGraph>, ApiError> {
    let topology = commands::data::get_topology(state.as_ref()).map_err(ApiError::bad_request)?;
    Ok(Json(topology))
}

pub(super) async fn get_assets(
    State(state): State<SharedState>,
    Query(query): Query<AssetPagingQuery>,
) -> Result<Json<AssetPage>, ApiError> {
    let page =
        commands::data::get_assets(state.as_ref(), query.page, query.page_size, query.sort_by)
            .map_err(ApiError::bad_request)?;
    Ok(Json(page))
}

pub(super) async fn get_connections(
    State(state): State<SharedState>,
    Query(query): Query<ConnectionPagingQuery>,
) -> Result<Json<ConnectionPage>, ApiError> {
    let page =
        commands::data::get_connections(state.as_ref(), query.page, query.page_size, query.sort_by)
            .map_err(ApiError::bad_request)?;
    Ok(Json(page))
}

pub(super) async fn get_counts(
    State(state): State<SharedState>,
) -> Result<Json<DataCounts>, ApiError> {
    let counts = commands::data::get_data_counts(state.as_ref()).map_err(ApiError::bad_request)?;
    Ok(Json(counts))
}

pub(super) async fn get_protocol_stats(
    State(state): State<SharedState>,
    Query(query): Query<ProtocolStatsQuery>,
) -> Result<Json<Vec<ProtocolStatInfo>>, ApiError> {
    let stats = commands::data::get_protocol_stats(state.as_ref(), query.sort_by)
        .map_err(ApiError::bad_request)?;
    Ok(Json(stats))
}

pub(super) async fn get_connection_packets(
    State(state): State<SharedState>,
    Path(connection_id): Path<String>,
) -> Result<Json<Vec<commands::PacketSummary>>, ApiError> {
    let packets = commands::data::get_connection_packets(connection_id, state.as_ref())
        .map_err(ApiError::bad_request)?;
    Ok(Json(packets))
}

pub(super) async fn cancel_import_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    commands::capture::cancel_import(state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    Ok(Json(json!({})))
}

pub(super) async fn start_capture_handler(
    State(state): State<SharedState>,
    Json(body): Json<StartCaptureRequest>,
) -> Result<Json<Value>, ApiError> {
    start_capture_headless(state, body.interface_name, body.bpf_filter).await?;
    Ok(Json(json!({})))
}

pub(super) async fn stop_capture_handler(
    State(state): State<SharedState>,
    Json(body): Json<StopCaptureRequest>,
) -> Result<Json<Value>, ApiError> {
    let save_path = body
        .save_path
        .as_deref()
        .map(|v| resolve_export_output_path(v, "capture.pcap"))
        .transpose()?;
    to_json(
        commands::capture::stop_capture(save_path, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn pause_capture_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    commands::capture::pause_capture(state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    Ok(Json(json!({})))
}

pub(super) async fn resume_capture_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    commands::capture::resume_capture(state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    Ok(Json(json!({})))
}

pub(super) async fn get_capture_status_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::capture::get_capture_status(state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn update_asset_handler(
    Path(asset_id): Path<String>,
    State(state): State<SharedState>,
    Json(body): Json<UpdateAssetRequest>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::session::update_asset(asset_id, body.updates, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn bulk_update_assets_handler(
    State(state): State<SharedState>,
    Json(body): Json<BulkUpdateAssetsRequest>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::session::bulk_update_assets(body.asset_ids, body.updates, state.as_ref())
            .await
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn get_deep_parse_info_handler(
    Path(ip_address): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::data::get_deep_parse_info(ip_address, state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn get_function_code_stats_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(commands::data::get_function_code_stats(state.as_ref()).map_err(ApiError::bad_request)?)
}

pub(super) async fn get_timeline_range_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(commands::data::get_timeline_range(state.as_ref()).map_err(ApiError::bad_request)?)
}

pub(super) async fn get_signatures_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(commands::signatures::get_signatures(state.as_ref()).map_err(ApiError::bad_request)?)
}

pub(super) async fn reload_signatures_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(commands::signatures::reload_signatures(state.as_ref()).map_err(ApiError::bad_request)?)
}

pub(super) async fn test_signature_handler(
    State(state): State<SharedState>,
    Json(body): Json<TestSignatureRequest>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::signatures::test_signature(body.yaml, state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

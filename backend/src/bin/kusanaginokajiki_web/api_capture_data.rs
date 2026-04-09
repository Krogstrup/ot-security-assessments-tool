use axum::extract::{Path, Query, State};
use axum::routing::{get, post, put};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

use super::web_api_paths::capture_data as api_path;
use super::web_runtime::{start_capture_headless, to_json};
use super::web_support::{resolve_export_output_path, ApiError};
use super::SharedState;
use crate::commands;
use crate::commands::data::{
    AssetPage, AssetSortBy, ConnectionPage, ConnectionSortBy, DataCounts, ProtocolStatsSortBy,
};
use crate::commands::ProtocolStatInfo;
use gm_topology::TopologyGraph;

#[derive(Debug, Deserialize)]
struct AssetPagingQuery {
    page: Option<usize>,
    #[serde(alias = "pageSize")]
    page_size: Option<usize>,
    #[serde(alias = "sortBy")]
    sort_by: Option<AssetSortBy>,
}

#[derive(Debug, Deserialize)]
struct ConnectionPagingQuery {
    page: Option<usize>,
    #[serde(alias = "pageSize")]
    page_size: Option<usize>,
    #[serde(alias = "sortBy")]
    sort_by: Option<ConnectionSortBy>,
}

#[derive(Debug, Deserialize)]
struct ProtocolStatsQuery {
    #[serde(alias = "sortBy")]
    sort_by: Option<ProtocolStatsSortBy>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StartCaptureRequest {
    interface_name: String,
    bpf_filter: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StopCaptureRequest {
    save_path: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateAssetRequest {
    updates: commands::session::AssetUpdate,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BulkUpdateAssetsRequest {
    asset_ids: Vec<String>,
    updates: commands::session::AssetUpdate,
}

pub(super) fn add_routes(router: Router<SharedState>) -> Router<SharedState> {
    router
        .route(api_path::DATA_TOPOLOGY, get(get_topology))
        .route(api_path::DATA_ASSETS, get(get_assets))
        .route(api_path::DATA_CONNECTIONS, get(get_connections))
        .route(api_path::DATA_COUNTS, get(get_counts))
        .route(api_path::DATA_PROTOCOL_STATS, get(get_protocol_stats))
        .route(
            api_path::DATA_CONNECTION_PACKETS_BY_ID,
            get(get_connection_packets),
        )
        .route(api_path::V1_CAPTURE_CANCEL, post(cancel_import))
        .route(api_path::V1_CAPTURE_START, post(start_capture))
        .route(api_path::V1_CAPTURE_STOP, post(stop_capture))
        .route(api_path::V1_CAPTURE_PAUSE, post(pause_capture))
        .route(api_path::V1_CAPTURE_RESUME, post(resume_capture))
        .route(api_path::V1_CAPTURE_STATUS, get(get_capture_status))
        .route(api_path::V1_ASSETS_BULK_UPDATE, put(bulk_update_assets))
        .route(api_path::V1_ASSET_BY_ID, put(update_asset))
        .route(api_path::V1_DATA_DEEP_PARSE_BY_IP, get(get_deep_parse_info))
        .route(
            api_path::V1_DATA_FUNCTION_CODE_STATS,
            get(get_function_code_stats),
        )
        .route(api_path::V1_DATA_TIMELINE_RANGE, get(get_timeline_range))
}

async fn get_topology(State(state): State<SharedState>) -> Result<Json<TopologyGraph>, ApiError> {
    let topology = commands::data::get_topology(state.as_ref()).map_err(ApiError::from)?;
    Ok(Json(topology))
}

async fn get_assets(
    State(state): State<SharedState>,
    Query(query): Query<AssetPagingQuery>,
) -> Result<Json<AssetPage>, ApiError> {
    let page =
        commands::data::get_assets(state.as_ref(), query.page, query.page_size, query.sort_by)
            .map_err(ApiError::from)?;
    Ok(Json(page))
}

async fn get_connections(
    State(state): State<SharedState>,
    Query(query): Query<ConnectionPagingQuery>,
) -> Result<Json<ConnectionPage>, ApiError> {
    let page =
        commands::data::get_connections(state.as_ref(), query.page, query.page_size, query.sort_by)
            .map_err(ApiError::from)?;
    Ok(Json(page))
}

async fn get_counts(State(state): State<SharedState>) -> Result<Json<DataCounts>, ApiError> {
    let counts = commands::data::get_data_counts(state.as_ref()).map_err(ApiError::from)?;
    Ok(Json(counts))
}

async fn get_protocol_stats(
    State(state): State<SharedState>,
    Query(query): Query<ProtocolStatsQuery>,
) -> Result<Json<Vec<ProtocolStatInfo>>, ApiError> {
    let stats = commands::data::get_protocol_stats(state.as_ref(), query.sort_by)
        .map_err(ApiError::from)?;
    Ok(Json(stats))
}

async fn get_connection_packets(
    State(state): State<SharedState>,
    Path(connection_id): Path<String>,
) -> Result<Json<Vec<commands::PacketSummary>>, ApiError> {
    let packets = commands::data::get_connection_packets(connection_id, state.as_ref())
        .map_err(ApiError::from)?;
    Ok(Json(packets))
}

async fn cancel_import(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    commands::capture::cancel_import(state.as_ref())
        .await
        .map_err(ApiError::from)?;
    Ok(Json(json!({})))
}

async fn start_capture(
    State(state): State<SharedState>,
    Json(body): Json<StartCaptureRequest>,
) -> Result<Json<Value>, ApiError> {
    start_capture_headless(state, body.interface_name, body.bpf_filter).await?;
    Ok(Json(json!({})))
}

async fn stop_capture(
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
            .map_err(ApiError::from)?,
    )
}

async fn pause_capture(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    commands::capture::pause_capture(state.as_ref())
        .await
        .map_err(ApiError::from)?;
    Ok(Json(json!({})))
}

async fn resume_capture(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    commands::capture::resume_capture(state.as_ref())
        .await
        .map_err(ApiError::from)?;
    Ok(Json(json!({})))
}

async fn get_capture_status(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::capture::get_capture_status(state.as_ref())
            .await
            .map_err(ApiError::from)?,
    )
}

async fn update_asset(
    Path(asset_id): Path<String>,
    State(state): State<SharedState>,
    Json(body): Json<UpdateAssetRequest>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::session::update_asset(asset_id, body.updates, state.as_ref())
            .await
            .map_err(ApiError::from)?,
    )
}

async fn bulk_update_assets(
    State(state): State<SharedState>,
    Json(body): Json<BulkUpdateAssetsRequest>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::session::bulk_update_assets(body.asset_ids, body.updates, state.as_ref())
            .await
            .map_err(ApiError::from)?,
    )
}

async fn get_deep_parse_info(
    Path(ip_address): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::data::get_deep_parse_info(ip_address, state.as_ref()).map_err(ApiError::from)?,
    )
}

async fn get_function_code_stats(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(commands::data::get_function_code_stats(state.as_ref()).map_err(ApiError::from)?)
}

async fn get_timeline_range(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    to_json(commands::data::get_timeline_range(state.as_ref()).map_err(ApiError::from)?)
}

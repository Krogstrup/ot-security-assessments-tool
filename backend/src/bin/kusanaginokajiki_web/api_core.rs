use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

use super::web_api_paths::core as api_path;
use super::web_runtime::to_json;
use super::web_support::{
    list_import_files_for_kind, resolve_import_input_path, ApiError, ImportKind,
    ImportPcapFilesResponse,
};
use super::SharedState;
use crate::commands;
use crate::commands::capture::ImportResult;

#[derive(Debug, Deserialize)]
struct ImportPcapRequest {
    paths: Vec<String>,
}

pub(super) fn add_routes(router: Router<SharedState>) -> Router<SharedState> {
    router
        .route(api_path::HEALTH, get(health))
        .route(api_path::SYSTEM_APP_INFO, get(get_app_info))
        .route(api_path::SYSTEM_INTERFACES, get(get_interfaces))
        .route(api_path::SYSTEM_IMPORT_PCAP_FILES, get(list_import_pcap_files))
        .route(api_path::SYSTEM_IMPORT_FILES_BY_KIND, get(list_import_files))
        .route(api_path::CAPTURE_IMPORT_PCAP, post(import_pcap))
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "ok": true }))
}

async fn get_app_info() -> Result<Json<Value>, ApiError> {
    to_json(commands::system::get_app_info())
}

async fn get_interfaces() -> Result<Json<Vec<gm_capture::NetworkInterface>>, ApiError> {
    let interfaces = commands::system::list_interfaces().map_err(ApiError::bad_request)?;
    Ok(Json(interfaces))
}

async fn list_import_pcap_files() -> Result<Json<ImportPcapFilesResponse>, ApiError> {
    Ok(Json(list_import_files_for_kind(ImportKind::Pcap)?))
}

async fn list_import_files(
    Path(kind): Path<String>,
) -> Result<Json<ImportPcapFilesResponse>, ApiError> {
    let kind = kind.parse::<ImportKind>().map_err(|_| {
        ApiError::bad_request(format!(
            "unsupported import kind '{}'. expected one of: {}",
            kind,
            ImportKind::supported_values_csv()
        ))
    })?;
    Ok(Json(list_import_files_for_kind(kind)?))
}

async fn import_pcap(
    State(state): State<SharedState>,
    Json(request): Json<ImportPcapRequest>,
) -> Result<Json<ImportResult>, ApiError> {
    if request.paths.is_empty() {
        return Err(ApiError::bad_request(
            "paths must contain at least one PCAP path",
        ));
    }

    let paths: Vec<String> = request
        .paths
        .iter()
        .map(|path| resolve_import_input_path(path, ImportKind::Pcap))
        .collect::<Result<Vec<_>, _>>()?;

    let result = commands::capture::import_pcap_files(paths, state.as_ref())
        .map_err(ApiError::bad_request)?;
    Ok(Json(result))
}

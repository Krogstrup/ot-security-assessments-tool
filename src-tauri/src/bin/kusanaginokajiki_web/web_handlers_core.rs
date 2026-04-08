use axum::extract::{Path, State};
use axum::Json;
use serde_json::{json, Value};

use super::web_requests::ImportPcapRequest;
use super::web_support::{
    list_import_files_for_kind, resolve_import_input_path, ApiError, ImportKind,
    ImportPcapFilesResponse,
};
use super::SharedState;
use crate::commands;
use crate::commands::capture::ImportResult;

pub(super) async fn health() -> Json<serde_json::Value> {
    Json(json!({ "ok": true }))
}

pub(super) async fn get_app_info() -> Result<Json<Value>, ApiError> {
    super::web_runtime::to_json(commands::system::get_app_info())
}

pub(super) async fn get_interfaces() -> Result<Json<Vec<gm_capture::NetworkInterface>>, ApiError> {
    let interfaces = commands::system::list_interfaces().map_err(ApiError::bad_request)?;
    Ok(Json(interfaces))
}

pub(super) async fn list_import_pcap_files() -> Result<Json<ImportPcapFilesResponse>, ApiError> {
    Ok(Json(list_import_files_for_kind(ImportKind::Pcap)?))
}

pub(super) async fn list_import_files(
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

pub(super) async fn import_pcap(
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

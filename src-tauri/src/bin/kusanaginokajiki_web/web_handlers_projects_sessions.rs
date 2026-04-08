use axum::extract::{Path, State};
use axum::Json;
use serde_json::{json, Value};

use super::web_requests::{
    CompareSessionsRequest, CreateProjectRequest, ExportSessionRequest, ImportSessionRequest,
    SaveSessionRequest, SetActiveProjectRequest, UpdateProjectRequest,
};
use super::web_runtime::to_json;
use super::web_support::{
    resolve_export_output_path, resolve_import_input_path, ApiError, ImportKind,
};
use super::SharedState;
use crate::commands;

// ── /api/v1/projects ─────────────────────────────────────────────────────────

pub(super) async fn list_projects_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    let projects = commands::projects::list_projects(state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    to_json(projects)
}

pub(super) async fn create_project_handler(
    State(state): State<SharedState>,
    Json(body): Json<CreateProjectRequest>,
) -> Result<Json<Value>, ApiError> {
    let project = commands::projects::create_project(
        state.as_ref(),
        body.name,
        body.client_name,
        body.site_name,
        body.assessor_name,
        body.engagement_start,
        body.engagement_end,
        body.notes,
    )
    .await
    .map_err(ApiError::bad_request)?;
    to_json(project)
}

pub(super) async fn get_project_handler(
    Path(id): Path<i64>,
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    let project = commands::projects::get_project(state.as_ref(), id)
        .await
        .map_err(ApiError::bad_request)?;
    to_json(project)
}

pub(super) async fn update_project_handler(
    Path(id): Path<i64>,
    State(state): State<SharedState>,
    Json(body): Json<UpdateProjectRequest>,
) -> Result<Json<Value>, ApiError> {
    let project = commands::projects::update_project(
        state.as_ref(),
        id,
        body.name,
        body.client_name,
        body.site_name,
        body.assessor_name,
        body.engagement_start,
        body.engagement_end,
        body.notes,
    )
    .await
    .map_err(ApiError::bad_request)?;
    to_json(project)
}

pub(super) async fn delete_project_handler(
    Path(id): Path<i64>,
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    commands::projects::delete_project(state.as_ref(), id)
        .await
        .map_err(ApiError::bad_request)?;
    Ok(Json(json!({})))
}

pub(super) async fn set_active_project_handler(
    State(state): State<SharedState>,
    Json(body): Json<SetActiveProjectRequest>,
) -> Result<Json<Value>, ApiError> {
    let project = commands::projects::set_active_project(state.as_ref(), body.id)
        .await
        .map_err(ApiError::bad_request)?;
    to_json(project)
}

pub(super) async fn clear_active_project_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    commands::projects::clear_active_project(state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    Ok(Json(json!({})))
}

// ── /api/v1/sessions ─────────────────────────────────────────────────────────

pub(super) async fn list_sessions_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    let sessions = commands::session::list_sessions(state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    to_json(sessions)
}

pub(super) async fn save_session_handler(
    State(state): State<SharedState>,
    Json(body): Json<SaveSessionRequest>,
) -> Result<Json<Value>, ApiError> {
    let session = commands::session::save_session(body.name, body.description, state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    to_json(session)
}

pub(super) async fn load_session_handler(
    Path(id): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    let session = commands::session::load_session(id, state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    to_json(session)
}

pub(super) async fn delete_session_handler(
    Path(id): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    commands::session::delete_session(id, state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    Ok(Json(json!({})))
}

pub(super) async fn export_session_handler(
    Path(id): Path<String>,
    State(state): State<SharedState>,
    Json(body): Json<ExportSessionRequest>,
) -> Result<Json<Value>, ApiError> {
    let output_path = resolve_export_output_path(&body.output_path, "session.kkj")?;
    let path = commands::session::export_session_archive(id, output_path, state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    to_json(path)
}

pub(super) async fn import_session_handler(
    State(state): State<SharedState>,
    Json(body): Json<ImportSessionRequest>,
) -> Result<Json<Value>, ApiError> {
    let archive_path = resolve_import_input_path(&body.archive_path, ImportKind::SessionArchive)?;
    let session = commands::session::import_session_archive(archive_path, state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    to_json(session)
}

pub(super) async fn compare_sessions_handler(
    State(state): State<SharedState>,
    Json(body): Json<CompareSessionsRequest>,
) -> Result<Json<Value>, ApiError> {
    let diff = commands::baseline::compare_sessions(body.baseline_session_id, state.as_ref())
        .map_err(ApiError::bad_request)?;
    to_json(diff)
}

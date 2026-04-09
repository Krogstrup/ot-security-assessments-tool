use axum::extract::{Path, Query, State};
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

use super::web_api_paths::projects_sessions_analysis_events as api_path;
use super::web_runtime::to_json;
use super::web_support::{
    resolve_export_output_path, resolve_import_input_path, ApiError, ImportKind,
};
use super::{events_handler, SharedState};
use crate::commands;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateProjectRequest {
    name: String,
    client_name: Option<String>,
    site_name: Option<String>,
    assessor_name: Option<String>,
    engagement_start: Option<String>,
    engagement_end: Option<String>,
    notes: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateProjectRequest {
    name: String,
    client_name: Option<String>,
    site_name: Option<String>,
    assessor_name: Option<String>,
    engagement_start: Option<String>,
    engagement_end: Option<String>,
    notes: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SetActiveProjectRequest {
    id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SaveSessionRequest {
    name: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExportSessionRequest {
    output_path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImportSessionRequest {
    archive_path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CompareSessionsRequest {
    baseline_session_id: String,
}

#[derive(Debug, Deserialize)]
struct ComplianceQuery {
    framework: String,
}

#[derive(Debug, Deserialize)]
struct CveQuery {
    ip: String,
}

pub(super) fn add_routes(router: Router<SharedState>) -> Router<SharedState> {
    router
        // NOTE: /v1/projects/active must be registered before /v1/projects/{id}
        .route(api_path::V1_PROJECTS, get(list_projects).post(create_project))
        .route(
            api_path::V1_PROJECTS_ACTIVE,
            put(set_active_project).delete(clear_active_project),
        )
        .route(
            api_path::V1_PROJECT_BY_ID,
            get(get_project).put(update_project).delete(delete_project),
        )
        // NOTE: static sub-paths (/import, /compare) must be registered before /sessions/{id}
        .route(api_path::V1_SESSIONS, get(list_sessions).post(save_session))
        .route(api_path::V1_SESSIONS_IMPORT, post(import_session))
        .route(api_path::V1_SESSIONS_COMPARE, post(compare_sessions))
        .route(api_path::V1_SESSION_LOAD_BY_ID, post(load_session))
        .route(api_path::V1_SESSION_EXPORT_BY_ID, post(export_session))
        .route(api_path::V1_SESSION_BY_ID, delete(delete_session))
        .route(api_path::V1_ANALYSIS_RUN, post(run_analysis))
        .route(api_path::V1_ANALYSIS_FINDINGS, get(get_findings))
        .route(api_path::V1_ANALYSIS_PURDUE, get(get_purdue))
        .route(api_path::V1_ANALYSIS_ANOMALIES, get(get_anomalies))
        .route(api_path::V1_ANALYSIS_CREDENTIALS, get(get_credentials))
        .route(api_path::V1_ANALYSIS_CRITICALITY, get(get_criticality))
        .route(
            api_path::V1_ANALYSIS_NAMING_SUGGESTIONS,
            get(get_naming_suggestions),
        )
        .route(api_path::V1_ANALYSIS_MALWARE, get(get_malware))
        .route(api_path::V1_ANALYSIS_SWITCH_SECURITY, get(get_switch_security))
        .route(api_path::V1_ANALYSIS_COMPLIANCE, get(get_compliance))
        .route(api_path::V1_ANALYSIS_CVE, get(get_cve))
        .route(api_path::V1_EVENTS, get(events_handler))
}

async fn list_projects(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    let projects = commands::projects::list_projects(state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    to_json(projects)
}

async fn create_project(
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

async fn get_project(
    Path(id): Path<i64>,
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    let project = commands::projects::get_project(state.as_ref(), id)
        .await
        .map_err(ApiError::bad_request)?;
    to_json(project)
}

async fn update_project(
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

async fn delete_project(
    Path(id): Path<i64>,
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    commands::projects::delete_project(state.as_ref(), id)
        .await
        .map_err(ApiError::bad_request)?;
    Ok(Json(json!({})))
}

async fn set_active_project(
    State(state): State<SharedState>,
    Json(body): Json<SetActiveProjectRequest>,
) -> Result<Json<Value>, ApiError> {
    let project = commands::projects::set_active_project(state.as_ref(), body.id)
        .await
        .map_err(ApiError::bad_request)?;
    to_json(project)
}

async fn clear_active_project(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    commands::projects::clear_active_project(state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    Ok(Json(json!({})))
}

async fn list_sessions(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    let sessions = commands::session::list_sessions(state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    to_json(sessions)
}

async fn save_session(
    State(state): State<SharedState>,
    Json(body): Json<SaveSessionRequest>,
) -> Result<Json<Value>, ApiError> {
    let session = commands::session::save_session(body.name, body.description, state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    to_json(session)
}

async fn load_session(
    Path(id): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    let session = commands::session::load_session(id, state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    to_json(session)
}

async fn delete_session(
    Path(id): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    commands::session::delete_session(id, state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    Ok(Json(json!({})))
}

async fn export_session(
    Path(id): Path<String>,
    State(state): State<SharedState>,
    Json(body): Json<ExportSessionRequest>,
) -> Result<Json<Value>, ApiError> {
    let output_path = resolve_export_output_path(&body.output_path, "session.kkj")?;
    let archive = commands::session::export_session_archive(id, output_path, state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    to_json(archive)
}

async fn import_session(
    State(state): State<SharedState>,
    Json(body): Json<ImportSessionRequest>,
) -> Result<Json<Value>, ApiError> {
    let archive_path = resolve_import_input_path(&body.archive_path, ImportKind::SessionArchive)?;
    let session = commands::session::import_session_archive(archive_path, state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    to_json(session)
}

async fn compare_sessions(
    State(state): State<SharedState>,
    Json(body): Json<CompareSessionsRequest>,
) -> Result<Json<Value>, ApiError> {
    let diff = commands::baseline::compare_sessions(body.baseline_session_id, state.as_ref())
        .map_err(ApiError::bad_request)?;
    to_json(diff)
}

async fn run_analysis(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    let result = commands::analysis::run_analysis(state.as_ref()).map_err(ApiError::bad_request)?;
    to_json(result)
}

async fn get_findings(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    to_json(commands::analysis::get_findings(state.as_ref()).map_err(ApiError::bad_request)?)
}

async fn get_purdue(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::analysis::get_purdue_assignments(state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

async fn get_anomalies(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    to_json(commands::analysis::get_anomalies(state.as_ref()).map_err(ApiError::bad_request)?)
}

async fn get_credentials(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::analysis::get_credential_warnings(state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

async fn get_criticality(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    to_json(commands::analysis::get_criticality(state.as_ref()).map_err(ApiError::bad_request)?)
}

async fn get_naming_suggestions(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::analysis::get_naming_suggestions(state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

async fn get_malware(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    to_json(commands::analysis::get_malware_findings(state.as_ref()).map_err(ApiError::bad_request)?)
}

async fn get_switch_security(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::analysis::get_switch_security_findings(state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

async fn get_compliance(
    State(state): State<SharedState>,
    Query(query): Query<ComplianceQuery>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::analysis::get_compliance_report(state.as_ref(), query.framework)
            .map_err(ApiError::bad_request)?,
    )
}

async fn get_cve(
    State(state): State<SharedState>,
    Query(query): Query<CveQuery>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::analysis::get_cve_warnings(query.ip, state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

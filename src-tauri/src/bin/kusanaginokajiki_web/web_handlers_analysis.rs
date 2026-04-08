use axum::extract::{Query, State};
use axum::Json;
use serde_json::Value;

use super::web_requests::{ComplianceQuery, CveQuery};
use super::web_runtime::to_json;
use super::web_support::ApiError;
use super::SharedState;
use crate::commands;

// ── /api/v1/analysis ─────────────────────────────────────────────────────────

pub(super) async fn run_analysis_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    let result = commands::analysis::run_analysis(state.as_ref()).map_err(ApiError::bad_request)?;
    to_json(result)
}

pub(super) async fn get_findings_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(commands::analysis::get_findings(state.as_ref()).map_err(ApiError::bad_request)?)
}

pub(super) async fn get_purdue_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::analysis::get_purdue_assignments(state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn get_anomalies_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(commands::analysis::get_anomalies(state.as_ref()).map_err(ApiError::bad_request)?)
}

pub(super) async fn get_credentials_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::analysis::get_credential_warnings(state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn get_criticality_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(commands::analysis::get_criticality(state.as_ref()).map_err(ApiError::bad_request)?)
}

pub(super) async fn get_naming_suggestions_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::analysis::get_naming_suggestions(state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn get_malware_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::analysis::get_malware_findings(state.as_ref()).map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn get_switch_security_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::analysis::get_switch_security_findings(state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn get_compliance_handler(
    State(state): State<SharedState>,
    Query(query): Query<ComplianceQuery>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::analysis::get_compliance_report(state.as_ref(), query.framework)
            .map_err(ApiError::bad_request)?,
    )
}

pub(super) async fn get_cve_handler(
    State(state): State<SharedState>,
    Query(query): Query<CveQuery>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::analysis::get_cve_warnings(query.ip, state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

use axum::routing::{delete, get, post, put};
use axum::Router;

use super::super::web_api_paths::projects_sessions_analysis_events as path;
use super::super::web_handlers_analysis as analysis;
use super::super::web_handlers_projects_sessions as projects_sessions;
use super::super::{events_handler, SharedState};

pub(super) fn add_projects_sessions_analysis_and_events_routes(
    router: Router<SharedState>,
) -> Router<SharedState> {
    router
        // NOTE: /v1/projects/active must be registered before /v1/projects/{id}
        .route(
            path::V1_PROJECTS,
            get(projects_sessions::list_projects_handler)
                .post(projects_sessions::create_project_handler),
        )
        .route(
            path::V1_PROJECTS_ACTIVE,
            put(projects_sessions::set_active_project_handler)
                .delete(projects_sessions::clear_active_project_handler),
        )
        .route(
            path::V1_PROJECT_BY_ID,
            get(projects_sessions::get_project_handler)
                .put(projects_sessions::update_project_handler)
                .delete(projects_sessions::delete_project_handler),
        )
        // NOTE: static sub-paths (/import, /compare) must be registered before /sessions/{id}
        .route(
            path::V1_SESSIONS,
            get(projects_sessions::list_sessions_handler)
                .post(projects_sessions::save_session_handler),
        )
        .route(
            path::V1_SESSIONS_IMPORT,
            post(projects_sessions::import_session_handler),
        )
        .route(
            path::V1_SESSIONS_COMPARE,
            post(projects_sessions::compare_sessions_handler),
        )
        .route(
            path::V1_SESSION_LOAD_BY_ID,
            post(projects_sessions::load_session_handler),
        )
        .route(
            path::V1_SESSION_EXPORT_BY_ID,
            post(projects_sessions::export_session_handler),
        )
        .route(
            path::V1_SESSION_BY_ID,
            delete(projects_sessions::delete_session_handler),
        )
        .route(path::V1_ANALYSIS_RUN, post(analysis::run_analysis_handler))
        .route(
            path::V1_ANALYSIS_FINDINGS,
            get(analysis::get_findings_handler),
        )
        .route(path::V1_ANALYSIS_PURDUE, get(analysis::get_purdue_handler))
        .route(
            path::V1_ANALYSIS_ANOMALIES,
            get(analysis::get_anomalies_handler),
        )
        .route(
            path::V1_ANALYSIS_CREDENTIALS,
            get(analysis::get_credentials_handler),
        )
        .route(
            path::V1_ANALYSIS_CRITICALITY,
            get(analysis::get_criticality_handler),
        )
        .route(
            path::V1_ANALYSIS_NAMING_SUGGESTIONS,
            get(analysis::get_naming_suggestions_handler),
        )
        .route(
            path::V1_ANALYSIS_MALWARE,
            get(analysis::get_malware_handler),
        )
        .route(
            path::V1_ANALYSIS_SWITCH_SECURITY,
            get(analysis::get_switch_security_handler),
        )
        .route(
            path::V1_ANALYSIS_COMPLIANCE,
            get(analysis::get_compliance_handler),
        )
        .route(path::V1_ANALYSIS_CVE, get(analysis::get_cve_handler))
        .route(path::V1_EVENTS, get(events_handler))
}

use axum::Router;

use super::SharedState;

#[path = "api_capture_data.rs"]
mod capture_data;
#[path = "api_core.rs"]
mod core;
#[path = "api_patterns_exports.rs"]
mod patterns_exports;
#[path = "api_physical_ingest_wireshark.rs"]
mod physical_ingest_wireshark;
#[path = "api_projects_sessions_analysis.rs"]
mod projects_sessions_analysis;

pub(super) fn build_api_router() -> Router<SharedState> {
    let router = Router::new();
    let router = core::add_routes(router);
    let router = capture_data::add_routes(router);
    let router = physical_ingest_wireshark::add_routes(router);
    let router = patterns_exports::add_routes(router);
    let router = projects_sessions_analysis::add_routes(router);
    router
}

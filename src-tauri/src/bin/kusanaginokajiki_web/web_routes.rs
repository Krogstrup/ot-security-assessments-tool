use axum::Router;

use super::SharedState;

#[path = "web_routes_capture_data.rs"]
mod capture_data;
#[path = "web_routes_core.rs"]
mod core;
#[path = "web_routes_patterns_exports.rs"]
mod patterns_exports;
#[path = "web_routes_physical_ingest_wireshark.rs"]
mod physical_ingest_wireshark;
#[path = "web_routes_projects_sessions_analysis.rs"]
mod projects_sessions_analysis;

pub(super) fn build_api_router() -> Router<SharedState> {
    let router = Router::new();
    let router = core::add_core_routes(router);
    let router = capture_data::add_capture_data_routes(router);
    let router = physical_ingest_wireshark::add_physical_routes(router);
    let router = physical_ingest_wireshark::add_ingest_wireshark_routes(router);
    let router = patterns_exports::add_signatures_patterns_and_correlation_routes(router);
    let router = patterns_exports::add_system_export_and_segmentation_routes(router);
    let router =
        projects_sessions_analysis::add_projects_sessions_analysis_and_events_routes(router);
    router
}

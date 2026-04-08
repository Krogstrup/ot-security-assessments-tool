use axum::routing::{get, post};
use axum::Router;

use super::super::web_api_paths::core as path;
use super::super::web_handlers_core as core;
use super::super::SharedState;

pub(super) fn add_core_routes(router: Router<SharedState>) -> Router<SharedState> {
    router
        .route(path::HEALTH, get(core::health))
        .route(path::SYSTEM_APP_INFO, get(core::get_app_info))
        .route(path::SYSTEM_INTERFACES, get(core::get_interfaces))
        .route(
            path::SYSTEM_IMPORT_PCAP_FILES,
            get(core::list_import_pcap_files),
        )
        .route(
            path::SYSTEM_IMPORT_FILES_BY_KIND,
            get(core::list_import_files),
        )
        .route(path::CAPTURE_IMPORT_PCAP, post(core::import_pcap))
}

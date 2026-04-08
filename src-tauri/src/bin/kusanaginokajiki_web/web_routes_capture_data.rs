use axum::routing::{get, post, put};
use axum::Router;

use super::super::web_api_paths::capture_data as path;
use super::super::web_handlers_capture_data as capture_data;
use super::super::SharedState;

pub(super) fn add_capture_data_routes(router: Router<SharedState>) -> Router<SharedState> {
    router
        .route(path::DATA_TOPOLOGY, get(capture_data::get_topology))
        .route(path::DATA_ASSETS, get(capture_data::get_assets))
        .route(path::DATA_CONNECTIONS, get(capture_data::get_connections))
        .route(path::DATA_COUNTS, get(capture_data::get_counts))
        .route(
            path::DATA_PROTOCOL_STATS,
            get(capture_data::get_protocol_stats),
        )
        .route(
            path::DATA_CONNECTION_PACKETS_BY_ID,
            get(capture_data::get_connection_packets),
        )
        .route(
            path::V1_CAPTURE_CANCEL,
            post(capture_data::cancel_import_handler),
        )
        .route(
            path::V1_CAPTURE_START,
            post(capture_data::start_capture_handler),
        )
        .route(
            path::V1_CAPTURE_STOP,
            post(capture_data::stop_capture_handler),
        )
        .route(
            path::V1_CAPTURE_PAUSE,
            post(capture_data::pause_capture_handler),
        )
        .route(
            path::V1_CAPTURE_RESUME,
            post(capture_data::resume_capture_handler),
        )
        .route(
            path::V1_CAPTURE_STATUS,
            get(capture_data::get_capture_status_handler),
        )
        .route(
            path::V1_ASSETS_BULK_UPDATE,
            put(capture_data::bulk_update_assets_handler),
        )
        .route(
            path::V1_ASSET_BY_ID,
            put(capture_data::update_asset_handler),
        )
        .route(
            path::V1_DATA_DEEP_PARSE_BY_IP,
            get(capture_data::get_deep_parse_info_handler),
        )
        .route(
            path::V1_DATA_FUNCTION_CODE_STATS,
            get(capture_data::get_function_code_stats_handler),
        )
        .route(
            path::V1_DATA_TIMELINE_RANGE,
            get(capture_data::get_timeline_range_handler),
        )
}

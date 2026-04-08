use axum::routing::{get, post};
use axum::Router;

use super::super::web_api_paths::signatures_patterns_correlation as signatures_patterns_correlation_path;
use super::super::web_api_paths::system_exports_segmentation as system_exports_segmentation_path;
use super::super::web_handlers_capture_data as capture_data;
use super::super::web_handlers_patterns_exports as patterns_exports;
use super::super::SharedState;

pub(super) fn add_signatures_patterns_and_correlation_routes(
    router: Router<SharedState>,
) -> Router<SharedState> {
    router
        .route(
            signatures_patterns_correlation_path::V1_SIGNATURES,
            get(capture_data::get_signatures_handler),
        )
        .route(
            signatures_patterns_correlation_path::V1_SIGNATURES_RELOAD,
            post(capture_data::reload_signatures_handler),
        )
        .route(
            signatures_patterns_correlation_path::V1_SIGNATURES_TEST,
            post(capture_data::test_signature_handler),
        )
        .route(
            signatures_patterns_correlation_path::V1_PATTERNS_CONNECTION_STATS,
            get(patterns_exports::get_connection_stats_handler),
        )
        .route(
            signatures_patterns_correlation_path::V1_PATTERNS_ANOMALIES,
            get(patterns_exports::get_pattern_anomalies_handler),
        )
        .route(
            signatures_patterns_correlation_path::V1_PATTERNS_REDUNDANCY_PROTOCOLS,
            get(patterns_exports::get_redundancy_protocols_handler),
        )
        .route(
            signatures_patterns_correlation_path::V1_CORRELATION_ALERTS,
            get(patterns_exports::get_correlated_alerts_handler)
                .delete(patterns_exports::clear_alerts_handler),
        )
        .route(
            signatures_patterns_correlation_path::V1_CORRELATION_ALERTS_BY_IP,
            get(patterns_exports::get_alerts_for_ip_handler),
        )
}

pub(super) fn add_system_export_and_segmentation_routes(
    router: Router<SharedState>,
) -> Router<SharedState> {
    router
        .route(
            system_exports_segmentation_path::V1_SYSTEM_SETTINGS,
            get(patterns_exports::get_settings_handler)
                .put(patterns_exports::save_settings_handler),
        )
        .route(
            system_exports_segmentation_path::V1_SYSTEM_PLUGINS,
            get(patterns_exports::list_plugins_handler),
        )
        .route(
            system_exports_segmentation_path::V1_EXPORTS_ASSETS_CSV,
            post(patterns_exports::export_assets_csv_handler),
        )
        .route(
            system_exports_segmentation_path::V1_EXPORTS_CONNECTIONS_CSV,
            post(patterns_exports::export_connections_csv_handler),
        )
        .route(
            system_exports_segmentation_path::V1_EXPORTS_TOPOLOGY_JSON,
            post(patterns_exports::export_topology_json_handler),
        )
        .route(
            system_exports_segmentation_path::V1_EXPORTS_ASSETS_JSON,
            post(patterns_exports::export_assets_json_handler),
        )
        .route(
            system_exports_segmentation_path::V1_EXPORTS_REPORT_PDF,
            post(patterns_exports::generate_pdf_report_handler),
        )
        .route(
            system_exports_segmentation_path::V1_EXPORTS_SBOM,
            post(patterns_exports::export_sbom_handler),
        )
        .route(
            system_exports_segmentation_path::V1_EXPORTS_STIX,
            post(patterns_exports::export_stix_bundle_handler),
        )
        .route(
            system_exports_segmentation_path::V1_EXPORTS_TOPOLOGY_IMAGE,
            post(patterns_exports::save_topology_image_handler),
        )
        .route(
            system_exports_segmentation_path::V1_EXPORTS_PCAP_FILTERED,
            post(patterns_exports::export_filtered_pcap_handler),
        )
        .route(
            system_exports_segmentation_path::V1_EXPORTS_ALLOWLIST,
            get(patterns_exports::generate_communication_allowlist_handler),
        )
        .route(
            system_exports_segmentation_path::V1_EXPORTS_ALLOWLIST_CSV,
            post(patterns_exports::export_allowlist_csv_handler),
        )
        .route(
            system_exports_segmentation_path::V1_EXPORTS_FIREWALL_RULES,
            post(patterns_exports::export_firewall_rules_handler),
        )
        .route(
            system_exports_segmentation_path::V1_SEGMENTATION_RUN,
            post(patterns_exports::run_segmentation_handler),
        )
        .route(
            system_exports_segmentation_path::V1_SEGMENTATION_ENFORCEMENT_CONFIG,
            post(patterns_exports::export_enforcement_config_handler),
        )
}

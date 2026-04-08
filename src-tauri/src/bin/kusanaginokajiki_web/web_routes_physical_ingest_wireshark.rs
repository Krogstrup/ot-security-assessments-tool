use axum::routing::{get, post};
use axum::Router;

use super::super::web_api_paths::ingest_wireshark as ingest_wireshark_path;
use super::super::web_api_paths::physical as physical_path;
use super::super::web_handlers_physical_ingest_wireshark as physical_ingest_wireshark;
use super::super::SharedState;

pub(super) fn add_physical_routes(router: Router<SharedState>) -> Router<SharedState> {
    router
        .route(
            physical_path::V1_PHYSICAL_TOPOLOGY,
            get(physical_ingest_wireshark::get_physical_topology_handler)
                .delete(physical_ingest_wireshark::clear_physical_topology_handler),
        )
        .route(
            physical_path::V1_PHYSICAL_CISCO_CONFIG,
            post(physical_ingest_wireshark::import_cisco_config_handler),
        )
        .route(
            physical_path::V1_PHYSICAL_MAC_TABLE,
            post(physical_ingest_wireshark::import_mac_table_handler),
        )
        .route(
            physical_path::V1_PHYSICAL_CDP_NEIGHBORS,
            post(physical_ingest_wireshark::import_cdp_neighbors_handler),
        )
        .route(
            physical_path::V1_PHYSICAL_ARP_TABLE,
            post(physical_ingest_wireshark::import_arp_table_handler),
        )
        .route(
            physical_path::V1_PHYSICAL_NETWORK_CONFIG,
            post(physical_ingest_wireshark::import_network_config_handler),
        )
        .route(
            physical_path::V1_PHYSICAL_MAC_TABLE_AUTO,
            post(physical_ingest_wireshark::import_mac_table_auto_handler),
        )
        .route(
            physical_path::V1_PHYSICAL_NEIGHBOR_TABLE,
            post(physical_ingest_wireshark::import_neighbor_table_handler),
        )
        .route(
            physical_path::V1_PHYSICAL_INFERENCE_RUN,
            post(physical_ingest_wireshark::run_topology_inference_handler),
        )
        .route(
            physical_path::V1_PHYSICAL_INFERENCE,
            get(physical_ingest_wireshark::get_inferred_topology_handler),
        )
}

pub(super) fn add_ingest_wireshark_routes(router: Router<SharedState>) -> Router<SharedState> {
    router
        .route(
            ingest_wireshark_path::V1_INGEST_ZEEK,
            post(physical_ingest_wireshark::import_zeek_logs_handler),
        )
        .route(
            ingest_wireshark_path::V1_INGEST_SURICATA,
            post(physical_ingest_wireshark::import_suricata_eve_handler),
        )
        .route(
            ingest_wireshark_path::V1_INGEST_NMAP,
            post(physical_ingest_wireshark::import_nmap_xml_handler),
        )
        .route(
            ingest_wireshark_path::V1_INGEST_MASSCAN,
            post(physical_ingest_wireshark::import_masscan_json_handler),
        )
        .route(
            ingest_wireshark_path::V1_INGEST_WAZUH,
            post(physical_ingest_wireshark::import_wazuh_alerts_handler),
        )
        .route(
            ingest_wireshark_path::V1_INGEST_SINEMA,
            post(physical_ingest_wireshark::import_sinema_csv_handler),
        )
        .route(
            ingest_wireshark_path::V1_INGEST_TIA,
            post(physical_ingest_wireshark::import_tia_xml_handler),
        )
        .route(
            ingest_wireshark_path::V1_INGEST_ZEEK_DEVICE_EVENTS_BY_IP,
            get(physical_ingest_wireshark::get_device_zeek_events_handler),
        )
        .route(
            ingest_wireshark_path::V1_WIRESHARK_INFO,
            get(physical_ingest_wireshark::detect_wireshark_handler),
        )
        .route(
            ingest_wireshark_path::V1_WIRESHARK_OPEN_CONNECTION,
            post(physical_ingest_wireshark::open_in_wireshark_handler),
        )
        .route(
            ingest_wireshark_path::V1_WIRESHARK_OPEN_NODE,
            post(physical_ingest_wireshark::open_wireshark_for_node_handler),
        )
        .route(
            ingest_wireshark_path::V1_WIRESHARK_FRAMES_BY_CONNECTION_ID,
            get(physical_ingest_wireshark::get_connection_frames_handler),
        )
        .route(
            ingest_wireshark_path::V1_WIRESHARK_FRAMES_CSV_BY_CONNECTION_ID,
            get(physical_ingest_wireshark::export_frames_csv_handler)
                .post(physical_ingest_wireshark::save_frames_csv_handler),
        )
}

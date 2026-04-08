pub(crate) mod core {
    pub const HEALTH: &str = "/health";
    pub const SYSTEM_APP_INFO: &str = "/system/app-info";
    pub const SYSTEM_INTERFACES: &str = "/system/interfaces";
    pub const SYSTEM_IMPORT_PCAP_FILES: &str = "/system/import-pcap-files";
    pub const SYSTEM_IMPORT_FILES_BY_KIND: &str = "/system/import-files/{kind}";
    pub const CAPTURE_IMPORT_PCAP: &str = "/capture/import-pcap";
}

pub(crate) mod capture_data {
    pub const DATA_TOPOLOGY: &str = "/data/topology";
    pub const DATA_ASSETS: &str = "/data/assets";
    pub const DATA_CONNECTIONS: &str = "/data/connections";
    pub const DATA_COUNTS: &str = "/data/counts";
    pub const DATA_PROTOCOL_STATS: &str = "/data/protocol-stats";
    pub const DATA_CONNECTION_PACKETS_BY_ID: &str = "/data/connection-packets/{connection_id}";

    pub const V1_CAPTURE_CANCEL: &str = "/v1/capture/cancel";
    pub const V1_CAPTURE_START: &str = "/v1/capture/start";
    pub const V1_CAPTURE_STOP: &str = "/v1/capture/stop";
    pub const V1_CAPTURE_PAUSE: &str = "/v1/capture/pause";
    pub const V1_CAPTURE_RESUME: &str = "/v1/capture/resume";
    pub const V1_CAPTURE_STATUS: &str = "/v1/capture/status";

    pub const V1_ASSETS_BULK_UPDATE: &str = "/v1/assets/bulk-update";
    pub const V1_ASSET_BY_ID: &str = "/v1/assets/{asset_id}";

    pub const V1_DATA_DEEP_PARSE_BY_IP: &str = "/v1/data/deep-parse/{ip_address}";
    pub const V1_DATA_FUNCTION_CODE_STATS: &str = "/v1/data/function-code-stats";
    pub const V1_DATA_TIMELINE_RANGE: &str = "/v1/data/timeline-range";
}

pub(crate) mod physical {
    pub const V1_PHYSICAL_TOPOLOGY: &str = "/v1/physical/topology";
    pub const V1_PHYSICAL_CISCO_CONFIG: &str = "/v1/physical/cisco-config";
    pub const V1_PHYSICAL_MAC_TABLE: &str = "/v1/physical/mac-table";
    pub const V1_PHYSICAL_CDP_NEIGHBORS: &str = "/v1/physical/cdp-neighbors";
    pub const V1_PHYSICAL_ARP_TABLE: &str = "/v1/physical/arp-table";
    pub const V1_PHYSICAL_NETWORK_CONFIG: &str = "/v1/physical/network-config";
    pub const V1_PHYSICAL_MAC_TABLE_AUTO: &str = "/v1/physical/mac-table-auto";
    pub const V1_PHYSICAL_NEIGHBOR_TABLE: &str = "/v1/physical/neighbor-table";
    pub const V1_PHYSICAL_INFERENCE_RUN: &str = "/v1/physical/inference/run";
    pub const V1_PHYSICAL_INFERENCE: &str = "/v1/physical/inference";
}

pub(crate) mod ingest_wireshark {
    pub const V1_INGEST_ZEEK: &str = "/v1/ingest/zeek";
    pub const V1_INGEST_SURICATA: &str = "/v1/ingest/suricata";
    pub const V1_INGEST_NMAP: &str = "/v1/ingest/nmap";
    pub const V1_INGEST_MASSCAN: &str = "/v1/ingest/masscan";
    pub const V1_INGEST_WAZUH: &str = "/v1/ingest/wazuh";
    pub const V1_INGEST_SINEMA: &str = "/v1/ingest/sinema";
    pub const V1_INGEST_TIA: &str = "/v1/ingest/tia";
    pub const V1_INGEST_ZEEK_DEVICE_EVENTS_BY_IP: &str =
        "/v1/ingest/zeek-device-events/{device_ip}";

    pub const V1_WIRESHARK_INFO: &str = "/v1/wireshark/info";
    pub const V1_WIRESHARK_OPEN_CONNECTION: &str = "/v1/wireshark/open-connection";
    pub const V1_WIRESHARK_OPEN_NODE: &str = "/v1/wireshark/open-node";
    pub const V1_WIRESHARK_FRAMES_BY_CONNECTION_ID: &str = "/v1/wireshark/frames/{connection_id}";
    pub const V1_WIRESHARK_FRAMES_CSV_BY_CONNECTION_ID: &str =
        "/v1/wireshark/frames/{connection_id}/csv";
}

pub(crate) mod signatures_patterns_correlation {
    pub const V1_SIGNATURES: &str = "/v1/signatures";
    pub const V1_SIGNATURES_RELOAD: &str = "/v1/signatures/reload";
    pub const V1_SIGNATURES_TEST: &str = "/v1/signatures/test";

    pub const V1_PATTERNS_CONNECTION_STATS: &str = "/v1/patterns/connection-stats";
    pub const V1_PATTERNS_ANOMALIES: &str = "/v1/patterns/anomalies";
    pub const V1_PATTERNS_REDUNDANCY_PROTOCOLS: &str = "/v1/patterns/redundancy-protocols";

    pub const V1_CORRELATION_ALERTS: &str = "/v1/correlation/alerts";
    pub const V1_CORRELATION_ALERTS_BY_IP: &str = "/v1/correlation/alerts/{ip}";
}

pub(crate) mod system_exports_segmentation {
    pub const V1_SYSTEM_SETTINGS: &str = "/v1/system/settings";
    pub const V1_SYSTEM_PLUGINS: &str = "/v1/system/plugins";

    pub const V1_EXPORTS_ASSETS_CSV: &str = "/v1/exports/assets/csv";
    pub const V1_EXPORTS_CONNECTIONS_CSV: &str = "/v1/exports/connections/csv";
    pub const V1_EXPORTS_TOPOLOGY_JSON: &str = "/v1/exports/topology/json";
    pub const V1_EXPORTS_ASSETS_JSON: &str = "/v1/exports/assets/json";
    pub const V1_EXPORTS_REPORT_PDF: &str = "/v1/exports/report/pdf";
    pub const V1_EXPORTS_SBOM: &str = "/v1/exports/sbom";
    pub const V1_EXPORTS_STIX: &str = "/v1/exports/stix";
    pub const V1_EXPORTS_TOPOLOGY_IMAGE: &str = "/v1/exports/topology/image";
    pub const V1_EXPORTS_PCAP_FILTERED: &str = "/v1/exports/pcap/filtered";
    pub const V1_EXPORTS_ALLOWLIST: &str = "/v1/exports/allowlist";
    pub const V1_EXPORTS_ALLOWLIST_CSV: &str = "/v1/exports/allowlist/csv";
    pub const V1_EXPORTS_FIREWALL_RULES: &str = "/v1/exports/firewall-rules";

    pub const V1_SEGMENTATION_RUN: &str = "/v1/segmentation/run";
    pub const V1_SEGMENTATION_ENFORCEMENT_CONFIG: &str = "/v1/segmentation/enforcement-config";
}

pub(crate) mod projects_sessions_analysis_events {
    pub const V1_PROJECTS: &str = "/v1/projects";
    pub const V1_PROJECTS_ACTIVE: &str = "/v1/projects/active";
    pub const V1_PROJECT_BY_ID: &str = "/v1/projects/{id}";

    pub const V1_SESSIONS: &str = "/v1/sessions";
    pub const V1_SESSIONS_IMPORT: &str = "/v1/sessions/import";
    pub const V1_SESSIONS_COMPARE: &str = "/v1/sessions/compare";
    pub const V1_SESSION_LOAD_BY_ID: &str = "/v1/sessions/{id}/load";
    pub const V1_SESSION_EXPORT_BY_ID: &str = "/v1/sessions/{id}/export";
    pub const V1_SESSION_BY_ID: &str = "/v1/sessions/{id}";

    pub const V1_ANALYSIS_RUN: &str = "/v1/analysis/run";
    pub const V1_ANALYSIS_FINDINGS: &str = "/v1/analysis/findings";
    pub const V1_ANALYSIS_PURDUE: &str = "/v1/analysis/purdue";
    pub const V1_ANALYSIS_ANOMALIES: &str = "/v1/analysis/anomalies";
    pub const V1_ANALYSIS_CREDENTIALS: &str = "/v1/analysis/credentials";
    pub const V1_ANALYSIS_CRITICALITY: &str = "/v1/analysis/criticality";
    pub const V1_ANALYSIS_NAMING_SUGGESTIONS: &str = "/v1/analysis/naming-suggestions";
    pub const V1_ANALYSIS_MALWARE: &str = "/v1/analysis/malware";
    pub const V1_ANALYSIS_SWITCH_SECURITY: &str = "/v1/analysis/switch-security";
    pub const V1_ANALYSIS_COMPLIANCE: &str = "/v1/analysis/compliance";
    pub const V1_ANALYSIS_CVE: &str = "/v1/analysis/cve";

    pub const V1_EVENTS: &str = "/v1/events";
}

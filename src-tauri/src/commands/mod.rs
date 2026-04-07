pub mod analysis;
pub mod baseline;
pub mod capture;
pub mod error;
pub mod correlation;
pub mod data;
pub mod export;
pub mod handlers;
pub mod ingest;
pub mod patterns;
pub mod physical;
pub mod processor;
pub mod projects;
pub mod protocol_handler;
pub mod resource_paths;
pub mod segmentation;
pub mod session;
pub mod signatures;
pub mod system;
pub mod wireshark;

use gm_analysis::{AnomalyScore, ConnectionStats, Finding, PatternAnomaly, PurdueAssignment};
use gm_capture::LiveCaptureHandle;
use gm_db::{Database, GeoIpLookup, OuiLookup};
pub use gm_ingest::{DeviceZeekEvents, StoredAlert, ZeekEventSummary};
pub use gm_models::{
    AssetInfo, AssetSignatureMatch, ConnectionInfo, PacketSummary, ProtocolStatInfo,
};
use gm_parsers::IcsProtocol;
pub use gm_parsers::{
    BacnetDetail, DeepParseInfo, Dnp3Detail, Dnp3Relationship, EnipDetail, FunctionCodeStat,
    Iec104Detail, LldpDetail, ModbusDetail, ModbusDeviceIdInfo, ModbusRelationship, PollingInterval,
    ProfinetDcpDetail, RegisterRangeInfo, S7Detail, SnmpDetail,
};
use gm_parsers::RedundancyInfo;
use gm_physical::{InferredTopology, PhysicalTopology};
use gm_segmentation::SegmentationReport;
use gm_signatures::SignatureEngine;
use gm_topology::TopologyGraph;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

/// Shared application state, managed by Tauri.
///
/// This is wrapped in Mutex for thread-safe access from command handlers.
/// Tauri's state management ensures this is available to all commands
/// via the `State<'_, AppState>` parameter.
pub struct AppState {
    pub inner: Mutex<AppStateInner>,
    /// Set to true to cancel an in-progress PCAP import. Lives outside the
    /// Mutex so it can be read/written by the import thread and the cancel
    /// command without acquiring the heavy state lock.
    pub import_cancelled: Arc<AtomicBool>,
}

pub struct AppStateInner {
    /// The current network topology graph
    pub topology: TopologyGraph,
    /// All discovered assets
    pub assets: Vec<AssetInfo>,
    /// All observed connections
    pub connections: Vec<ConnectionInfo>,
    /// Packet summaries grouped by connection ID, for the connection tree
    pub packet_summaries: HashMap<String, Vec<PacketSummary>>,
    /// List of imported PCAP files
    pub imported_files: Vec<String>,
    /// Signature engine for device fingerprinting
    pub signature_engine: SignatureEngine,
    /// Deep parse results grouped by IP address
    pub deep_parse_info: HashMap<String, DeepParseInfo>,
    /// Handle to the running live capture (None if not capturing)
    pub live_capture: Option<LiveCaptureHandle>,
    /// Join handle for the live capture processing thread
    pub processing_thread: Option<JoinHandle<()>>,
    /// IEEE OUI vendor lookup table
    pub oui_lookup: OuiLookup,
    /// GeoIP country lookup
    pub geoip_lookup: GeoIpLookup,
    /// SQLite database for persistence
    pub db: Option<Database>,
    /// Currently loaded session ID (None if no session loaded)
    pub current_session_id: Option<String>,
    /// Currently loaded session name
    pub current_session_name: Option<String>,
    /// Active project ID (None if no project selected)
    pub current_project_id: Option<i64>,
    /// Physical topology from Cisco/JunOS/Aruba config/CAM/CDP/ARP imports
    pub physical_topology: PhysicalTopology,
    /// Traffic-inferred topology from packet analysis
    pub inferred_topology: Option<InferredTopology>,
    /// Security findings from the last analysis run
    pub findings: Vec<Finding>,
    /// Purdue level assignments from the last analysis run
    pub purdue_assignments: Vec<PurdueAssignment>,
    /// Anomaly scores from the last analysis run
    pub anomalies: Vec<AnomalyScore>,
    /// Per-connection timing statistics (computed after import / capture)
    pub connection_stats: Vec<ConnectionStats>,
    /// Communication pattern anomalies (computed alongside connection_stats)
    pub pattern_anomalies: Vec<PatternAnomaly>,
    /// Redundancy protocol frames observed (MRP/RSTP/HSR/PRP/DLR)
    pub redundancy_protocols: Vec<RedundancyInfo>,
    /// Alerts imported from external IDS/SIEM tools (Suricata, Wazuh)
    pub imported_alerts: Vec<StoredAlert>,
    /// Per-device Zeek event summaries (rebuilt on each Zeek import)
    pub zeek_device_events: HashMap<String, DeviceZeekEvents>,
    /// Cached result of the last segmentation analysis run (Phase 15)
    pub segmentation_report: Option<SegmentationReport>,
}

impl AppState {
    pub fn new(paths: resource_paths::ResourcePaths) -> Self {
        let mut engine = SignatureEngine::new();

        // Load signatures from the resolved directory.
        if paths.signatures_dir.exists() {
            match engine.load_directory(&paths.signatures_dir) {
                Ok(count) => log::info!(
                    "Loaded {} signatures from {}",
                    count,
                    paths.signatures_dir.display()
                ),
                Err(e) => log::warn!(
                    "Failed to load signatures from {}: {}",
                    paths.signatures_dir.display(),
                    e
                ),
            }
        } else {
            log::warn!(
                "Signatures directory not found: {}",
                paths.signatures_dir.display()
            );
        }

        // Load OUI database
        let oui_lookup = OuiLookup::load_from_file(&paths.oui_path)
            .unwrap_or_else(|e| {
                log::warn!("Failed to load OUI from {}: {}", paths.oui_path.display(), e);
                OuiLookup::empty()
            });

        // Load GeoIP database
        let geoip_lookup = GeoIpLookup::load_from_file(&paths.geoip_path)
            .unwrap_or_else(|e| {
                log::warn!(
                    "Failed to load GeoIP from {}: {}",
                    paths.geoip_path.display(),
                    e
                );
                GeoIpLookup::empty()
            });

        // Open SQLite database at ~/.kusanaginokajiki/data.db
        let db = match dirs::home_dir() {
            Some(home) => {
                let db_path = home.join(".kusanaginokajiki").join("data.db");
                match Database::open(&db_path) {
                    Ok(db) => Some(db),
                    Err(e) => {
                        log::warn!("Failed to open database at {}: {}", db_path.display(), e);
                        None
                    }
                }
            }
            None => {
                log::warn!("Could not determine home directory for database");
                None
            }
        };

        AppState {
            import_cancelled: Arc::new(AtomicBool::new(false)),
            inner: Mutex::new(AppStateInner {
                topology: TopologyGraph::default(),
                assets: Vec::new(),
                connections: Vec::new(),
                packet_summaries: HashMap::new(),
                imported_files: Vec::new(),
                signature_engine: engine,
                deep_parse_info: HashMap::new(),
                live_capture: None,
                processing_thread: None,
                oui_lookup,
                geoip_lookup,
                db,
                current_session_id: None,
                current_session_name: None,
                current_project_id: None,
                physical_topology: PhysicalTopology::default(),
                inferred_topology: None,
                findings: Vec::new(),
                purdue_assignments: Vec::new(),
                anomalies: Vec::new(),
                connection_stats: Vec::new(),
                pattern_anomalies: Vec::new(),
                redundancy_protocols: Vec::new(),
                imported_alerts: Vec::new(),
                zeek_device_events: HashMap::new(),
                segmentation_report: None,
            }),
        }
    }
}

/// Infer device type based on which protocols it speaks and its role.
pub fn infer_device_type(protocols: &[IcsProtocol], is_server: bool) -> String {
    // If it responds on OT protocol ports, it's likely an OT device
    let has_modbus = protocols.contains(&IcsProtocol::Modbus);
    let has_dnp3 = protocols.contains(&IcsProtocol::Dnp3);
    let has_ethernet_ip = protocols.contains(&IcsProtocol::EthernetIp);
    let has_s7 = protocols.contains(&IcsProtocol::S7comm);
    let has_bacnet = protocols.contains(&IcsProtocol::Bacnet);
    let has_opc_ua = protocols.contains(&IcsProtocol::OpcUa);
    let has_ge_srtp = protocols.contains(&IcsProtocol::GeSrtp);
    let has_suitelink = protocols.contains(&IcsProtocol::WonderwareSuitelink);

    let ot_protocol_count = protocols.iter().filter(|p| p.is_ot()).count();

    if is_server && ot_protocol_count >= 1 {
        // Server responding on OT ports → likely PLC/RTU
        if has_ethernet_ip || has_s7 || has_ge_srtp || has_bacnet {
            // Allen-Bradley (EtherNet/IP), Siemens (S7), GE (SRTP), BACnet controller
            "plc".to_string()
        } else if has_modbus || has_dnp3 {
            "rtu".to_string()
        } else {
            "unknown".to_string()
        }
    } else if has_suitelink && is_server {
        "scada_server".to_string() // Wonderware SuiteLink server
    } else if ot_protocol_count >= 2 {
        // Client talking multiple OT protocols → likely HMI or SCADA server
        "hmi".to_string()
    } else if has_opc_ua && ot_protocol_count == 1 {
        "historian".to_string()
    } else if ot_protocol_count == 0 {
        "it_device".to_string()
    } else {
        "unknown".to_string()
    }
}

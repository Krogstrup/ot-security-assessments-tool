pub mod analysis;
pub mod baseline;
pub mod capture;
pub mod correlation;
pub mod data;
pub mod error;
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
use gm_parsers::RedundancyInfo;
pub use gm_parsers::{
    BacnetDetail, DeepParseInfo, Dnp3Detail, Dnp3Relationship, EnipDetail, FunctionCodeStat,
    Iec104Detail, LldpDetail, ModbusDetail, ModbusDeviceIdInfo, ModbusRelationship,
    PollingInterval, ProfinetDcpDetail, RegisterRangeInfo, S7Detail, SnmpDetail,
};
use gm_physical::{InferredTopology, PhysicalTopology};
use gm_segmentation::SegmentationReport;
use gm_signatures::SignatureEngine;
use gm_topology::TopologyGraph;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex, RwLock};
use std::thread::JoinHandle;
use tokio::sync::broadcast;

// ── Domain state structs ─────────────────────────────────────────────────────
//
// Lock acquisition order (must be followed everywhere to avoid deadlocks):
//   capture → inventory → analysis → session → physical → segmentation → signatures

/// Capture and topology state: network graph, connections, live capture handle.
pub struct CaptureState {
    pub topology: TopologyGraph,
    pub connections: Vec<ConnectionInfo>,
    pub packet_summaries: HashMap<String, Vec<PacketSummary>>,
    pub imported_files: Vec<String>,
    pub live_capture: Option<LiveCaptureHandle>,
    pub processing_thread: Option<JoinHandle<()>>,
    pub redundancy_protocols: Vec<RedundancyInfo>,
}

/// Device inventory: assets, deep-parse results, OUI/GeoIP lookups, alert imports.
pub struct InventoryState {
    pub assets: Vec<AssetInfo>,
    pub deep_parse_info: HashMap<String, DeepParseInfo>,
    /// IEEE OUI vendor lookup table
    pub oui_lookup: OuiLookup,
    /// GeoIP country lookup
    pub geoip_lookup: GeoIpLookup,
    pub zeek_device_events: HashMap<String, DeviceZeekEvents>,
    pub imported_alerts: Vec<StoredAlert>,
}

/// Security analysis results: findings, Purdue assignments, anomaly scores.
pub struct AnalysisState {
    pub findings: Vec<Finding>,
    pub purdue_assignments: Vec<PurdueAssignment>,
    pub anomalies: Vec<AnomalyScore>,
    pub connection_stats: Vec<ConnectionStats>,
    pub pattern_anomalies: Vec<PatternAnomaly>,
}

/// Session and project persistence: SQLite DB handle + current session/project IDs.
pub struct SessionState {
    pub db: Option<Database>,
    pub current_session_id: Option<String>,
    pub current_session_name: Option<String>,
    pub current_project_id: Option<i64>,
}

/// Physical topology from switch config/CAM/CDP/ARP imports.
pub struct PhysicalState {
    pub physical_topology: PhysicalTopology,
    pub inferred_topology: Option<InferredTopology>,
}

/// Cached microsegmentation analysis result.
pub struct SegmentationState {
    pub segmentation_report: Option<SegmentationReport>,
}

/// Signature engine for device fingerprinting (read-only at runtime after init).
pub struct SignatureState {
    pub signature_engine: SignatureEngine,
}

// ── AppState ─────────────────────────────────────────────────────────────────

/// Shared application state for the Web/API runtime.
///
/// Each domain has its own lock, scoped to the set of fields it owns.
/// Use `RwLock` for read-heavy domains (concurrent UI reads) and `Mutex`
/// for domains that require serialised writes (DB, live-capture handles).
///
/// **Lock ordering**: always acquire in the order declared above
/// (capture → inventory → analysis → session → physical → segmentation → signatures)
/// to prevent deadlocks when multiple domains must be locked together.
pub struct AppState {
    /// Network topology, connections, packet data, live capture handle.
    pub capture: RwLock<CaptureState>,
    /// Device assets, deep-parse results, OUI/GeoIP, imported alerts.
    pub inventory: RwLock<InventoryState>,
    /// Findings, Purdue assignments, anomaly scores, connection statistics.
    pub analysis: RwLock<AnalysisState>,
    /// SQLite database and session/project identity.  Uses Mutex because the
    /// Database handle requires serialised access.
    pub session: Mutex<SessionState>,
    /// Physical topology from switch configs and ARP tables.
    pub physical: RwLock<PhysicalState>,
    /// Cached microsegmentation report.
    pub segmentation: RwLock<SegmentationState>,
    /// Device-fingerprint signature engine (read-only after init).
    pub signatures: RwLock<SignatureState>,
    /// Set to true to cancel an in-progress PCAP import.  Lives outside any
    /// domain lock so it can be written by the cancel command without
    /// acquiring the heavy capture lock.
    pub import_cancelled: Arc<AtomicBool>,
    /// Optional SSE broadcast channel for real-time web event streaming.
    /// Set by the web binary when event streaming is enabled.
    pub event_tx: Option<broadcast::Sender<(String, serde_json::Value)>>,
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
        let oui_lookup = OuiLookup::load_from_file(&paths.oui_path).unwrap_or_else(|e| {
            log::warn!(
                "Failed to load OUI from {}: {}",
                paths.oui_path.display(),
                e
            );
            OuiLookup::empty()
        });

        // Load GeoIP database
        let geoip_lookup = GeoIpLookup::load_from_file(&paths.geoip_path).unwrap_or_else(|e| {
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
            event_tx: None,
            capture: RwLock::new(CaptureState {
                topology: TopologyGraph::default(),
                connections: Vec::new(),
                packet_summaries: HashMap::new(),
                imported_files: Vec::new(),
                live_capture: None,
                processing_thread: None,
                redundancy_protocols: Vec::new(),
            }),
            inventory: RwLock::new(InventoryState {
                assets: Vec::new(),
                deep_parse_info: HashMap::new(),
                oui_lookup,
                geoip_lookup,
                zeek_device_events: HashMap::new(),
                imported_alerts: Vec::new(),
            }),
            analysis: RwLock::new(AnalysisState {
                findings: Vec::new(),
                purdue_assignments: Vec::new(),
                anomalies: Vec::new(),
                connection_stats: Vec::new(),
                pattern_anomalies: Vec::new(),
            }),
            session: Mutex::new(SessionState {
                db,
                current_session_id: None,
                current_session_name: None,
                current_project_id: None,
            }),
            physical: RwLock::new(PhysicalState {
                physical_topology: PhysicalTopology::default(),
                inferred_topology: None,
            }),
            segmentation: RwLock::new(SegmentationState {
                segmentation_report: None,
            }),
            signatures: RwLock::new(SignatureState {
                signature_engine: engine,
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

//! Shared domain constants and models for OT/ICS protocol analysis.
//!
//! Single source of truth for port lists, function code sets, scale
//! boundaries, and domain model types used across `gm-analysis`,
//! `gm-parsers`, and the commands layer.

// ── OT/ICS service ports ────────────────────────────────────────────────────

/// Well-known OT/ICS server-side TCP/UDP ports.
///
/// A device that *listens* on one of these ports is classified as a
/// field-level device (PLC, RTU, etc.) for role and Purdue assignment.
pub const OT_SERVER_PORTS: &[u16] = &[
    102,   // ISO-TSAP / S7comm (Siemens)
    502,   // Modbus TCP
    1089,  // FF Annunciation (Foundation Fieldbus)
    1090,  // FF Fieldbus Message Specification
    1091,  // FF System Management
    1883,  // MQTT (unencrypted)
    2222,  // EtherNet/IP implicit (UDP)
    2404,  // IEC 60870-5-104 (IEC 104)
    4840,  // OPC UA
    5007,  // CIP / DeviceNet
    5094,  // HART-IP
    8883,  // MQTT (TLS)
    18245, // GE SRTP
    18246, // GE SRTP (alternate)
    20000, // DNP3
    34962, // PROFINET RT (unicast data)
    34963, // PROFINET RT (multicast data)
    34964, // PROFINET DCP (discovery / config)
    44818, // EtherNet/IP explicit TCP
    47808, // BACnet/IP (UDP)
];

/// Ports that carry cleartext OT protocols — findings should flag these as
/// lacking transport-layer encryption.
pub const CLEARTEXT_OT_PORTS: &[u16] = &[
    502,   // Modbus TCP
    20000, // DNP3
    102,   // S7comm / ISO-TSAP
    44818, // EtherNet/IP explicit
    2222,  // EtherNet/IP implicit
    47808, // BACnet/IP
    2404,  // IEC 104
    4840,  // OPC UA (unencrypted endpoint)
];

/// Remote-access / IT management ports whose presence on an OT segment
/// warrants a T0822 (External Remote Services) finding.
pub const REMOTE_ACCESS_PORTS: &[u16] = &[
    22,   // SSH
    23,   // Telnet
    3389, // RDP
    5900, 5901, 5902, 5903, 5904, 5905, 5906, 5907, 5908, 5909, 5910, // VNC
    5938, // TeamViewer
    7070, // AnyDesk
];

// ── Modbus function codes ────────────────────────────────────────────────────

/// Modbus write function codes (FC 5, 6, 15, 16).
///
/// Used to identify write-capable masters and to detect T0855
/// (Unauthorized Command Message).
pub const MODBUS_WRITE_FCS: &[u8] = &[5, 6, 15, 16];

/// Modbus FC 8 — Diagnostics.  Sent to diagnose/reset a remote device.
/// Receipt from a non-engineering-workstation triggers T0814.
pub const MODBUS_DIAGNOSTIC_FC: u8 = 8;

// ── Confidence scale ────────────────────────────────────────────────────────

/// Maximum value on the device-identification confidence scale (0–5).
///
/// | Score | Meaning            |
/// |-------|--------------------|
/// | 0     | No identification  |
/// | 1     | Port-based only    |
/// | 2     | Payload pattern    |
/// | 3     | MAC OUI match      |
/// | 4     | Payload deep parse |
/// | 5     | Full deep parse    |
pub const CONFIDENCE_MAX: u8 = 5;

// ── Purdue Model levels ──────────────────────────────────────────────────────

/// Maximum Purdue level index used in this tool (0 = field/process, 5 = DMZ).
pub const PURDUE_LEVEL_MAX: u8 = 5;

// ── Runtime caps ─────────────────────────────────────────────────────────────

/// Maximum topology nodes returned by the data API.  Excess nodes (ordered by
/// descending packet count) are dropped to prevent the frontend from rendering
/// a graph too large to be useful.
pub const MAX_TOPOLOGY_NODES: usize = 5_000;

/// Maximum topology edges returned by the data API.
pub const MAX_TOPOLOGY_EDGES: usize = 20_000;

/// Maximum anomaly scores returned by the analysis API in a single response.
pub const MAX_ANOMALY_RESULTS: usize = 500;

/// Maximum findings returned by the findings API in a single response.
pub const MAX_FINDINGS: usize = 1_000;

/// Packet batch size for the live-capture processing thread.  Once this many
/// packets have accumulated (or the flush interval elapses) the batch is
/// committed to shared state.
pub const LIVE_CAPTURE_BATCH_SIZE: usize = 500;

// ── Device type strings ───────────────────────────────────────────────────────

/// Canonical device type string constants.
///
/// Use these instead of inline string literals so that typos are caught by
/// referencing code and renaming is a one-line change. These map 1:1 to the
/// `device_type` field on `AssetInfo` / `AssetSnapshot`.
pub const DEVICE_TYPE_UNKNOWN: &str = "unknown";
pub const DEVICE_TYPE_PLC: &str = "plc";
pub const DEVICE_TYPE_RTU: &str = "rtu";
pub const DEVICE_TYPE_HMI: &str = "hmi";
pub const DEVICE_TYPE_HISTORIAN: &str = "historian";
pub const DEVICE_TYPE_EWS: &str = "engineering_workstation";
pub const DEVICE_TYPE_SCADA_SERVER: &str = "scada_server";
pub const DEVICE_TYPE_IO_SERVER: &str = "io_server";
pub const DEVICE_TYPE_FIELD_DEVICE: &str = "field_device";
pub const DEVICE_TYPE_CONTROLLER: &str = "controller";
pub const DEVICE_TYPE_SWITCH: &str = "switch";
pub const DEVICE_TYPE_ROUTER: &str = "router";
pub const DEVICE_TYPE_FIREWALL: &str = "firewall";
pub const DEVICE_TYPE_IT_DEVICE: &str = "it_device";
pub const DEVICE_TYPE_WORKSTATION: &str = "workstation";

/// OT/ICS field-device types.
///
/// Aggregate of the `DEVICE_TYPE_*` constants that represent operational
/// technology endpoints.  Use this slice for membership tests instead of
/// duplicating the list at each call site.
pub const OT_DEVICE_TYPES: &[&str] = &[
    DEVICE_TYPE_PLC,
    DEVICE_TYPE_RTU,
    DEVICE_TYPE_HMI,
    DEVICE_TYPE_HISTORIAN,
    DEVICE_TYPE_EWS,
    DEVICE_TYPE_SCADA_SERVER,
    DEVICE_TYPE_IO_SERVER,
    DEVICE_TYPE_FIELD_DEVICE,
    DEVICE_TYPE_CONTROLLER,
];

// ── OT protocol names ─────────────────────────────────────────────────────────

/// Canonical PascalCase OT/ICS protocol name strings.
///
/// These match the values stored in `AssetSnapshot::protocols` and
/// `ConnectionSnapshot::protocol` when populated from the connection layer.
/// Use this slice for membership tests instead of maintaining per-module
/// copies that risk drifting out of sync.
pub const OT_PROTOCOL_NAMES: &[&str] = &[
    "Modbus",
    "Dnp3",
    "EthernetIp",
    "S7comm",
    "Bacnet",
    "OpcUa",
    "Iec104",
    "Profinet",
    "ProfinetDcp",
    "HartIp",
    "GeSrtp",
    "WonderwareSuitelink",
    "FoundationFieldbus",
    "FfHse",
    "Mqtt",
    "Snmp",
];

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Returns `true` if `port` is a recognised OT/ICS server-side port.
///
/// Equivalent to `OT_SERVER_PORTS.contains(&port)` but expressed as a
/// named function so call sites read more clearly.
#[inline]
pub fn is_ot_server_port(port: u16) -> bool {
    OT_SERVER_PORTS.contains(&port)
}

/// Returns the canonical protocol name for a well-known OT port, or `None`
/// if the port is not in the OT port list.
pub fn ot_port_to_protocol(port: u16) -> Option<&'static str> {
    match port {
        102 => Some("S7comm"),
        502 => Some("Modbus"),
        1089 | 1090 | 1091 => Some("FoundationFieldbus"),
        1883 | 8883 => Some("MQTT"),
        2222 => Some("EtherNetIP"),
        2404 => Some("IEC104"),
        4840 => Some("OpcUA"),
        5007 => Some("CIP"),
        5094 => Some("HART-IP"),
        18245 | 18246 => Some("GESRTP"),
        20000 => Some("DNP3"),
        34962 | 34963 | 34964 => Some("PROFINET"),
        44818 => Some("EtherNetIP"),
        47808 => Some("BACnet"),
        _ => None,
    }
}

// ── Domain models ─────────────────────────────────────────────────────────────

use serde::{Deserialize, Serialize};

#[cfg(test)]
use ts_rs::TS;

/// Asset information stored in application state.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export, export_to = "../../bindings/gen/types/"))]
pub struct AssetInfo {
    pub id: String,
    pub ip_address: String,
    pub mac_address: Option<String>,
    pub hostname: Option<String>,
    pub device_type: String,
    pub vendor: Option<String>,
    pub protocols: Vec<String>,
    pub first_seen: String,
    pub last_seen: String,
    pub notes: String,
    pub purdue_level: Option<u8>,
    pub tags: Vec<String>,
    pub packet_count: u64,
    /// Overall confidence score (1-5), highest from any signature match
    pub confidence: u8,
    /// Vendor-specific product identification from signatures
    pub product_family: Option<String>,
    /// All signature matches for this asset
    pub signature_matches: Vec<AssetSignatureMatch>,
    /// Vendor name from IEEE OUI database (MAC prefix lookup)
    #[serde(default)]
    pub oui_vendor: Option<String>,
    /// ISO 3166-1 alpha-2 country code (public IPs only)
    #[serde(default)]
    pub country: Option<String>,
    /// Whether this IP is a public (routable) address
    #[serde(default)]
    pub is_public_ip: bool,
}

/// A signature match result attached to an asset.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export, export_to = "../../bindings/gen/types/"))]
pub struct AssetSignatureMatch {
    pub signature_name: String,
    pub confidence: u8,
    pub vendor: Option<String>,
    pub product_family: Option<String>,
    pub device_type: Option<String>,
    pub role: Option<String>,
}

/// Connection information stored in application state.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export, export_to = "../../bindings/gen/types/"))]
pub struct ConnectionInfo {
    pub id: String,
    pub src_ip: String,
    pub src_port: u16,
    pub src_mac: Option<String>,
    pub dst_ip: String,
    pub dst_port: u16,
    pub dst_mac: Option<String>,
    pub protocol: String,
    pub transport: String,
    pub packet_count: u64,
    pub byte_count: u64,
    pub first_seen: String,
    pub last_seen: String,
    /// Which PCAP files contributed packets to this connection
    pub origin_files: Vec<String>,
}

/// Lightweight packet summary for the connection tree detail view.
/// Full payload is not included — this is for display only.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export, export_to = "../../bindings/gen/types/"))]
pub struct PacketSummary {
    pub timestamp: String,
    pub src_ip: String,
    pub dst_ip: String,
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: String,
    pub length: usize,
    pub origin_file: String,
}

/// Protocol statistics.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export, export_to = "../../bindings/gen/types/"))]
pub struct ProtocolStatInfo {
    pub protocol: String,
    pub packet_count: u64,
    pub byte_count: u64,
    pub connection_count: u64,
    pub unique_devices: u64,
}

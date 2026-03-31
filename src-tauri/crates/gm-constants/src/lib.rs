//! Shared domain constants for OT/ICS protocol analysis.
//!
//! Single source of truth for port lists, function code sets, and scale
//! boundaries used across `gm-analysis`, `gm-parsers`, and the commands
//! layer. All items are `pub const` or `pub fn` — no heap allocation, no
//! external dependencies.

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

//! Alert and Zeek event view models for the command layer.
//!
//! These types represent data imported from external IDS/SIEM tools
//! (Suricata, Wazuh, Zeek) after it has been accumulated into app state.

#[cfg(test)]
use ts_rs::TS;

use serde::{Deserialize, Serialize};

/// An alert imported from an external IDS/SIEM and stored in AppState.
///
/// Flattened from IngestedAlert for direct serialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export, export_to = "../../bindings/gen/types/"))]
pub struct StoredAlert {
    /// RFC 3339 timestamp of the alert
    pub timestamp: String,
    pub src_ip: String,
    pub src_port: u16,
    pub dst_ip: String,
    pub dst_port: u16,
    /// IDS signature/rule ID (e.g. Suricata SID or Wazuh rule ID)
    pub signature_id: u64,
    /// Human-readable rule description
    pub signature: String,
    /// Alert category (e.g. "Attempted Attack", "ics")
    pub category: String,
    /// Severity: 1 = high, 2 = medium, 3 = low
    pub severity: u8,
    /// Source tool name: "Suricata" or "Wazuh"
    pub source: String,
}

/// A single Zeek-observed event summarised for display.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export, export_to = "../../bindings/gen/types/"))]
pub struct ZeekEventSummary {
    /// RFC 3339 timestamp (connection first_seen)
    pub timestamp: String,
    /// Zeek log type: "conn", "modbus", "dnp3", "s7comm", "dns", "http"
    pub log_type: String,
    /// The remote peer IP address
    pub peer_ip: String,
    /// One-line human-readable summary
    pub summary: String,
}

/// Per-device aggregate of Zeek-observed events.
///
/// Built from connections that have "[Zeek]" in their origin_files.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export, export_to = "../../bindings/gen/types/"))]
pub struct DeviceZeekEvents {
    pub device_ip: String,
    /// Total conn.log-type entries for this device
    pub conn_log_entries: u32,
    pub modbus_events: u32,
    pub dnp3_events: u32,
    pub dns_queries: u32,
    pub http_requests: u32,
    /// Unique peer IPs this device communicated with in Zeek data
    pub unique_peers: u32,
    /// Number of Suricata/Wazuh alerts correlated with this device
    pub alert_count: u32,
    /// Sample events (capped at 50)
    pub sample_events: Vec<ZeekEventSummary>,
}

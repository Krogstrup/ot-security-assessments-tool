//! # gm-models
//!
//! Shared domain models that cross crate boundaries: assets, connections,
//! packet summaries, and protocol statistics.
//!
//! These types are produced by the packet processing pipeline and consumed
//! by the command layer, export, session, and analysis crates.

#[cfg(test)]
use ts_rs::TS;

use serde::{Deserialize, Serialize};

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

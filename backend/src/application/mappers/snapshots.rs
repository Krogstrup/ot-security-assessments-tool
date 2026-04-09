//! Canonical snapshot mappers: convert in-memory state slices into the
//! lightweight `AssetSnapshot` / `ConnectionSnapshot` types consumed by
//! `gm-analysis` (analysis, allowlist, segmentation, etc.).

use gm_analysis::{AssetSnapshot, ConnectionSnapshot};
use gm_types::{AssetInfo, ConnectionInfo};

/// Map every asset to an `AssetSnapshot`.
pub fn asset_snapshots(assets: &[AssetInfo]) -> Vec<AssetSnapshot> {
    assets
        .iter()
        .map(|a| AssetSnapshot {
            ip_address: a.ip_address.clone(),
            device_type: a.device_type.clone(),
            protocols: a.protocols.clone(),
            purdue_level: a.purdue_level,
            is_public_ip: a.is_public_ip,
            tags: a.tags.clone(),
            vendor: a.vendor.clone(),
            hostname: a.hostname.clone(),
            product_family: a.product_family.clone(),
        })
        .collect()
}

/// Map every connection to a `ConnectionSnapshot`.
pub fn connection_snapshots(connections: &[ConnectionInfo]) -> Vec<ConnectionSnapshot> {
    connections
        .iter()
        .map(|c| ConnectionSnapshot {
            src_ip: c.src_ip.clone(),
            dst_ip: c.dst_ip.clone(),
            src_port: c.src_port,
            dst_port: c.dst_port,
            protocol: c.protocol.clone(),
            packet_count: c.packet_count,
        })
        .collect()
}

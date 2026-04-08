//! Canonical snapshot mappers: convert in-memory state slices into the
//! lightweight `AssetSnapshot` / `ConnectionSnapshot` types consumed by
//! `gm-analysis` (analysis, allowlist, segmentation, etc.).
//!
//! # Layering note
//! These mappers borrow `CaptureState` / `InventoryState` from the commands
//! module, which is a known interim dependency on the interface layer.
//! These state types will move to `application/state` in a future step.

use gm_analysis::{AssetSnapshot, ConnectionSnapshot};

use crate::commands::{CaptureState, InventoryState};

/// Map every asset in `InventoryState` to an `AssetSnapshot`.
pub fn asset_snapshots(inventory: &InventoryState) -> Vec<AssetSnapshot> {
    inventory
        .assets
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

/// Map every connection in `CaptureState` to a `ConnectionSnapshot`.
pub fn connection_snapshots(capture: &CaptureState) -> Vec<ConnectionSnapshot> {
    capture
        .connections
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

//! Asset upsert helpers: enrich existing assets or create new ones from ingested data.

use gm_analysis::infer_device_type;
use gm_ingest::IngestedAsset;
use gm_parsers::IcsProtocol;

use crate::commands::AssetInfo;

/// Enrich an existing asset with data from an ingested asset.
pub(super) fn enrich_asset(existing: &mut AssetInfo, ingested: &IngestedAsset, is_active: bool) {
    for proto in &ingested.protocols {
        if !existing.protocols.contains(proto) {
            existing.protocols.push(proto.clone());
        }
    }
    if existing.hostname.is_none() && ingested.hostname.is_some() {
        existing.hostname = ingested.hostname.clone();
    }
    if existing.vendor.is_none() && ingested.vendor.is_some() {
        existing.vendor = ingested.vendor.clone();
    }
    if let Some(ref os) = ingested.os_info {
        let os_note = format!(
            "[{}] OS: {}",
            if is_active { "scan" } else { "passive" },
            os
        );
        if !existing.notes.contains(&os_note) {
            if !existing.notes.is_empty() {
                existing.notes.push_str("; ");
            }
            existing.notes.push_str(&os_note);
        }
    }
    let source_tag = format!("[{}]", ingested.source.display_name());
    if !existing.tags.contains(&source_tag) {
        existing.tags.push(source_tag);
    }
    if is_active && !existing.tags.contains(&"[active-scan]".to_string()) {
        existing.tags.push("[active-scan]".to_string());
    }
}

/// Create a new [`AssetInfo`] from ingested data.
pub(super) fn create_asset_from_ingested(ingested: &IngestedAsset, is_active: bool) -> AssetInfo {
    let mut tags = vec![format!("[{}]", ingested.source.display_name())];
    if is_active {
        tags.push("[active-scan]".to_string());
    }

    let mut notes = String::new();
    if let Some(ref os) = ingested.os_info {
        notes = format!(
            "[{}] OS: {}",
            if is_active { "scan" } else { "passive" },
            os
        );
    }

    let protocols_as_ics: Vec<IcsProtocol> = ingested
        .protocols
        .iter()
        .map(|p| IcsProtocol::from_name(p))
        .collect();
    let has_server_ports = ingested
        .open_ports
        .iter()
        .any(|p| gm_types::OT_SERVER_PORTS.contains(&p.port));
    let device_type = ingested
        .device_type
        .clone()
        .unwrap_or_else(|| infer_device_type(&protocols_as_ics, has_server_ports));

    AssetInfo {
        id: ingested.ip_address.clone(),
        ip_address: ingested.ip_address.clone(),
        mac_address: ingested.mac_address.clone(),
        hostname: ingested.hostname.clone(),
        device_type,
        vendor: ingested.vendor.clone(),
        protocols: ingested.protocols.clone(),
        first_seen: String::new(),
        last_seen: String::new(),
        notes,
        purdue_level: None,
        tags,
        packet_count: 0,
        confidence: if ingested.vendor.is_some() { 2 } else { 1 },
        product_family: None,
        signature_matches: Vec::new(),
        oui_vendor: None,
        country: None,
        is_public_ip: gm_db::GeoIpLookup::is_public_ip(&ingested.ip_address),
    }
}

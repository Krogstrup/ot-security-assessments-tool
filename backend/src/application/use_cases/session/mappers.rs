//! DB row ↔ domain type converters for session persistence.

use gm_db::{AssetRow, ConnectionRow};

use crate::commands::{AssetInfo, ConnectionInfo};

pub fn asset_info_to_row(asset: &AssetInfo, session_id: &str) -> AssetRow {
    AssetRow {
        id: asset.id.clone(),
        session_id: session_id.to_string(),
        ip_address: asset.ip_address.clone(),
        mac_address: asset.mac_address.clone(),
        hostname: asset.hostname.clone(),
        device_type: asset.device_type.clone(),
        vendor: asset.vendor.clone(),
        product_family: asset.product_family.clone(),
        protocols: serde_json::to_string(&asset.protocols).unwrap_or_else(|_| "[]".to_string()),
        confidence: asset.confidence as i64,
        purdue_level: asset.purdue_level.map(|l| l as i64),
        tags: serde_json::to_string(&asset.tags).unwrap_or_else(|_| "[]".to_string()),
        notes: asset.notes.clone(),
        packet_count: asset.packet_count as i64,
        signature_matches: serde_json::to_string(&asset.signature_matches)
            .unwrap_or_else(|_| "[]".to_string()),
        oui_vendor: asset.oui_vendor.clone(),
        country: asset.country.clone(),
        is_public_ip: asset.is_public_ip,
        first_seen: asset.first_seen.clone(),
        last_seen: asset.last_seen.clone(),
    }
}

pub fn connection_info_to_row(conn: &ConnectionInfo, session_id: &str) -> ConnectionRow {
    ConnectionRow {
        id: conn.id.clone(),
        session_id: session_id.to_string(),
        src_ip: conn.src_ip.clone(),
        src_port: conn.src_port as i64,
        src_mac: conn.src_mac.clone(),
        dst_ip: conn.dst_ip.clone(),
        dst_port: conn.dst_port as i64,
        dst_mac: conn.dst_mac.clone(),
        protocol: conn.protocol.clone(),
        transport: conn.transport.clone(),
        packet_count: conn.packet_count as i64,
        byte_count: conn.byte_count as i64,
        first_seen: conn.first_seen.clone(),
        last_seen: conn.last_seen.clone(),
        origin_files: serde_json::to_string(&conn.origin_files)
            .unwrap_or_else(|_| "[]".to_string()),
    }
}

pub fn row_to_asset_info(row: AssetRow) -> AssetInfo {
    let protocols: Vec<String> = serde_json::from_str(&row.protocols).unwrap_or_default();
    let tags: Vec<String> = serde_json::from_str(&row.tags).unwrap_or_default();
    let signature_matches = serde_json::from_str(&row.signature_matches).unwrap_or_default();

    AssetInfo {
        id: row.id,
        ip_address: row.ip_address,
        mac_address: row.mac_address,
        hostname: row.hostname,
        device_type: row.device_type,
        vendor: row.vendor,
        protocols,
        first_seen: row.first_seen,
        last_seen: row.last_seen,
        notes: row.notes,
        purdue_level: row.purdue_level.map(|l| l as u8),
        tags,
        packet_count: row.packet_count as u64,
        confidence: row.confidence as u8,
        product_family: row.product_family,
        signature_matches,
        oui_vendor: row.oui_vendor,
        country: row.country,
        is_public_ip: row.is_public_ip,
    }
}

pub fn row_to_connection_info(row: ConnectionRow) -> ConnectionInfo {
    let origin_files: Vec<String> = serde_json::from_str(&row.origin_files).unwrap_or_default();

    ConnectionInfo {
        id: row.id,
        src_ip: row.src_ip,
        src_port: row.src_port as u16,
        src_mac: row.src_mac,
        dst_ip: row.dst_ip,
        dst_port: row.dst_port as u16,
        dst_mac: row.dst_mac,
        protocol: row.protocol,
        transport: row.transport,
        packet_count: row.packet_count as u64,
        byte_count: row.byte_count as u64,
        first_seen: row.first_seen,
        last_seen: row.last_seen,
        origin_files,
    }
}

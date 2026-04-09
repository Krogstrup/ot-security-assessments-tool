//! State → gm-report type converters and report data assembly.

use gm_report::{ExportAsset, ExportConnection, ExportProtocolStat, ReportData};

use crate::application::queries::data::protocol_stats_from_connections;
use crate::commands::{CaptureState, InventoryState};

pub fn state_assets_to_export(inventory: &InventoryState) -> Vec<ExportAsset> {
    inventory
        .assets
        .iter()
        .map(|a| ExportAsset {
            ip_address: a.ip_address.clone(),
            mac_address: a.mac_address.clone(),
            hostname: a.hostname.clone(),
            device_type: a.device_type.clone(),
            vendor: a.vendor.clone(),
            product_family: a.product_family.clone(),
            protocols: a.protocols.clone(),
            confidence: a.confidence,
            purdue_level: a.purdue_level,
            oui_vendor: a.oui_vendor.clone(),
            country: a.country.clone(),
            is_public_ip: a.is_public_ip,
            first_seen: a.first_seen.clone(),
            last_seen: a.last_seen.clone(),
            notes: a.notes.clone(),
            tags: a.tags.clone(),
            packet_count: a.packet_count,
        })
        .collect()
}

pub fn state_connections_to_export(capture: &CaptureState) -> Vec<ExportConnection> {
    capture
        .connections
        .iter()
        .map(|c| ExportConnection {
            src_ip: c.src_ip.clone(),
            src_port: c.src_port,
            dst_ip: c.dst_ip.clone(),
            dst_port: c.dst_port,
            protocol: c.protocol.clone(),
            transport: c.transport.clone(),
            packet_count: c.packet_count,
            byte_count: c.byte_count,
            first_seen: c.first_seen.clone(),
            last_seen: c.last_seen.clone(),
        })
        .collect()
}

pub fn compute_protocol_stats(capture: &CaptureState) -> Vec<ExportProtocolStat> {
    protocol_stats_from_connections(&capture.connections)
        .into_iter()
        .map(|stat| ExportProtocolStat {
            protocol: stat.protocol,
            packet_count: stat.packet_count,
            byte_count: stat.byte_count,
            connection_count: stat.connection_count,
            unique_devices: stat.unique_devices,
        })
        .collect()
}

pub fn build_report_data(
    capture: &CaptureState,
    inventory: &InventoryState,
    session_name: Option<&str>,
) -> ReportData {
    ReportData {
        assets: state_assets_to_export(inventory),
        connections: state_connections_to_export(capture),
        protocol_stats: compute_protocol_stats(capture),
        findings: Vec::new(),
        session_name: session_name.map(|s| s.to_string()),
    }
}

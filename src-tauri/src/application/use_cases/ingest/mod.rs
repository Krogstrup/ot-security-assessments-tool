//! Ingest use-case: merge an [`IngestResult`] from any external source into
//! the shared application state.
//!
//! # Responsibilities
//! - Asset merge / enrichment (upsert by IP)
//! - Connection merge (dedup by 4-tuple, accumulate counts)
//! - Alert storage
//! - Topology rebuild after merge
//! - Zeek per-device event index rebuild
//!
//! # Lock order
//! Always acquire: capture (write) → inventory (write).

use std::collections::HashMap;
use std::time::Instant;

use gm_analysis::infer_device_type;
use gm_ingest::{IngestResult, IngestSource, IngestedAlert, IngestedAsset};
use gm_parsers::IcsProtocol;
use serde::Serialize;

use crate::commands::{
    support::write_state, AppState, AssetInfo, ConnectionInfo, DeviceZeekEvents, StoredAlert,
    ZeekEventSummary,
};

// ─── Public result type ───────────────────────────────────────────────────────

/// Result returned to callers (and ultimately the frontend) from an ingest
/// operation.  Re-exported from `commands::ingest` so web handlers continue
/// to resolve it at the same path.
#[derive(Serialize)]
pub struct IngestImportResult {
    pub source: String,
    pub files_processed: usize,
    pub asset_count: usize,
    pub connection_count: usize,
    pub alert_count: usize,
    pub new_assets: usize,
    pub updated_assets: usize,
    pub duration_ms: u64,
    pub errors: Vec<String>,
}

// ─── Public use-case entry point ──────────────────────────────────────────────

/// Merge an [`IngestResult`] into application state and return a summary.
///
/// This is the single shared path used by every ingest source (Zeek, Suricata,
/// Nmap, Masscan, Wazuh, SINEMA, TIA Portal).  The thin adapter functions in
/// `commands::ingest` call this after parsing their source-specific format.
///
/// Lock order: capture (write) → inventory (write).
pub fn run_ingest(
    ingest: IngestResult,
    state: &AppState,
    start: Instant,
) -> Result<IngestImportResult, String> {
    let mut capture = write_state(&state.capture, "capture")?;
    let mut inventory = write_state(&state.inventory, "inventory")?;

    let source_name = ingest
        .source
        .map(|s| s.display_name().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let is_active = ingest.source.map(|s| s.is_active_scan()).unwrap_or(false);
    let ingest_source = ingest.source;

    let total_assets = ingest.assets.len();
    let total_connections = ingest.connections.len();
    let total_alerts = ingest.alerts.len();

    // ── Asset merge ───────────────────────────────────────────────────────────
    let mut new_count = 0;
    let mut updated_count = 0;

    for ingested_asset in &ingest.assets {
        if let Some(existing) = inventory
            .assets
            .iter_mut()
            .find(|a| a.ip_address == ingested_asset.ip_address)
        {
            enrich_asset(existing, ingested_asset, is_active);
            updated_count += 1;
        } else {
            let asset = create_asset_from_ingested(ingested_asset, is_active);
            inventory.assets.push(asset);
            new_count += 1;
        }
    }

    // ── Connection merge ──────────────────────────────────────────────────────
    for ingested_conn in &ingest.connections {
        let origin = format!("[{}]", source_name);

        if let Some(existing) = capture.connections.iter_mut().find(|c| {
            c.src_ip == ingested_conn.src_ip
                && c.dst_ip == ingested_conn.dst_ip
                && c.src_port == ingested_conn.src_port
                && c.dst_port == ingested_conn.dst_port
        }) {
            existing.packet_count += ingested_conn.packet_count;
            existing.byte_count += ingested_conn.byte_count;
            if !existing.origin_files.contains(&origin) {
                existing.origin_files.push(origin);
            }
        } else {
            let conn = ConnectionInfo {
                id: uuid::Uuid::new_v4().to_string(),
                src_ip: ingested_conn.src_ip.clone(),
                src_port: ingested_conn.src_port,
                src_mac: None,
                dst_ip: ingested_conn.dst_ip.clone(),
                dst_port: ingested_conn.dst_port,
                dst_mac: None,
                protocol: ingested_conn.protocol.clone(),
                transport: ingested_conn.transport.clone(),
                packet_count: ingested_conn.packet_count,
                byte_count: ingested_conn.byte_count,
                first_seen: ingested_conn
                    .first_seen
                    .map(|t| t.to_rfc3339())
                    .unwrap_or_default(),
                last_seen: ingested_conn
                    .last_seen
                    .map(|t| t.to_rfc3339())
                    .unwrap_or_default(),
                origin_files: vec![origin],
            };
            capture.connections.push(conn);
        }
    }

    // ── Ingest source tracking ────────────────────────────────────────────────
    if let Some(source) = ingest_source {
        let source_tag = format!("[{}]", source.display_name());
        if !capture.imported_files.contains(&source_tag) {
            capture.imported_files.push(source_tag);
        }
    }

    // ── Alert storage ─────────────────────────────────────────────────────────
    for alert in &ingest.alerts {
        inventory
            .imported_alerts
            .push(ingested_alert_to_stored(alert));
    }

    // ── Zeek event index ──────────────────────────────────────────────────────
    if ingest_source == Some(IngestSource::Zeek) {
        inventory.zeek_device_events =
            rebuild_zeek_device_events(&capture.connections, &inventory.imported_alerts);
    }

    // ── Topology rebuild ──────────────────────────────────────────────────────
    let mut topo = gm_topology::TopologyBuilder::new();
    for conn in &capture.connections {
        let protocol = IcsProtocol::from_name(&conn.protocol);
        topo.add_connection(
            &conn.src_ip,
            &conn.dst_ip,
            None,
            None,
            protocol,
            conn.byte_count,
        );
    }
    capture.topology = topo.snapshot();

    // Enrich topology nodes with asset data
    let asset_lookup: HashMap<String, (Option<String>, String, u8)> = inventory
        .assets
        .iter()
        .map(|a| {
            (
                a.ip_address.clone(),
                (a.vendor.clone(), a.device_type.clone(), a.confidence),
            )
        })
        .collect();

    for node in &mut capture.topology.nodes {
        if let Some((vendor, device_type, confidence)) = asset_lookup.get(&node.ip_address) {
            if let Some(ref v) = vendor {
                node.vendor = Some(v.clone());
            }
            if *confidence >= 3 {
                node.device_type = device_type.clone();
            }
        }
    }

    let duration_ms = start.elapsed().as_millis() as u64;

    Ok(IngestImportResult {
        source: source_name,
        files_processed: ingest.files_processed,
        asset_count: total_assets,
        connection_count: total_connections,
        alert_count: total_alerts,
        new_assets: new_count,
        updated_assets: updated_count,
        duration_ms,
        errors: ingest.errors,
    })
}

// ─── Private helpers ──────────────────────────────────────────────────────────

/// Enrich an existing asset with data from an ingested asset.
fn enrich_asset(existing: &mut AssetInfo, ingested: &IngestedAsset, is_active: bool) {
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

/// Convert an [`IngestedAlert`] to the [`StoredAlert`] type used in AppState.
fn ingested_alert_to_stored(alert: &IngestedAlert) -> StoredAlert {
    StoredAlert {
        timestamp: alert.timestamp.to_rfc3339(),
        src_ip: alert.src_ip.clone(),
        src_port: alert.src_port,
        dst_ip: alert.dst_ip.clone(),
        dst_port: alert.dst_port,
        signature_id: alert.signature_id,
        signature: alert.signature.clone(),
        category: alert.category.clone(),
        severity: alert.severity,
        source: alert.source.display_name().to_string(),
    }
}

/// Create a new [`AssetInfo`] from ingested data.
fn create_asset_from_ingested(ingested: &IngestedAsset, is_active: bool) -> AssetInfo {
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
    let has_server_ports = ingested.open_ports.iter().any(|p| {
        gm_constants::OT_SERVER_PORTS.contains(&p.port)
    });
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

/// Rebuild per-device Zeek event summaries from all connections tagged `[Zeek]`.
///
/// Called after each Zeek import to refresh the event index.
fn rebuild_zeek_device_events(
    connections: &[ConnectionInfo],
    imported_alerts: &[StoredAlert],
) -> HashMap<String, DeviceZeekEvents> {
    let mut map: HashMap<String, (DeviceZeekEvents, std::collections::HashSet<String>)> =
        HashMap::new();

    for conn in connections {
        if !conn.origin_files.iter().any(|f| f.contains("Zeek")) {
            continue;
        }

        let log_type = classify_zeek_log_type(&conn.protocol, conn.dst_port);
        let timestamp = conn.first_seen.clone();

        for (device_ip, peer_ip) in [
            (conn.src_ip.clone(), conn.dst_ip.clone()),
            (conn.dst_ip.clone(), conn.src_ip.clone()),
        ] {
            let (events, peers) = map.entry(device_ip.clone()).or_insert_with(|| {
                (
                    DeviceZeekEvents {
                        device_ip: device_ip.clone(),
                        ..Default::default()
                    },
                    std::collections::HashSet::new(),
                )
            });

            match log_type.as_str() {
                "modbus" => events.modbus_events += 1,
                "dnp3" => events.dnp3_events += 1,
                "dns" => events.dns_queries += 1,
                "http" => events.http_requests += 1,
                _ => events.conn_log_entries += 1,
            }

            peers.insert(peer_ip.clone());

            if events.sample_events.len() < 50 {
                events.sample_events.push(ZeekEventSummary {
                    timestamp: timestamp.clone(),
                    log_type: log_type.clone(),
                    peer_ip: peer_ip.clone(),
                    summary: format!(
                        "{} {}:{} → {}:{} ({} pkts)",
                        conn.transport.to_uppercase(),
                        conn.src_ip,
                        conn.src_port,
                        conn.dst_ip,
                        conn.dst_port,
                        conn.packet_count
                    ),
                });
            }
        }
    }

    let alert_map: HashMap<String, u32> = {
        let mut m: HashMap<String, u32> = HashMap::new();
        for alert in imported_alerts {
            *m.entry(alert.src_ip.clone()).or_insert(0) += 1;
            *m.entry(alert.dst_ip.clone()).or_insert(0) += 1;
        }
        m
    };

    map.into_iter()
        .map(|(ip, (mut events, peers))| {
            events.unique_peers = peers.len() as u32;
            events.alert_count = alert_map.get(&ip).copied().unwrap_or(0);
            (ip, events)
        })
        .collect()
}

/// Map a connection protocol string + port to a Zeek log type label.
fn classify_zeek_log_type(protocol: &str, dst_port: u16) -> String {
    match protocol.to_lowercase().as_str() {
        "modbus" => "modbus".to_string(),
        "dnp3" => "dnp3".to_string(),
        "s7comm" => "s7comm".to_string(),
        _ => match dst_port {
            53 => "dns".to_string(),
            80 | 443 | 8080 | 8443 => "http".to_string(),
            _ => "conn".to_string(),
        },
    }
}

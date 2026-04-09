//! Ingest use-case: merge an [`IngestResult`] from any external source into
//! the shared application state.
//!
//! # Responsibilities
//! - Asset merge / enrichment (upsert by IP) — see [`asset_merge`]
//! - Connection merge (dedup by 4-tuple, accumulate counts)
//! - Alert storage — see [`alert_index`]
//! - Topology rebuild after merge
//! - Zeek per-device event index rebuild — see [`alert_index`]
//!
//! # Lock order
//! Always acquire: capture (write) → inventory (write).

mod alert_index;
mod asset_merge;

use std::collections::HashMap;
use std::time::Instant;

use gm_ingest::{IngestResult, IngestSource};
use gm_parsers::IcsProtocol;
use serde::Serialize;

use crate::commands::{support::write_state, AppState, ConnectionInfo};

use alert_index::{ingested_alert_to_stored, rebuild_zeek_device_events};
use asset_merge::{create_asset_from_ingested, enrich_asset};

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

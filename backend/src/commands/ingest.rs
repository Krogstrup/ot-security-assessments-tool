//! Adapter layer for external data ingestion.
//!
//! Each function parses its source-specific format, then delegates all
//! state-mutation work to [`crate::application::use_cases::ingest::run_ingest`].

use std::path::Path;
use std::time::Instant;

use super::{support::read_state, AppState, DeviceZeekEvents};

pub use crate::application::use_cases::ingest::IngestImportResult;

use crate::application::use_cases::ingest::run_ingest;

/// Import Zeek TSV log files (conn.log, modbus.log, dnp3.log, s7comm.log).
pub async fn import_zeek_logs(
    paths: Vec<String>,
    state: &AppState,
) -> Result<IngestImportResult, String> {
    let start = Instant::now();
    let path_refs: Vec<&Path> = paths.iter().map(|p| Path::new(p.as_str())).collect();
    let ingest_result = gm_ingest::zeek::parse_zeek_logs(&path_refs).map_err(|e| e.to_string())?;
    let result = run_ingest(ingest_result, state, start)?;
    log::info!(
        "Zeek import: {} files → {} assets ({} new), {} connections, {}ms",
        result.files_processed, result.asset_count, result.new_assets,
        result.connection_count, result.duration_ms
    );
    Ok(result)
}

/// Import a Suricata eve.json file.
pub async fn import_suricata_eve(
    path: String,
    state: &AppState,
) -> Result<IngestImportResult, String> {
    let start = Instant::now();
    let ingest_result =
        gm_ingest::suricata::parse_eve_json(Path::new(&path)).map_err(|e| e.to_string())?;
    let result = run_ingest(ingest_result, state, start)?;
    log::info!(
        "Suricata import: {} assets ({} new), {} connections, {} alerts, {}ms",
        result.asset_count, result.new_assets, result.connection_count,
        result.alert_count, result.duration_ms
    );
    Ok(result)
}

/// Import an Nmap XML file (-oX output).
///
/// **WARNING:** This imports results from an ACTIVE SCAN performed externally.
/// Kusanagi Kajiki NEVER performs active scans itself.
pub async fn import_nmap_xml(
    path: String,
    state: &AppState,
) -> Result<IngestImportResult, String> {
    let start = Instant::now();
    let ingest_result =
        gm_ingest::nmap::parse_nmap_xml(Path::new(&path)).map_err(|e| e.to_string())?;
    let result = run_ingest(ingest_result, state, start)?;
    log::info!(
        "Nmap import: {} assets ({} new), {}ms [ACTIVE SCAN DATA]",
        result.asset_count, result.new_assets, result.duration_ms
    );
    Ok(result)
}

/// Import a Wazuh HIDS/SIEM alert export file.
///
/// Accepts both line-delimited JSON and JSON array formats.
pub async fn import_wazuh_alerts(
    path: String,
    state: &AppState,
) -> Result<IngestImportResult, String> {
    let start = Instant::now();
    let ingest_result =
        gm_ingest::wazuh::parse_wazuh_alerts(Path::new(&path)).map_err(|e| e.to_string())?;
    let result = run_ingest(ingest_result, state, start)?;
    log::info!(
        "Wazuh import: {} alerts, {}ms",
        result.alert_count, result.duration_ms
    );
    Ok(result)
}

/// Import a Masscan JSON file (-oJ output).
///
/// **WARNING:** This imports results from an ACTIVE SCAN performed externally.
/// Kusanagi Kajiki NEVER performs active scans itself.
pub async fn import_masscan_json(
    path: String,
    state: &AppState,
) -> Result<IngestImportResult, String> {
    let start = Instant::now();
    let ingest_result =
        gm_ingest::masscan::parse_masscan_json(Path::new(&path)).map_err(|e| e.to_string())?;
    let result = run_ingest(ingest_result, state, start)?;
    log::info!(
        "Masscan import: {} assets ({} new), {}ms [ACTIVE SCAN DATA]",
        result.asset_count, result.new_assets, result.duration_ms
    );
    Ok(result)
}

/// Import a SINEMA Server CSV device inventory export.
pub async fn import_sinema_csv(
    path: String,
    state: &AppState,
) -> Result<IngestImportResult, String> {
    let start = Instant::now();
    let ingest_result =
        gm_ingest::sinema::import_sinema_csv(Path::new(&path)).map_err(|e| e.to_string())?;
    let result = run_ingest(ingest_result, state, start)?;
    log::info!(
        "SINEMA CSV import: {} assets ({} new), {}ms",
        result.asset_count, result.new_assets, result.duration_ms
    );
    Ok(result)
}

/// Import a TIA Portal network configuration XML export.
///
/// Extracts device names, IP addresses, hardware models, and firmware versions
/// from TIA Portal V15+ XML exports.
pub async fn import_tia_xml(
    path: String,
    state: &AppState,
) -> Result<IngestImportResult, String> {
    let start = Instant::now();
    let ingest_result =
        gm_ingest::sinema::import_tia_xml(Path::new(&path)).map_err(|e| e.to_string())?;
    let result = run_ingest(ingest_result, state, start)?;
    log::info!(
        "TIA Portal XML import: {} assets ({} new), {}ms",
        result.asset_count, result.new_assets, result.duration_ms
    );
    Ok(result)
}

/// Get Zeek-observed event statistics for a specific device IP.
pub async fn get_device_zeek_events(
    device_ip: String,
    state: &AppState,
) -> Result<DeviceZeekEvents, String> {
    let inventory = read_state(&state.inventory, "inventory")?;
    Ok(inventory
        .zeek_device_events
        .get(&device_ip)
        .cloned()
        .unwrap_or_else(|| DeviceZeekEvents {
            device_ip: device_ip.clone(),
            ..Default::default()
        }))
}

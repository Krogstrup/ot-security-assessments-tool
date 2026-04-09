use serde::Serialize;
use std::collections::{HashMap, HashSet};

use gm_types::{AssetInfo, ConnectionInfo};

/// Full diff result between current state and a baseline session.
#[derive(Debug, Clone, Serialize)]
pub struct BaselineDiff {
    /// Name of the baseline session used for comparison
    pub baseline_session_name: String,
    /// Devices present in current state but not in baseline
    pub new_assets: Vec<DriftAsset>,
    /// Devices present in baseline but not in current state
    pub missing_assets: Vec<DriftAsset>,
    /// Devices present in both but with changed properties
    pub changed_assets: Vec<ChangedAsset>,
    /// Connections in current state but not in baseline
    pub new_connections: Vec<DriftConnection>,
    /// Connections in baseline but not in current state
    pub missing_connections: Vec<DriftConnection>,
    /// Summary statistics
    pub summary: DriftSummary,
}

/// A device in the drift report (new or missing).
#[derive(Debug, Clone, Serialize)]
pub struct DriftAsset {
    pub ip_address: String,
    pub mac_address: Option<String>,
    pub device_type: String,
    pub vendor: Option<String>,
    pub protocols: Vec<String>,
    pub confidence: u8,
}

/// A device that exists in both baseline and current but has changes.
#[derive(Debug, Clone, Serialize)]
pub struct ChangedAsset {
    pub ip_address: String,
    pub changes: Vec<AssetChange>,
}

/// A single field change between baseline and current.
#[derive(Debug, Clone, Serialize)]
pub struct AssetChange {
    pub field: String,
    pub baseline_value: String,
    pub current_value: String,
}

/// A connection in the drift report (new or missing).
#[derive(Debug, Clone, Serialize)]
pub struct DriftConnection {
    pub src_ip: String,
    pub dst_ip: String,
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: String,
}

/// Summary of drift statistics.
#[derive(Debug, Clone, Serialize)]
pub struct DriftSummary {
    pub total_baseline_assets: usize,
    pub total_current_assets: usize,
    pub new_asset_count: usize,
    pub missing_asset_count: usize,
    pub changed_asset_count: usize,
    pub new_connection_count: usize,
    pub missing_connection_count: usize,
    /// Overall drift score: 0.0 (identical) to 1.0 (completely different)
    pub drift_score: f64,
}

pub fn compare_baseline(
    baseline_session_name: String,
    baseline_asset_rows: &[gm_db::AssetRow],
    baseline_conn_rows: &[gm_db::ConnectionRow],
    current_assets_vec: &[AssetInfo],
    current_connections_vec: &[ConnectionInfo],
) -> BaselineDiff {
    // Convert baseline assets to a map by IP for comparison
    let baseline_assets: HashMap<String, _> = baseline_asset_rows
        .iter()
        .map(|a| (a.ip_address.clone(), a))
        .collect();

    // Current assets by IP
    let current_assets: HashMap<String, &AssetInfo> = current_assets_vec
        .iter()
        .map(|a| (a.ip_address.clone(), a))
        .collect();

    let mut new_assets = Vec::new();
    let mut missing_assets = Vec::new();
    let mut changed_assets = Vec::new();

    // Find new and changed assets (in current but not baseline, or changed)
    for (ip, current) in &current_assets {
        if let Some(baseline) = baseline_assets.get(ip) {
            // Asset exists in both — check for changes
            let changes = diff_asset(baseline, current);
            if !changes.is_empty() {
                changed_assets.push(ChangedAsset {
                    ip_address: ip.clone(),
                    changes,
                });
            }
        } else {
            // New device
            new_assets.push(DriftAsset {
                ip_address: current.ip_address.clone(),
                mac_address: current.mac_address.clone(),
                device_type: current.device_type.clone(),
                vendor: current.vendor.clone(),
                protocols: current.protocols.clone(),
                confidence: current.confidence,
            });
        }
    }

    // Find missing assets (in baseline but not current)
    for (ip, baseline) in &baseline_assets {
        if !current_assets.contains_key(ip) {
            let protocols: Vec<String> =
                serde_json::from_str(&baseline.protocols).unwrap_or_default();
            missing_assets.push(DriftAsset {
                ip_address: baseline.ip_address.clone(),
                mac_address: baseline.mac_address.clone(),
                device_type: baseline.device_type.clone(),
                vendor: baseline.vendor.clone(),
                protocols,
                confidence: baseline.confidence as u8,
            });
        }
    }

    // Connection key: (src_ip, dst_ip, dst_port, protocol)
    let baseline_conn_keys: HashSet<(String, String, i64, String)> = baseline_conn_rows
        .iter()
        .map(|c| {
            (
                c.src_ip.clone(),
                c.dst_ip.clone(),
                c.dst_port,
                c.protocol.clone(),
            )
        })
        .collect();

    let current_conn_keys: HashSet<(String, String, u16, String)> = current_connections_vec
        .iter()
        .map(|c| {
            (
                c.src_ip.clone(),
                c.dst_ip.clone(),
                c.dst_port,
                c.protocol.clone(),
            )
        })
        .collect();

    let mut new_connections = Vec::new();
    let mut missing_connections = Vec::new();

    // New connections (in current but not baseline)
    for conn in current_connections_vec {
        let key = (
            conn.src_ip.clone(),
            conn.dst_ip.clone(),
            conn.dst_port as i64,
            conn.protocol.clone(),
        );
        if !baseline_conn_keys.contains(&key) {
            new_connections.push(DriftConnection {
                src_ip: conn.src_ip.clone(),
                dst_ip: conn.dst_ip.clone(),
                src_port: conn.src_port,
                dst_port: conn.dst_port,
                protocol: conn.protocol.clone(),
            });
        }
    }

    // Missing connections (in baseline but not current)
    for conn in baseline_conn_rows {
        let key = (
            conn.src_ip.clone(),
            conn.dst_ip.clone(),
            conn.dst_port as u16,
            conn.protocol.clone(),
        );
        if !current_conn_keys.contains(&key) {
            missing_connections.push(DriftConnection {
                src_ip: conn.src_ip.clone(),
                dst_ip: conn.dst_ip.clone(),
                src_port: conn.src_port as u16,
                dst_port: conn.dst_port as u16,
                protocol: conn.protocol.clone(),
            });
        }
    }

    let total_baseline = baseline_assets.len();
    let total_current = current_assets.len();
    let total_changes = new_assets.len()
        + missing_assets.len()
        + changed_assets.len()
        + new_connections.len()
        + missing_connections.len();
    let total_items =
        total_baseline.max(total_current) + baseline_conn_keys.len().max(current_conn_keys.len());
    let drift_score = if total_items > 0 {
        (total_changes as f64 / total_items as f64).min(1.0)
    } else {
        0.0
    };

    let summary = DriftSummary {
        total_baseline_assets: total_baseline,
        total_current_assets: total_current,
        new_asset_count: new_assets.len(),
        missing_asset_count: missing_assets.len(),
        changed_asset_count: changed_assets.len(),
        new_connection_count: new_connections.len(),
        missing_connection_count: missing_connections.len(),
        drift_score,
    };

    BaselineDiff {
        baseline_session_name,
        new_assets,
        missing_assets,
        changed_assets,
        new_connections,
        missing_connections,
        summary,
    }
}

/// Compare a baseline asset (DB row) against a current asset (in-memory).
/// Returns a list of field changes.
fn diff_asset(baseline: &gm_db::AssetRow, current: &AssetInfo) -> Vec<AssetChange> {
    let mut changes = Vec::new();

    if baseline.device_type != current.device_type {
        changes.push(AssetChange {
            field: "device_type".to_string(),
            baseline_value: baseline.device_type.clone(),
            current_value: current.device_type.clone(),
        });
    }

    let baseline_vendor = baseline.vendor.as_deref().unwrap_or("");
    let current_vendor = current.vendor.as_deref().unwrap_or("");
    if baseline_vendor != current_vendor {
        changes.push(AssetChange {
            field: "vendor".to_string(),
            baseline_value: baseline_vendor.to_string(),
            current_value: current_vendor.to_string(),
        });
    }

    if baseline.confidence as u8 != current.confidence {
        changes.push(AssetChange {
            field: "confidence".to_string(),
            baseline_value: baseline.confidence.to_string(),
            current_value: current.confidence.to_string(),
        });
    }

    let baseline_protocols: Vec<String> =
        serde_json::from_str(&baseline.protocols).unwrap_or_default();
    let mut bp_sorted = baseline_protocols.clone();
    bp_sorted.sort();
    let mut cp_sorted = current.protocols.clone();
    cp_sorted.sort();
    if bp_sorted != cp_sorted {
        changes.push(AssetChange {
            field: "protocols".to_string(),
            baseline_value: bp_sorted.join(", "),
            current_value: cp_sorted.join(", "),
        });
    }

    let baseline_hostname = baseline.hostname.as_deref().unwrap_or("");
    let current_hostname = current.hostname.as_deref().unwrap_or("");
    if baseline_hostname != current_hostname {
        changes.push(AssetChange {
            field: "hostname".to_string(),
            baseline_value: baseline_hostname.to_string(),
            current_value: current_hostname.to_string(),
        });
    }

    let baseline_purdue = baseline.purdue_level.map(|l| l as u8);
    if baseline_purdue != current.purdue_level {
        changes.push(AssetChange {
            field: "purdue_level".to_string(),
            baseline_value: baseline_purdue
                .map(|l| l.to_string())
                .unwrap_or_else(|| "none".to_string()),
            current_value: current
                .purdue_level
                .map(|l| l.to_string())
                .unwrap_or_else(|| "none".to_string()),
        });
    }

    changes
}

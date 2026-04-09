//! Paginated asset and connection queries.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use gm_types::{AssetInfo, ConnectionInfo};

// ─── Types ────────────────────────────────────────────────────────────────────

#[derive(Serialize, Clone)]
pub struct AssetPage {
    pub assets: Vec<AssetInfo>,
    pub total: usize,
    pub page: usize,
    pub page_size: usize,
    pub has_more: bool,
}

#[derive(Serialize, Clone)]
pub struct ConnectionPage {
    pub connections: Vec<ConnectionInfo>,
    pub total: usize,
    pub page: usize,
    pub page_size: usize,
    pub has_more: bool,
}

#[derive(Serialize, Clone)]
pub struct DataCounts {
    pub asset_count: usize,
    pub connection_count: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AssetSortBy {
    Ip,
    Packets,
    Protocol,
    Connections,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionSortBy {
    Packets,
    Bytes,
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn paginate<T>(items: Vec<T>, page: usize, page_size: usize) -> (Vec<T>, bool) {
    let start = page * page_size;
    if start >= items.len() {
        return (Vec::new(), false);
    }
    let has_more = start + page_size < items.len();
    (
        items.into_iter().skip(start).take(page_size).collect(),
        has_more,
    )
}

fn count_asset_connections(connections: &[ConnectionInfo]) -> HashMap<String, u64> {
    let mut counts: HashMap<String, u64> = HashMap::new();
    for conn in connections {
        *counts.entry(conn.src_ip.clone()).or_insert(0) += 1;
        *counts.entry(conn.dst_ip.clone()).or_insert(0) += 1;
    }
    counts
}

// ─── Queries ──────────────────────────────────────────────────────────────────

/// Get discovered assets, paginated and optionally sorted.
pub fn get_assets(
    assets: &[AssetInfo],
    connections: &[ConnectionInfo],
    page: Option<usize>,
    page_size: Option<usize>,
    sort_by: Option<AssetSortBy>,
) -> Result<AssetPage, String> {
    let connection_counts = if matches!(sort_by, Some(AssetSortBy::Connections)) {
        Some(count_asset_connections(connections))
    } else {
        None
    };

    let page = page.unwrap_or(0);
    let page_size = page_size.unwrap_or(200);

    let mut all_assets = assets.to_vec();
    let total = all_assets.len();

    match sort_by {
        Some(AssetSortBy::Ip) => all_assets.sort_by(|a, b| a.ip_address.cmp(&b.ip_address)),
        Some(AssetSortBy::Packets) => {
            all_assets.sort_by(|a, b| b.packet_count.cmp(&a.packet_count))
        }
        Some(AssetSortBy::Protocol) => {
            all_assets.sort_by(|a, b| {
                let ap = a.protocols.first().map(|s| s.as_str()).unwrap_or("");
                let bp = b.protocols.first().map(|s| s.as_str()).unwrap_or("");
                ap.cmp(bp)
            });
        }
        Some(AssetSortBy::Connections) => {
            let counts = connection_counts
                .as_ref()
                .expect("connection counts must be present for Connections sort");
            all_assets.sort_by(|a, b| {
                let ac = counts.get(a.ip_address.as_str()).copied().unwrap_or(0);
                let bc = counts.get(b.ip_address.as_str()).copied().unwrap_or(0);
                bc.cmp(&ac).then_with(|| a.ip_address.cmp(&b.ip_address))
            });
        }
        _ => {}
    }

    let (assets, has_more) = paginate(all_assets, page, page_size);

    Ok(AssetPage {
        assets,
        total,
        page,
        page_size,
        has_more,
    })
}

/// Get observed connections, paginated and optionally sorted.
pub fn get_connections(
    connections: &[ConnectionInfo],
    page: Option<usize>,
    page_size: Option<usize>,
    sort_by: Option<ConnectionSortBy>,
) -> Result<ConnectionPage, String> {
    let page = page.unwrap_or(0);
    let page_size = page_size.unwrap_or(500);

    let mut all_connections = connections.to_vec();
    let total = all_connections.len();

    match sort_by {
        Some(ConnectionSortBy::Packets) => {
            all_connections.sort_by(|a, b| b.packet_count.cmp(&a.packet_count));
        }
        Some(ConnectionSortBy::Bytes) => {
            all_connections.sort_by(|a, b| b.byte_count.cmp(&a.byte_count));
        }
        _ => {}
    }

    let (connections, has_more) = paginate(all_connections, page, page_size);

    Ok(ConnectionPage {
        connections,
        total,
        page,
        page_size,
        has_more,
    })
}

/// Get lightweight asset/connection counts (avoids serializing full datasets).
pub fn get_data_counts(
    assets: &[AssetInfo],
    connections: &[ConnectionInfo],
) -> Result<DataCounts, String> {
    let asset_count = assets.len();
    let connection_count = connections.len();
    Ok(DataCounts {
        asset_count,
        connection_count,
    })
}

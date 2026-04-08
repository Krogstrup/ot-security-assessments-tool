use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use super::{
    support::read_state, AppState, AssetInfo, ConnectionInfo, DeepParseInfo, FunctionCodeStat,
    PacketSummary, ProtocolStatInfo,
};
use gm_topology::TopologyGraph;

/// Maximum nodes returned by get_topology. Excess nodes (by packet count) are
/// dropped to prevent the webview from being asked to render a massive graph.
const MAX_TOPOLOGY_NODES: usize = 5_000;
/// Maximum edges returned by get_topology.
const MAX_TOPOLOGY_EDGES: usize = 20_000;

/// Get the current network topology graph for visualization.
///
/// Nodes are capped at MAX_TOPOLOGY_NODES (5 000) by packet_count descending.
/// Edges are then filtered to only include connections between remaining nodes
/// and capped at MAX_TOPOLOGY_EDGES (20 000) by packet_count descending.
/// For smaller datasets the full graph is returned unchanged.
pub fn get_topology(state: &AppState) -> Result<TopologyGraph, String> {
    let capture = read_state(&state.capture, "capture")?;
    let topo = &capture.topology;

    if topo.nodes.len() <= MAX_TOPOLOGY_NODES && topo.edges.len() <= MAX_TOPOLOGY_EDGES {
        return Ok(topo.clone());
    }

    // Cap nodes: keep the highest-traffic devices.
    let mut nodes = topo.nodes.clone();
    nodes.sort_by(|a, b| b.packet_count.cmp(&a.packet_count));
    nodes.truncate(MAX_TOPOLOGY_NODES);

    // Build a set of the retained node IDs so we can filter edges cheaply.
    let retained: HashSet<&str> = nodes.iter().map(|n| n.id.as_str()).collect();

    // Filter edges to connections between retained nodes, then cap.
    let mut edges: Vec<_> = topo
        .edges
        .iter()
        .filter(|e| retained.contains(e.source.as_str()) && retained.contains(e.target.as_str()))
        .cloned()
        .collect();
    edges.sort_by(|a, b| b.packet_count.cmp(&a.packet_count));
    edges.truncate(MAX_TOPOLOGY_EDGES);

    Ok(TopologyGraph { nodes, edges })
}

// ─── Paginated data responses ──────────────────────────────────

/// A page of assets returned by `get_assets`.
#[derive(Serialize, Clone)]
pub struct AssetPage {
    pub assets: Vec<AssetInfo>,
    pub total: usize,
    pub page: usize,
    pub page_size: usize,
    pub has_more: bool,
}

/// A page of connections returned by `get_connections`.
#[derive(Serialize, Clone)]
pub struct ConnectionPage {
    pub connections: Vec<ConnectionInfo>,
    pub total: usize,
    pub page: usize,
    pub page_size: usize,
    pub has_more: bool,
}

/// Lightweight counts for the sidebar (no payload).
#[derive(Serialize, Clone)]
pub struct DataCounts {
    pub asset_count: usize,
    pub connection_count: usize,
}

/// Supported sort keys for asset paging.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AssetSortBy {
    Ip,
    Packets,
    Protocol,
    Connections,
}

/// Supported sort keys for connection paging.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionSortBy {
    Packets,
    Bytes,
}

/// Supported sort keys for protocol statistics.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProtocolStatsSortBy {
    Packets,
    Bytes,
    Connections,
    Devices,
}

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

fn count_asset_connections(state: &AppState) -> Result<HashMap<String, u64>, String> {
    let capture = read_state(&state.capture, "capture")?;
    let mut counts: HashMap<String, u64> = HashMap::new();
    for conn in &capture.connections {
        *counts.entry(conn.src_ip.clone()).or_insert(0) += 1;
        *counts.entry(conn.dst_ip.clone()).or_insert(0) += 1;
    }
    Ok(counts)
}

/// Get discovered assets, paginated.
///
/// Parameters:
/// - `page`: zero-based page index (default 0)
/// - `page_size`: items per page (default 200)
/// - `sort_by`: optional sort key — "ip", "packets", "protocol", "connections"
pub fn get_assets(
    state: &AppState,
    page: Option<usize>,
    page_size: Option<usize>,
    sort_by: Option<AssetSortBy>,
) -> Result<AssetPage, String> {
    // If sorting by connection count, read capture first to keep lock order
    // consistent: capture -> inventory.
    let connection_counts = if matches!(sort_by, Some(AssetSortBy::Connections)) {
        Some(count_asset_connections(state)?)
    } else {
        None
    };

    let inventory = read_state(&state.inventory, "inventory")?;

    let page = page.unwrap_or(0);
    let page_size = page_size.unwrap_or(200);

    let mut all_assets = inventory.assets.clone();
    let total = all_assets.len();

    // Sort
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
        _ => {} // default insertion order
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

/// Get observed connections, paginated.
///
/// Parameters:
/// - `page`: zero-based page index (default 0)
/// - `page_size`: items per page (default 500)
/// - `sort_by`: optional sort key — "packets", "bytes"
pub fn get_connections(
    state: &AppState,
    page: Option<usize>,
    page_size: Option<usize>,
    sort_by: Option<ConnectionSortBy>,
) -> Result<ConnectionPage, String> {
    let capture = read_state(&state.capture, "capture")?;

    let page = page.unwrap_or(0);
    let page_size = page_size.unwrap_or(500);

    let mut all_connections = capture.connections.clone();
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

/// Get lightweight asset/connection counts for the sidebar.
///
/// This avoids serializing the full dataset just to show totals.
pub fn get_data_counts(state: &AppState) -> Result<DataCounts, String> {
    let asset_count = read_state(&state.inventory, "inventory")?.assets.len();
    let connection_count = read_state(&state.capture, "capture")?.connections.len();
    Ok(DataCounts {
        asset_count,
        connection_count,
    })
}

/// Compute protocol breakdown statistics from current connections.
///
/// Single-pass O(n_connections): accumulates stats and unique-device sets for
/// all protocols in one loop, avoiding the previous O(protocols × connections)
/// double-loop.
pub fn get_protocol_stats(
    state: &AppState,
    sort_by: Option<ProtocolStatsSortBy>,
) -> Result<Vec<ProtocolStatInfo>, String> {
    let capture = read_state(&state.capture, "capture")?;
    Ok(protocol_stats_from_connections_with_sort(
        &capture.connections,
        sort_by.unwrap_or(ProtocolStatsSortBy::Packets),
    ))
}

/// Shared protocol-stat computation used by both API reads and export paths.
pub(crate) fn protocol_stats_from_connections(
    connections: &[ConnectionInfo],
) -> Vec<ProtocolStatInfo> {
    protocol_stats_from_connections_with_sort(connections, ProtocolStatsSortBy::Packets)
}

pub(crate) fn protocol_stats_from_connections_with_sort(
    connections: &[ConnectionInfo],
    sort_by: ProtocolStatsSortBy,
) -> Vec<ProtocolStatInfo> {
    let mut stats: HashMap<String, ProtocolStatInfo> = HashMap::new();
    // Track unique devices per protocol in the same pass.
    let mut devices_per_proto: HashMap<String, HashSet<String>> = HashMap::new();

    for conn in connections {
        let entry = stats
            .entry(conn.protocol.clone())
            .or_insert_with(|| ProtocolStatInfo {
                protocol: conn.protocol.clone(),
                packet_count: 0,
                byte_count: 0,
                connection_count: 0,
                unique_devices: 0,
            });
        entry.packet_count += conn.packet_count;
        entry.byte_count += conn.byte_count;
        entry.connection_count += 1;

        let dev = devices_per_proto.entry(conn.protocol.clone()).or_default();
        dev.insert(conn.src_ip.clone());
        dev.insert(conn.dst_ip.clone());
    }

    // Merge unique device counts into stats.
    for (proto, dev_set) in &devices_per_proto {
        if let Some(stat) = stats.get_mut(proto) {
            stat.unique_devices = dev_set.len() as u64;
        }
    }

    let mut result: Vec<ProtocolStatInfo> = stats.into_values().collect();
    sort_protocol_stats(&mut result, sort_by);
    result
}

pub(crate) fn sort_protocol_stats(stats: &mut [ProtocolStatInfo], sort_by: ProtocolStatsSortBy) {
    match sort_by {
        ProtocolStatsSortBy::Packets => stats.sort_by(|a, b| b.packet_count.cmp(&a.packet_count)),
        ProtocolStatsSortBy::Bytes => stats.sort_by(|a, b| b.byte_count.cmp(&a.byte_count)),
        ProtocolStatsSortBy::Connections => {
            stats.sort_by(|a, b| b.connection_count.cmp(&a.connection_count))
        }
        ProtocolStatsSortBy::Devices => {
            stats.sort_by(|a, b| b.unique_devices.cmp(&a.unique_devices))
        }
    }
}

/// Get packet summaries for a specific connection (for the connection tree detail view).
///
/// Already capped at 1000 per connection during ingestion (see processor.rs).
pub fn get_connection_packets(
    connection_id: String,
    state: &AppState,
) -> Result<Vec<PacketSummary>, String> {
    let capture = read_state(&state.capture, "capture")?;
    Ok(capture
        .packet_summaries
        .get(&connection_id)
        .cloned()
        .unwrap_or_default())
}

/// Get deep parse information for a specific device by IP address.
///
/// Returns Modbus/DNP3 details including function codes, unit IDs,
/// register ranges, device identification, and polling intervals.
pub fn get_deep_parse_info(
    ip_address: String,
    state: &AppState,
) -> Result<Option<DeepParseInfo>, String> {
    let inventory = read_state(&state.inventory, "inventory")?;
    Ok(inventory.deep_parse_info.get(&ip_address).cloned())
}

/// Get function code distribution across all protocols.
///
/// Returns aggregated function code stats for the protocol stats view,
/// showing which function codes are most used across the network.
pub fn get_function_code_stats(
    state: &AppState,
) -> Result<HashMap<String, Vec<FunctionCodeStat>>, String> {
    let inventory = read_state(&state.inventory, "inventory")?;

    let mut modbus_fcs: HashMap<u8, u64> = HashMap::new();
    let mut dnp3_fcs: HashMap<u8, u64> = HashMap::new();

    for info in inventory.deep_parse_info.values() {
        if let Some(ref modbus) = info.modbus {
            for fc in &modbus.function_codes {
                *modbus_fcs.entry(fc.code).or_insert(0) += fc.count;
            }
        }
        if let Some(ref dnp3) = info.dnp3 {
            for fc in &dnp3.function_codes {
                *dnp3_fcs.entry(fc.code).or_insert(0) += fc.count;
            }
        }
    }

    let mut result: HashMap<String, Vec<FunctionCodeStat>> = HashMap::new();

    if !modbus_fcs.is_empty() {
        let mut fcs: Vec<FunctionCodeStat> = modbus_fcs
            .into_iter()
            .map(|(code, count)| FunctionCodeStat {
                code,
                name: gm_parsers::modbus_function_code_name(code).to_string(),
                count,
                is_write: matches!(code, 5 | 6 | 15 | 16 | 22 | 23),
            })
            .collect();
        fcs.sort_by(|a, b| b.count.cmp(&a.count));
        result.insert("modbus".to_string(), fcs);
    }

    if !dnp3_fcs.is_empty() {
        let mut fcs: Vec<FunctionCodeStat> = dnp3_fcs
            .into_iter()
            .map(|(code, count)| FunctionCodeStat {
                code,
                name: gm_parsers::dnp3_function_code_name(code).to_string(),
                count,
                is_write: matches!(code, 2..=6),
            })
            .collect();
        fcs.sort_by(|a, b| b.count.cmp(&a.count));
        result.insert("dnp3".to_string(), fcs);
    }

    Ok(result)
}

// ─── Timeline (Phase 11) ────────────────────────────────────

/// Timeline range: earliest and latest timestamps across all connections.
#[derive(Debug, Clone, Serialize)]
pub struct TimelineRange {
    pub earliest: Option<String>,
    pub latest: Option<String>,
    /// Total number of connections with timestamps
    pub connection_count: usize,
}

/// Get the time range of the current dataset.
///
/// Returns the earliest and latest timestamps from all connections,
/// used by the timeline scrubber to set slider bounds.
/// Scans all connections (not capped) to ensure accurate bounds.
pub fn get_timeline_range(state: &AppState) -> Result<TimelineRange, String> {
    let capture = read_state(&state.capture, "capture")?;

    let mut earliest: Option<&str> = None;
    let mut latest: Option<&str> = None;

    for conn in &capture.connections {
        let fs = conn.first_seen.as_str();
        let ls = conn.last_seen.as_str();

        match earliest {
            None => earliest = Some(fs),
            Some(e) if fs < e => earliest = Some(fs),
            _ => {}
        }
        match latest {
            None => latest = Some(ls),
            Some(l) if ls > l => latest = Some(ls),
            _ => {}
        }
    }

    Ok(TimelineRange {
        earliest: earliest.map(|s| s.to_string()),
        latest: latest.map(|s| s.to_string()),
        connection_count: capture.connections.len(),
    })
}

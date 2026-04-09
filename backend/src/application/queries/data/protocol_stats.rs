//! Protocol statistics query.

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::commands::{support::read_state, AppState, ConnectionInfo, ProtocolStatInfo};

// ─── Types ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProtocolStatsSortBy {
    Packets,
    Bytes,
    Connections,
    Devices,
}

// ─── Queries ──────────────────────────────────────────────────────────────────

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
pub fn protocol_stats_from_connections(connections: &[ConnectionInfo]) -> Vec<ProtocolStatInfo> {
    protocol_stats_from_connections_with_sort(connections, ProtocolStatsSortBy::Packets)
}

pub fn protocol_stats_from_connections_with_sort(
    connections: &[ConnectionInfo],
    sort_by: ProtocolStatsSortBy,
) -> Vec<ProtocolStatInfo> {
    let mut stats: HashMap<String, ProtocolStatInfo> = HashMap::new();
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

    for (proto, dev_set) in &devices_per_proto {
        if let Some(stat) = stats.get_mut(proto) {
            stat.unique_devices = dev_set.len() as u64;
        }
    }

    let mut result: Vec<ProtocolStatInfo> = stats.into_values().collect();
    sort_protocol_stats(&mut result, sort_by);
    result
}

pub fn sort_protocol_stats(stats: &mut [ProtocolStatInfo], sort_by: ProtocolStatsSortBy) {
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

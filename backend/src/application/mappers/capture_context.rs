//! Build `CaptureContext` snapshots from runtime-domain slices.

use std::collections::{HashMap, HashSet};

use gm_analysis::ConnectionStats;
use gm_analysis::CaptureContext;
use gm_parsers::DeepParseInfo;
use gm_types::AssetInfo;
use gm_types::ConnectionInfo;
use gm_types::OT_DEVICE_TYPES;
use gm_types::OT_PROTOCOL_NAMES;
use gm_types::OT_SERVER_PORTS;

/// Build a [`CaptureContext`] from immutable domain snapshots.
pub fn build_capture_context_snapshot(
    assets: &[AssetInfo],
    connections: &[ConnectionInfo],
    connection_stats: &[ConnectionStats],
    deep_parse_info: &HashMap<String, DeepParseInfo>,
) -> CaptureContext {
    let mut ot_device_ips: HashSet<String> = assets
        .iter()
        .filter(|a| {
            OT_DEVICE_TYPES.contains(&a.device_type.as_str())
                || a.protocols
                    .iter()
                    .any(|p| OT_PROTOCOL_NAMES.contains(&p.as_str()))
        })
        .map(|a| a.ip_address.clone())
        .collect();

    for conn in connections {
        if OT_SERVER_PORTS.contains(&conn.dst_port) {
            ot_device_ips.insert(conn.dst_ip.clone());
        }
    }

    let external_ips: HashSet<String> = assets
        .iter()
        .filter(|a| a.is_public_ip)
        .map(|a| a.ip_address.clone())
        .collect();

    let mut ip_to_macs: HashMap<String, Vec<String>> = HashMap::new();
    for asset in assets {
        if let Some(mac) = &asset.mac_address {
            let macs = ip_to_macs.entry(asset.ip_address.clone()).or_default();
            if !macs.contains(mac) {
                macs.push(mac.clone());
            }
        }
    }
    for conn in connections {
        if let Some(mac) = &conn.src_mac {
            let macs = ip_to_macs.entry(conn.src_ip.clone()).or_default();
            if !macs.contains(mac) {
                macs.push(mac.clone());
            }
        }
        if let Some(mac) = &conn.dst_mac {
            let macs = ip_to_macs.entry(conn.dst_ip.clone()).or_default();
            if !macs.contains(mac) {
                macs.push(mac.clone());
            }
        }
    }

    let mut mac_to_ips: HashMap<String, Vec<String>> = HashMap::new();
    for (ip, macs) in &ip_to_macs {
        for mac in macs {
            mac_to_ips.entry(mac.clone()).or_default().push(ip.clone());
        }
    }

    let mut device_first_seen: HashMap<String, f64> = HashMap::new();
    let mut device_last_seen: HashMap<String, f64> = HashMap::new();
    let mut capture_start = f64::INFINITY;
    let mut capture_end = f64::NEG_INFINITY;

    for cs in connection_stats {
        if cs.first_seen < capture_start {
            capture_start = cs.first_seen;
        }
        if cs.last_seen > capture_end {
            capture_end = cs.last_seen;
        }

        let src_first = device_first_seen
            .entry(cs.src_ip.clone())
            .or_insert(f64::INFINITY);
        if cs.first_seen < *src_first {
            *src_first = cs.first_seen;
        }
        let src_last = device_last_seen
            .entry(cs.src_ip.clone())
            .or_insert(f64::NEG_INFINITY);
        if cs.last_seen > *src_last {
            *src_last = cs.last_seen;
        }

        let dst_first = device_first_seen
            .entry(cs.dst_ip.clone())
            .or_insert(f64::INFINITY);
        if cs.first_seen < *dst_first {
            *dst_first = cs.first_seen;
        }
        let dst_last = device_last_seen
            .entry(cs.dst_ip.clone())
            .or_insert(f64::NEG_INFINITY);
        if cs.last_seen > *dst_last {
            *dst_last = cs.last_seen;
        }
    }

    let capture_start = if capture_start.is_finite() {
        capture_start
    } else {
        0.0
    };
    let capture_end = if capture_end.is_finite() {
        capture_end
    } else {
        0.0
    };

    for v in device_first_seen.values_mut() {
        if !v.is_finite() {
            *v = 0.0;
        }
    }
    for v in device_last_seen.values_mut() {
        if !v.is_finite() {
            *v = 0.0;
        }
    }

    let mut per_source_dst_ports: HashMap<String, HashSet<u16>> = HashMap::new();
    for conn in connections {
        per_source_dst_ports
            .entry(conn.src_ip.clone())
            .or_default()
            .insert(conn.dst_port);
    }

    let mut per_source_write_targets: HashMap<String, HashSet<String>> = HashMap::new();
    let mut per_connection_write_rate: HashMap<(String, String), u64> = HashMap::new();
    for (ip, dp) in deep_parse_info {
        if let Some(modbus) = &dp.modbus {
            if modbus.role == "master" || modbus.role == "both" {
                let write_count: u64 = modbus
                    .function_codes
                    .iter()
                    .filter(|fc| matches!(fc.code, 5 | 6 | 15 | 16))
                    .map(|fc| fc.count)
                    .sum();
                for rel in &modbus.relationships {
                    if rel.remote_role == "slave" {
                        per_source_write_targets
                            .entry(ip.clone())
                            .or_default()
                            .insert(rel.remote_ip.clone());
                        if write_count > 0 {
                            *per_connection_write_rate
                                .entry((ip.clone(), rel.remote_ip.clone()))
                                .or_insert(0) += write_count;
                        }
                    }
                }
            }
        }
    }

    let mut per_source_read_targets: HashMap<String, HashSet<String>> = HashMap::new();
    for conn in connections {
        if OT_SERVER_PORTS.contains(&conn.dst_port) {
            let is_write_target = per_source_write_targets
                .get(&conn.src_ip)
                .map(|wt| wt.contains(&conn.dst_ip))
                .unwrap_or(false);
            if !is_write_target {
                per_source_read_targets
                    .entry(conn.src_ip.clone())
                    .or_default()
                    .insert(conn.dst_ip.clone());
            }
        }
    }

    CaptureContext {
        capture_start,
        capture_end,
        ip_to_macs,
        mac_to_ips,
        device_first_seen,
        device_last_seen,
        per_source_read_targets,
        per_source_write_targets,
        per_source_dst_ports,
        per_connection_write_rate,
        ot_device_ips,
        external_ips,
    }
}

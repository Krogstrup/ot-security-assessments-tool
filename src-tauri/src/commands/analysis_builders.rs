//! Input builders for security analysis commands.
//!
//! These functions construct [`AnalysisInput`] and [`CaptureContext`] from
//! domain state slices, keeping the construction logic out of the command
//! dispatch layer.

use std::collections::{HashMap, HashSet};

use gm_analysis::{
    AnalysisInput, AssetSnapshot, BacnetSnapshot, CaptureContext, ConnectionSnapshot,
    DeepParseSnapshot, Dnp3Snapshot, EnipSnapshot, FcSnapshot, Iec104Snapshot, ModbusSnapshot,
    PollingSnapshot, ProfinetDcpSnapshot, RelationshipSnapshot, S7Snapshot,
};

use super::{AnalysisState, CaptureState, InventoryState};

/// Build AnalysisInput from capture + inventory domain slices.
pub fn build_analysis_input(capture: &CaptureState, inventory: &InventoryState) -> AnalysisInput {
    let assets = asset_snapshots(inventory);

    let connections: Vec<ConnectionSnapshot> = capture
        .connections
        .iter()
        .map(|c| ConnectionSnapshot {
            src_ip: c.src_ip.clone(),
            dst_ip: c.dst_ip.clone(),
            src_port: c.src_port,
            dst_port: c.dst_port,
            protocol: c.protocol.clone(),
            packet_count: c.packet_count,
        })
        .collect();

    let mut deep_parse = std::collections::HashMap::new();
    for (ip, dp) in &inventory.deep_parse_info {
        let modbus = dp.modbus.as_ref().map(|m| ModbusSnapshot {
            role: m.role.clone(),
            unit_ids: m.unit_ids.clone(),
            function_codes: m
                .function_codes
                .iter()
                .map(|fc| FcSnapshot {
                    code: fc.code,
                    count: fc.count,
                    is_write: fc.is_write,
                })
                .collect(),
            relationships: m
                .relationships
                .iter()
                .map(|r| RelationshipSnapshot {
                    remote_ip: r.remote_ip.clone(),
                    remote_role: r.remote_role.clone(),
                    packet_count: r.packet_count,
                })
                .collect(),
            polling_intervals: m
                .polling_intervals
                .iter()
                .map(|pi| PollingSnapshot {
                    remote_ip: pi.remote_ip.clone(),
                    function_code: pi.function_code,
                    avg_interval_ms: pi.avg_interval_ms,
                    min_interval_ms: pi.min_interval_ms,
                    max_interval_ms: pi.max_interval_ms,
                    sample_count: pi.sample_count,
                })
                .collect(),
        });

        let dnp3 = dp.dnp3.as_ref().map(|d| Dnp3Snapshot {
            role: d.role.clone(),
            has_unsolicited: d.has_unsolicited,
            function_codes: d
                .function_codes
                .iter()
                .map(|fc| FcSnapshot {
                    code: fc.code,
                    count: fc.count,
                    is_write: fc.is_write,
                })
                .collect(),
            relationships: d
                .relationships
                .iter()
                .map(|r| RelationshipSnapshot {
                    remote_ip: r.remote_ip.clone(),
                    remote_role: r.remote_role.clone(),
                    packet_count: r.packet_count,
                })
                .collect(),
        });

        let enip = dp.enip.as_ref().map(|e| EnipSnapshot {
            role: e.role.clone(),
            cip_writes_to_assembly: e.cip_writes_to_assembly,
            cip_file_access: e.cip_file_access,
            list_identity_requests: e.list_identity_requests,
        });

        let s7 = dp.s7.as_ref().map(|s| S7Snapshot {
            role: s.role.clone(),
            functions_seen: s.functions_seen.clone(),
        });

        let bacnet = dp.bacnet.as_ref().map(|b| BacnetSnapshot {
            role: b.role.clone(),
            write_to_output: b.write_to_output,
            write_to_notification_class: b.write_to_notification_class,
            reinitialize_device: b.reinitialize_device,
            device_communication_control: b.device_communication_control,
        });

        let iec104 = dp.iec104.as_ref().map(|i| Iec104Snapshot {
            role: i.role.clone(),
            has_control_commands: i.has_control_commands,
            has_reset_process: i.has_reset_process,
            has_interrogation: i.has_interrogation,
        });

        let profinet_dcp = dp.profinet_dcp.as_ref().map(|p| ProfinetDcpSnapshot {
            role: p.role.clone(),
        });

        deep_parse.insert(
            ip.clone(),
            DeepParseSnapshot {
                modbus,
                dnp3,
                enip,
                s7,
                bacnet,
                iec104,
                profinet_dcp,
            },
        );
    }

    AnalysisInput {
        assets,
        connections,
        deep_parse,
    }
}

pub(super) fn asset_snapshots(inventory: &InventoryState) -> Vec<AssetSnapshot> {
    inventory
        .assets
        .iter()
        .map(|a| AssetSnapshot {
            ip_address: a.ip_address.clone(),
            device_type: a.device_type.clone(),
            protocols: a.protocols.clone(),
            purdue_level: a.purdue_level,
            is_public_ip: a.is_public_ip,
            tags: a.tags.clone(),
            vendor: a.vendor.clone(),
            hostname: a.hostname.clone(),
            product_family: a.product_family.clone(),
        })
        .collect()
}

/// Build a [`CaptureContext`] from domain state slices for Phase 14C detections.
pub fn build_capture_context(
    capture: &CaptureState,
    inventory: &InventoryState,
    analysis: &AnalysisState,
) -> CaptureContext {
    // OT device IPs: assets running OT protocols or with OT device types.
    let ot_device_types = [
        "plc",
        "rtu",
        "hmi",
        "historian",
        "engineering_workstation",
        "scada_server",
        "io_server",
        "field_device",
        "controller",
    ];
    let ot_protocol_names = [
        "Modbus",
        "Dnp3",
        "EthernetIp",
        "S7comm",
        "Bacnet",
        "OpcUa",
        "Iec104",
        "ProfinetDcp",
        "HartIp",
        "GeSrtp",
        "WonderwareSuitelink",
    ];

    let mut ot_device_ips: HashSet<String> = inventory
        .assets
        .iter()
        .filter(|a| {
            ot_device_types.contains(&a.device_type.as_str())
                || a.protocols
                    .iter()
                    .any(|p| ot_protocol_names.contains(&p.as_str()))
        })
        .map(|a| a.ip_address.clone())
        .collect();

    // Also include IPs from connections to OT ports (passive inference).
    let ot_ports: &[u16] = &[
        102, 502, 1089, 1090, 1091, 2222, 2404, 4840, 5007, 5094, 18245, 18246, 20000, 34962,
        34963, 34964, 44818, 47808,
    ];
    for conn in &capture.connections {
        if ot_ports.contains(&conn.dst_port) {
            ot_device_ips.insert(conn.dst_ip.clone());
        }
    }

    // External IPs from asset classification.
    let external_ips: HashSet<String> = inventory
        .assets
        .iter()
        .filter(|a| a.is_public_ip)
        .map(|a| a.ip_address.clone())
        .collect();

    // IP ↔ MAC mappings: from assets and connection headers.
    let mut ip_to_macs: HashMap<String, Vec<String>> = HashMap::new();
    for asset in &inventory.assets {
        if let Some(mac) = &asset.mac_address {
            let macs = ip_to_macs.entry(asset.ip_address.clone()).or_default();
            if !macs.contains(mac) {
                macs.push(mac.clone());
            }
        }
    }
    for conn in &capture.connections {
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

    // Per-device first/last seen from connection_stats (f64 Unix timestamps).
    let mut device_first_seen: HashMap<String, f64> = HashMap::new();
    let mut device_last_seen: HashMap<String, f64> = HashMap::new();
    let mut capture_start = f64::INFINITY;
    let mut capture_end = f64::NEG_INFINITY;

    for cs in &analysis.connection_stats {
        if cs.first_seen < capture_start {
            capture_start = cs.first_seen;
        }
        if cs.last_seen > capture_end {
            capture_end = cs.last_seen;
        }

        let e = device_first_seen
            .entry(cs.src_ip.clone())
            .or_insert(f64::INFINITY);
        if cs.first_seen < *e {
            *e = cs.first_seen;
        }
        let e = device_last_seen
            .entry(cs.src_ip.clone())
            .or_insert(f64::NEG_INFINITY);
        if cs.last_seen > *e {
            *e = cs.last_seen;
        }

        let e = device_first_seen
            .entry(cs.dst_ip.clone())
            .or_insert(f64::INFINITY);
        if cs.first_seen < *e {
            *e = cs.first_seen;
        }
        let e = device_last_seen
            .entry(cs.dst_ip.clone())
            .or_insert(f64::NEG_INFINITY);
        if cs.last_seen > *e {
            *e = cs.last_seen;
        }
    }
    // Sanitise infinity values.
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

    // Per-source dst ports.
    let mut per_source_dst_ports: HashMap<String, HashSet<u16>> = HashMap::new();
    for conn in &capture.connections {
        per_source_dst_ports
            .entry(conn.src_ip.clone())
            .or_default()
            .insert(conn.dst_port);
    }

    // Write targets and write rates from Modbus deep parse.
    let mut per_source_write_targets: HashMap<String, HashSet<String>> = HashMap::new();
    let mut per_connection_write_rate: HashMap<(String, String), u64> = HashMap::new();

    for (ip, dp) in &inventory.deep_parse_info {
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

    // Read targets: OT connections that are NOT write targets.
    let mut per_source_read_targets: HashMap<String, HashSet<String>> = HashMap::new();
    for conn in &capture.connections {
        if ot_ports.contains(&conn.dst_port) {
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

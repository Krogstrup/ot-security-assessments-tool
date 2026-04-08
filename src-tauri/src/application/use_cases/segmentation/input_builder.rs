//! Assemble `SegmentationInput` from domain state slices.

use std::collections::{HashMap, HashSet};

use gm_segmentation::{
    AssetProfile, ObservedConnection, ProtocolRole, SecurityFinding, SegmentationInput,
};

use crate::application::mappers::snapshots::{asset_snapshots, connection_snapshots};
use crate::commands::{AnalysisState, CaptureState, InventoryState};

/// Assemble a `SegmentationInput` from domain state slices.
///
/// Lock order for callers: capture → inventory → analysis
pub fn build_segmentation_input(
    capture: &CaptureState,
    inventory: &InventoryState,
    analysis: &AnalysisState,
) -> SegmentationInput {
    // ── Assets → AssetProfile ─────────────────────────────────────────────────
    let assets: Vec<AssetProfile> = inventory
        .assets
        .iter()
        .map(|a| {
            let ot_protocols: HashSet<&str> = [
                "modbus",
                "Modbus",
                "dnp3",
                "Dnp3",
                "ethernet_ip",
                "EthernetIp",
                "s7comm",
                "S7comm",
                "bacnet",
                "Bacnet",
                "iec104",
                "Iec104",
                "profinet_dcp",
                "ProfinetDcp",
                "hart_ip",
                "HartIp",
                "ge_srtp",
                "GeSrtp",
                "wonderware_suitelink",
                "WonderwareSuitelink",
            ]
            .iter()
            .copied()
            .collect();

            let is_ot = a
                .protocols
                .iter()
                .any(|p| ot_protocols.contains(p.as_str()));
            let is_it = !is_ot && !a.protocols.is_empty();

            let mut protocol_roles: Vec<ProtocolRole> = Vec::new();
            if let Some(dp) = inventory.deep_parse_info.get(&a.ip_address) {
                if let Some(m) = &dp.modbus {
                    protocol_roles.push(ProtocolRole {
                        protocol: "modbus".to_string(),
                        role: m.role.clone(),
                    });
                }
                if let Some(d) = &dp.dnp3 {
                    protocol_roles.push(ProtocolRole {
                        protocol: "dnp3".to_string(),
                        role: d.role.clone(),
                    });
                }
                if let Some(e) = &dp.enip {
                    protocol_roles.push(ProtocolRole {
                        protocol: "ethernet_ip".to_string(),
                        role: e.role.clone(),
                    });
                }
                if let Some(s) = &dp.s7 {
                    protocol_roles.push(ProtocolRole {
                        protocol: "s7comm".to_string(),
                        role: s.role.clone(),
                    });
                }
                if let Some(b) = &dp.bacnet {
                    protocol_roles.push(ProtocolRole {
                        protocol: "bacnet".to_string(),
                        role: b.role.clone(),
                    });
                }
            }

            let subnet = compute_subnet_24(&a.ip_address);

            AssetProfile {
                ip: a.ip_address.clone(),
                mac: a.mac_address.clone(),
                hostname: a.hostname.clone(),
                vendor: a.vendor.clone(),
                device_type: a.device_type.clone(),
                product_name: a.product_family.clone(),
                purdue_level: a.purdue_level,
                protocols: a.protocols.clone(),
                protocol_roles,
                confidence: a.confidence,
                criticality: None,
                subnet,
                is_ot,
                is_it,
                is_dual_homed: is_ot && is_it,
                connection_count: a.packet_count,
                has_cves: false,
                has_default_creds: false,
            }
        })
        .collect();

    // ── Connections → ObservedConnection ─────────────────────────────────────
    let stats_map: HashMap<(&str, &str, u16), &gm_analysis::ConnectionStats> = analysis
        .connection_stats
        .iter()
        .map(|s| (s.src_ip.as_str(), s.dst_ip.as_str(), s.port))
        .zip(analysis.connection_stats.iter())
        .collect();

    let mut write_ops_set: HashSet<String> = HashSet::new();
    let mut config_ops_set: HashSet<String> = HashSet::new();
    for (ip, dp) in &inventory.deep_parse_info {
        if let Some(m) = &dp.modbus {
            let has_write = m.function_codes.iter().any(|fc| fc.is_write);
            if has_write {
                write_ops_set.insert(ip.clone());
            }
        }
        if let Some(e) = &dp.enip {
            if e.cip_writes_to_assembly {
                write_ops_set.insert(ip.clone());
            }
            if e.cip_file_access {
                config_ops_set.insert(ip.clone());
            }
        }
        if let Some(b) = &dp.bacnet {
            if b.write_to_output || b.write_to_notification_class {
                write_ops_set.insert(ip.clone());
            }
        }
        if let Some(s7) = &dp.s7 {
            let config_fns = ["download", "upload", "stop", "program_transfer"];
            if s7
                .functions_seen
                .iter()
                .any(|f| config_fns.iter().any(|cf| f.contains(cf)))
            {
                config_ops_set.insert(ip.clone());
            }
        }
    }

    let allowlist_set: HashSet<String> = {
        use gm_analysis::generate_allowlist;
        let asset_snaps = asset_snapshots(inventory);
        let conn_snaps = connection_snapshots(capture);
        let entries = generate_allowlist(&conn_snaps, &asset_snaps, &analysis.connection_stats);
        entries
            .iter()
            .map(|e| format!("{}→{}:{}:{}", e.src_ip, e.dst_ip, e.protocol, e.dst_port))
            .collect()
    };

    let connections: Vec<ObservedConnection> = capture
        .connections
        .iter()
        .map(|c| {
            let stat = stats_map
                .get(&(c.src_ip.as_str(), c.dst_ip.as_str(), c.dst_port))
                .copied();
            let is_periodic = stat.map(|s| s.is_periodic).unwrap_or(false);
            let pattern_anomaly = analysis
                .pattern_anomalies
                .iter()
                .any(|pa| pa.src_ip == c.src_ip && pa.dst_ip == c.dst_ip && pa.port == c.dst_port);

            let has_write = write_ops_set.contains(&c.src_ip) || write_ops_set.contains(&c.dst_ip);
            let has_config =
                config_ops_set.contains(&c.src_ip) || config_ops_set.contains(&c.dst_ip);

            let allowlist_key = format!("{}→{}:{}:{}", c.src_ip, c.dst_ip, c.protocol, c.dst_port);
            let is_in_allowlist = allowlist_set.contains(&allowlist_key);

            ObservedConnection {
                src_ip: c.src_ip.clone(),
                src_port: c.src_port,
                dst_ip: c.dst_ip.clone(),
                dst_port: c.dst_port,
                protocol: c.protocol.clone(),
                packet_count: c.packet_count,
                byte_count: c.byte_count,
                first_seen: c.first_seen.clone(),
                last_seen: c.last_seen.clone(),
                is_periodic,
                pattern_anomaly,
                has_write_operations: has_write,
                has_read_operations: true,
                has_config_operations: has_config,
                attack_techniques: Vec::new(),
                is_in_allowlist,
            }
        })
        .collect();

    // ── SecurityFindings ──────────────────────────────────────────────────────
    let findings: Vec<SecurityFinding> = analysis
        .findings
        .iter()
        .map(|f| SecurityFinding {
            id: f.id.clone(),
            technique_id: f.technique_id.clone(),
            severity: format!("{:?}", f.severity).to_lowercase(),
            affected_ips: f.affected_assets.clone(),
            description: f.description.clone(),
        })
        .collect();

    SegmentationInput {
        assets,
        connections,
        findings,
    }
}

/// Compute the /24 subnet string for a given IPv4 address.
pub fn compute_subnet_24(ip: &str) -> Option<String> {
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() == 4 {
        Some(format!("{}.{}.{}.0/24", parts[0], parts[1], parts[2]))
    } else {
        None
    }
}

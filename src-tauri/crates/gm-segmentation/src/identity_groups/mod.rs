//! Phase 15A — Identity Group Engine.
//!
//! Clusters every discovered asset into exactly one [`PolicyGroup`] by identity
//! attributes (Purdue level, protocol role, vendor, communication community).
//!
//! ## Algorithm
//!
//! 1. **Primary partition** by Purdue level (unassigned → separate pool).
//! 2. **Secondary partition** by device role within each level:
//!    - L0/L1: protocol server role (Modbus slave, DNP3 outstation, S7 server, EtherNet/IP adapter, …)
//!    - L2: HMI (read clients) vs Engineering (config/program operations observed)
//!    - L3: Historian vs SCADA server vs dual-homed DMZ gateway
//!    - L4+: by IT protocol type (web, remote access, other)
//! 3. **Tertiary vendor split** — split a role subgroup by vendor only if the
//!    vendor subgroups communicate with disjoint peer sets (Jaccard similarity
//!    of neighbor sets < [`thresholds::VENDOR_SPLIT_JACCARD_THRESHOLD`]).
//!    Otherwise keep together.
//! 4. **Community detection** for unassigned assets — greedy clustering by
//!    neighbor overlap (merge into existing community if Jaccard >
//!    [`thresholds::COMMUNITY_MERGE_JACCARD_THRESHOLD`]).
//! 5. **Auto-naming, SecurityLevel, Criticality** assignment.

mod community_detection;
mod graph;
mod role_partition;
mod vendor_split;

use std::collections::HashMap;

use crate::{AssetProfile, PolicyGroup, SegmentationInput};

// Re-export for callers in sibling modules (e.g., zones.rs).
pub use graph::security_level_from_purdue;

// ── Public entry point ────────────────────────────────────────────────────────

/// Build identity-based policy groups from the segmentation input.
///
/// Returns one [`PolicyGroup`] per identity cluster. Every asset in
/// `input.assets` ends up in exactly one group.
pub fn build_policy_groups(input: &SegmentationInput) -> Vec<PolicyGroup> {
    // Build per-IP neighbor sets (used for vendor split and community detection).
    let neighbor_sets = graph::build_neighbor_sets(&input.connections);

    // Step 1: Partition by Purdue level.
    let mut by_level: HashMap<Option<u8>, Vec<&AssetProfile>> = HashMap::new();
    for asset in &input.assets {
        by_level.entry(asset.purdue_level).or_default().push(asset);
    }

    // Process levels in sorted order for deterministic output.
    let mut sorted_keys: Vec<Option<u8>> = by_level.keys().copied().collect();
    sorted_keys.sort();

    let mut groups: Vec<PolicyGroup> = Vec::new();

    for key in sorted_keys {
        // Unassigned (None) is handled separately after all levels.
        let Some(level) = key else { continue };
        let assets = &by_level[&Some(level)];

        // Step 2: Role-based sub-groups within this level.
        let role_subgroups = role_partition::split_by_role(level, assets, input);

        // Step 3: Optional vendor split within each role sub-group.
        for (role_label, ips, category) in role_subgroups {
            let vendor_subgroups =
                vendor_split::maybe_split_by_vendor(&ips, input, &neighbor_sets);
            for (vendor_label, vendor_ips) in vendor_subgroups {
                let group_name = if vendor_label.is_empty() {
                    format!("L{}-{}", level, role_label)
                } else {
                    format!("L{}-{}-{}", level, role_label, vendor_label)
                };

                let security_level = graph::security_level_from_purdue(Some(level));
                let criticality = graph::max_criticality_for_ips(&vendor_ips, &input.assets);

                groups.push(PolicyGroup::new(
                    group_name,
                    vendor_ips,
                    Some(level),
                    category,
                    security_level,
                    criticality,
                ));
            }
        }
    }

    // Step 4: Community detection for unassigned assets.
    if let Some(unassigned) = by_level.get(&None) {
        let community_groups =
            community_detection::detect_communities(unassigned, input, &neighbor_sets);
        groups.extend(community_groups);
    }

    groups
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Criticality, DeviceCategory, ObservedConnection, ProtocolRole, SegmentationInput};

    fn make_asset(ip: &str, device_type: &str, purdue_level: Option<u8>) -> AssetProfile {
        crate::AssetProfile {
            ip: ip.to_string(),
            mac: None,
            hostname: None,
            vendor: None,
            device_type: device_type.to_string(),
            product_name: None,
            purdue_level,
            protocols: Vec::new(),
            protocol_roles: Vec::new(),
            confidence: 1,
            criticality: None,
            subnet: None,
            is_ot: purdue_level.map(|l| l <= 3).unwrap_or(false),
            is_it: purdue_level.map(|l| l >= 4).unwrap_or(false),
            is_dual_homed: false,
            connection_count: 0,
            has_cves: false,
            has_default_creds: false,
        }
    }

    fn make_conn(src: &str, dst: &str) -> ObservedConnection {
        ObservedConnection {
            src_ip: src.to_string(),
            src_port: 1024,
            dst_ip: dst.to_string(),
            dst_port: 502,
            protocol: "modbus".to_string(),
            packet_count: 100,
            byte_count: 1000,
            first_seen: "2026-01-01T00:00:00Z".to_string(),
            last_seen: "2026-01-01T01:00:00Z".to_string(),
            is_periodic: true,
            pattern_anomaly: false,
            has_write_operations: false,
            has_read_operations: true,
            has_config_operations: false,
            attack_techniques: Vec::new(),
            is_in_allowlist: false,
        }
    }

    #[test]
    fn test_empty_input_returns_no_groups() {
        let input = SegmentationInput::default();
        let groups = build_policy_groups(&input);
        assert!(groups.is_empty());
    }

    #[test]
    fn test_single_l1_asset_gets_one_group() {
        let input = SegmentationInput {
            assets: vec![make_asset("10.0.0.1", "plc", Some(1))],
            ..Default::default()
        };
        let groups = build_policy_groups(&input);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].member_ips, vec!["10.0.0.1"]);
        assert_eq!(groups[0].purdue_level, Some(1));
    }

    #[test]
    fn test_l1_and_l2_get_separate_groups() {
        let input = SegmentationInput {
            assets: vec![
                make_asset("10.0.0.1", "plc", Some(1)),
                make_asset("10.0.0.10", "hmi", Some(2)),
            ],
            ..Default::default()
        };
        let groups = build_policy_groups(&input);
        assert_eq!(groups.len(), 2);
        let levels: Vec<Option<u8>> = groups.iter().map(|g| g.purdue_level).collect();
        assert!(levels.contains(&Some(1)));
        assert!(levels.contains(&Some(2)));
    }

    #[test]
    fn test_l1_split_by_protocol_role() {
        let mut modbus_plc = make_asset("10.0.0.1", "plc", Some(1));
        modbus_plc.protocol_roles = vec![ProtocolRole {
            protocol: "modbus".to_string(),
            role: "slave".to_string(),
        }];

        let mut profinet_plc = make_asset("10.0.0.2", "plc", Some(1));
        profinet_plc.protocol_roles = vec![ProtocolRole {
            protocol: "profinet".to_string(),
            role: "io_device".to_string(),
        }];

        let input = SegmentationInput {
            assets: vec![modbus_plc, profinet_plc],
            ..Default::default()
        };
        let groups = build_policy_groups(&input);
        assert_eq!(groups.len(), 2);
        assert!(groups.iter().all(|g| g.purdue_level == Some(1)));
        assert_ne!(groups[0].name, groups[1].name);
    }

    #[test]
    fn test_l2_engineering_splits_from_hmi() {
        let hmi = make_asset("10.0.0.10", "hmi", Some(2));
        let eng = make_asset("10.0.0.11", "engineering_workstation", Some(2));

        let mut config_conn = make_conn("10.0.0.11", "10.0.0.1");
        config_conn.has_config_operations = true;

        let input = SegmentationInput {
            assets: vec![hmi, eng],
            connections: vec![config_conn],
            ..Default::default()
        };
        let groups = build_policy_groups(&input);

        let categories: Vec<DeviceCategory> = groups
            .iter()
            .filter(|g| g.purdue_level == Some(2))
            .map(|g| g.device_category)
            .collect();

        assert!(categories.contains(&DeviceCategory::Hmi));
        assert!(categories.contains(&DeviceCategory::EngineeringStation));
    }

    #[test]
    fn test_l1_security_level_is_sl3() {
        let input = SegmentationInput {
            assets: vec![make_asset("10.0.0.1", "plc", Some(1))],
            ..Default::default()
        };
        let groups = build_policy_groups(&input);
        assert_eq!(groups[0].security_level, SecurityLevel::Sl3);
    }

    #[test]
    fn test_l0_security_level_is_sl3() {
        let input = SegmentationInput {
            assets: vec![make_asset("10.0.0.1", "sensor", Some(0))],
            ..Default::default()
        };
        let groups = build_policy_groups(&input);
        assert_eq!(groups[0].security_level, SecurityLevel::Sl3);
    }

    #[test]
    fn test_l2_security_level_is_sl2() {
        let input = SegmentationInput {
            assets: vec![make_asset("10.0.0.10", "hmi", Some(2))],
            ..Default::default()
        };
        let groups = build_policy_groups(&input);
        assert_eq!(groups[0].security_level, SecurityLevel::Sl2);
    }

    #[test]
    fn test_l4_security_level_is_sl1() {
        let input = SegmentationInput {
            assets: vec![make_asset("192.168.1.100", "it_device", Some(4))],
            ..Default::default()
        };
        let groups = build_policy_groups(&input);
        assert_eq!(groups[0].security_level, SecurityLevel::Sl1);
    }

    #[test]
    fn test_criticality_max_of_members() {
        let mut plc1 = make_asset("10.0.0.1", "plc", Some(1));
        plc1.criticality = Some("critical".to_string());
        let mut plc2 = make_asset("10.0.0.2", "plc", Some(1));
        plc2.criticality = Some("medium".to_string());

        let input = SegmentationInput {
            assets: vec![plc1, plc2],
            ..Default::default()
        };
        let groups = build_policy_groups(&input);

        let l1_group = groups
            .iter()
            .find(|g| g.purdue_level == Some(1))
            .expect("L1 group should exist");
        assert_eq!(l1_group.criticality, Criticality::Critical);
    }

    #[test]
    fn test_vendor_split_disjoint_neighbors() {
        let mut rock_plc = make_asset("10.0.0.1", "plc", Some(1));
        rock_plc.vendor = Some("Rockwell".to_string());
        rock_plc.protocol_roles = vec![ProtocolRole {
            protocol: "ethernet_ip".to_string(),
            role: "adapter".to_string(),
        }];

        let mut sie_plc = make_asset("10.0.0.2", "plc", Some(1));
        sie_plc.vendor = Some("Siemens".to_string());
        sie_plc.protocol_roles = vec![ProtocolRole {
            protocol: "ethernet_ip".to_string(),
            role: "adapter".to_string(),
        }];

        let input = SegmentationInput {
            assets: vec![rock_plc, sie_plc],
            connections: vec![
                make_conn("10.0.1.1", "10.0.0.1"),
                make_conn("10.0.2.1", "10.0.0.2"),
            ],
            ..Default::default()
        };

        let groups = build_policy_groups(&input);
        let l1_groups: Vec<&PolicyGroup> = groups
            .iter()
            .filter(|g| g.purdue_level == Some(1))
            .collect();
        assert_eq!(
            l1_groups.len(),
            2,
            "Disjoint vendors should split into 2 groups"
        );
    }

    #[test]
    fn test_vendor_no_split_shared_neighbors() {
        let mut rock_plc = make_asset("10.0.0.1", "plc", Some(1));
        rock_plc.vendor = Some("Rockwell".to_string());
        rock_plc.protocol_roles = vec![ProtocolRole {
            protocol: "ethernet_ip".to_string(),
            role: "adapter".to_string(),
        }];

        let mut sie_plc = make_asset("10.0.0.2", "plc", Some(1));
        sie_plc.vendor = Some("Siemens".to_string());
        sie_plc.protocol_roles = vec![ProtocolRole {
            protocol: "ethernet_ip".to_string(),
            role: "adapter".to_string(),
        }];

        let input = SegmentationInput {
            assets: vec![rock_plc, sie_plc],
            connections: vec![
                make_conn("10.0.1.1", "10.0.0.1"),
                make_conn("10.0.1.1", "10.0.0.2"),
            ],
            ..Default::default()
        };

        let groups = build_policy_groups(&input);
        let l1_groups: Vec<&PolicyGroup> = groups
            .iter()
            .filter(|g| g.purdue_level == Some(1))
            .collect();
        assert_eq!(
            l1_groups.len(),
            1,
            "Shared neighbors should merge into 1 group"
        );
    }

    #[test]
    fn test_unassigned_merged_into_community() {
        let a1 = make_asset("10.0.0.1", "unknown", None);
        let a2 = make_asset("10.0.0.2", "unknown", None);

        let conns: Vec<ObservedConnection> = (1u8..=4)
            .flat_map(|i| {
                let target = format!("10.0.1.{}", i);
                vec![
                    make_conn("10.0.0.1", &target),
                    make_conn("10.0.0.2", &target),
                ]
            })
            .collect();

        let input = SegmentationInput {
            assets: vec![a1, a2],
            connections: conns,
            ..Default::default()
        };

        let groups = build_policy_groups(&input);
        let unassigned: Vec<&PolicyGroup> =
            groups.iter().filter(|g| g.purdue_level.is_none()).collect();

        assert_eq!(
            unassigned.len(),
            1,
            "High-overlap assets should merge into one community"
        );
        assert_eq!(
            unassigned[0].member_ips.len(),
            2,
            "Community should contain both assets"
        );
    }

    #[test]
    fn test_group_name_starts_with_level() {
        let input = SegmentationInput {
            assets: vec![
                make_asset("10.0.0.1", "plc", Some(1)),
                make_asset("10.0.0.10", "hmi", Some(2)),
                make_asset("192.168.1.1", "it_device", Some(4)),
            ],
            ..Default::default()
        };
        let groups = build_policy_groups(&input);

        for group in &groups {
            if let Some(level) = group.purdue_level {
                assert!(
                    group.name.starts_with(&format!("L{}-", level)),
                    "Group '{}' at L{} should start with 'L{}-'",
                    group.name,
                    level,
                    level
                );
            }
        }
    }

    #[test]
    fn test_l0_gets_sensor_category() {
        let input = SegmentationInput {
            assets: vec![make_asset("10.0.0.1", "sensor", Some(0))],
            ..Default::default()
        };
        let groups = build_policy_groups(&input);
        assert_eq!(groups[0].device_category, DeviceCategory::Sensor);
    }

    #[test]
    fn test_security_level_from_purdue_mapping() {
        assert_eq!(security_level_from_purdue(Some(0)), SecurityLevel::Sl3);
        assert_eq!(security_level_from_purdue(Some(1)), SecurityLevel::Sl3);
        assert_eq!(security_level_from_purdue(Some(2)), SecurityLevel::Sl2);
        assert_eq!(security_level_from_purdue(Some(3)), SecurityLevel::Sl2);
        assert_eq!(security_level_from_purdue(Some(4)), SecurityLevel::Sl1);
        assert_eq!(security_level_from_purdue(None), SecurityLevel::Sl1);
    }
}

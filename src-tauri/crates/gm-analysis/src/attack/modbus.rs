use std::collections::HashSet;

use gm_constants::MODBUS_WRITE_FCS;

use crate::{AnalysisInput, Finding, FindingType, Severity};

/// T0855 — Unauthorized Command Message
///
/// Detects Modbus broadcast/mass writes: FC 5/6/15/16 sent to
/// unit ID 0 (broadcast) or unit ID 255 (all devices), or
/// a single source writing to many targets (high fan-out).
pub(super) fn detect_t0855_unauthorized_writes(input: &AnalysisInput) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (ip, dp) in &input.deep_parse {
        let modbus = match &dp.modbus {
            Some(m) => m,
            None => continue,
        };

        // Only check masters (devices sending write commands)
        if modbus.role != "master" && modbus.role != "both" {
            continue;
        }

        // Check for broadcast writes (unit ID 0 or 255)
        let has_broadcast_unit = modbus.unit_ids.contains(&0) || modbus.unit_ids.contains(&255);
        let has_write_fcs = modbus
            .function_codes
            .iter()
            .any(|fc| MODBUS_WRITE_FCS.contains(&fc.code) && fc.count > 0);

        if has_broadcast_unit && has_write_fcs {
            let write_count: u64 = modbus
                .function_codes
                .iter()
                .filter(|fc| MODBUS_WRITE_FCS.contains(&fc.code))
                .map(|fc| fc.count)
                .sum();

            let broadcast_ids: Vec<String> = modbus
                .unit_ids
                .iter()
                .filter(|&&uid| uid == 0 || uid == 255)
                .map(|uid| uid.to_string())
                .collect();

            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::Critical,
                format!("Modbus broadcast write from {}", ip),
                "Modbus write commands (FC 5/6/15/16) sent to broadcast unit IDs. \
                 This could indicate unauthorized command injection targeting all \
                 devices on the Modbus network simultaneously."
                    .to_string(),
                vec![ip.clone()],
                format!(
                    "Source {} sent {} write commands to broadcast unit ID(s): {}",
                    ip,
                    write_count,
                    broadcast_ids.join(", ")
                ),
                Some(crate::attack_codes::T0855.to_string()),
            ));
        }

        // Check for high fan-out writes (writing to many different targets)
        let write_targets: Vec<&str> = modbus
            .relationships
            .iter()
            .filter(|r| r.remote_role == "slave")
            .map(|r| r.remote_ip.as_str())
            .collect();

        if write_targets.len() >= 5 && has_write_fcs {
            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::High,
                format!("High fan-out Modbus writes from {}", ip),
                "A single device is sending Modbus write commands to many targets. \
                 This pattern may indicate unauthorized mass command injection."
                    .to_string(),
                std::iter::once(ip.clone())
                    .chain(write_targets.iter().map(|s| s.to_string()))
                    .collect(),
                format!(
                    "Source {} writing to {} targets: {}",
                    ip,
                    write_targets.len(),
                    write_targets.join(", ")
                ),
                Some(crate::attack_codes::T0855.to_string()),
            ));
        }
    }

    findings
}

/// T0814 — Denial of Service
///
/// Detects Modbus FC 8 (Diagnostics) from devices that are not
/// classified as engineering workstations. FC 8 can restart or
/// clear PLC memory — risky from unauthorized sources.
pub(super) fn detect_t0814_diagnostic_dos(input: &AnalysisInput) -> Vec<Finding> {
    let mut findings = Vec::new();

    // Build set of engineering workstation IPs
    let eng_ws_ips: HashSet<&str> = input
        .assets
        .iter()
        .filter(|a| a.device_type == gm_constants::DEVICE_TYPE_EWS)
        .map(|a| a.ip_address.as_str())
        .collect();

    for (ip, dp) in &input.deep_parse {
        let modbus = match &dp.modbus {
            Some(m) => m,
            None => continue,
        };

        // Look for FC 8 (Diagnostics) usage
        let fc8_count: u64 = modbus
            .function_codes
            .iter()
            .filter(|fc| fc.code == 8)
            .map(|fc| fc.count)
            .sum();

        if fc8_count == 0 {
            continue;
        }

        // If the source is not an engineering workstation, flag it
        if !eng_ws_ips.contains(ip.as_str()) {
            let device_type = input
                .assets
                .iter()
                .find(|a| a.ip_address == *ip)
                .map(|a| a.device_type.as_str())
                .unwrap_or("unknown");

            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::High,
                format!("Modbus diagnostics (FC 8) from non-engineer: {}", ip),
                "Modbus Function Code 8 (Diagnostics) can restart slave devices, \
                 clear counters, or force listen-only mode. This function code \
                 should only originate from authorized engineering workstations."
                    .to_string(),
                vec![ip.clone()],
                format!(
                    "Device {} (type: {}) sent {} Modbus FC 8 diagnostic commands",
                    ip, device_type, fc8_count
                ),
                Some(crate::attack_codes::T0814.to_string()),
            ));
        }
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(test)]
    use crate::attack::test_utils::make_input;
    use crate::*;


    #[test]
    fn test_t0855_broadcast_write() {
        let mut input = make_input();
        input.deep_parse.insert(
            "10.0.0.100".to_string(),
            DeepParseSnapshot {
                modbus: Some(ModbusSnapshot {
                    role: "master".to_string(),
                    unit_ids: vec![0, 1, 255],
                    function_codes: vec![
                        FcSnapshot {
                            code: 6,
                            count: 50,
                            is_write: true,
                        },
                        FcSnapshot {
                            code: 3,
                            count: 200,
                            is_write: false,
                        },
                    ],
                    relationships: vec![],
                    polling_intervals: vec![],
                }),
                ..Default::default()
            },
        );

        let findings = detect_t0855_unauthorized_writes(&input);
        assert!(!findings.is_empty());
        assert_eq!(findings[0].severity, Severity::Critical);
        assert_eq!(findings[0].technique_id, Some(crate::attack_codes::T0855.to_string()));
    }

    #[test]
    fn test_t0855_high_fanout() {
        let mut input = make_input();
        let targets: Vec<RelationshipSnapshot> = (1..=6)
            .map(|i| RelationshipSnapshot {
                remote_ip: format!("10.0.0.{}", i),
                remote_role: "slave".to_string(),
                packet_count: 100,
            })
            .collect();

        input.deep_parse.insert(
            "10.0.0.100".to_string(),
            DeepParseSnapshot {
                modbus: Some(ModbusSnapshot {
                    role: "master".to_string(),
                    unit_ids: vec![1, 2, 3],
                    function_codes: vec![FcSnapshot {
                        code: 16,
                        count: 100,
                        is_write: true,
                    }],
                    relationships: targets,
                    polling_intervals: vec![],
                }),
                ..Default::default()
            },
        );

        let findings = detect_t0855_unauthorized_writes(&input);
        assert!(findings.iter().any(|f| f.title.contains("fan-out")));
    }

    #[test]
    fn test_t0814_fc8_from_non_engineer() {
        let mut input = make_input();
        input.assets.push(AssetSnapshot {
            ip_address: "10.0.0.50".to_string(),
            device_type: "it_device".to_string(),
            protocols: vec!["modbus".to_string()],
            purdue_level: None,
            is_public_ip: false,
            tags: vec![],
            vendor: None,
            hostname: None,
            product_family: None,
        });

        input.deep_parse.insert(
            "10.0.0.50".to_string(),
            DeepParseSnapshot {
                modbus: Some(ModbusSnapshot {
                    role: "master".to_string(),
                    unit_ids: vec![1],
                    function_codes: vec![FcSnapshot {
                        code: 8,
                        count: 10,
                        is_write: false,
                    }],
                    relationships: vec![],
                    polling_intervals: vec![],
                }),
                ..Default::default()
            },
        );

        let findings = detect_t0814_diagnostic_dos(&input);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, Severity::High);
        assert_eq!(findings[0].technique_id, Some(crate::attack_codes::T0814.to_string()));
    }

    #[test]
    fn test_t0814_fc8_from_engineer_ok() {
        let mut input = make_input();
        input.assets.push(AssetSnapshot {
            ip_address: "10.0.0.50".to_string(),
            device_type: "engineering_workstation".to_string(),
            protocols: vec!["modbus".to_string()],
            purdue_level: None,
            is_public_ip: false,
            tags: vec![],
            vendor: None,
            hostname: None,
            product_family: None,
        });

        input.deep_parse.insert(
            "10.0.0.50".to_string(),
            DeepParseSnapshot {
                modbus: Some(ModbusSnapshot {
                    role: "master".to_string(),
                    unit_ids: vec![1],
                    function_codes: vec![FcSnapshot {
                        code: 8,
                        count: 10,
                        is_write: false,
                    }],
                    relationships: vec![],
                    polling_intervals: vec![],
                }),
                ..Default::default()
            },
        );

        let findings = detect_t0814_diagnostic_dos(&input);
        assert!(
            findings.is_empty(),
            "FC 8 from engineering workstation should not be flagged"
        );
    }
}

use std::collections::{HashMap, HashSet};

use gm_constants::OT_SERVER_PORTS;

use crate::{AnalysisInput, Finding, FindingType, Severity};

/// T0846 — Remote System Discovery
///
/// Detects unknown/IT devices polling OT devices on well-known
/// ICS ports. An unknown device scanning PLC ports suggests
/// reconnaissance.
pub(super) fn detect_t0846_remote_discovery(input: &AnalysisInput) -> Vec<Finding> {
    let mut findings = Vec::new();

    // Build set of known OT device IPs (PLCs, RTUs, HMIs, etc.)
    let ot_device_ips: HashSet<&str> = input
        .assets
        .iter()
        .filter(|a| {
            matches!(
                a.device_type.as_str(),
                "plc" | "rtu" | "hmi" | "historian" | "engineering_workstation" | "scada_server"
            )
        })
        .map(|a| a.ip_address.as_str())
        .collect();

    // Find connections where an unknown/IT device connects to OT ports
    let mut scanner_targets: HashMap<String, HashSet<String>> = HashMap::new();

    for conn in &input.connections {
        // Check if destination is an OT server port
        if !OT_SERVER_PORTS.contains(&conn.dst_port) {
            continue;
        }

        // Check if the source is NOT a known OT device
        let src_asset = input.assets.iter().find(|a| a.ip_address == conn.src_ip);
        let is_known_ot = ot_device_ips.contains(conn.src_ip.as_str());

        // Flag if source is IT/unknown AND targeting OT ports on multiple devices
        if !is_known_ot {
            let src_type = src_asset
                .map(|a| a.device_type.as_str())
                .unwrap_or("unknown");
            if src_type == "it_device" || src_type == "unknown" {
                scanner_targets
                    .entry(conn.src_ip.clone())
                    .or_default()
                    .insert(conn.dst_ip.clone());
            }
        }
    }

    // Only flag scanners targeting 3+ different OT devices
    for (scanner_ip, targets) in &scanner_targets {
        if targets.len() >= 3 {
            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::High,
                format!(
                    "Unknown device {} polling {} PLCs/RTUs",
                    scanner_ip,
                    targets.len()
                ),
                "A device not classified as OT equipment is connecting to \
                 multiple ICS devices on well-known OT service ports. This \
                 behavior resembles network reconnaissance or unauthorized polling."
                    .to_string(),
                std::iter::once(scanner_ip.clone())
                    .chain(targets.iter().cloned())
                    .collect(),
                format!(
                    "Device {} connected to OT ports on {} targets: {}",
                    scanner_ip,
                    targets.len(),
                    targets.iter().cloned().collect::<Vec<_>>().join(", ")
                ),
                Some("T0846".to_string()),
            ));
        }
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    fn make_input() -> AnalysisInput {
        AnalysisInput::default()
    }

    #[test]
    fn test_t0846_unknown_device_scanning() {
        let mut input = make_input();

        // Known OT devices
        for i in 1..=5 {
            input.assets.push(AssetSnapshot {
                ip_address: format!("10.0.0.{}", i),
                device_type: "plc".to_string(),
                protocols: vec!["modbus".to_string()],
                purdue_level: Some(1),
                is_public_ip: false,
                tags: vec![],
                vendor: None,
                hostname: None,
                product_family: None,
            });
        }

        // Unknown scanner
        input.assets.push(AssetSnapshot {
            ip_address: "10.0.0.200".to_string(),
            device_type: "unknown".to_string(),
            protocols: vec![],
            purdue_level: None,
            is_public_ip: false,
            tags: vec![],
            vendor: None,
            hostname: None,
            product_family: None,
        });

        // Scanner connecting to 3+ PLCs on Modbus port
        for i in 1..=4 {
            input.connections.push(ConnectionSnapshot {
                src_ip: "10.0.0.200".to_string(),
                dst_ip: format!("10.0.0.{}", i),
                src_port: 49152,
                dst_port: 502,
                protocol: "Modbus".to_string(),
                packet_count: 10,
            });
        }

        let findings = detect_t0846_remote_discovery(&input);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, Severity::High);
        assert_eq!(findings[0].technique_id, Some("T0846".to_string()));
    }

    #[test]
    fn test_no_false_positive_known_hmi_polling() {
        let mut input = make_input();

        // Known HMI polling PLCs is normal behavior
        input.assets.push(AssetSnapshot {
            ip_address: "10.0.0.100".to_string(),
            device_type: "hmi".to_string(),
            protocols: vec!["modbus".to_string()],
            purdue_level: Some(2),
            is_public_ip: false,
            tags: vec![],
            vendor: None,
            hostname: None,
            product_family: None,
        });

        for i in 1..=5 {
            input.assets.push(AssetSnapshot {
                ip_address: format!("10.0.0.{}", i),
                device_type: "plc".to_string(),
                protocols: vec!["modbus".to_string()],
                purdue_level: Some(1),
                is_public_ip: false,
                tags: vec![],
                vendor: None,
                hostname: None,
                product_family: None,
            });

            input.connections.push(ConnectionSnapshot {
                src_ip: "10.0.0.100".to_string(),
                dst_ip: format!("10.0.0.{}", i),
                src_port: 49152,
                dst_port: 502,
                protocol: "Modbus".to_string(),
                packet_count: 1000,
            });
        }

        let findings = detect_t0846_remote_discovery(&input);
        assert!(
            findings.is_empty(),
            "HMI polling PLCs should not trigger T0846"
        );
    }
}

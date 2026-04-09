use std::collections::{HashMap, HashSet};

use gm_types::CLEARTEXT_OT_PORTS;

use crate::{AnalysisInput, Finding, FindingType, Severity};

/// Convert an IPv4 address to its /24 network prefix (e.g., "192.168.1.100" → "192.168.1.0/24").
fn ip_to_slash24(ip: &str) -> String {
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() == 4 {
        format!("{}.{}.{}.0/24", parts[0], parts[1], parts[2])
    } else {
        ip.to_string()
    }
}

/// Flat Network Detection
///
/// If >80% of discovered devices share the same /24 subnet and
/// there are more than 5 devices, this indicates a flat (unsegmented)
/// network, which is a critical OT security risk.
pub(super) fn detect_flat_network(input: &AnalysisInput) -> Vec<Finding> {
    let mut findings = Vec::new();

    if input.assets.len() < 6 {
        return findings; // Too few devices to make this determination
    }

    // Count devices per /24 subnet
    let mut subnet_counts: HashMap<String, Vec<String>> = HashMap::new();
    for asset in &input.assets {
        let subnet = ip_to_slash24(&asset.ip_address);
        subnet_counts
            .entry(subnet)
            .or_default()
            .push(asset.ip_address.clone());
    }

    let total = input.assets.len();
    for (subnet, ips) in &subnet_counts {
        let pct = ips.len() as f64 / total as f64;
        if pct > 0.8 && ips.len() > 5 {
            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::High,
                format!(
                    "Flat network detected — {} of {} devices on {}",
                    ips.len(),
                    total,
                    subnet
                ),
                "More than 80% of discovered devices reside on a single /24 subnet. \
                 A flat network provides no lateral movement barriers — a single \
                 compromised device can reach all OT assets without traversing any \
                 security boundary."
                    .to_string(),
                ips.clone(),
                format!(
                    "{}/{} devices ({:.0}%) are on subnet {} — no segmentation detected",
                    ips.len(),
                    total,
                    pct * 100.0,
                    subnet
                ),
                Some(crate::attack_codes::T0886.to_string()),
            ));
        }
    }

    findings
}

/// Protocol Encryption Audit
///
/// All standard ICS protocols (Modbus, DNP3, EtherNet/IP, S7comm, etc.)
/// are cleartext. This function computes what percentage of OT connections
/// are unencrypted and generates a finding if any cleartext OT traffic exists.
pub(super) fn detect_cleartext_ot(input: &AnalysisInput) -> Vec<Finding> {
    let mut findings = Vec::new();
    let mut cleartext_by_proto: HashMap<String, u64> = HashMap::new();
    let mut total_cleartext: u64 = 0;
    let mut total_ot: u64 = 0;

    for conn in &input.connections {
        if CLEARTEXT_OT_PORTS.contains(&conn.dst_port) {
            *cleartext_by_proto.entry(conn.protocol.clone()).or_insert(0) += conn.packet_count;
            total_cleartext += conn.packet_count;
            total_ot += conn.packet_count;
        } else if conn.protocol == "OpcUa" && conn.dst_port == 4843 {
            // OPC UA with TLS — don't count as cleartext
            total_ot += conn.packet_count;
        }
    }

    if total_cleartext == 0 {
        return findings;
    }

    let pct = if total_ot > 0 {
        (total_cleartext as f64 / total_ot as f64) * 100.0
    } else {
        100.0
    };

    let severity = if pct >= 99.0 {
        Severity::High
    } else if pct >= 50.0 {
        Severity::Medium
    } else {
        Severity::Low
    };

    let proto_breakdown: Vec<String> = cleartext_by_proto
        .iter()
        .map(|(p, c)| format!("{}: {} packets", p, c))
        .collect();

    findings.push(Finding::new(
        FindingType::AttackTechnique,
        severity,
        format!("{:.0}% of OT traffic is unencrypted", pct),
        "Standard OT protocols (Modbus, DNP3, EtherNet/IP, S7comm, BACnet, IEC 104, PROFINET) \
         transmit all data in cleartext. An attacker with network access can read all process \
         values, setpoints, and control commands without any decryption."
            .to_string(),
        vec![],
        format!(
            "{:.1}% cleartext OT traffic ({} of {} packets). Breakdown: {}",
            pct,
            total_cleartext,
            total_ot,
            proto_breakdown.join(", ")
        ),
        None,
    ));

    findings
}

/// Internet Exposure Check
///
/// If an OT device has a public IP address (determined by GeoIP returning
/// a country code), it may be directly internet-accessible. This is a
/// Critical finding — internet-facing PLCs/RTUs are a common attack vector.
pub(super) fn detect_internet_exposed_ot(input: &AnalysisInput) -> Vec<Finding> {
    let mut findings = Vec::new();

    for asset in &input.assets {
        if !asset.is_public_ip {
            continue;
        }

        // Check if this device speaks any OT protocol
        let ot_protocols: Vec<&str> = asset
            .protocols
            .iter()
            .filter(|p| {
                let pl = p.to_lowercase();
                pl.contains("modbus")
                    || pl.contains("dnp3")
                    || pl.contains("s7comm")
                    || pl.contains("ethernet")
                    || pl.contains("bacnet")
                    || pl.contains("iec104")
                    || pl.contains("profinet")
                    || pl.contains("opcua")
            })
            .map(|p| p.as_str())
            .collect();

        if ot_protocols.is_empty() {
            continue;
        }

        // Build Shodan query for assessor
        let ot_ports: Vec<&str> = ot_protocols
            .iter()
            .filter_map(|p| match p.to_lowercase().as_str() {
                s if s.contains("modbus") => Some("port:502"),
                s if s.contains("dnp3") => Some("port:20000"),
                s if s.contains("s7comm") => Some("port:102"),
                s if s.contains("ethernet") => Some("port:44818"),
                s if s.contains("bacnet") => Some("port:47808"),
                s if s.contains("iec104") => Some("port:2404"),
                _ => None,
            })
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        let shodan_query = if ot_ports.is_empty() {
            format!("ip:{}", asset.ip_address)
        } else {
            format!("ip:{} {}", asset.ip_address, ot_ports.join(" "))
        };

        findings.push(Finding::new(
            FindingType::AttackTechnique,
            Severity::Critical,
            format!("Internet-exposed OT device: {}", asset.ip_address),
            format!(
                "Device {} has a public IP address and communicates using OT protocols ({}). \
                 Internet-facing OT devices are directly reachable by global threat actors \
                 and are frequently indexed by Shodan, Censys, and similar scanners. \
                 Verify firewall rules and consider segmenting this device behind a NAT/DMZ.",
                asset.ip_address,
                ot_protocols.join(", ")
            ),
            vec![asset.ip_address.clone()],
            format!(
                "Public IP {} speaks OT protocols: {}. Shodan query: {}",
                asset.ip_address,
                ot_protocols.join(", "),
                shodan_query
            ),
            Some(crate::attack_codes::T0846.to_string()),
        ));
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
    fn test_flat_network_detection() {
        let mut input = make_input();
        // 9 devices on same /24
        for i in 1..=9 {
            input.assets.push(AssetSnapshot {
                ip_address: format!("192.168.1.{}", i),
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
        // 1 device on different subnet
        input.assets.push(AssetSnapshot {
            ip_address: "10.0.0.1".to_string(),
            device_type: "it_device".to_string(),
            protocols: vec![],
            purdue_level: Some(4),
            is_public_ip: false,
            tags: vec![],
            vendor: None,
            hostname: None,
            product_family: None,
        });

        let findings = detect_flat_network(&input);
        assert!(!findings.is_empty(), "should detect flat network");
        assert_eq!(findings[0].technique_id, Some(crate::attack_codes::T0886.to_string()));
    }

    #[test]
    fn test_no_flat_network_with_few_devices() {
        let mut input = make_input();
        for i in 1..=4 {
            input.assets.push(AssetSnapshot {
                ip_address: format!("192.168.1.{}", i),
                device_type: "plc".to_string(),
                protocols: vec![],
                purdue_level: Some(1),
                is_public_ip: false,
                tags: vec![],
                vendor: None,
                hostname: None,
                product_family: None,
            });
        }
        let findings = detect_flat_network(&input);
        assert!(
            findings.is_empty(),
            "too few devices should not trigger flat network"
        );
    }
}

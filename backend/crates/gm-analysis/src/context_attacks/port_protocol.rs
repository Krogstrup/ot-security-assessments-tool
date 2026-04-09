//! Group 1 — Port / protocol context detections.
//!
//! Techniques: T0822, T0867, T0885, T0849

use std::collections::HashSet;

use gm_types::REMOTE_ACCESS_PORTS;

use crate::helpers::canonical_ot_ports;
use crate::{AnalysisInput, Finding, FindingType, Severity};

use super::CaptureContext;

pub(super) fn detect_t0822_external_remote_services(
    input: &AnalysisInput,
    ctx: &CaptureContext,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    let ot_ips = super::effective_ot_ips(input, ctx);
    let mut flagged: HashSet<(&str, &str, u16)> = HashSet::new();

    for conn in &input.connections {
        if !REMOTE_ACCESS_PORTS.contains(&conn.dst_port) {
            continue;
        }
        if !ot_ips.contains(conn.src_ip.as_str()) {
            continue;
        }
        let key = (conn.src_ip.as_str(), conn.dst_ip.as_str(), conn.dst_port);
        if flagged.insert(key) {
            let service = super::remote_service_name(conn.dst_port);
            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::High,
                format!(
                    "OT device {} accessing {} (port {})",
                    conn.src_ip, service, conn.dst_port
                ),
                "An OT device is initiating connections to remote access services. \
                 Legitimate OT controllers do not originate remote desktop or shell \
                 sessions — this may indicate a compromised device being used as a \
                 pivot point into external systems."
                    .to_string(),
                vec![conn.src_ip.clone(), conn.dst_ip.clone()],
                format!(
                    "OT device {} → {} on {} (port {}), {} packets",
                    conn.src_ip, conn.dst_ip, service, conn.dst_port, conn.packet_count
                ),
                Some(crate::attack_codes::T0822.to_string()),
            ));
        }
    }

    findings
}

pub(super) fn detect_t0867_lateral_tool_transfer(
    input: &AnalysisInput,
    ctx: &CaptureContext,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    let ot_ips = super::effective_ot_ips(input, ctx);
    let mut flagged: HashSet<(&str, &str)> = HashSet::new();

    for conn in &input.connections {
        if conn.dst_port != 21 && conn.dst_port != 69 {
            continue;
        }
        let src_is_ot = ot_ips.contains(conn.src_ip.as_str());
        let dst_is_ot = ot_ips.contains(conn.dst_ip.as_str());
        if !src_is_ot && !dst_is_ot {
            continue;
        }
        let key = (conn.src_ip.as_str(), conn.dst_ip.as_str());
        if flagged.insert(key) {
            let proto_name = if conn.dst_port == 21 { "FTP" } else { "TFTP" };
            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::High,
                format!(
                    "{} file transfer involving OT device: {} → {}",
                    proto_name, conn.src_ip, conn.dst_ip
                ),
                format!(
                    "{} (port {}) traffic involving an OT device was detected. File \
                     transfers within or to/from OT segments may represent firmware \
                     uploads, malicious tool staging, or configuration data exfiltration.",
                    proto_name, conn.dst_port
                ),
                vec![conn.src_ip.clone(), conn.dst_ip.clone()],
                format!(
                    "{} from {} to {} (port {}), {} packets",
                    proto_name, conn.src_ip, conn.dst_ip, conn.dst_port, conn.packet_count
                ),
                Some(crate::attack_codes::T0867.to_string()),
            ));
        }
    }

    findings
}

pub(super) fn detect_t0885_commonly_used_port(input: &AnalysisInput) -> Vec<Finding> {
    let mut findings = Vec::new();
    let mut flagged: HashSet<(&str, &str, u16)> = HashSet::new();

    for conn in &input.connections {
        let canonical = canonical_ot_ports(&conn.protocol);
        if canonical.is_empty() {
            continue; // unknown or non-OT protocol
        }
        // Traffic on both src and dst are not the canonical port
        if canonical.contains(&conn.dst_port) || canonical.contains(&conn.src_port) {
            continue;
        }
        let key = (conn.src_ip.as_str(), conn.protocol.as_str(), conn.dst_port);
        if flagged.insert(key) {
            let canonical_str: Vec<String> = canonical.iter().map(|p| p.to_string()).collect();
            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::Medium,
                format!(
                    "{} on non-standard port {} ({} → {})",
                    conn.protocol, conn.dst_port, conn.src_ip, conn.dst_ip
                ),
                format!(
                    "{} was identified on port {}, which is not its standard port. \
                     Non-standard port usage may indicate deliberate port remapping to \
                     evade protocol-specific firewall rules.",
                    conn.protocol, conn.dst_port
                ),
                vec![conn.src_ip.clone(), conn.dst_ip.clone()],
                format!(
                    "{} on port {} (standard: {}), {} packets",
                    conn.protocol,
                    conn.dst_port,
                    canonical_str.join("/"),
                    conn.packet_count
                ),
                Some(crate::attack_codes::T0885.to_string()),
            ));
        }
    }

    findings
}

pub(super) fn detect_t0849_masquerading(input: &AnalysisInput) -> Vec<Finding> {
    let mut findings = Vec::new();

    // Map canonical OT port → expected protocol name.
    let ot_port_expected: &[(u16, &str)] = &[
        (502, "Modbus"),
        (20000, "Dnp3"),
        (44818, "EthernetIp"),
        (102, "S7comm"),
        (47808, "Bacnet"),
        (4840, "OpcUa"),
        (2404, "Iec104"),
        (5094, "HartIp"),
        (18245, "GeSrtp"),
        (18246, "GeSrtp"),
        (5007, "WonderwareSuitelink"),
    ];

    // Protocols that are clearly NOT OT (and would constitute masquerading).
    let non_ot: &[&str] = &["Http", "Https", "Ssh", "Ftp", "Tftp", "Smtp", "Telnet"];

    let mut flagged: HashSet<(&str, &str, u16)> = HashSet::new();

    for conn in &input.connections {
        let proto = conn.protocol.as_str();
        if !non_ot.contains(&proto) {
            continue;
        }
        // Check if dst_port is a canonical OT port
        if let Some(&(_, expected)) = ot_port_expected.iter().find(|(p, _)| *p == conn.dst_port) {
            let key = (conn.src_ip.as_str(), proto, conn.dst_port);
            if flagged.insert(key) {
                findings.push(Finding::new(
                    FindingType::AttackTechnique,
                    Severity::Medium,
                    format!(
                        "{} traffic masquerading on OT port {} ({} → {})",
                        proto, conn.dst_port, conn.src_ip, conn.dst_ip
                    ),
                    format!(
                        "Non-OT protocol '{}' was observed on port {}, normally reserved \
                         for {}. This may indicate C2 traffic disguised as OT protocol \
                         traffic to bypass port-based firewall rules.",
                        proto, conn.dst_port, expected
                    ),
                    vec![conn.src_ip.clone(), conn.dst_ip.clone()],
                    format!(
                        "{}→{}:{} uses '{}', expected '{}'",
                        conn.src_ip, conn.dst_ip, conn.dst_port, proto, expected
                    ),
                    Some(crate::attack_codes::T0849.to_string()),
                ));
            }
        }
    }

    findings
}

// ── tests ──────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AnalysisInput, AssetSnapshot, ConnectionSnapshot};

    fn asset(ip: &str, device_type: &str, protocols: &[&str]) -> AssetSnapshot {
        AssetSnapshot {
            ip_address: ip.to_string(),
            device_type: device_type.to_string(),
            protocols: protocols.iter().map(|s| s.to_string()).collect(),
            purdue_level: None,
            is_public_ip: false,
            tags: vec![],
            vendor: None,
            hostname: None,
            product_family: None,
        }
    }

    fn conn(
        src: &str,
        dst: &str,
        dst_port: u16,
        protocol: &str,
        packets: u64,
    ) -> ConnectionSnapshot {
        ConnectionSnapshot {
            src_ip: src.to_string(),
            dst_ip: dst.to_string(),
            src_port: 49152,
            dst_port,
            protocol: protocol.to_string(),
            packet_count: packets,
        }
    }

    // ── T0822 ──
    #[test]
    fn test_t0822_ot_device_initiates_rdp() {
        let mut input = AnalysisInput::default();
        input.assets = vec![asset("10.0.0.1", "plc", &["Modbus"])];
        input.connections = vec![conn("10.0.0.1", "1.2.3.4", 3389, "Unknown", 5)];
        let ctx = CaptureContext::default();
        let findings = detect_t0822_external_remote_services(&input, &ctx);
        assert!(!findings.is_empty(), "RDP from OT PLC should be flagged");
        assert_eq!(findings[0].technique_id, Some(crate::attack_codes::T0822.to_string()));
    }

    #[test]
    fn test_t0822_it_device_rdp_not_flagged() {
        let mut input = AnalysisInput::default();
        input.assets = vec![asset("10.0.0.200", "it_device", &[])];
        input.connections = vec![conn("10.0.0.200", "1.2.3.4", 3389, "Unknown", 5)];
        let ctx = CaptureContext::default();
        let findings = detect_t0822_external_remote_services(&input, &ctx);
        assert!(
            findings.is_empty(),
            "RDP from IT device should not be flagged by T0822"
        );
    }

    // ── T0867 ──
    #[test]
    fn test_t0867_ftp_between_ot_devices() {
        let mut input = AnalysisInput::default();
        input.assets = vec![
            asset("10.0.0.1", "plc", &["Modbus"]),
            asset("10.0.0.2", "engineering_workstation", &["Modbus"]),
        ];
        input.connections = vec![conn("10.0.0.2", "10.0.0.1", 21, "Ftp", 20)];
        let ctx = CaptureContext::default();
        let findings = detect_t0867_lateral_tool_transfer(&input, &ctx);
        assert!(
            !findings.is_empty(),
            "FTP involving OT device should be flagged"
        );
        assert_eq!(findings[0].technique_id, Some(crate::attack_codes::T0867.to_string()));
    }

    #[test]
    fn test_t0867_tftp_to_plc_flagged() {
        let mut input = AnalysisInput::default();
        input.assets = vec![asset("10.0.0.5", "plc", &["Modbus"])];
        input.connections = vec![conn("192.168.0.1", "10.0.0.5", 69, "Tftp", 3)];
        let ctx = CaptureContext::default();
        let findings = detect_t0867_lateral_tool_transfer(&input, &ctx);
        assert!(!findings.is_empty());
        assert_eq!(findings[0].technique_id, Some(crate::attack_codes::T0867.to_string()));
    }

    // ── T0885 ──
    #[test]
    fn test_t0885_modbus_on_wrong_port() {
        let mut input = AnalysisInput::default();
        // Modbus identified on port 503 (not 502)
        input.connections = vec![conn("10.0.0.1", "10.0.0.2", 503, "Modbus", 100)];
        let findings = detect_t0885_commonly_used_port(&input);
        assert!(
            !findings.is_empty(),
            "Modbus on port 503 should trigger T0885"
        );
        assert_eq!(findings[0].technique_id, Some(crate::attack_codes::T0885.to_string()));
    }

    #[test]
    fn test_t0885_modbus_standard_port_ok() {
        let mut input = AnalysisInput::default();
        input.connections = vec![conn("10.0.0.1", "10.0.0.2", 502, "Modbus", 100)];
        let findings = detect_t0885_commonly_used_port(&input);
        assert!(findings.is_empty(), "Modbus on port 502 is normal");
    }

    // ── T0849 ──
    #[test]
    fn test_t0849_http_on_modbus_port() {
        let mut input = AnalysisInput::default();
        // HTTP traffic on port 502
        input.connections = vec![conn("10.0.0.100", "10.0.0.1", 502, "Http", 10)];
        let findings = detect_t0849_masquerading(&input);
        assert!(
            !findings.is_empty(),
            "HTTP on port 502 should trigger T0849"
        );
        assert_eq!(findings[0].technique_id, Some(crate::attack_codes::T0849.to_string()));
    }
}

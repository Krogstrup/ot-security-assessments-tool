//! Group 4 — Advanced / state-tracking context detections.
//!
//! Techniques: T0830, T0884, T0866, T0800, T0801

use std::collections::{HashMap, HashSet};

use gm_constants::OT_SERVER_PORTS as OT_PORTS;

use crate::{AnalysisInput, Finding, FindingType, Severity};

use super::CaptureContext;

pub(super) fn detect_t0830_adversary_in_the_middle(ctx: &CaptureContext) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (ip, macs) in &ctx.ip_to_macs {
        let unique: HashSet<String> = macs.iter().map(|m| m.to_lowercase()).collect();
        if unique.len() < 2 {
            continue;
        }
        let mac_list: Vec<String> = unique.into_iter().collect();
        findings.push(Finding::new(
            FindingType::AttackTechnique,
            Severity::Critical,
            format!(
                "IP {} seen with {} distinct MACs (AiTM indicator)",
                ip,
                mac_list.len()
            ),
            "A single IP address has been observed with multiple different MAC addresses. \
             This may indicate ARP cache poisoning, MAC spoofing, or an adversary \
             positioning themselves between legitimate OT communication partners."
                .to_string(),
            vec![ip.clone()],
            format!("IP {} associated with MACs: {}", ip, mac_list.join(", ")),
            Some(crate::attack_codes::T0830.to_string()),
        ));
    }

    findings
}

pub(super) fn detect_t0884_connection_proxy(
    input: &AnalysisInput,
    ctx: &CaptureContext,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    let ot_ips = super::effective_ot_ips(input, ctx);

    // For each IP: OT dst ports it connects to (client role).
    let mut client_ports: HashMap<&str, HashSet<u16>> = HashMap::new();
    // For each IP: OT dst ports it receives connections on (server role).
    let mut server_ports: HashMap<&str, HashSet<u16>> = HashMap::new();

    for conn in &input.connections {
        if OT_PORTS.contains(&conn.dst_port) {
            client_ports
                .entry(conn.src_ip.as_str())
                .or_default()
                .insert(conn.dst_port);
            server_ports
                .entry(conn.dst_ip.as_str())
                .or_default()
                .insert(conn.dst_port);
        }
    }

    let mut flagged: HashSet<&str> = HashSet::new();
    for (ip, srv) in &server_ports {
        if ot_ips.contains(ip) || flagged.contains(ip) {
            continue; // legitimate OT server role
        }
        if let Some(cli) = client_ports.get(ip) {
            let shared: Vec<u16> = srv.intersection(cli).copied().collect();
            if !shared.is_empty() {
                flagged.insert(ip);
                let port_list: Vec<String> = shared.iter().map(|p| p.to_string()).collect();
                findings.push(Finding::new(
                    FindingType::AttackTechnique,
                    Severity::High,
                    format!("Possible OT traffic proxy at {}", ip),
                    "A non-OT device is acting as both client and server on the same OT \
                     protocol ports. This topology is characteristic of a connection proxy \
                     or man-in-the-middle device injected into an OT communication path."
                        .to_string(),
                    vec![ip.to_string()],
                    format!(
                        "{} both receives and originates OT connections on port(s): {}",
                        ip,
                        port_list.join(", ")
                    ),
                    Some(crate::attack_codes::T0884.to_string()),
                ));
            }
        }
    }

    findings
}

pub(super) fn detect_t0866_exploitation_remote_services(
    input: &AnalysisInput,
    ctx: &CaptureContext,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    const MGMT_PORTS: &[u16] = &[22, 23, 80, 443, 3389];

    let ot_ips = super::effective_ot_ips(input, ctx);
    let mut flagged: HashSet<(&str, &str, u16)> = HashSet::new();

    for conn in &input.connections {
        if !MGMT_PORTS.contains(&conn.dst_port) {
            continue;
        }
        if !ot_ips.contains(conn.dst_ip.as_str()) {
            continue;
        }
        let src_is_external = ctx.external_ips.contains(&conn.src_ip);
        let src_is_ot = ot_ips.contains(conn.src_ip.as_str());
        // Skip OT-to-OT management (legitimate engineering workstation access).
        if src_is_ot && !src_is_external {
            continue;
        }
        let key = (conn.src_ip.as_str(), conn.dst_ip.as_str(), conn.dst_port);
        if flagged.insert(key) {
            let service = super::remote_service_name(conn.dst_port);
            let src_label = if src_is_external {
                "External"
            } else {
                "Non-OT"
            };
            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::High,
                format!(
                    "{} host {} connecting to OT device {} via {} (port {})",
                    src_label, conn.src_ip, conn.dst_ip, service, conn.dst_port
                ),
                "A non-OT or external host is connecting to an OT field device on a \
                 remote management port. This may represent exploitation of an exposed \
                 service or unauthorised remote access to a controller."
                    .to_string(),
                vec![conn.src_ip.clone(), conn.dst_ip.clone()],
                format!(
                    "{} {} → OT device {} on {} (port {}), {} packets",
                    src_label, conn.src_ip, conn.dst_ip, service, conn.dst_port, conn.packet_count
                ),
                Some(crate::attack_codes::T0866.to_string()),
            ));
        }
    }

    findings
}

pub(super) fn detect_t0800_firmware_update_mode(input: &AnalysisInput) -> Vec<Finding> {
    let mut findings = Vec::new();

    let plc_rtu_ips: HashSet<&str> = input
        .assets
        .iter()
        .filter(|a| matches!(a.device_type.as_str(), "plc" | "rtu" | "field_device"))
        .map(|a| a.ip_address.as_str())
        .collect();

    if plc_rtu_ips.is_empty() {
        return findings;
    }

    for (ip, dp) in &input.deep_parse {
        // EtherNet/IP: CIP File class access from a scanner targeting known PLCs.
        if let Some(enip) = &dp.enip {
            if enip.cip_file_access && enip.role == "scanner" {
                let targets: Vec<String> = input
                    .connections
                    .iter()
                    .filter(|c| c.src_ip == *ip && plc_rtu_ips.contains(c.dst_ip.as_str()))
                    .map(|c| c.dst_ip.clone())
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .collect();
                if !targets.is_empty() {
                    findings.push(Finding::new(
                        FindingType::AttackTechnique,
                        Severity::Critical,
                        format!("CIP File Access from {} targeting PLC(s)", ip),
                        "CIP File class access (EtherNet/IP) was observed targeting PLC or \
                         RTU devices. CIP File operations can read/write firmware image files \
                         and activate firmware update mode, enabling malicious firmware upload."
                            .to_string(),
                        std::iter::once(ip.clone())
                            .chain(targets.iter().cloned())
                            .collect(),
                        format!(
                            "{} used CIP File Access targeting: {}",
                            ip,
                            targets.join(", ")
                        ),
                        Some(crate::attack_codes::T0800.to_string()),
                    ));
                }
            }
        }

        // S7comm: Upload/Download from a client targeting known PLCs.
        if let Some(s7) = &dp.s7 {
            let has_up_down = s7.functions_seen.iter().any(|f| {
                matches!(
                    f.as_str(),
                    "upload" | "download" | "start_upload" | "end_upload"
                )
            });
            if has_up_down && s7.role == "client" {
                let targets: Vec<String> = input
                    .connections
                    .iter()
                    .filter(|c| c.src_ip == *ip && plc_rtu_ips.contains(c.dst_ip.as_str()))
                    .map(|c| c.dst_ip.clone())
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .collect();
                if !targets.is_empty() {
                    findings.push(Finding::new(
                        FindingType::AttackTechnique,
                        Severity::Critical,
                        format!("S7 program upload/download from {} targeting PLC(s)", ip),
                        "S7comm Upload or Download functions were directed at known PLC devices. \
                         These operations can read or activate firmware update mode on Siemens \
                         controllers, enabling malicious control logic upload."
                            .to_string(),
                        std::iter::once(ip.clone())
                            .chain(targets.iter().cloned())
                            .collect(),
                        format!(
                            "{} used S7 upload/download targeting PLC(s): {}",
                            ip,
                            targets.join(", ")
                        ),
                        Some(crate::attack_codes::T0800.to_string()),
                    ));
                }
            }
        }
    }

    findings
}

pub(super) fn detect_t0801_monitor_process_state(
    input: &AnalysisInput,
    ctx: &CaptureContext,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    const THRESHOLD: usize = 20;

    // Count unique OT (host, port) pairs per source.
    let mut src_to_endpoints: HashMap<&str, HashSet<(&str, u16)>> = HashMap::new();
    for conn in &input.connections {
        if OT_PORTS.contains(&conn.dst_port) {
            src_to_endpoints
                .entry(conn.src_ip.as_str())
                .or_default()
                .insert((conn.dst_ip.as_str(), conn.dst_port));
        }
    }

    for (src, endpoints) in src_to_endpoints {
        if endpoints.len() < THRESHOLD {
            continue;
        }
        // Skip write-dominant sources — T0806/T0855 is more appropriate for those.
        let write_dominant = ctx
            .per_source_write_targets
            .get(src)
            .map(|wt| wt.len() >= endpoints.len() / 2)
            .unwrap_or(false);
        if write_dominant {
            continue;
        }
        let sample: Vec<String> = endpoints
            .iter()
            .take(5)
            .map(|(h, p)| format!("{}:{}", h, p))
            .collect();
        findings.push(Finding::new(
            FindingType::AttackTechnique,
            Severity::Medium,
            format!(
                "Wide process state monitoring by {} ({} OT endpoints)",
                src,
                endpoints.len()
            ),
            "A single device is issuing read/poll requests to many OT service endpoints. \
             Comprehensive process state monitoring across controllers may indicate \
             adversarial surveillance of industrial processes in preparation for \
             targeted disruption."
                .to_string(),
            vec![src.to_string()],
            format!(
                "{} polled {} unique OT endpoints (sample: {})",
                src,
                endpoints.len(),
                sample.join(", ")
            ),
            Some(crate::attack_codes::T0801.to_string()),
        ));
    }

    findings
}

// ── tests ──────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AnalysisInput, AssetSnapshot, ConnectionSnapshot, DeepParseSnapshot, EnipSnapshot,
    };

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

    // ── T0830 ──
    #[test]
    fn test_t0830_multiple_macs_for_ip() {
        let ctx = CaptureContext {
            ip_to_macs: {
                let mut m = HashMap::new();
                m.insert(
                    "10.0.0.1".to_string(),
                    vec![
                        "AA:BB:CC:DD:EE:FF".to_string(),
                        "11:22:33:44:55:66".to_string(),
                    ],
                );
                m
            },
            ..Default::default()
        };
        let findings = detect_t0830_adversary_in_the_middle(&ctx);
        assert!(
            !findings.is_empty(),
            "Two MACs for one IP should trigger T0830"
        );
        assert_eq!(findings[0].technique_id, Some(crate::attack_codes::T0830.to_string()));
        assert_eq!(findings[0].severity, Severity::Critical);
    }

    // ── T0884 ──
    #[test]
    fn test_t0884_proxy_device() {
        let mut input = AnalysisInput::default();
        // 10.0.0.50 is NOT an OT device. It receives from 10.0.0.100 AND connects to 10.0.0.1,
        // both on port 502 → proxy.
        input.assets = vec![
            asset("10.0.0.1", "plc", &["Modbus"]),
            asset("10.0.0.100", "hmi", &["Modbus"]),
        ];
        input.connections = vec![
            conn("10.0.0.100", "10.0.0.50", 502, "Modbus", 100), // HMI → proxy
            conn("10.0.0.50", "10.0.0.1", 502, "Modbus", 100),   // proxy → PLC
        ];
        let ctx = CaptureContext::default();
        let findings = detect_t0884_connection_proxy(&input, &ctx);
        assert!(!findings.is_empty(), "Relay device should trigger T0884");
        assert_eq!(findings[0].technique_id, Some(crate::attack_codes::T0884.to_string()));
    }

    // ── T0866 ──
    #[test]
    fn test_t0866_external_host_sshing_to_plc() {
        let mut input = AnalysisInput::default();
        input.assets = vec![asset("10.0.0.1", "plc", &["Modbus"])];
        input.connections = vec![conn("203.0.113.5", "10.0.0.1", 22, "Ssh", 8)];
        let mut ctx = CaptureContext::default();
        ctx.ot_device_ips.insert("10.0.0.1".to_string());
        ctx.external_ips.insert("203.0.113.5".to_string());
        let findings = detect_t0866_exploitation_remote_services(&input, &ctx);
        assert!(
            !findings.is_empty(),
            "External SSH to OT PLC should trigger T0866"
        );
        assert_eq!(findings[0].technique_id, Some(crate::attack_codes::T0866.to_string()));
    }

    // ── T0800 ──
    #[test]
    fn test_t0800_cip_file_access_to_plc() {
        let mut input = AnalysisInput::default();
        input.assets = vec![asset("10.0.0.1", "plc", &["EthernetIp"])];
        input.connections = vec![conn("10.0.0.200", "10.0.0.1", 44818, "EthernetIp", 50)];
        input.deep_parse.insert(
            "10.0.0.200".to_string(),
            DeepParseSnapshot {
                enip: Some(EnipSnapshot {
                    role: "scanner".to_string(),
                    cip_writes_to_assembly: false,
                    cip_file_access: true,
                    list_identity_requests: false,
                }),
                ..Default::default()
            },
        );
        let findings = detect_t0800_firmware_update_mode(&input);
        assert!(
            !findings.is_empty(),
            "CIP File access to PLC should trigger T0800"
        );
        assert_eq!(findings[0].technique_id, Some(crate::attack_codes::T0800.to_string()));
    }

    // ── T0801 ──
    #[test]
    fn test_t0801_wide_process_monitoring() {
        let mut input = AnalysisInput::default();
        // One source reading from 25 unique (host, port) OT endpoints.
        for i in 1..=25_u32 {
            input.connections.push(conn(
                "10.0.0.200",
                &format!("10.0.0.{}", i),
                502,
                "Modbus",
                100,
            ));
        }
        let ctx = CaptureContext::default();
        let findings = detect_t0801_monitor_process_state(&input, &ctx);
        assert!(
            !findings.is_empty(),
            "25 OT endpoints polled should trigger T0801"
        );
        assert_eq!(findings[0].technique_id, Some(crate::attack_codes::T0801.to_string()));
    }
}

//! Group 2 — Rate / pattern-based context detections.
//!
//! Techniques: T0868, T0806, T0802, T0861, T0840

use std::collections::{HashMap, HashSet};

use gm_constants::OT_SERVER_PORTS as OT_PORTS;

use crate::thresholds;
use crate::{AnalysisInput, Finding, FindingType, Severity};

use super::CaptureContext;

pub(super) fn detect_t0868_detect_operating_mode(input: &AnalysisInput) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (ip, dp) in &input.deep_parse {
        let s7 = match &dp.s7 {
            Some(s) => s,
            None => continue,
        };

        let upload_download: Vec<&str> = s7
            .functions_seen
            .iter()
            .filter(|f| {
                matches!(
                    f.as_str(),
                    "upload" | "download" | "start_upload" | "end_upload"
                )
            })
            .map(String::as_str)
            .collect();

        if upload_download.is_empty() {
            continue;
        }

        findings.push(Finding::new(
            FindingType::AttackTechnique,
            Severity::High,
            format!("S7 program upload/download from {}", ip),
            "S7comm Upload or Download functions were observed. These operations \
             read or write PLC program blocks, enabling an adversary to map the \
             control logic and identify targets for process manipulation."
                .to_string(),
            vec![ip.clone()],
            format!(
                "Device {} used S7 functions: {}",
                ip,
                upload_download.join(", ")
            ),
            Some("T0868".to_string()),
        ));
    }

    findings
}

pub(super) fn detect_t0806_brute_force_io(
    input: &AnalysisInput,
    ctx: &CaptureContext,
) -> Vec<Finding> {
    let mut findings = Vec::new();

    // Context path: use pre-computed write counts per (src, dst).
    let mut ctx_reported: HashSet<(&str, &str)> = HashSet::new();
    for ((src, dst), &count) in &ctx.per_connection_write_rate {
        if count >= thresholds::WRITE_RATE_THRESHOLD {
            ctx_reported.insert((src.as_str(), dst.as_str()));
            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::High,
                format!("High-rate I/O writes from {} to {}", src, dst),
                "A single source is sending a very high number of write commands to an \
                 OT device. Rapid forced writes can overwhelm the controller scan cycle \
                 and cause process disruption or equipment damage."
                    .to_string(),
                vec![src.clone(), dst.clone()],
                format!("{} sent {} write-class commands to {}", src, count, dst),
                Some("T0806".to_string()),
            ));
        }
    }

    // Fallback: Modbus write FC totals targeting a single slave.
    for (ip, dp) in &input.deep_parse {
        let modbus = match &dp.modbus {
            Some(m) => m,
            None => continue,
        };
        if modbus.role != "master" && modbus.role != "both" {
            continue;
        }
        let write_total: u64 = modbus
            .function_codes
            .iter()
            .filter(|fc| matches!(fc.code, 5 | 6 | 15 | 16))
            .map(|fc| fc.count)
            .sum();
        if write_total < thresholds::WRITE_RATE_THRESHOLD {
            continue;
        }
        let slaves: Vec<&str> = modbus
            .relationships
            .iter()
            .filter(|r| r.remote_role == "slave")
            .map(|r| r.remote_ip.as_str())
            .collect();
        if slaves.len() != 1 {
            continue; // only flag single-target saturation here
        }
        let slave = slaves[0];
        if ctx_reported.contains(&(ip.as_str(), slave)) {
            continue; // already reported via context
        }
        findings.push(Finding::new(
            FindingType::AttackTechnique,
            Severity::High,
            format!("Modbus brute force I/O from {} to {}", ip, slave),
            "Extremely high Modbus write rate detected targeting a single slave. \
             This may represent forced setpoint manipulation or coil-flooding."
                .to_string(),
            vec![ip.clone(), slave.to_string()],
            format!(
                "{} sent {} Modbus write commands (FC 5/6/15/16) to {}",
                ip, write_total, slave
            ),
            Some("T0806".to_string()),
        ));
    }

    findings
}

pub(super) fn detect_t0802_automated_collection(
    input: &AnalysisInput,
    ctx: &CaptureContext,
) -> Vec<Finding> {
    let mut findings = Vec::new();

    // Context path.
    for (src, targets) in &ctx.per_source_read_targets {
        if targets.len() >= thresholds::AUTOMATED_COLLECTION_TARGET_THRESHOLD {
            let sample: Vec<String> = targets.iter().take(5).cloned().collect();
            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::Medium,
                format!(
                    "Automated collection: {} polling {} OT devices",
                    src,
                    targets.len()
                ),
                "A single source is reading data from many OT devices. This pattern \
                 resembles automated collection — systematically harvesting process \
                 state from controllers across the network."
                    .to_string(),
                std::iter::once(src.clone())
                    .chain(targets.iter().cloned())
                    .collect(),
                format!(
                    "{} sent read requests to {} OT targets (sample: {})",
                    src,
                    targets.len(),
                    sample.join(", ")
                ),
                Some("T0802".to_string()),
            ));
        }
    }

    // Fallback: count unique OT hosts reached per source from connection list.
    if ctx.per_source_read_targets.is_empty() {
        let ot_ips = super::effective_ot_ips(input, ctx);
        let mut src_to_ot: HashMap<&str, HashSet<&str>> = HashMap::new();
        for conn in &input.connections {
            if OT_PORTS.contains(&conn.dst_port) && ot_ips.contains(conn.dst_ip.as_str()) {
                src_to_ot
                    .entry(conn.src_ip.as_str())
                    .or_default()
                    .insert(conn.dst_ip.as_str());
            }
        }
        for (src, targets) in src_to_ot {
            if targets.len() >= thresholds::AUTOMATED_COLLECTION_TARGET_THRESHOLD {
                let target_list: Vec<String> = targets.iter().map(|s| s.to_string()).collect();
                findings.push(Finding::new(
                    FindingType::AttackTechnique,
                    Severity::Medium,
                    format!(
                        "Automated collection: {} connecting to {} OT devices",
                        src,
                        targets.len()
                    ),
                    "A single source is connecting to many OT devices on ICS protocol \
                     ports. This pattern resembles automated collection of process state."
                        .to_string(),
                    std::iter::once(src.to_string())
                        .chain(target_list.iter().cloned())
                        .collect(),
                    format!(
                        "{} connected to {} OT targets on ICS ports",
                        src,
                        targets.len()
                    ),
                    Some("T0802".to_string()),
                ));
            }
        }
    }

    findings
}

pub(super) fn detect_t0861_point_tag_identification(input: &AnalysisInput) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (ip, dp) in &input.deep_parse {
        let modbus = match &dp.modbus {
            Some(m) => m,
            None => continue,
        };
        if modbus.role != "master" && modbus.role != "both" {
            continue;
        }
        let has_reads = modbus
            .function_codes
            .iter()
            .any(|fc| matches!(fc.code, 1..=4) && fc.count > 0);
        if !has_reads || modbus.unit_ids.len() < thresholds::UNIT_ID_SCAN_THRESHOLD {
            continue;
        }
        let uid_str: Vec<String> = modbus.unit_ids.iter().map(|u| u.to_string()).collect();
        findings.push(Finding::new(
            FindingType::AttackTechnique,
            Severity::Medium,
            format!(
                "Modbus point/tag scan from {} ({} unit IDs)",
                ip,
                modbus.unit_ids.len()
            ),
            "A Modbus master is reading from many distinct unit IDs. Enumerating unit \
             IDs discovers all slave devices on the RS-485 bus and identifies their \
             data point layout — a precursor to targeted process manipulation."
                .to_string(),
            vec![ip.clone()],
            format!(
                "{} read from {} unit IDs: {}",
                ip,
                modbus.unit_ids.len(),
                uid_str.join(", ")
            ),
            Some("T0861".to_string()),
        ));
    }

    findings
}

pub(super) fn detect_t0840_network_connection_enumeration(
    input: &AnalysisInput,
    ctx: &CaptureContext,
) -> Vec<Finding> {
    let mut findings = Vec::new();

    // Context path: count unique OT dst ports per source.
    for (src, ports) in &ctx.per_source_dst_ports {
        let ot_port_count = ports.iter().filter(|&&p| OT_PORTS.contains(&p)).count();
        if ot_port_count >= thresholds::PORT_SWEEP_THRESHOLD {
            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::High,
                format!("OT port sweep by {} ({} OT ports)", src, ot_port_count),
                "A single source is connecting to many different OT service ports. \
                 This is characteristic of automated network enumeration mapping the \
                 ICS network topology."
                    .to_string(),
                vec![src.clone()],
                format!(
                    "{} contacted {} distinct OT service ports",
                    src, ot_port_count
                ),
                Some("T0840".to_string()),
            ));
        }
    }

    // Fallback: count unique OT hosts per source from connection list.
    if ctx.per_source_dst_ports.is_empty() {
        let mut src_to_hosts: HashMap<&str, HashSet<&str>> = HashMap::new();
        for conn in &input.connections {
            if OT_PORTS.contains(&conn.dst_port) {
                src_to_hosts
                    .entry(conn.src_ip.as_str())
                    .or_default()
                    .insert(conn.dst_ip.as_str());
            }
        }
        for (src, hosts) in src_to_hosts {
            if hosts.len() >= thresholds::PORT_SWEEP_THRESHOLD {
                findings.push(Finding::new(
                    FindingType::AttackTechnique,
                    Severity::High,
                    format!("OT host sweep by {} ({} hosts)", src, hosts.len()),
                    "A single source is connecting to many OT devices on ICS ports. \
                     This is characteristic of automated host enumeration."
                        .to_string(),
                    std::iter::once(src.to_string())
                        .chain(hosts.iter().map(|s| s.to_string()))
                        .collect(),
                    format!(
                        "{} connected to {} distinct OT hosts on ICS ports",
                        src,
                        hosts.len()
                    ),
                    Some("T0840".to_string()),
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
    use crate::{
        AnalysisInput, AssetSnapshot, ConnectionSnapshot, DeepParseSnapshot, FcSnapshot,
        ModbusSnapshot, RelationshipSnapshot, S7Snapshot,
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

    // ── T0868 ──
    #[test]
    fn test_t0868_s7_upload_detected() {
        let mut input = AnalysisInput::default();
        input.deep_parse.insert(
            "10.0.0.50".to_string(),
            DeepParseSnapshot {
                s7: Some(S7Snapshot {
                    role: "client".to_string(),
                    functions_seen: vec!["upload".to_string(), "read_var".to_string()],
                }),
                ..Default::default()
            },
        );
        let findings = detect_t0868_detect_operating_mode(&input);
        assert!(!findings.is_empty(), "S7 upload should trigger T0868");
        assert_eq!(findings[0].technique_id, Some("T0868".to_string()));
    }

    // ── T0806 ──
    #[test]
    fn test_t0806_high_write_rate_from_context() {
        let mut input = AnalysisInput::default();
        let mut ctx = CaptureContext::default();
        ctx.per_connection_write_rate
            .insert(("10.0.0.10".to_string(), "10.0.0.1".to_string()), 600);
        let findings = detect_t0806_brute_force_io(&input, &ctx);
        assert!(!findings.is_empty(), "600 writes should trigger T0806");
        assert_eq!(findings[0].technique_id, Some("T0806".to_string()));
        let _ = &mut input; // suppress unused warning
    }

    #[test]
    fn test_t0806_modbus_fallback() {
        let mut input = AnalysisInput::default();
        input.assets = vec![asset("10.0.0.5", "plc", &["Modbus"])];
        input.deep_parse.insert(
            "10.0.0.10".to_string(),
            DeepParseSnapshot {
                modbus: Some(ModbusSnapshot {
                    role: "master".to_string(),
                    unit_ids: vec![1],
                    function_codes: vec![FcSnapshot {
                        code: 6,
                        count: 550,
                        is_write: true,
                    }],
                    relationships: vec![RelationshipSnapshot {
                        remote_ip: "10.0.0.5".to_string(),
                        remote_role: "slave".to_string(),
                        packet_count: 550,
                    }],
                    polling_intervals: vec![],
                }),
                ..Default::default()
            },
        );
        let ctx = CaptureContext::default();
        let findings = detect_t0806_brute_force_io(&input, &ctx);
        assert!(
            !findings.is_empty(),
            "550 Modbus writes to single slave should trigger T0806"
        );
    }

    // ── T0802 ──
    #[test]
    fn test_t0802_many_ot_targets_from_context() {
        let mut input = AnalysisInput::default();
        let mut ctx = CaptureContext::default();
        let targets: std::collections::HashSet<String> =
            (1..=12).map(|i| format!("10.0.0.{}", i)).collect();
        ctx.per_source_read_targets
            .insert("192.168.1.99".to_string(), targets);
        let findings = detect_t0802_automated_collection(&input, &ctx);
        assert!(
            !findings.is_empty(),
            "Polling 12 OT targets should trigger T0802"
        );
        assert_eq!(findings[0].technique_id, Some("T0802".to_string()));
        let _ = &mut input;
    }

    // ── T0861 ──
    #[test]
    fn test_t0861_many_unit_ids() {
        let mut input = AnalysisInput::default();
        input.deep_parse.insert(
            "10.0.0.20".to_string(),
            DeepParseSnapshot {
                modbus: Some(ModbusSnapshot {
                    role: "master".to_string(),
                    unit_ids: (1..=8).collect(),
                    function_codes: vec![FcSnapshot {
                        code: 3,
                        count: 80,
                        is_write: false,
                    }],
                    relationships: vec![],
                    polling_intervals: vec![],
                }),
                ..Default::default()
            },
        );
        let findings = detect_t0861_point_tag_identification(&input);
        assert!(!findings.is_empty(), "8 unit IDs should trigger T0861");
        assert_eq!(findings[0].technique_id, Some("T0861".to_string()));
    }

    // ── T0840 ──
    #[test]
    fn test_t0840_ot_host_sweep() {
        let mut input = AnalysisInput::default();
        for i in 1..=12_u32 {
            input.connections.push(conn(
                "10.0.0.200",
                &format!("10.0.0.{}", i),
                502,
                "Modbus",
                1,
            ));
        }
        let ctx = CaptureContext::default();
        let findings = detect_t0840_network_connection_enumeration(&input, &ctx);
        assert!(
            !findings.is_empty(),
            "Connecting to 12 OT hosts should trigger T0840"
        );
        assert_eq!(findings[0].technique_id, Some("T0840".to_string()));
    }
}

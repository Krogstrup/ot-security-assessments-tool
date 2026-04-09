//! Group 3 — Network-defence context detections.
//!
//! Techniques: T0803, T0804, T0881, T0864

use std::collections::{HashMap, HashSet};

use gm_types::OT_SERVER_PORTS as OT_PORTS;

use crate::helpers::{is_ot_protocol_name, is_plc_or_rtu_family_device_type};
use crate::thresholds;
use crate::{AnalysisInput, Finding, FindingType, Severity};

use super::CaptureContext;

pub(super) fn detect_t0803_block_command_reporting(input: &AnalysisInput) -> Vec<Finding> {
    let mut findings = Vec::new();

    let field_device_ips: HashSet<&str> = input
        .assets
        .iter()
        .filter(|a| is_plc_or_rtu_family_device_type(&a.device_type))
        .map(|a| a.ip_address.as_str())
        .collect();

    if field_device_ips.is_empty() {
        return findings;
    }

    // Require that some OT controller traffic exists at all.
    let has_ot_controllers = input
        .connections
        .iter()
        .any(|c| OT_PORTS.contains(&c.dst_port));
    if !has_ot_controllers {
        return findings;
    }

    // Build set of field devices that received at least one incoming OT command.
    let mut receiving_commands: HashSet<&str> = HashSet::new();
    for conn in &input.connections {
        if OT_PORTS.contains(&conn.dst_port) && field_device_ips.contains(conn.dst_ip.as_str()) {
            receiving_commands.insert(conn.dst_ip.as_str());
        }
    }

    // Build set of field devices that have any network traffic at all.
    let mut has_any_traffic: HashSet<&str> = HashSet::new();
    for conn in &input.connections {
        if field_device_ips.contains(conn.src_ip.as_str()) {
            has_any_traffic.insert(conn.src_ip.as_str());
        }
        if field_device_ips.contains(conn.dst_ip.as_str()) {
            has_any_traffic.insert(conn.dst_ip.as_str());
        }
    }

    for ip in &field_device_ips {
        if receiving_commands.contains(ip) || !has_any_traffic.contains(ip) {
            continue;
        }
        findings.push(Finding::new(
            FindingType::AttackTechnique,
            Severity::Medium,
            format!("PLC/RTU {} receiving no OT commands", ip),
            "A PLC or RTU is present on the network but is not receiving any commands \
             on OT protocol ports from any controller. This may indicate that legitimate \
             command traffic is being blocked, filtered, or intercepted."
                .to_string(),
            vec![ip.to_string()],
            format!(
                "Field device {} has network traffic but received no OT-port commands",
                ip
            ),
            Some(crate::attack_codes::T0803.to_string()),
        ));
    }

    findings
}

pub(super) fn detect_t0804_block_reporting_message(input: &AnalysisInput) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (ip, dp) in &input.deep_parse {
        let dnp3 = match &dp.dnp3 {
            Some(d) => d,
            None => continue,
        };
        if dnp3.role != "outstation" {
            continue;
        }
        // Outstation has a master relationship with some traffic.
        let has_master = dnp3
            .relationships
            .iter()
            .any(|r| r.remote_role == "master" && r.packet_count > 0);
        if !has_master {
            continue;
        }
        // Outstation sends no outgoing OT traffic.
        let outstation_sends = input
            .connections
            .iter()
            .any(|c| c.src_ip == *ip && OT_PORTS.contains(&c.dst_port));
        if outstation_sends {
            continue;
        }
        findings.push(Finding::new(
            FindingType::AttackTechnique,
            Severity::Medium,
            format!("DNP3 outstation {} not reporting to master", ip),
            "A DNP3 outstation has a master relationship but is sending no outgoing \
             data on OT ports. Blocked reporting prevents the control system from \
             receiving process state updates from this field device."
                .to_string(),
            vec![ip.clone()],
            format!(
                "DNP3 outstation {} has master relationship but sends no OT-port traffic",
                ip
            ),
            Some(crate::attack_codes::T0804.to_string()),
        ));
    }

    findings
}

pub(super) fn detect_t0881_service_stop(input: &AnalysisInput) -> Vec<Finding> {
    let mut findings = Vec::new();

    // Count incoming packets per OT device.
    let mut device_incoming: HashMap<&str, u64> = HashMap::new();
    for conn in &input.connections {
        if OT_PORTS.contains(&conn.dst_port) {
            *device_incoming.entry(conn.dst_ip.as_str()).or_insert(0) += conn.packet_count;
        }
    }

    let device_count = device_incoming.len() as u64;
    if device_count == 0 {
        return findings;
    }
    let total_incoming: u64 = device_incoming.values().sum();
    let avg = total_incoming / device_count;
    if avg < 10 {
        return findings; // too little traffic to make the comparison meaningful
    }

    // Threshold: flag devices receiving < 5% of the average.
    let threshold = (avg / 20).max(1);

    let ot_asset_ips: HashSet<&str> = input
        .assets
        .iter()
        .filter(|a| a.protocols.iter().any(|p| is_ot_protocol_name(p)))
        .map(|a| a.ip_address.as_str())
        .collect();

    for ip in &ot_asset_ips {
        let incoming = device_incoming.get(ip).copied().unwrap_or(0);
        if incoming <= threshold {
            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::High,
                format!(
                    "Possible service stop on OT device {} (very low traffic)",
                    ip
                ),
                "An OT device has active protocol classifications but is receiving \
                 significantly fewer packets than peer OT devices. This may indicate \
                 that a process service has been forced offline or stopped responding."
                    .to_string(),
                vec![ip.to_string()],
                format!(
                    "Device {} received {} OT-port packets vs average {} across OT devices",
                    ip, incoming, avg
                ),
                Some(crate::attack_codes::T0881.to_string()),
            ));
        }
    }

    findings
}

pub(super) fn detect_t0864_transient_cyber_asset(
    input: &AnalysisInput,
    ctx: &CaptureContext,
) -> Vec<Finding> {
    let mut findings = Vec::new();

    if ctx.device_first_seen.is_empty() {
        return findings; // no timing data
    }

    let known_ot: HashSet<&str> = ctx.ot_device_ips.iter().map(String::as_str).collect();

    for (ip, &first) in &ctx.device_first_seen {
        if known_ot.contains(ip.as_str()) {
            continue; // expected OT device
        }
        let last = ctx.device_last_seen.get(ip).copied().unwrap_or(first);
        let duration = last - first;
        if duration <= 0.0 || duration >= thresholds::TRANSIENT_ASSET_SECS {
            continue;
        }
        // Only flag if it communicated with an OT device.
        let reached_ot = input.connections.iter().any(|c| {
            (c.src_ip == *ip && ctx.ot_device_ips.contains(&c.dst_ip))
                || (c.dst_ip == *ip && ctx.ot_device_ips.contains(&c.src_ip))
        });
        if !reached_ot {
            continue;
        }
        findings.push(Finding::new(
            FindingType::AttackTechnique,
            Severity::Medium,
            format!(
                "Transient device {} seen for only {:.0}s on OT segment",
                ip, duration
            ),
            "A non-OT device appeared briefly on the OT network and communicated with \
             OT devices. Short-lived devices such as maintenance laptops or USB adapters \
             represent uncontrolled access vectors that may introduce malware or \
             exfiltrate configuration data."
                .to_string(),
            vec![ip.clone()],
            format!(
                "Device {} seen for {:.0}s ({:.1} min), communicated with OT devices",
                ip,
                duration,
                duration / 60.0
            ),
            Some(crate::attack_codes::T0864.to_string()),
        ));
    }

    findings
}

// ── tests ──────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AnalysisInput, AssetSnapshot, ConnectionSnapshot, DeepParseSnapshot, Dnp3Snapshot,
        RelationshipSnapshot,
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

    // ── T0803 ──
    #[test]
    fn test_t0803_plc_receives_no_commands() {
        let mut input = AnalysisInput::default();
        input.assets = vec![
            asset("10.0.0.1", "plc", &["Modbus"]),   // field device
            asset("10.0.0.100", "hmi", &["Modbus"]), // controller
        ];
        // Controller sends to port 502 but to a different PLC (not 10.0.0.1)
        input.connections = vec![
            conn("10.0.0.100", "10.0.0.2", 502, "Modbus", 100), // not to 10.0.0.1
            conn("10.0.0.1", "10.0.0.100", 49152, "Modbus", 10), // PLC has some traffic
        ];
        let findings = detect_t0803_block_command_reporting(&input);
        assert!(
            !findings.is_empty(),
            "PLC with no incoming OT commands should trigger T0803"
        );
        assert_eq!(findings[0].technique_id, Some(crate::attack_codes::T0803.to_string()));
    }

    // ── T0804 ──
    #[test]
    fn test_t0804_dnp3_outstation_not_reporting() {
        let mut input = AnalysisInput::default();
        input.deep_parse.insert(
            "10.0.0.5".to_string(),
            DeepParseSnapshot {
                dnp3: Some(Dnp3Snapshot {
                    role: "outstation".to_string(),
                    has_unsolicited: false,
                    function_codes: vec![],
                    relationships: vec![RelationshipSnapshot {
                        remote_ip: "10.0.0.100".to_string(),
                        remote_role: "master".to_string(),
                        packet_count: 50,
                    }],
                }),
                ..Default::default()
            },
        );
        // Outstation has NO outgoing connections at all — so detect_t0804 should flag it.
        let findings = detect_t0804_block_reporting_message(&input);
        assert!(
            !findings.is_empty(),
            "Outstation with master but no outgoing traffic → T0804"
        );
        assert_eq!(findings[0].technique_id, Some(crate::attack_codes::T0804.to_string()));
    }

    // ── T0881 ──
    #[test]
    fn test_t0881_silent_ot_device() {
        let mut input = AnalysisInput::default();
        // Three OT devices; one receives far less traffic.
        input.assets = vec![
            asset("10.0.0.1", "plc", &["Modbus"]),
            asset("10.0.0.2", "plc", &["Modbus"]),
            asset("10.0.0.3", "plc", &["Modbus"]),
        ];
        // 10.0.0.1 and 10.0.0.2 receive lots of traffic; 10.0.0.3 receives almost none.
        for _ in 0..10 {
            input
                .connections
                .push(conn("10.0.0.100", "10.0.0.1", 502, "Modbus", 1000));
            input
                .connections
                .push(conn("10.0.0.100", "10.0.0.2", 502, "Modbus", 1000));
        }
        input
            .connections
            .push(conn("10.0.0.100", "10.0.0.3", 502, "Modbus", 1));
        let findings = detect_t0881_service_stop(&input);
        let t0881 = findings.iter().any(|f| {
            f.technique_id == Some(crate::attack_codes::T0881.to_string())
                && f.affected_assets.contains(&"10.0.0.3".to_string())
        });
        assert!(t0881, "Silent OT device should trigger T0881");
    }

    // ── T0864 ──
    #[test]
    fn test_t0864_transient_device() {
        let mut input = AnalysisInput::default();
        let mut ctx = CaptureContext::default();
        ctx.ot_device_ips.insert("10.0.0.1".to_string());
        // Transient laptop: seen for 120 seconds, connected to OT device.
        ctx.device_first_seen
            .insert("192.168.0.99".to_string(), 0.0);
        ctx.device_last_seen
            .insert("192.168.0.99".to_string(), 120.0);
        input.connections = vec![conn("192.168.0.99", "10.0.0.1", 502, "Modbus", 3)];
        let findings = detect_t0864_transient_cyber_asset(&input, &ctx);
        assert!(
            !findings.is_empty(),
            "Device seen for 120s touching OT should trigger T0864"
        );
        assert_eq!(findings[0].technique_id, Some(crate::attack_codes::T0864.to_string()));
    }
}

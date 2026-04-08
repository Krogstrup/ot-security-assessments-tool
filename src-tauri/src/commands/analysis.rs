//! Security analysis commands: ATT&CK detection, Purdue assignment, anomaly scoring.
//!
//! These commands bridge the gm-analysis crate to the Tauri frontend.
//! They construct AnalysisInput from AppState, run analysis, and
//! store results back into AppState.

use std::collections::{HashMap, HashSet};

use gm_analysis::{
    assess_switch_security, detect_malware_patterns, generate_compliance_report, AnalysisInput,
    AnalysisResult, AnomalyScore, AssetSnapshot, BacnetSnapshot, CaptureContext, ComplianceMapping,
    ConnectionSnapshot, CredentialChecker, CriticalityAssessment, CveMatch, CveMatcher,
    DeepParseSnapshot, DefaultCredential, Dnp3Snapshot, EnipSnapshot, FcSnapshot, Finding,
    Iec104Snapshot, MalwareFinding, ModbusSnapshot, NamingSuggestion, PollingSnapshot,
    ProfinetDcpSnapshot, PurdueAssignment, RelationshipSnapshot, S7Snapshot, SwitchSecurityFinding,
    SwitchSecurityInput,
};

use super::{
    support::{read_state, write_state},
    AnalysisState, AppState, CaptureState, InventoryState,
};

// ─── Input builders ───────────────────────────────────────────────────────────

/// Build AnalysisInput from capture + inventory domain slices.
fn build_analysis_input(capture: &CaptureState, inventory: &InventoryState) -> AnalysisInput {
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

fn asset_snapshots(inventory: &InventoryState) -> Vec<AssetSnapshot> {
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
fn build_capture_context(
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

fn build_malware_deep_parse(
    inventory: &InventoryState,
) -> HashMap<String, gm_analysis::DeepParseSnapshot> {
    let mut deep_parse = HashMap::new();
    for (ip, dp) in &inventory.deep_parse_info {
        let modbus = dp.modbus.as_ref().map(|m| gm_analysis::ModbusSnapshot {
            role: m.role.clone(),
            unit_ids: m.unit_ids.clone(),
            function_codes: m
                .function_codes
                .iter()
                .map(|fc| gm_analysis::FcSnapshot {
                    code: fc.code,
                    count: fc.count,
                    is_write: fc.is_write,
                })
                .collect(),
            relationships: m
                .relationships
                .iter()
                .map(|r| gm_analysis::RelationshipSnapshot {
                    remote_ip: r.remote_ip.clone(),
                    remote_role: r.remote_role.clone(),
                    packet_count: r.packet_count,
                })
                .collect(),
            polling_intervals: m
                .polling_intervals
                .iter()
                .map(|pi| gm_analysis::PollingSnapshot {
                    remote_ip: pi.remote_ip.clone(),
                    function_code: pi.function_code,
                    avg_interval_ms: pi.avg_interval_ms,
                    min_interval_ms: pi.min_interval_ms,
                    max_interval_ms: pi.max_interval_ms,
                    sample_count: pi.sample_count,
                })
                .collect(),
        });
        let iec104 = dp.iec104.as_ref().map(|i| gm_analysis::Iec104Snapshot {
            role: i.role.clone(),
            has_control_commands: i.has_control_commands,
            has_reset_process: i.has_reset_process,
            has_interrogation: i.has_interrogation,
        });
        deep_parse.insert(
            ip.clone(),
            gm_analysis::DeepParseSnapshot {
                modbus,
                iec104,
                ..Default::default()
            },
        );
    }
    deep_parse
}

// ─── Commands ─────────────────────────────────────────────────────────────────

/// Maximum findings returned by get_findings — nobody reads 50 000 findings.
const MAX_FINDINGS: usize = 1_000;
/// Maximum anomaly scores returned by get_anomalies.
const MAX_ANOMALIES: usize = 500;

fn persist_analysis_result(
    result: &AnalysisResult,
    inventory: &mut InventoryState,
    analysis: &mut AnalysisState,
) {
    analysis.findings = result.findings.clone();
    analysis.purdue_assignments = result.purdue_assignments.clone();
    analysis.anomalies = result.anomalies.clone();

    let purdue_map: HashMap<&str, u8> = result
        .purdue_assignments
        .iter()
        .map(|a| (a.ip_address.as_str(), a.level))
        .collect();

    for asset in &mut inventory.assets {
        if asset.purdue_level.is_none() {
            if let Some(&level) = purdue_map.get(asset.ip_address.as_str()) {
                asset.purdue_level = Some(level);
            }
        }
    }
}

/// Run the full security analysis pipeline.
///
/// Detects ATT&CK techniques, auto-assigns Purdue levels, scores anomalies.
/// Results are stored in AppState and returned to the frontend.
///
/// Lock order: capture (read) → inventory (write) → analysis (write)
pub fn run_analysis(state: &AppState) -> Result<AnalysisResult, String> {
    let capture = read_state(&state.capture, "capture")?;
    let mut inventory = write_state(&state.inventory, "inventory")?;
    let mut analysis = write_state(&state.analysis, "analysis")?;

    let input = build_analysis_input(&capture, &inventory);
    let ctx = build_capture_context(&capture, &inventory, &analysis);
    let result = gm_analysis::run_full_analysis(&input, &ctx);

    persist_analysis_result(&result, &mut inventory, &mut analysis);

    Ok(result)
}

/// Get findings from the last analysis run (capped at MAX_FINDINGS = 1 000).
pub fn get_findings(state: &AppState) -> Result<Vec<Finding>, String> {
    let analysis = read_state(&state.analysis, "analysis")?;
    if analysis.findings.len() <= MAX_FINDINGS {
        return Ok(analysis.findings.clone());
    }
    Ok(analysis.findings[..MAX_FINDINGS].to_vec())
}

/// Get Purdue level assignments from the last analysis run.
pub fn get_purdue_assignments(state: &AppState) -> Result<Vec<PurdueAssignment>, String> {
    let analysis = read_state(&state.analysis, "analysis")?;
    Ok(analysis.purdue_assignments.clone())
}

/// Get anomaly scores from the last analysis run (capped at MAX_ANOMALIES = 500).
pub fn get_anomalies(state: &AppState) -> Result<Vec<AnomalyScore>, String> {
    let analysis = read_state(&state.analysis, "analysis")?;
    if analysis.anomalies.len() <= MAX_ANOMALIES {
        return Ok(analysis.anomalies.clone());
    }
    Ok(analysis.anomalies[..MAX_ANOMALIES].to_vec())
}

/// Get credential warnings for all discovered devices.
///
/// Checks vendor+product strings against the default credential database.
pub fn get_credential_warnings(state: &AppState) -> Result<Vec<DefaultCredential>, String> {
    let inventory = read_state(&state.inventory, "inventory")?;

    let checker = CredentialChecker::new()?;
    let mut results = Vec::new();

    for asset in &inventory.assets {
        let vendor = asset.vendor.as_deref().unwrap_or("");
        let product = asset.product_family.as_deref().unwrap_or("");
        let matches = checker.check_device(vendor, product);
        results.extend(matches);
    }

    // Deduplicate by vendor+product_pattern
    results.dedup_by(|a, b| a.vendor == b.vendor && a.product_pattern == b.product_pattern);

    Ok(results)
}

/// Assess criticality for all discovered assets.
pub fn get_criticality(state: &AppState) -> Result<Vec<CriticalityAssessment>, String> {
    let capture = read_state(&state.capture, "capture")?;
    let inventory = read_state(&state.inventory, "inventory")?;
    let input = build_analysis_input(&capture, &inventory);
    Ok(gm_analysis::assess_criticality_all(&input.assets))
}

/// Get naming suggestions for all discovered assets.
pub fn get_naming_suggestions(state: &AppState) -> Result<Vec<NamingSuggestion>, String> {
    let capture = read_state(&state.capture, "capture")?;
    let inventory = read_state(&state.inventory, "inventory")?;
    let input = build_analysis_input(&capture, &inventory);
    Ok(gm_analysis::suggest_names_all(&input.assets))
}

/// Run switch port security assessment against the current dataset.
///
/// Uses asset list, protocol observations, redundancy frames, LLDP VLAN data,
/// and default credential matches to produce actionable switch security findings.
///
/// Lock order: capture (read) → inventory (read)
pub fn get_switch_security_findings(
    state: &AppState,
) -> Result<Vec<SwitchSecurityFinding>, String> {
    let capture = read_state(&state.capture, "capture")?;
    let inventory = read_state(&state.inventory, "inventory")?;

    let assets = asset_snapshots(&inventory);

    // Build protocols_by_ip from asset protocol lists
    let protocols_by_ip = inventory
        .assets
        .iter()
        .map(|a| (a.ip_address.clone(), a.protocols.clone()))
        .collect();

    // Collect redundancy protocol names and topology change flag
    let redundancy_protocols_seen: Vec<String> = capture
        .redundancy_protocols
        .iter()
        .map(|r| r.protocol.hint().to_string())
        .collect();

    let topology_change_seen = capture
        .redundancy_protocols
        .iter()
        .any(|r| r.topology_change);

    // Collect VLAN IDs from LLDP data
    let vlan_ids_seen: Vec<u16> = inventory
        .deep_parse_info
        .values()
        .filter_map(|dp| dp.lldp.as_ref())
        .flat_map(|lldp| lldp.vlan_ids.iter().copied())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    // Find switches that match default credentials
    let checker = CredentialChecker::new()?;
    let default_cred_switch_ips: Vec<String> = inventory
        .assets
        .iter()
        .filter(|a| {
            let dt = a.device_type.to_lowercase();
            dt.contains("switch")
        })
        .filter(|a| {
            let vendor = a.vendor.as_deref().unwrap_or("");
            let product = a.product_family.as_deref().unwrap_or("");
            !checker.check_device(vendor, product).is_empty()
        })
        .map(|a| a.ip_address.clone())
        .collect();

    let input = SwitchSecurityInput {
        assets: &assets,
        protocols_by_ip,
        redundancy_protocols_seen,
        topology_change_seen,
        vlan_ids_seen,
        default_cred_switch_ips,
    };

    Ok(assess_switch_security(&input))
}

/// Detect ICS malware behavioral patterns in the current capture.
///
/// Checks for FrostyGoop (Modbus write-only master), PIPEDREAM/INCONTROLLER
/// (multi-protocol reconnaissance), and Industroyer2 (IEC 104 burst commands).
///
/// Lock order: capture (read) → inventory (read) → analysis (read)
pub fn get_malware_findings(state: &AppState) -> Result<Vec<MalwareFinding>, String> {
    let capture = read_state(&state.capture, "capture")?;
    let inventory = read_state(&state.inventory, "inventory")?;
    let analysis = read_state(&state.analysis, "analysis")?;

    let ctx = build_capture_context(&capture, &inventory, &analysis);
    let connections = build_analysis_input(&capture, &inventory).connections;
    let deep_parse = build_malware_deep_parse(&inventory);

    Ok(detect_malware_patterns(&ctx, &connections, &deep_parse))
}

/// Get CVE warnings for a specific device based on its LLDP/SNMP identity.
///
/// Checks vendor, model, and firmware (extracted from LLDP or SNMP deep parse
/// data) against the bundled OT infrastructure CVE database.
pub fn get_cve_warnings(ip: String, state: &AppState) -> Result<Vec<CveMatch>, String> {
    let inventory = read_state(&state.inventory, "inventory")?;

    // Priority for vendor/model/firmware: LLDP > SNMP > asset info
    let dp = inventory.deep_parse_info.get(&ip);
    let asset = inventory.assets.iter().find(|a| a.ip_address == ip);

    let (vendor, model, firmware) = if let Some(lldp) = dp.and_then(|d| d.lldp.as_ref()) {
        (
            lldp.vendor
                .clone()
                .or_else(|| asset.and_then(|a| a.vendor.clone()))
                .unwrap_or_default(),
            lldp.model.clone().unwrap_or_default(),
            lldp.firmware.clone(),
        )
    } else if let Some(snmp) = dp.and_then(|d| d.snmp.as_ref()) {
        (
            snmp.vendor
                .clone()
                .or_else(|| asset.and_then(|a| a.vendor.clone()))
                .unwrap_or_default(),
            snmp.sys_descr.clone().unwrap_or_default(),
            None,
        )
    } else {
        (
            asset.and_then(|a| a.vendor.clone()).unwrap_or_default(),
            asset
                .and_then(|a| a.product_family.clone())
                .unwrap_or_default(),
            None,
        )
    };

    if vendor.is_empty() && model.is_empty() {
        return Ok(Vec::new());
    }

    let matcher = CveMatcher::new()?;
    Ok(matcher.check_device(&vendor, &model, firmware.as_deref()))
}

/// Generate a compliance report mapping findings to a specific framework.
///
/// `framework` must be one of: `"iec62443"`, `"nist80082"`, `"nerccip"`.
///
/// Lock order: capture (read) → inventory (read) → analysis (read)
pub fn get_compliance_report(
    state: &AppState,
    framework: String,
) -> Result<Vec<ComplianceMapping>, String> {
    if !["iec62443", "nist80082", "nerccip"].contains(&framework.as_str()) {
        return Err(format!(
            "Unknown framework '{}'. Supported: iec62443, nist80082, nerccip",
            framework
        ));
    }

    let capture = read_state(&state.capture, "capture")?;
    let inventory = read_state(&state.inventory, "inventory")?;
    let analysis = read_state(&state.analysis, "analysis")?;

    let input = build_analysis_input(&capture, &inventory);
    Ok(generate_compliance_report(
        &analysis.findings,
        &input.assets,
        &input.connections,
        &framework,
    ))
}

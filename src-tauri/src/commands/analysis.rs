//! Security analysis commands: ATT&CK detection, Purdue assignment, anomaly scoring.
//!
//! These commands bridge the gm-analysis crate to the Tauri frontend.
//! They construct AnalysisInput from AppState, run analysis, and
//! store results back into AppState.

use std::collections::HashMap;

use gm_analysis::{
    assess_switch_security, detect_malware_patterns, generate_compliance_report, AnalysisResult,
    AnomalyScore, ComplianceMapping, CredentialChecker, CriticalityAssessment, CveMatch,
    CveMatcher, DefaultCredential, Finding, MalwareFinding, NamingSuggestion, PurdueAssignment,
    SwitchSecurityFinding, SwitchSecurityInput,
};

use super::{
    analysis_builders::{asset_snapshots, build_analysis_input, build_capture_context},
    support::{read_state, write_state},
    AnalysisState, AppState, InventoryState,
};

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

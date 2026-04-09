//! Analysis read-model/use-case helpers that are independent of runtime state containers.

use std::collections::HashMap;

use gm_analysis::{
    assess_switch_security, detect_malware_patterns, generate_compliance_report,
    run_full_analysis as run_engine_full_analysis, AnalysisResult, AnomalyScore, ComplianceMapping,
    CredentialChecker, CriticalityAssessment, CveMatch, CveMatcher, DefaultCredential, Finding,
    MalwareFinding, NamingSuggestion, PurdueAssignment, SwitchSecurityFinding, SwitchSecurityInput,
};
use gm_parsers::{DeepParseInfo, RedundancyInfo};
use gm_types::{AssetInfo, ConnectionInfo};

use crate::application::mappers::{
    analysis_input::build_analysis_input, deep_parse::build_deep_parse_snapshot_map,
    snapshots::{asset_snapshots, connection_snapshots},
};

/// Projection written into runtime analysis state after a full run.
pub struct AnalysisProjection {
    pub findings: Vec<Finding>,
    pub purdue_assignments: Vec<PurdueAssignment>,
    pub anomalies: Vec<AnomalyScore>,
}

pub fn run_full_analysis(
    assets: &[AssetInfo],
    connections: &[ConnectionInfo],
    deep_parse_info: &HashMap<String, DeepParseInfo>,
    context: &gm_analysis::CaptureContext,
) -> AnalysisResult {
    let input = build_analysis_input(assets, connections, deep_parse_info);
    run_engine_full_analysis(&input, context)
}

pub fn project_analysis_state(result: &AnalysisResult) -> AnalysisProjection {
    AnalysisProjection {
        findings: result.findings.clone(),
        purdue_assignments: result.purdue_assignments.clone(),
        anomalies: result.anomalies.clone(),
    }
}

pub fn apply_purdue_assignments(assets: &mut [AssetInfo], assignments: &[PurdueAssignment]) {
    let purdue_map: HashMap<&str, u8> = assignments
        .iter()
        .map(|a| (a.ip_address.as_str(), a.level))
        .collect();

    for asset in assets {
        if asset.purdue_level.is_none() {
            if let Some(&level) = purdue_map.get(asset.ip_address.as_str()) {
                asset.purdue_level = Some(level);
            }
        }
    }
}

pub fn assess_criticality(
    assets: &[AssetInfo],
    connections: &[ConnectionInfo],
    deep_parse_info: &HashMap<String, DeepParseInfo>,
) -> Vec<CriticalityAssessment> {
    let input = build_analysis_input(assets, connections, deep_parse_info);
    gm_analysis::assess_criticality_all(&input.assets)
}

pub fn suggest_names(
    assets: &[AssetInfo],
    connections: &[ConnectionInfo],
    deep_parse_info: &HashMap<String, DeepParseInfo>,
) -> Vec<NamingSuggestion> {
    let input = build_analysis_input(assets, connections, deep_parse_info);
    gm_analysis::suggest_names_all(&input.assets)
}

pub fn malware_findings(
    context: &gm_analysis::CaptureContext,
    connections: &[ConnectionInfo],
    deep_parse_info: &HashMap<String, DeepParseInfo>,
) -> Vec<MalwareFinding> {
    let deep_parse = build_deep_parse_snapshot_map(deep_parse_info);
    let connections = connection_snapshots(connections);
    detect_malware_patterns(context, &connections, &deep_parse)
}

pub fn compliance_report(
    framework: &str,
    findings: &[Finding],
    assets: &[AssetInfo],
    connections: &[ConnectionInfo],
    deep_parse_info: &HashMap<String, DeepParseInfo>,
) -> Vec<ComplianceMapping> {
    let input = build_analysis_input(assets, connections, deep_parse_info);
    generate_compliance_report(findings, &input.assets, &input.connections, framework)
}

pub fn credential_warnings(assets: &[AssetInfo]) -> Result<Vec<DefaultCredential>, String> {
    let checker = CredentialChecker::new()?;
    let mut results = Vec::new();

    for asset in assets {
        let vendor = asset.vendor.as_deref().unwrap_or("");
        let product = asset.product_family.as_deref().unwrap_or("");
        let matches = checker.check_device(vendor, product);
        results.extend(matches);
    }

    results.dedup_by(|a, b| a.vendor == b.vendor && a.product_pattern == b.product_pattern);
    Ok(results)
}

pub fn switch_security_findings(
    assets: &[AssetInfo],
    deep_parse_info: &HashMap<String, DeepParseInfo>,
    redundancy_protocols: &[RedundancyInfo],
) -> Result<Vec<SwitchSecurityFinding>, String> {
    let assets = asset_snapshots(assets);

    let protocols_by_ip = assets
        .iter()
        .map(|a| (a.ip_address.clone(), a.protocols.clone()))
        .collect();

    let redundancy_protocols_seen: Vec<String> = redundancy_protocols
        .iter()
        .map(|r| r.protocol.hint().to_string())
        .collect();

    let topology_change_seen = redundancy_protocols.iter().any(|r| r.topology_change);

    let vlan_ids_seen: Vec<u16> = deep_parse_info
        .values()
        .filter_map(|dp| dp.lldp.as_ref())
        .flat_map(|lldp| lldp.vlan_ids.iter().copied())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let checker = CredentialChecker::new()?;
    let default_cred_switch_ips: Vec<String> = assets
        .iter()
        .filter(|a| a.device_type.to_lowercase().contains("switch"))
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

pub fn cve_warnings_for_ip(
    ip: &str,
    assets: &[AssetInfo],
    deep_parse_info: &HashMap<String, DeepParseInfo>,
) -> Result<Vec<CveMatch>, String> {
    let dp = deep_parse_info.get(ip);
    let asset = assets.iter().find(|a| a.ip_address == ip);

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

pub fn validate_compliance_framework(framework: &str) -> Result<(), String> {
    if ["iec62443", "nist80082", "nerccip"].contains(&framework) {
        Ok(())
    } else {
        Err(format!(
            "Unknown framework '{}'. Supported: iec62443, nist80082, nerccip",
            framework
        ))
    }
}

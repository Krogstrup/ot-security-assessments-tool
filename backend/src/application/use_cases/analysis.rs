//! Analysis read-model/use-case helpers that are independent of runtime state containers.

use std::collections::HashMap;

use gm_analysis::{
    assess_switch_security, CredentialChecker, CveMatch, CveMatcher, DefaultCredential,
    SwitchSecurityFinding, SwitchSecurityInput,
};
use gm_parsers::{DeepParseInfo, RedundancyInfo};
use gm_types::AssetInfo;

use crate::application::mappers::snapshots::asset_snapshots;

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

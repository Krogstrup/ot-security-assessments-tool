//! Segmentation analysis runner and enforcement config export.

use std::collections::HashMap;
use std::fmt;

use gm_analysis::{ConnectionStats, Finding, PatternAnomaly};
use gm_parsers::DeepParseInfo;
use gm_segmentation::{run_segmentation_analysis, EnforcementFormat, SegmentationReport};
use gm_types::{AssetInfo, ConnectionInfo};

use super::input_builder::build_segmentation_input;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SegmentationUseCaseError {
    UnknownEnforcementFormat(String),
    EnforcementConfigNotFound(String),
}

impl fmt::Display for SegmentationUseCaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SegmentationUseCaseError::UnknownEnforcementFormat(format) => {
                write!(f, "Unknown enforcement format: '{}'", format)
            }
            SegmentationUseCaseError::EnforcementConfigNotFound(format) => write!(
                f,
                "Enforcement config for format '{}' not found in report",
                format
            ),
        }
    }
}

impl std::error::Error for SegmentationUseCaseError {}

/// Run the full microsegmentation analysis (Phases 15A–15E).
pub fn run_segmentation(
    assets: &[AssetInfo],
    connections: &[ConnectionInfo],
    deep_parse_info: &HashMap<String, DeepParseInfo>,
    connection_stats: &[ConnectionStats],
    pattern_anomalies: &[PatternAnomaly],
    findings: &[Finding],
) -> Result<SegmentationReport, SegmentationUseCaseError> {
    let input = build_segmentation_input(
        assets,
        connections,
        deep_parse_info,
        connection_stats,
        pattern_anomalies,
        findings,
    );
    Ok(run_segmentation_analysis(&input))
}

/// Export one of the five enforcement config formats from a segmentation report.
pub fn export_enforcement_config(
    format: String,
    report: &SegmentationReport,
) -> Result<String, SegmentationUseCaseError> {
    let fmt = parse_enforcement_format(&format)?;

    let config = report
        .enforcement_configs
        .iter()
        .find(|c| c.format == fmt)
        .ok_or_else(|| SegmentationUseCaseError::EnforcementConfigNotFound(format.clone()))?;

    Ok(config.content.clone())
}

/// Parse enforcement format string to enum.
pub fn parse_enforcement_format(s: &str) -> Result<EnforcementFormat, SegmentationUseCaseError> {
    match s {
        "cisco_ios_acl" | "cisco_acl" => Ok(EnforcementFormat::CiscoIosAcl),
        "cisco_asa_acl" => Ok(EnforcementFormat::CiscoAsaAcl),
        "generic_firewall_table" | "palo_alto" | "fortinet" | "iptables" | "windows_firewall" => {
            Ok(EnforcementFormat::GenericFirewallTable)
        }
        "suricata_rules" => Ok(EnforcementFormat::SuricataRules),
        "json_policy" => Ok(EnforcementFormat::JsonPolicy),
        other => Err(SegmentationUseCaseError::UnknownEnforcementFormat(
            other.to_string(),
        )),
    }
}

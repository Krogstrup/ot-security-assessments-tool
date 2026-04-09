//! Segmentation analysis runner and enforcement config export.

use gm_segmentation::{run_segmentation_analysis, EnforcementFormat, SegmentationReport};

use super::input_builder::build_segmentation_input;
use crate::commands::support::{read_state, write_state};
use crate::commands::AppState;

/// Run the full microsegmentation analysis (Phases 15A–15E) and return
/// the complete [`SegmentationReport`].
///
/// The result is cached in AppState for subsequent `export_enforcement_config`
/// calls without re-running analysis.
///
/// Lock order: capture → inventory → analysis (read), then segmentation (write).
pub fn run_segmentation(state: &AppState) -> Result<SegmentationReport, String> {
    let capture = read_state(&state.capture, "capture")?;
    let inventory = read_state(&state.inventory, "inventory")?;
    let analysis = read_state(&state.analysis, "analysis")?;

    let input = build_segmentation_input(&capture, &inventory, &analysis);
    let report = run_segmentation_analysis(&input);

    drop(capture);
    drop(inventory);
    drop(analysis);

    let mut seg = write_state(&state.segmentation, "segmentation")?;
    seg.segmentation_report = Some(report.clone());

    log::info!(
        "Segmentation analysis complete: {} groups, {} zones, {} conduits, {} rules",
        report.policy_groups.len(),
        report.zone_model.zones.len(),
        report.zone_model.conduits.len(),
        report.communication_matrix.zone_pairs.len(),
    );

    Ok(report)
}

/// Export one of the five enforcement config formats from the last segmentation run.
///
/// Returns the full text content of the generated configuration file.
/// Returns an error if `run_segmentation` has not been called yet in this session.
pub fn export_enforcement_config(format: String, state: &AppState) -> Result<String, String> {
    let seg = read_state(&state.segmentation, "segmentation")?;

    let report = seg.segmentation_report.as_ref().ok_or_else(|| {
        "No segmentation report available. Run segmentation analysis first.".to_string()
    })?;

    let fmt = parse_enforcement_format(&format)?;

    let config = report
        .enforcement_configs
        .iter()
        .find(|c| c.format == fmt)
        .ok_or_else(|| format!("Enforcement config for format '{format}' not found in report"))?;

    Ok(config.content.clone())
}

/// Parse enforcement format string to enum.
pub fn parse_enforcement_format(s: &str) -> Result<EnforcementFormat, String> {
    match s {
        "cisco_ios_acl" | "cisco_acl" => Ok(EnforcementFormat::CiscoIosAcl),
        "cisco_asa_acl" => Ok(EnforcementFormat::CiscoAsaAcl),
        "generic_firewall_table" | "palo_alto" | "fortinet" | "iptables" | "windows_firewall" => {
            Ok(EnforcementFormat::GenericFirewallTable)
        }
        "suricata_rules" => Ok(EnforcementFormat::SuricataRules),
        "json_policy" => Ok(EnforcementFormat::JsonPolicy),
        other => Err(format!("Unknown enforcement format: '{other}'")),
    }
}

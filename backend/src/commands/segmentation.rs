//! Adapter layer for segmentation use-cases.

use gm_segmentation::SegmentationReport;

use crate::application::use_cases::segmentation as use_case;

use super::{
    support::{read_state, write_state},
    AppState,
};

/// Run segmentation and cache the report in runtime state.
pub fn run_segmentation(state: &AppState) -> Result<SegmentationReport, String> {
    let capture = read_state(&state.capture, "capture")?;
    let inventory = read_state(&state.inventory, "inventory")?;
    let analysis = read_state(&state.analysis, "analysis")?;

    let report = use_case::run_segmentation(
        &inventory.assets,
        &capture.connections,
        &inventory.deep_parse_info,
        &analysis.connection_stats,
        &analysis.pattern_anomalies,
        &analysis.findings,
    )?;

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

/// Export cached enforcement config from last segmentation report.
pub fn export_enforcement_config(format: String, state: &AppState) -> Result<String, String> {
    let seg = read_state(&state.segmentation, "segmentation")?;
    let report = seg
        .segmentation_report
        .as_ref()
        .ok_or_else(|| "No segmentation report available. Run segmentation analysis first.".to_string())?;
    use_case::export_enforcement_config(format, report)
}

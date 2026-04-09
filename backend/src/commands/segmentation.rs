//! Adapter layer for segmentation use-cases.

use gm_segmentation::SegmentationReport;

use crate::application::use_cases::segmentation as use_case;

use super::{
    error::AppError,
    support::{read_state, write_state},
    AppState,
};

/// Run segmentation and cache the report in runtime state.
pub fn run_segmentation(state: &AppState) -> Result<SegmentationReport, AppError> {
    let capture = read_state(&state.capture, "capture").map_err(AppError::state_lock)?;
    let inventory = read_state(&state.inventory, "inventory").map_err(AppError::state_lock)?;
    let analysis = read_state(&state.analysis, "analysis").map_err(AppError::state_lock)?;

    let report = use_case::run_segmentation(
        &inventory.assets,
        &capture.connections,
        &inventory.deep_parse_info,
        &analysis.connection_stats,
        &analysis.pattern_anomalies,
        &analysis.findings,
    )
    .map_err(AppError::from)?;

    drop(capture);
    drop(inventory);
    drop(analysis);

    let mut seg = write_state(&state.segmentation, "segmentation").map_err(AppError::state_lock)?;
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
pub fn export_enforcement_config(format: String, state: &AppState) -> Result<String, AppError> {
    let seg = read_state(&state.segmentation, "segmentation").map_err(AppError::state_lock)?;
    let report = seg.segmentation_report.as_ref().ok_or_else(|| {
        AppError::invalid_input(
            "No segmentation report available. Run segmentation analysis first.",
        )
    })?;
    use_case::export_enforcement_config(format, report).map_err(AppError::from)
}

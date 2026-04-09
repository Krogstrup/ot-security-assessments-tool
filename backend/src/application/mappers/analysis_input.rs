//! Builds [`AnalysisInput`] from capture + inventory state slices.

use gm_analysis::AnalysisInput;

use crate::{
    application::mappers::{
        deep_parse::build_deep_parse_snapshot_map,
        snapshots::{asset_snapshots, connection_snapshots},
    },
    // TODO: CaptureState/InventoryState should migrate to the application layer;
    // this upward dependency (application → commands) is a known layering violation.
    commands::{CaptureState, InventoryState},
};

/// Build `AnalysisInput` from capture + inventory domain slices.
pub fn build_analysis_input(capture: &CaptureState, inventory: &InventoryState) -> AnalysisInput {
    AnalysisInput {
        assets: asset_snapshots(inventory),
        connections: connection_snapshots(capture),
        deep_parse: build_deep_parse_snapshot_map(inventory),
    }
}

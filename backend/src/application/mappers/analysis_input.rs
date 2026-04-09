//! Builds [`AnalysisInput`] from capture + inventory state slices.

use std::collections::HashMap;

use gm_analysis::AnalysisInput;
use gm_parsers::DeepParseInfo;
use gm_types::{AssetInfo, ConnectionInfo};

use crate::{
    application::mappers::{
        deep_parse::build_deep_parse_snapshot_map,
        snapshots::{asset_snapshots, connection_snapshots},
    },
};

/// Build `AnalysisInput` from capture + inventory domain slices.
pub fn build_analysis_input(
    assets: &[AssetInfo],
    connections: &[ConnectionInfo],
    deep_parse_info: &HashMap<String, DeepParseInfo>,
) -> AnalysisInput {
    AnalysisInput {
        assets: asset_snapshots(assets),
        connections: connection_snapshots(connections),
        deep_parse: build_deep_parse_snapshot_map(deep_parse_info),
    }
}

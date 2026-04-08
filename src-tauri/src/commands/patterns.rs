//! Communication pattern analysis commands.
//!
//! Returns pre-computed `ConnectionStats` and `PatternAnomaly` values that
//! were populated by `PacketProcessor::build_pattern_results()` during
//! PCAP import or live capture.

use super::{support::read_state, AppState};
use gm_analysis::{ConnectionStats, PatternAnomaly};
use gm_parsers::RedundancyInfo;

/// Get per-connection timing statistics for the current dataset.
pub fn get_connection_stats(state: &AppState) -> Result<Vec<ConnectionStats>, String> {
    let analysis = read_state(&state.analysis, "analysis")?;
    Ok(analysis.connection_stats.clone())
}

/// Get detected communication pattern anomalies for the current dataset.
pub fn get_pattern_anomalies(state: &AppState) -> Result<Vec<PatternAnomaly>, String> {
    let analysis = read_state(&state.analysis, "analysis")?;
    Ok(analysis.pattern_anomalies.clone())
}

/// Get observed Layer-2 redundancy protocol frames (MRP/RSTP/HSR/PRP/DLR).
///
/// Returns one entry per unique source MAC address (last-frame-wins).
/// Empty list if no redundancy frames were seen in the current dataset.
pub fn get_redundancy_protocols(state: &AppState) -> Result<Vec<RedundancyInfo>, String> {
    let capture = read_state(&state.capture, "capture")?;
    Ok(capture.redundancy_protocols.clone())
}

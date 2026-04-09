//! Communication allowlist generation and formatting.

use gm_analysis::{
    allowlist_to_csv, format_firewall_rules, generate_allowlist, AllowlistEntry, AssetSnapshot,
    ConnectionSnapshot, ConnectionStats as AnalysisConnectionStats,
};
use gm_types::{AssetInfo, ConnectionInfo};

use crate::application::mappers::snapshots::{asset_snapshots, connection_snapshots};

fn build_allowlist_inputs(
    assets: &[AssetInfo],
    connections: &[ConnectionInfo],
) -> (Vec<AssetSnapshot>, Vec<ConnectionSnapshot>) {
    (asset_snapshots(assets), connection_snapshots(connections))
}

/// Generate a communication allowlist from observed network traffic.
pub fn generate_communication_allowlist(
    assets: &[AssetInfo],
    connections: &[ConnectionInfo],
    connection_stats: &[AnalysisConnectionStats],
) -> Vec<AllowlistEntry> {
    let (assets, connections) = build_allowlist_inputs(assets, connections);
    generate_allowlist(&connections, &assets, connection_stats)
}

/// Render allowlist entries to CSV content.
pub fn export_allowlist_csv(entries: &[AllowlistEntry]) -> String {
    allowlist_to_csv(entries)
}

/// Render allowlist entries to firewall-rule text content.
pub fn export_firewall_rules(entries: &[AllowlistEntry]) -> String {
    format_firewall_rules(entries)
}

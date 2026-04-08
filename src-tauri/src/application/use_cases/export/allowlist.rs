//! Communication allowlist generation and export.

use gm_analysis::{
    allowlist_to_csv, format_firewall_rules, generate_allowlist, AllowlistEntry, AssetSnapshot,
    ConnectionSnapshot, ConnectionStats as AnalysisConnectionStats,
};

use crate::application::mappers::snapshots::{asset_snapshots, connection_snapshots};
use crate::commands::{support::read_state, AppState};

fn build_allowlist_inputs(
    state: &AppState,
) -> Result<
    (
        Vec<AssetSnapshot>,
        Vec<ConnectionSnapshot>,
        Vec<AnalysisConnectionStats>,
    ),
    String,
> {
    let capture = read_state(&state.capture, "capture")?;
    let inventory = read_state(&state.inventory, "inventory")?;
    let comm_stats = read_state(&state.analysis, "analysis")?
        .connection_stats
        .clone();

    let assets = asset_snapshots(&inventory);
    let connections = connection_snapshots(&capture);
    Ok((assets, connections, comm_stats))
}

/// Generate a communication allowlist from observed network traffic.
pub async fn generate_communication_allowlist(
    state: &AppState,
) -> Result<Vec<AllowlistEntry>, String> {
    let (assets, connections, comm_stats) = build_allowlist_inputs(state)?;
    Ok(generate_allowlist(&connections, &assets, &comm_stats))
}

/// Export the communication allowlist as a CSV file.
pub async fn export_allowlist_csv(
    output_path: String,
    state: &AppState,
) -> Result<String, String> {
    use crate::commands::support::write_text_file;
    use std::path::Path;

    let (assets, connections, comm_stats) = build_allowlist_inputs(state)?;
    let entries = generate_allowlist(&connections, &assets, &comm_stats);
    let csv = allowlist_to_csv(&entries);
    write_text_file(Path::new(&output_path), &csv)?;
    log::info!(
        "Exported communication allowlist ({} entries) to CSV: {}",
        entries.len(),
        output_path
    );
    Ok(output_path)
}

/// Export firewall rule suggestions derived from the communication allowlist.
pub async fn export_firewall_rules(
    output_path: String,
    state: &AppState,
) -> Result<String, String> {
    use crate::commands::support::write_text_file;
    use std::path::Path;

    let (assets, connections, comm_stats) = build_allowlist_inputs(state)?;
    let entries = generate_allowlist(&connections, &assets, &comm_stats);
    let rules = format_firewall_rules(&entries);
    write_text_file(Path::new(&output_path), &rules)?;
    log::info!(
        "Exported firewall rule suggestions ({} rules) to: {}",
        entries.len(),
        output_path
    );
    Ok(output_path)
}

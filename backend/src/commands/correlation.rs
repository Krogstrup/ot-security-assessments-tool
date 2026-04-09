//! Alert–device correlation commands.
//!
//! Takes imported IDS/SIEM alerts (Suricata, Wazuh) and enriches them with
//! device inventory data — hostname, device type, Purdue level — for the
//! "External Alerts" tab in AnalysisView and the device detail panel.

use crate::application::use_cases::correlation as use_case;

use super::{support::read_state, support::write_state, AppState};

// ─── Commands ────────────────────────────────────────────────
pub use use_case::CorrelatedAlert;

/// Return all imported IDS/SIEM alerts, enriched with device inventory data.
pub async fn get_correlated_alerts(state: &AppState) -> Result<Vec<CorrelatedAlert>, String> {
    let inventory = read_state(&state.inventory, "inventory")?;
    Ok(use_case::correlate_alerts(
        &inventory.imported_alerts,
        &inventory.assets,
    ))
}

/// Return alerts involving a specific IP address (as src or dst).
pub async fn get_alerts_for_ip(
    ip: String,
    state: &AppState,
) -> Result<Vec<CorrelatedAlert>, String> {
    let inventory = read_state(&state.inventory, "inventory")?;
    Ok(use_case::correlate_alerts_for_ip(
        &ip,
        &inventory.imported_alerts,
        &inventory.assets,
    ))
}

/// Clear all stored alerts.
pub async fn clear_alerts(state: &AppState) -> Result<(), String> {
    let mut inventory = write_state(&state.inventory, "inventory")?;
    inventory.imported_alerts.clear();
    log::info!("Cleared all imported alerts");
    Ok(())
}

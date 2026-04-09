//! Baseline drift comparison command adapter.
//!
//! Adapter responsibilities:
//! - Snapshot current in-memory state
//! - Read baseline session rows from DB
//! - Delegate diff logic to application use-case

use crate::application::use_cases::baseline as use_case;

use super::{
    error::AppError,
    support::{mutex_state, read_state},
    AppState,
};

pub use use_case::BaselineDiff;

/// Compare current state against a saved baseline session.
///
/// Lock strategy: snapshot current data first, then query the DB under session lock.
pub fn compare_sessions(
    baseline_session_id: String,
    state: &AppState,
) -> Result<BaselineDiff, AppError> {
    // Snapshot current assets and connections.
    let current_assets_vec = read_state(&state.inventory, "inventory")
        .map_err(AppError::state_lock)?
        .assets
        .clone();
    let current_connections_vec = read_state(&state.capture, "capture")
        .map_err(AppError::state_lock)?
        .connections
        .clone();

    // Load baseline rows from DB.
    let (baseline_session_name, baseline_asset_rows, baseline_conn_rows) = {
        let session = mutex_state(&state.session, "session").map_err(AppError::state_lock)?;
        let db = session.db.as_ref().ok_or(AppError::NoSession)?;
        let session_row = db
            .get_session(&baseline_session_id)
            .map_err(AppError::from)?;
        let baseline_asset_rows = db
            .list_assets(&baseline_session_id)
            .map_err(AppError::from)?;
        let baseline_conn_rows = db
            .list_connections(&baseline_session_id)
            .map_err(AppError::from)?;
        (session_row.name, baseline_asset_rows, baseline_conn_rows)
    };

    let diff = use_case::compare_baseline(
        baseline_session_name,
        &baseline_asset_rows,
        &baseline_conn_rows,
        &current_assets_vec,
        &current_connections_vec,
    );

    log::info!(
        "Baseline drift: {} new, {} missing, {} changed assets; {} new, {} missing connections (drift={:.1}%)",
        diff.new_assets.len(),
        diff.missing_assets.len(),
        diff.changed_assets.len(),
        diff.new_connections.len(),
        diff.missing_connections.len(),
        diff.summary.drift_score * 100.0
    );

    Ok(diff)
}

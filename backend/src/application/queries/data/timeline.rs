//! Timeline range query.

use serde::Serialize;

use crate::commands::{support::read_state, AppState};

/// Timeline range: earliest and latest timestamps across all connections.
#[derive(Debug, Clone, Serialize)]
pub struct TimelineRange {
    pub earliest: Option<String>,
    pub latest: Option<String>,
    /// Total number of connections with timestamps
    pub connection_count: usize,
}

/// Get the time range of the current dataset.
///
/// Returns the earliest and latest timestamps from all connections,
/// used by the timeline scrubber to set slider bounds.
/// Scans all connections (not capped) to ensure accurate bounds.
pub fn get_timeline_range(state: &AppState) -> Result<TimelineRange, String> {
    let capture = read_state(&state.capture, "capture")?;

    let mut earliest: Option<&str> = None;
    let mut latest: Option<&str> = None;

    for conn in &capture.connections {
        let fs = conn.first_seen.as_str();
        let ls = conn.last_seen.as_str();

        match earliest {
            None => earliest = Some(fs),
            Some(e) if fs < e => earliest = Some(fs),
            _ => {}
        }
        match latest {
            None => latest = Some(ls),
            Some(l) if ls > l => latest = Some(ls),
            _ => {}
        }
    }

    Ok(TimelineRange {
        earliest: earliest.map(|s| s.to_string()),
        latest: latest.map(|s| s.to_string()),
        connection_count: capture.connections.len(),
    })
}

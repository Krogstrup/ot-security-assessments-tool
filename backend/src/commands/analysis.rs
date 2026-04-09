//! Security analysis commands: ATT&CK detection, Purdue assignment, anomaly scoring.
//!
//! These commands are thin adapters over application-layer analysis use-cases.
//! They snapshot runtime state, delegate, and persist returned projections.

use gm_analysis::{
    AnalysisResult, AnomalyScore, ComplianceMapping, ConnectionStats, CriticalityAssessment,
    CveMatch, DefaultCredential, Finding, MalwareFinding, NamingSuggestion, PatternAnomaly,
    PurdueAssignment, SwitchSecurityFinding,
};
use gm_parsers::RedundancyInfo;
use gm_types::{MAX_ANOMALY_RESULTS, MAX_FINDINGS};

use crate::application::mappers::capture_context::build_capture_context_snapshot;
use crate::application::use_cases::analysis as analysis_use_case;

use super::{
    error::AppError,
    support::{read_state, write_state},
    AppState,
};

// ─── Commands ─────────────────────────────────────────────────────────────────

/// Run the full security analysis pipeline.
///
/// Detects ATT&CK techniques, auto-assigns Purdue levels, scores anomalies.
/// Results are stored in AppState and returned to the frontend.
///
/// Lock order: capture (read) → inventory (write) → analysis (write)
pub fn run_analysis(state: &AppState) -> Result<AnalysisResult, AppError> {
    let capture = read_state(&state.capture, "capture").map_err(AppError::state_lock)?;
    let mut inventory = write_state(&state.inventory, "inventory").map_err(AppError::state_lock)?;
    let mut analysis = write_state(&state.analysis, "analysis").map_err(AppError::state_lock)?;

    let context = build_capture_context_snapshot(
        &inventory.assets,
        &capture.connections,
        &analysis.connection_stats,
        &inventory.deep_parse_info,
    );
    let result = analysis_use_case::run_full_analysis(
        &inventory.assets,
        &capture.connections,
        &inventory.deep_parse_info,
        &context,
    );
    let projection = analysis_use_case::project_analysis_state(&result);

    analysis.findings = projection.findings;
    analysis.purdue_assignments = projection.purdue_assignments;
    analysis.anomalies = projection.anomalies;

    analysis_use_case::apply_purdue_assignments(
        inventory.assets.as_mut_slice(),
        &analysis.purdue_assignments,
    );

    Ok(result)
}

/// Get findings from the last analysis run (capped at MAX_FINDINGS = 1 000).
pub fn get_findings(state: &AppState) -> Result<Vec<Finding>, AppError> {
    let analysis = read_state(&state.analysis, "analysis").map_err(AppError::state_lock)?;
    if analysis.findings.len() <= MAX_FINDINGS {
        return Ok(analysis.findings.clone());
    }
    Ok(analysis.findings[..MAX_FINDINGS].to_vec())
}

/// Get Purdue level assignments from the last analysis run.
pub fn get_purdue_assignments(state: &AppState) -> Result<Vec<PurdueAssignment>, AppError> {
    let analysis = read_state(&state.analysis, "analysis").map_err(AppError::state_lock)?;
    Ok(analysis.purdue_assignments.clone())
}

/// Get anomaly scores from the last analysis run (capped at [`gm_types::MAX_ANOMALY_RESULTS`]).
pub fn get_anomalies(state: &AppState) -> Result<Vec<AnomalyScore>, AppError> {
    let analysis = read_state(&state.analysis, "analysis").map_err(AppError::state_lock)?;
    if analysis.anomalies.len() <= MAX_ANOMALY_RESULTS {
        return Ok(analysis.anomalies.clone());
    }
    Ok(analysis.anomalies[..MAX_ANOMALY_RESULTS].to_vec())
}

/// Get credential warnings for all discovered devices.
///
/// Checks vendor+product strings against the default credential database.
pub fn get_credential_warnings(state: &AppState) -> Result<Vec<DefaultCredential>, AppError> {
    let inventory = read_state(&state.inventory, "inventory").map_err(AppError::state_lock)?;
    analysis_use_case::credential_warnings(&inventory.assets).map_err(AppError::invalid_input)
}

/// Assess criticality for all discovered assets.
pub fn get_criticality(state: &AppState) -> Result<Vec<CriticalityAssessment>, AppError> {
    let capture = read_state(&state.capture, "capture").map_err(AppError::state_lock)?;
    let inventory = read_state(&state.inventory, "inventory").map_err(AppError::state_lock)?;
    Ok(analysis_use_case::assess_criticality(
        &inventory.assets,
        &capture.connections,
        &inventory.deep_parse_info,
    ))
}

/// Get naming suggestions for all discovered assets.
pub fn get_naming_suggestions(state: &AppState) -> Result<Vec<NamingSuggestion>, AppError> {
    let capture = read_state(&state.capture, "capture").map_err(AppError::state_lock)?;
    let inventory = read_state(&state.inventory, "inventory").map_err(AppError::state_lock)?;
    Ok(analysis_use_case::suggest_names(
        &inventory.assets,
        &capture.connections,
        &inventory.deep_parse_info,
    ))
}

/// Run switch port security assessment against the current dataset.
///
/// Uses asset list, protocol observations, redundancy frames, LLDP VLAN data,
/// and default credential matches to produce actionable switch security findings.
///
/// Lock order: capture (read) → inventory (read)
pub fn get_switch_security_findings(
    state: &AppState,
) -> Result<Vec<SwitchSecurityFinding>, AppError> {
    let capture = read_state(&state.capture, "capture").map_err(AppError::state_lock)?;
    let inventory = read_state(&state.inventory, "inventory").map_err(AppError::state_lock)?;

    analysis_use_case::switch_security_findings(
        &inventory.assets,
        &inventory.deep_parse_info,
        &capture.redundancy_protocols,
    )
    .map_err(AppError::invalid_input)
}

/// Detect ICS malware behavioral patterns in the current capture.
///
/// Checks for FrostyGoop (Modbus write-only master), PIPEDREAM/INCONTROLLER
/// (multi-protocol reconnaissance), and Industroyer2 (IEC 104 burst commands).
///
/// Lock order: capture (read) → inventory (read) → analysis (read)
pub fn get_malware_findings(state: &AppState) -> Result<Vec<MalwareFinding>, AppError> {
    let capture = read_state(&state.capture, "capture").map_err(AppError::state_lock)?;
    let inventory = read_state(&state.inventory, "inventory").map_err(AppError::state_lock)?;
    let analysis = read_state(&state.analysis, "analysis").map_err(AppError::state_lock)?;

    let context = build_capture_context_snapshot(
        &inventory.assets,
        &capture.connections,
        &analysis.connection_stats,
        &inventory.deep_parse_info,
    );

    Ok(analysis_use_case::malware_findings(
        &context,
        &capture.connections,
        &inventory.deep_parse_info,
    ))
}

/// Get CVE warnings for a specific device based on its LLDP/SNMP identity.
///
/// Checks vendor, model, and firmware (extracted from LLDP or SNMP deep parse
/// data) against the bundled OT infrastructure CVE database.
pub fn get_cve_warnings(ip: String, state: &AppState) -> Result<Vec<CveMatch>, AppError> {
    let inventory = read_state(&state.inventory, "inventory").map_err(AppError::state_lock)?;
    analysis_use_case::cve_warnings_for_ip(&ip, &inventory.assets, &inventory.deep_parse_info)
        .map_err(AppError::invalid_input)
}

/// Generate a compliance report mapping findings to a specific framework.
///
/// `framework` must be one of: `"iec62443"`, `"nist80082"`, `"nerccip"`.
///
/// Lock order: capture (read) → inventory (read) → analysis (read)
pub fn get_compliance_report(
    state: &AppState,
    framework: String,
) -> Result<Vec<ComplianceMapping>, AppError> {
    analysis_use_case::validate_compliance_framework(&framework)
        .map_err(AppError::invalid_input)?;

    let capture = read_state(&state.capture, "capture").map_err(AppError::state_lock)?;
    let inventory = read_state(&state.inventory, "inventory").map_err(AppError::state_lock)?;
    let analysis = read_state(&state.analysis, "analysis").map_err(AppError::state_lock)?;

    Ok(analysis_use_case::compliance_report(
        &framework,
        &analysis.findings,
        &inventory.assets,
        &capture.connections,
        &inventory.deep_parse_info,
    ))
}

/// Get per-connection timing statistics for the current dataset.
pub fn get_connection_stats(state: &AppState) -> Result<Vec<ConnectionStats>, AppError> {
    let analysis = read_state(&state.analysis, "analysis").map_err(AppError::state_lock)?;
    Ok(analysis.connection_stats.clone())
}

/// Get detected communication pattern anomalies for the current dataset.
pub fn get_pattern_anomalies(state: &AppState) -> Result<Vec<PatternAnomaly>, AppError> {
    let analysis = read_state(&state.analysis, "analysis").map_err(AppError::state_lock)?;
    Ok(analysis.pattern_anomalies.clone())
}

/// Get observed Layer-2 redundancy protocol frames (MRP/RSTP/HSR/PRP/DLR).
///
/// Returns one entry per unique source MAC address (last-frame-wins).
/// Empty list if no redundancy frames were seen in the current dataset.
pub fn get_redundancy_protocols(state: &AppState) -> Result<Vec<RedundancyInfo>, AppError> {
    let capture = read_state(&state.capture, "capture").map_err(AppError::state_lock)?;
    Ok(capture.redundancy_protocols.clone())
}

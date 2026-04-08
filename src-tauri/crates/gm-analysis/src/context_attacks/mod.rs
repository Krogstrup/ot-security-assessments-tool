//! Phase 14C — 18 new MITRE ATT&CK for ICS technique detections.
//!
//! These detections require richer per-session context than the base detections
//! in `attack.rs`. The [`CaptureContext`] struct carries MAC-to-IP mappings,
//! per-device first/last-seen timestamps, write-rate counters, and OT/external
//! IP classification built from `AppStateInner` before calling
//! [`detect_context_attacks`].
//!
//! ## Detected Techniques
//!
//! | Technique | Description | Severity |
//! |-----------|-------------|----------|
//! | T0822 | External Remote Services (RDP/VNC from OT device) | High |
//! | T0867 | Lateral Tool Transfer (FTP/TFTP within OT segment) | High |
//! | T0885 | Commonly Used Port (OT protocol on wrong port) | Medium |
//! | T0849 | Masquerading (non-OT protocol on OT port) | Medium |
//! | T0868 | Detect Operating Mode (S7 upload/download) | High |
//! | T0806 | Brute Force I/O (high write rate to single OT device) | High |
//! | T0802 | Automated Collection (many OT targets polled) | Medium |
//! | T0861 | Point and Tag Identification (wide Modbus unit-ID scan) | Medium |
//! | T0840 | Network Connection Enumeration (OT port sweep) | High |
//! | T0803/T0811 | Block Command / Modify I/O Image (PLC receiving no commands) | Medium |
//! | T0804 | Block Reporting Message (DNP3 outstation not reporting) | Medium |
//! | T0881 | Service Stop (OT device with very low traffic vs peers) | High |
//! | T0864 | Transient Cyber Asset (device seen for < 5 minutes) | Medium |
//! | T0830 | Adversary-in-the-Middle (IP with multiple MACs) | Critical |
//! | T0884 | Connection Proxy (non-OT device relaying OT traffic) | High |
//! | T0866 | Exploitation of Remote Services | High |
//! | T0800 | Activate Firmware Update Mode (CIP File / S7 upload) | Critical |
//! | T0801 | Monitor Process State (reads across many OT endpoints) | Medium |

use std::collections::{HashMap, HashSet};

use crate::helpers::is_ot_device_type;
use crate::{AnalysisInput, Finding};

mod advanced;
mod device_behavior;
mod network_defense;
mod port_protocol;

/// Rich per-capture state used by context-aware ATT&CK detections.
///
/// Built once per analysis run from `AppStateInner` in the commands layer and
/// passed alongside [`AnalysisInput`] to [`detect_context_attacks`].
/// All fields default to empty / zero so the struct can be used in tests without
/// populating every field.
#[derive(Debug, Clone, Default)]
pub struct CaptureContext {
    /// Earliest packet timestamp in this capture (Unix seconds, 0.0 = unknown).
    pub capture_start: f64,
    /// Latest packet timestamp in this capture (Unix seconds, 0.0 = unknown).
    pub capture_end: f64,
    /// IP → list of distinct MAC addresses observed for that IP.
    ///
    /// An IP with ≥ 2 distinct MACs is an AiTM / MAC-spoofing indicator (T0830).
    pub ip_to_macs: HashMap<String, Vec<String>>,
    /// MAC → list of distinct IPs seen using that MAC.
    pub mac_to_ips: HashMap<String, Vec<String>>,
    /// Per-IP: earliest timestamp seen (Unix seconds, 0.0 = unknown).
    pub device_first_seen: HashMap<String, f64>,
    /// Per-IP: latest timestamp seen (Unix seconds, 0.0 = unknown).
    pub device_last_seen: HashMap<String, f64>,
    /// Per-source: set of OT destination IPs queried via read operations.
    pub per_source_read_targets: HashMap<String, HashSet<String>>,
    /// Per-source: set of OT destination IPs targeted by write operations.
    pub per_source_write_targets: HashMap<String, HashSet<String>>,
    /// Per-source: all destination ports contacted.
    pub per_source_dst_ports: HashMap<String, HashSet<u16>>,
    /// Per (src, dst): total write-class packet / command count.
    pub per_connection_write_rate: HashMap<(String, String), u64>,
    /// IPs confirmed as running OT protocols (PLCs, RTUs, HMIs, historians …).
    pub ot_device_ips: HashSet<String>,
    /// IPs that are external / public (non-RFC-1918).
    pub external_ips: HashSet<String>,
}

/// Run all Phase 14C ATT&CK detections.
///
/// Called from [`crate::attack::detect_attack_techniques`] with the same
/// `AnalysisInput` snapshot and a `CaptureContext` built from `AppStateInner`.
pub fn detect_context_attacks(input: &AnalysisInput, ctx: &CaptureContext) -> Vec<Finding> {
    let mut findings = Vec::new();

    findings.extend(port_protocol::detect_t0822_external_remote_services(input, ctx));
    findings.extend(port_protocol::detect_t0867_lateral_tool_transfer(input, ctx));
    findings.extend(port_protocol::detect_t0885_commonly_used_port(input));
    findings.extend(port_protocol::detect_t0849_masquerading(input));
    findings.extend(device_behavior::detect_t0868_detect_operating_mode(input));
    findings.extend(device_behavior::detect_t0806_brute_force_io(input, ctx));
    findings.extend(device_behavior::detect_t0802_automated_collection(input, ctx));
    findings.extend(device_behavior::detect_t0861_point_tag_identification(input));
    findings.extend(device_behavior::detect_t0840_network_connection_enumeration(input, ctx));
    findings.extend(network_defense::detect_t0803_block_command_reporting(input));
    findings.extend(network_defense::detect_t0804_block_reporting_message(input));
    findings.extend(network_defense::detect_t0881_service_stop(input));
    findings.extend(network_defense::detect_t0864_transient_cyber_asset(input, ctx));
    findings.extend(advanced::detect_t0830_adversary_in_the_middle(ctx));
    findings.extend(advanced::detect_t0884_connection_proxy(input, ctx));
    findings.extend(advanced::detect_t0866_exploitation_remote_services(input, ctx));
    findings.extend(advanced::detect_t0800_firmware_update_mode(input));
    findings.extend(advanced::detect_t0801_monitor_process_state(input, ctx));

    findings
}

// ── helpers (shared across sub-modules) ──────────────────────────────────────

/// Returns a human-readable name for a remote access port.
pub(super) fn remote_service_name(port: u16) -> &'static str {
    match port {
        22 => "SSH",
        23 => "Telnet",
        3389 => "RDP",
        5900..=5910 => "VNC",
        5938 => "TeamViewer",
        7070 => "AnyDesk",
        _ => "remote access",
    }
}

/// Build the effective OT IP set: prefer `ctx.ot_device_ips` when non-empty,
/// otherwise derive from asset classification.
pub(super) fn effective_ot_ips<'a>(
    input: &'a AnalysisInput,
    ctx: &'a CaptureContext,
) -> HashSet<&'a str> {
    if !ctx.ot_device_ips.is_empty() {
        ctx.ot_device_ips.iter().map(String::as_str).collect()
    } else {
        input
            .assets
            .iter()
            .filter(|a| is_ot_device_type(&a.device_type))
            .map(|a| a.ip_address.as_str())
            .collect()
    }
}

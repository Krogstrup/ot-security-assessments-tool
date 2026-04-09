//! MITRE ATT&CK for ICS technique detection.
//!
//! Analyzes deep parse info, asset data, and connections to detect
//! known attack patterns mapped to MITRE ATT&CK for ICS techniques.
//!
//! ## Detected Techniques
//!
//! | Technique | Behavior | Severity |
//! |-----------|----------|----------|
//! | T0855 | Modbus broadcast/mass writes (FC 5/6/15/16 to unit 0/255) | Critical |
//! | T0814 | Modbus FC 8 diagnostics from non-engineering workstation | High |
//! | T0856 | DNP3 unsolicited response to unknown master | Medium |
//! | T0846 | Unknown device polling PLCs (new source targeting OT ports) | High |
//! | T0886 | Cross-Purdue zone communication (L1 <-> L4) | Medium |

mod architectural;
mod bacnet;
mod discovery;
mod dnp3;
mod enip;
mod iec104;
mod modbus;
mod s7;
#[cfg(test)]
pub(super) mod test_utils;

use crate::{AnalysisInput, CaptureContext, Finding};

/// Run all ATT&CK technique detections on the input data.
///
/// Includes the 18 Phase 14C detections from [`crate::context_attacks`] that
/// require the richer [`CaptureContext`] snapshot.
pub fn detect_attack_techniques(input: &AnalysisInput, ctx: &CaptureContext) -> Vec<Finding> {
    let mut findings = Vec::new();

    findings.extend(modbus::detect_t0855_unauthorized_writes(input));
    findings.extend(modbus::detect_t0814_diagnostic_dos(input));
    findings.extend(dnp3::detect_t0856_dnp3_unsolicited(input));
    findings.extend(discovery::detect_t0846_remote_discovery(input));
    findings.extend(enip::detect_enip_attacks(input));
    findings.extend(s7::detect_s7_attacks(input));
    findings.extend(bacnet::detect_bacnet_attacks(input));
    findings.extend(iec104::detect_iec104_attacks(input));
    findings.extend(architectural::detect_flat_network(input));
    findings.extend(architectural::detect_cleartext_ot(input));
    findings.extend(architectural::detect_internet_exposed_ot(input));
    findings.extend(crate::context_attacks::detect_context_attacks(input, ctx));

    findings
}

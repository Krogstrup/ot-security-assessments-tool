use crate::{AnalysisInput, Finding, FindingType, Severity};

/// IEC 60870-5-104 ATT&CK detections: T0855 (control commands), T0816 (reset process), T0814 (interrogation flood).
pub(super) fn detect_iec104_attacks(input: &AnalysisInput) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (ip, dp) in &input.deep_parse {
        let iec104 = match &dp.iec104 {
            Some(i) => i,
            None => continue,
        };

        // T0855: Control command ASDUs (type IDs 45–69) — unauthorized command message
        if iec104.has_control_commands {
            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::High,
                format!("IEC 104 control commands from {}", ip),
                "IEC 60870-5-104 control command ASDUs (type IDs 45–69) were detected. \
                 These commands control physical process elements at the outstation \
                 (e.g., circuit breakers, valves, set-points). Unauthorized command \
                 injection can cause unexpected physical process changes."
                    .to_string(),
                vec![ip.clone()],
                format!(
                    "Source {} sent IEC 104 control command ASDUs (type IDs 45–69)",
                    ip
                ),
                Some(crate::attack_codes::T0855.to_string()),
            ));
        }

        // T0816: Reset Process command (type ID 105) — disrupts outstation process
        if iec104.has_reset_process {
            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::Critical,
                format!("IEC 104 Reset Process command from {}", ip),
                "An IEC 104 Reset Process command (C_RP_NA_1, type ID 105) was detected. \
                 This command resets the outstation's process, potentially disrupting \
                 power grid control or other critical infrastructure operations and \
                 causing a loss of control or availability."
                    .to_string(),
                vec![ip.clone()],
                format!(
                    "Source {} sent IEC 104 Reset Process (C_RP_NA_1, type ID 105)",
                    ip
                ),
                Some(crate::attack_codes::T0816.to_string()),
            ));
        }

        // T0814: Interrogation from non-OT device — potential DoS / reconnaissance
        if iec104.has_interrogation {
            let src_type = input
                .assets
                .iter()
                .find(|a| a.ip_address == *ip)
                .map(|a| a.device_type.as_str())
                .unwrap_or("unknown");
            if src_type == "it_device" || src_type == "unknown" {
                findings.push(Finding::new(
                    FindingType::AttackTechnique,
                    Severity::Medium,
                    format!("IEC 104 interrogation from non-OT device {}", ip),
                    "IEC 104 General Interrogation commands (C_IC_NA_1, type ID 100) were \
                     detected from a device not classified as OT equipment. Interrogation \
                     requests from unauthorized sources may indicate network reconnaissance \
                     or an attempt to flood the outstation's response queue."
                        .to_string(),
                    vec![ip.clone()],
                    format!(
                        "Device {} (type: {}) sent IEC 104 General Interrogation (C_IC_NA_1)",
                        ip, src_type
                    ),
                    Some(crate::attack_codes::T0814.to_string()),
                ));
            }
        }
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(test)]
    use crate::attack::test_utils::make_input;
    use crate::*;


    #[test]
    fn test_t0855_iec104_control_commands() {
        let mut input = make_input();
        input.deep_parse.insert(
            "10.0.0.40".to_string(),
            DeepParseSnapshot {
                iec104: Some(Iec104Snapshot {
                    role: "master".to_string(),
                    has_control_commands: true,
                    has_reset_process: false,
                    has_interrogation: false,
                }),
                ..Default::default()
            },
        );

        let findings = detect_iec104_attacks(&input);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, Severity::High);
        assert_eq!(findings[0].technique_id, Some(crate::attack_codes::T0855.to_string()));
    }

    #[test]
    fn test_t0816_iec104_reset_process() {
        let mut input = make_input();
        input.deep_parse.insert(
            "10.0.0.41".to_string(),
            DeepParseSnapshot {
                iec104: Some(Iec104Snapshot {
                    role: "master".to_string(),
                    has_control_commands: false,
                    has_reset_process: true,
                    has_interrogation: false,
                }),
                ..Default::default()
            },
        );

        let findings = detect_iec104_attacks(&input);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, Severity::Critical);
        assert_eq!(findings[0].technique_id, Some(crate::attack_codes::T0816.to_string()));
    }
}

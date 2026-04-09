use crate::{AnalysisInput, Finding, FindingType, Severity};

/// EtherNet/IP ATT&CK detections: T0855 (CIP write), T0836 (firmware), T0846 (discovery).
pub(super) fn detect_enip_attacks(input: &AnalysisInput) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (ip, dp) in &input.deep_parse {
        let enip = match &dp.enip {
            Some(e) => e,
            None => continue,
        };

        // T0855: CIP Write or ReadModifyWrite to Assembly object controls I/O data
        if enip.cip_writes_to_assembly {
            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::High,
                format!("CIP write to Assembly object from {}", ip),
                "CIP Write or ReadModifyWrite command targeting an Assembly object was \
                 detected. Assembly objects control I/O data for connected devices and \
                 writes may cause unexpected actuator behavior."
                    .to_string(),
                vec![ip.clone()],
                format!(
                    "Source {} sent CIP Write/ReadModifyWrite to Assembly (class 0x04)",
                    ip
                ),
                Some(crate::attack_codes::T0855.to_string()),
            ));
        }

        // T0836: CIP File class access — firmware upload/download or program file transfer
        if enip.cip_file_access {
            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::Critical,
                format!(
                    "CIP File class access from {} (possible firmware operation)",
                    ip
                ),
                "Access to the CIP File object class (0x37) was detected. File class objects \
                 are used for firmware uploads and program file transfers. Unauthorized access \
                 may indicate firmware modification or intellectual property theft."
                    .to_string(),
                vec![ip.clone()],
                format!("Source {} accessed CIP File object class (0x37)", ip),
                Some(crate::attack_codes::T0836.to_string()),
            ));
        }

        // T0846: ListIdentity from an IT/unknown device — OT network reconnaissance
        if enip.list_identity_requests {
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
                    format!("EtherNet/IP ListIdentity from non-OT device {}", ip),
                    "ListIdentity requests enumerate all EtherNet/IP devices on the network. \
                     This request from an unclassified or IT device may indicate network \
                     reconnaissance of the OT environment."
                        .to_string(),
                    vec![ip.clone()],
                    format!(
                        "Device {} (type: {}) sent EtherNet/IP ListIdentity requests",
                        ip, src_type
                    ),
                    Some(crate::attack_codes::T0846.to_string()),
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
    fn test_t0855_cip_write_assembly() {
        let mut input = make_input();
        input.deep_parse.insert(
            "10.0.0.50".to_string(),
            DeepParseSnapshot {
                enip: Some(EnipSnapshot {
                    role: "scanner".to_string(),
                    cip_writes_to_assembly: true,
                    cip_file_access: false,
                    list_identity_requests: false,
                }),
                ..Default::default()
            },
        );

        let findings = detect_enip_attacks(&input);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, Severity::High);
        assert_eq!(findings[0].technique_id, Some(crate::attack_codes::T0855.to_string()));
    }

    #[test]
    fn test_t0836_cip_file_access() {
        let mut input = make_input();
        input.deep_parse.insert(
            "10.0.0.51".to_string(),
            DeepParseSnapshot {
                enip: Some(EnipSnapshot {
                    role: "scanner".to_string(),
                    cip_writes_to_assembly: false,
                    cip_file_access: true,
                    list_identity_requests: false,
                }),
                ..Default::default()
            },
        );

        let findings = detect_enip_attacks(&input);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, Severity::Critical);
        assert_eq!(findings[0].technique_id, Some(crate::attack_codes::T0836.to_string()));
    }
}

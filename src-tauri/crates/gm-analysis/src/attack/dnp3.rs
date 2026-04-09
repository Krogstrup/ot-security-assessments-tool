use std::collections::HashSet;

use crate::{AnalysisInput, Finding, FindingType, Severity};

/// T0856 — Modify Alarm Settings (DNP3 Unsolicited Response)
///
/// Detects DNP3 unsolicited responses (FC 130) sent to devices
/// that are not known masters. Unsolicited responses from outstations
/// to unknown destinations may indicate alarm suppression or manipulation.
pub(super) fn detect_t0856_dnp3_unsolicited(input: &AnalysisInput) -> Vec<Finding> {
    let mut findings = Vec::new();

    // Build set of known DNP3 master IPs
    let known_masters: HashSet<String> = input
        .deep_parse
        .iter()
        .filter_map(|(ip, dp)| {
            dp.dnp3.as_ref().and_then(|d| {
                if d.role == "master" || d.role == "both" {
                    Some(ip.clone())
                } else {
                    None
                }
            })
        })
        .collect();

    for (ip, dp) in &input.deep_parse {
        let dnp3 = match &dp.dnp3 {
            Some(d) => d,
            None => continue,
        };

        if !dnp3.has_unsolicited {
            continue;
        }

        // Check if unsolicited responses go to unknown masters
        let unknown_targets: Vec<String> = dnp3
            .relationships
            .iter()
            .filter(|r| r.remote_role == "master" && !known_masters.contains(&r.remote_ip))
            .map(|r| r.remote_ip.clone())
            .collect();

        if !unknown_targets.is_empty() {
            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::Medium,
                format!("DNP3 unsolicited response from {} to unknown master", ip),
                "DNP3 unsolicited responses (FC 130) are being sent to devices \
                 not recognized as authorized masters. This could indicate alarm \
                 manipulation or unauthorized data exfiltration."
                    .to_string(),
                std::iter::once(ip.clone())
                    .chain(unknown_targets.iter().cloned())
                    .collect(),
                format!(
                    "Outstation {} sent unsolicited responses to unknown master(s): {}",
                    ip,
                    unknown_targets.join(", ")
                ),
                Some(crate::attack_codes::T0856.to_string()),
            ));
        }

        // Also flag if there are no known masters at all (suspicious standalone unsolicited)
        if known_masters.is_empty() && dnp3.has_unsolicited {
            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::Medium,
                format!("DNP3 unsolicited response with no known masters: {}", ip),
                "DNP3 unsolicited responses detected but no authorized masters \
                 have been identified in the network. All unsolicited traffic \
                 is potentially unauthorized."
                    .to_string(),
                vec![ip.clone()],
                format!(
                    "Device {} sending DNP3 unsolicited responses (FC 130) \
                     but no DNP3 masters detected on network",
                    ip
                ),
                Some(crate::attack_codes::T0856.to_string()),
            ));
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
    fn test_t0856_unsolicited_unknown_master() {
        let mut input = make_input();

        // Outstation sending unsolicited responses
        input.deep_parse.insert(
            "10.0.0.10".to_string(),
            DeepParseSnapshot {
                dnp3: Some(Dnp3Snapshot {
                    role: "outstation".to_string(),
                    has_unsolicited: true,
                    function_codes: vec![FcSnapshot {
                        code: 130,
                        count: 5,
                        is_write: false,
                    }],
                    relationships: vec![RelationshipSnapshot {
                        remote_ip: "192.168.1.50".to_string(),
                        remote_role: "master".to_string(),
                        packet_count: 5,
                    }],
                }),
                ..Default::default()
            },
        );

        let findings = detect_t0856_dnp3_unsolicited(&input);
        assert!(!findings.is_empty());
        assert_eq!(findings[0].technique_id, Some(crate::attack_codes::T0856.to_string()));
    }
}

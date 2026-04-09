use crate::{AnalysisInput, Finding, FindingType, Severity};

/// BACnet ATT&CK detections: T0855, T0856, T0816, T0811.
pub(super) fn detect_bacnet_attacks(input: &AnalysisInput) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (ip, dp) in &input.deep_parse {
        let bacnet = match &dp.bacnet {
            Some(b) => b,
            None => continue,
        };

        // T0855: WriteProperty to output object — directly controls physical actuators
        if bacnet.write_to_output {
            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::High,
                format!("BACnet WriteProperty to output object from {}", ip),
                "A BACnet WriteProperty to an AnalogOutput or BinaryOutput object was \
                 detected. Writing to output objects directly controls physical actuators \
                 such as valves, dampers, and relays in building automation systems."
                    .to_string(),
                vec![ip.clone()],
                format!(
                    "Source {} wrote to BACnet AnalogOutput or BinaryOutput object",
                    ip
                ),
                Some(crate::attack_codes::T0855.to_string()),
            ));
        }

        // T0856: WriteProperty to NotificationClass — suppresses alarms
        if bacnet.write_to_notification_class {
            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::High,
                format!("BACnet alarm suppression from {}", ip),
                "A BACnet WriteProperty to a NotificationClass object was detected. \
                 NotificationClass objects control alarm routing and notification. \
                 Modifying these can suppress alarms, preventing operators from \
                 detecting faults or process anomalies."
                    .to_string(),
                vec![ip.clone()],
                format!(
                    "Source {} modified BACnet NotificationClass object (alarm routing)",
                    ip
                ),
                Some(crate::attack_codes::T0856.to_string()),
            ));
        }

        // T0816: ReinitializeDevice — restarts or restores device to defaults
        if bacnet.reinitialize_device {
            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::High,
                format!("BACnet ReinitializeDevice from {}", ip),
                "A BACnet ReinitializeDevice service was detected. This command can \
                 restart or restore a BACnet device to defaults, causing loss of \
                 control and potentially overwriting operational configuration."
                    .to_string(),
                vec![ip.clone()],
                format!("Source {} sent BACnet ReinitializeDevice command", ip),
                Some(crate::attack_codes::T0816.to_string()),
            ));
        }

        // T0811: DeviceCommunicationControl — disables device communication (loss of view)
        if bacnet.device_communication_control {
            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::High,
                format!("BACnet DeviceCommunicationControl from {}", ip),
                "A BACnet DeviceCommunicationControl service was detected. This command \
                 can disable a device's ability to initiate communications, causing a \
                 denial of view for operators monitoring the building automation system."
                    .to_string(),
                vec![ip.clone()],
                format!(
                    "Source {} sent BACnet DeviceCommunicationControl command",
                    ip
                ),
                Some(crate::attack_codes::T0811.to_string()),
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
    fn test_t0855_bacnet_write_output() {
        let mut input = make_input();
        input.deep_parse.insert(
            "10.0.0.30".to_string(),
            DeepParseSnapshot {
                bacnet: Some(BacnetSnapshot {
                    role: "client".to_string(),
                    write_to_output: true,
                    write_to_notification_class: false,
                    reinitialize_device: false,
                    device_communication_control: false,
                }),
                ..Default::default()
            },
        );

        let findings = detect_bacnet_attacks(&input);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, Severity::High);
        assert_eq!(findings[0].technique_id, Some(crate::attack_codes::T0855.to_string()));
    }

    #[test]
    fn test_t0811_bacnet_comm_ctrl() {
        let mut input = make_input();
        input.deep_parse.insert(
            "10.0.0.31".to_string(),
            DeepParseSnapshot {
                bacnet: Some(BacnetSnapshot {
                    role: "client".to_string(),
                    write_to_output: false,
                    write_to_notification_class: false,
                    reinitialize_device: false,
                    device_communication_control: true,
                }),
                ..Default::default()
            },
        );

        let findings = detect_bacnet_attacks(&input);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, Severity::High);
        assert_eq!(findings[0].technique_id, Some(crate::attack_codes::T0811.to_string()));
    }
}

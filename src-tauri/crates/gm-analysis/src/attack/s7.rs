use crate::{AnalysisInput, Finding, FindingType, Severity};

/// S7comm ATT&CK detections: T0843, T0845, T0816, T0809, T0855.
pub(super) fn detect_s7_attacks(input: &AnalysisInput) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (ip, dp) in &input.deep_parse {
        let s7 = match &dp.s7 {
            Some(s) => s,
            None => continue,
        };

        // T0843: Program download — replaces PLC control logic
        if s7.functions_seen.contains(&"download_start".to_string()) {
            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::Critical,
                format!("S7 program download from {}", ip),
                "An S7 Download Start (function 0x1D) was detected. This initiates a \
                 program download to the PLC, which can replace the control logic and \
                 cause unexpected physical process behavior."
                    .to_string(),
                vec![ip.clone()],
                format!("Source {} initiated S7comm Download Start (FC 0x1D)", ip),
                Some("T0843".to_string()),
            ));
        }

        // T0845: Program upload — reads PLC logic (reconnaissance / IP theft)
        if s7.functions_seen.contains(&"upload_start".to_string()) {
            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::High,
                format!("S7 program upload from {}", ip),
                "An S7 Upload Start (function 0x1A) was detected. This reads the PLC \
                 program logic and may indicate intellectual property theft or \
                 reconnaissance to understand process control before an attack."
                    .to_string(),
                vec![ip.clone()],
                format!("Source {} initiated S7comm Upload Start (FC 0x1A)", ip),
                Some("T0845".to_string()),
            ));
        }

        // T0816: PLC Stop — halts PLC execution
        if s7.functions_seen.contains(&"plc_stop".to_string()) {
            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::Critical,
                format!("S7 PLC Stop command from {}", ip),
                "An S7 PLC Stop command (function 0x29) was detected. This halts PLC \
                 execution, which will cause controlled processes to enter a safe state \
                 or fail, potentially causing loss of control or production disruption."
                    .to_string(),
                vec![ip.clone()],
                format!("Source {} sent S7comm PLC Stop (FC 0x29)", ip),
                Some("T0816".to_string()),
            ));
        }

        // T0809: PI Service — can delete program blocks
        if s7.functions_seen.contains(&"pi_service".to_string()) {
            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::Critical,
                format!("S7 PI Service (possible block delete) from {}", ip),
                "An S7 PI Service command (function 0x28) was detected. This function \
                 can delete program blocks from the PLC, destroying control logic and \
                 requiring full system restoration."
                    .to_string(),
                vec![ip.clone()],
                format!("Source {} sent S7comm PI Service (FC 0x28)", ip),
                Some("T0809".to_string()),
            ));
        }

        // T0855: Write Var — writes directly to PLC memory
        if s7.functions_seen.contains(&"write_var".to_string()) {
            findings.push(Finding::new(
                FindingType::AttackTechnique,
                Severity::High,
                format!("S7 Write Var command from {}", ip),
                "An S7 Write Var command (function 0x05) was detected. This writes values \
                 directly to PLC memory areas (inputs, outputs, merkers, data blocks), \
                 which can cause unauthorized changes to process control variables."
                    .to_string(),
                vec![ip.clone()],
                format!("Source {} sent S7comm Write Var (FC 0x05)", ip),
                Some("T0855".to_string()),
            ));
        }
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    fn make_input() -> AnalysisInput {
        AnalysisInput::default()
    }

    #[test]
    fn test_t0843_s7_download_start() {
        let mut input = make_input();
        input.deep_parse.insert(
            "10.0.0.20".to_string(),
            DeepParseSnapshot {
                s7: Some(S7Snapshot {
                    role: "client".to_string(),
                    functions_seen: vec!["download_start".to_string()],
                }),
                ..Default::default()
            },
        );

        let findings = detect_s7_attacks(&input);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, Severity::Critical);
        assert_eq!(findings[0].technique_id, Some("T0843".to_string()));
    }

    #[test]
    fn test_t0816_s7_plc_stop() {
        let mut input = make_input();
        input.deep_parse.insert(
            "10.0.0.21".to_string(),
            DeepParseSnapshot {
                s7: Some(S7Snapshot {
                    role: "client".to_string(),
                    functions_seen: vec!["plc_stop".to_string()],
                }),
                ..Default::default()
            },
        );

        let findings = detect_s7_attacks(&input);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, Severity::Critical);
        assert_eq!(findings[0].technique_id, Some("T0816".to_string()));
    }
}

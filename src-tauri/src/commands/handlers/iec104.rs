//! IEC 60870-5-104 protocol handler.

use std::collections::{HashMap, HashSet};

use gm_capture::ParsedPacket;
use gm_parsers::{AsduTypeId, DeepParseResult, Iec104Role};

use crate::commands::{DeepParseInfo, Iec104Detail};
use crate::commands::protocol_handler::ProtocolHandler;

/// Accumulates IEC 60870-5-104 state per IP across all packets.
#[derive(Default)]
pub struct Iec104Handler {
    roles: HashMap<String, String>,
    control_commands: HashSet<String>,
    reset_process: HashSet<String>,
    interrogation: HashSet<String>,
}

impl ProtocolHandler for Iec104Handler {
    fn process(&mut self, packet: &ParsedPacket, deep_result: &DeepParseResult) {
        let info = match deep_result {
            DeepParseResult::Iec104(i) => i,
            _ => return,
        };

        let ip = &packet.src_ip;

        let role_str = match info.role {
            Iec104Role::Master => "master",
            Iec104Role::Outstation => "outstation",
            Iec104Role::Unknown => "unknown",
        };
        self.roles.insert(ip.clone(), role_str.to_string());

        if info.is_command {
            self.control_commands.insert(ip.clone());
        }
        if matches!(info.type_id, Some(AsduTypeId::ResetProcess)) {
            self.reset_process.insert(ip.clone());
        }
        if matches!(info.type_id, Some(AsduTypeId::Interrogation)) {
            self.interrogation.insert(ip.clone());
        }
    }

    fn finalize(&self, deep_parse: &mut HashMap<String, DeepParseInfo>) {
        for ip in self.roles.keys() {
            let role = self
                .roles
                .get(ip)
                .cloned()
                .unwrap_or_else(|| "unknown".to_string());
            deep_parse.entry(ip.clone()).or_default().iec104 = Some(Iec104Detail {
                role,
                has_control_commands: self.control_commands.contains(ip),
                has_reset_process: self.reset_process.contains(ip),
                has_interrogation: self.interrogation.contains(ip),
            });
        }
    }
}

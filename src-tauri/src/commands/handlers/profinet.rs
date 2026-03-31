//! PROFINET DCP protocol handler.

use std::collections::HashMap;

use gm_capture::ParsedPacket;
use gm_parsers::{DeepParseResult, ProfinetRole};

use crate::commands::{ProfinetDcpDetail};
use crate::commands::protocol_handler::{ProcessorOutput, ProtocolHandler};

/// Accumulates PROFINET DCP state per IP across all packets.
#[derive(Default)]
pub struct ProfinetDcpHandler {
    roles: HashMap<String, String>,
    device_names: HashMap<String, String>,
}

impl ProtocolHandler for ProfinetDcpHandler {
    fn process(&mut self, packet: &ParsedPacket, deep_result: &DeepParseResult) {
        let info = match deep_result {
            DeepParseResult::ProfinetDcp(i) => i,
            _ => return,
        };

        let ip = &packet.src_ip;

        let role_str = match info.role {
            ProfinetRole::IoDevice => "io_device",
            ProfinetRole::IoController => "io_controller",
            ProfinetRole::IoSupervisor => "io_supervisor",
            ProfinetRole::Unknown => "unknown",
        };
        if role_str != "unknown" {
            self.roles.insert(ip.clone(), role_str.to_string());
        } else {
            self.roles
                .entry(ip.clone())
                .or_insert_with(|| "unknown".to_string());
        }

        if let Some(ref name) = info.device_info.name_of_station {
            if !name.is_empty() {
                self.device_names.insert(ip.clone(), name.clone());
            }
        }
    }

    fn finalize(&self, output: &mut ProcessorOutput) {
        for ip in self.roles.keys() {
            let role = self
                .roles
                .get(ip)
                .cloned()
                .unwrap_or_else(|| "unknown".to_string());
            output.deep_parse.entry(ip.clone()).or_default().profinet_dcp = Some(ProfinetDcpDetail {
                role,
                device_name: self.device_names.get(ip).cloned(),
            });
        }
    }
}

//! EtherNet/IP protocol handler.

use std::collections::{HashMap, HashSet};

use gm_capture::ParsedPacket;
use gm_parsers::{CipClass, CipService, DeepParseResult, EnipCommand, EnipRole};

use crate::commands::protocol_handler::{ProcessorOutput, ProtocolHandler};
use crate::commands::EnipDetail;

/// Accumulates EtherNet/IP state per IP across all packets.
#[derive(Default)]
pub struct EnipHandler {
    roles: HashMap<String, String>,
    cip_writes_to_assembly: HashSet<String>,
    cip_file_access: HashSet<String>,
    list_identity: HashSet<String>,
}

impl ProtocolHandler for EnipHandler {
    fn process(&mut self, packet: &ParsedPacket, deep_result: &DeepParseResult) {
        let info = match deep_result {
            DeepParseResult::Enip(i) => i,
            _ => return,
        };

        let ip = &packet.src_ip;

        let role_str = match info.role {
            EnipRole::Scanner => "scanner",
            EnipRole::Adapter => "adapter",
            EnipRole::Unknown => "unknown",
        };
        self.roles.insert(ip.clone(), role_str.to_string());

        if matches!(info.command, EnipCommand::ListIdentity) && !info.is_response {
            self.list_identity.insert(ip.clone());
        }

        let is_write = matches!(
            info.cip_service,
            Some(CipService::Write) | Some(CipService::ReadModifyWrite)
        );
        let is_assembly = matches!(info.cip_class, Some(CipClass::Assembly));
        if is_write && is_assembly {
            self.cip_writes_to_assembly.insert(ip.clone());
        }

        if matches!(info.cip_class, Some(CipClass::File)) {
            self.cip_file_access.insert(ip.clone());
        }
    }

    fn finalize(&self, output: &mut ProcessorOutput) {
        for ip in self.roles.keys() {
            let role = self
                .roles
                .get(ip)
                .cloned()
                .unwrap_or_else(|| "unknown".to_string());
            output.deep_parse.entry(ip.clone()).or_default().enip = Some(EnipDetail {
                role,
                cip_writes_to_assembly: self.cip_writes_to_assembly.contains(ip),
                cip_file_access: self.cip_file_access.contains(ip),
                list_identity_requests: self.list_identity.contains(ip),
            });
        }
    }
}

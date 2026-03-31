//! S7comm protocol handler.

use std::collections::HashMap;

use gm_capture::ParsedPacket;
use gm_parsers::{DeepParseResult, S7Function, S7Role};

use crate::commands::{S7Detail};
use crate::commands::protocol_handler::{ProcessorOutput, ProtocolHandler};

/// Accumulates S7comm state per IP across all packets.
#[derive(Default)]
pub struct S7Handler {
    roles: HashMap<String, String>,
    functions_seen: HashMap<String, std::collections::HashSet<String>>,
}

impl ProtocolHandler for S7Handler {
    fn process(&mut self, packet: &ParsedPacket, deep_result: &DeepParseResult) {
        let info = match deep_result {
            DeepParseResult::S7(i) => i,
            _ => return,
        };

        let ip = &packet.src_ip;

        let role_str = match info.role {
            S7Role::Client => "client",
            S7Role::Server => "server",
            S7Role::Unknown => "unknown",
        };
        self.roles.insert(ip.clone(), role_str.to_string());

        if let Some(ref function) = info.s7_function {
            let fn_name = match function {
                S7Function::SetupCommunication => "setup_communication",
                S7Function::ReadVar => "read_var",
                S7Function::WriteVar => "write_var",
                S7Function::UploadStart => "upload_start",
                S7Function::Upload => "upload",
                S7Function::UploadEnd => "upload_end",
                S7Function::DownloadStart => "download_start",
                S7Function::Download => "download",
                S7Function::DownloadEnd => "download_end",
                S7Function::PlcStop => "plc_stop",
                S7Function::PiService => "pi_service",
                S7Function::Unknown(_) => "unknown",
            };
            self.functions_seen
                .entry(ip.clone())
                .or_default()
                .insert(fn_name.to_string());
        }
    }

    fn finalize(&self, output: &mut ProcessorOutput) {
        for ip in self.roles.keys() {
            let role = self
                .roles
                .get(ip)
                .cloned()
                .unwrap_or_else(|| "unknown".to_string());
            let mut functions_seen: Vec<String> = self
                .functions_seen
                .get(ip)
                .map(|s| s.iter().cloned().collect())
                .unwrap_or_default();
            functions_seen.sort();
            output.deep_parse.entry(ip.clone()).or_default().s7 = Some(S7Detail {
                role,
                functions_seen,
            });
        }
    }
}

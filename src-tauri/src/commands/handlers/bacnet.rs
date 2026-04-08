//! BACnet protocol handler.

use std::collections::{HashMap, HashSet};

use gm_capture::ParsedPacket;
use gm_parsers::{BacnetObjectType, BacnetRole, BacnetService, DeepParseResult};

use crate::commands::protocol_handler::{ProcessorOutput, ProtocolHandler};
use crate::commands::BacnetDetail;

/// Accumulates BACnet state per IP across all packets.
#[derive(Default)]
pub struct BacnetHandler {
    roles: HashMap<String, String>,
    write_to_output: HashSet<String>,
    write_to_notification_class: HashSet<String>,
    reinitialize: HashSet<String>,
    device_comm_ctrl: HashSet<String>,
}

impl ProtocolHandler for BacnetHandler {
    fn process(&mut self, packet: &ParsedPacket, deep_result: &DeepParseResult) {
        let info = match deep_result {
            DeepParseResult::Bacnet(i) => i,
            _ => return,
        };

        let ip = &packet.src_ip;

        let role_str = match info.role {
            BacnetRole::Client => "client",
            BacnetRole::Server => "server",
            BacnetRole::Unknown => "unknown",
        };
        self.roles.insert(ip.clone(), role_str.to_string());

        match info.service {
            Some(BacnetService::WriteProperty) | Some(BacnetService::WritePropertyMultiple) => {
                match info.object_type {
                    Some(BacnetObjectType::AnalogOutput) | Some(BacnetObjectType::BinaryOutput) => {
                        self.write_to_output.insert(ip.clone());
                    }
                    Some(BacnetObjectType::NotificationClass) => {
                        self.write_to_notification_class.insert(ip.clone());
                    }
                    _ => {}
                }
            }
            Some(BacnetService::ReinitializeDevice) => {
                self.reinitialize.insert(ip.clone());
            }
            Some(BacnetService::DeviceCommunicationControl) => {
                self.device_comm_ctrl.insert(ip.clone());
            }
            _ => {}
        }
    }

    fn finalize(&self, output: &mut ProcessorOutput) {
        for ip in self.roles.keys() {
            let role = self
                .roles
                .get(ip)
                .cloned()
                .unwrap_or_else(|| "unknown".to_string());
            output.deep_parse.entry(ip.clone()).or_default().bacnet = Some(BacnetDetail {
                role,
                write_to_output: self.write_to_output.contains(ip),
                write_to_notification_class: self.write_to_notification_class.contains(ip),
                reinitialize_device: self.reinitialize.contains(ip),
                device_communication_control: self.device_comm_ctrl.contains(ip),
            });
        }
    }
}

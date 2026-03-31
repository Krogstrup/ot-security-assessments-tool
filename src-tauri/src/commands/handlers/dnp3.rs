//! DNP3 protocol handler.

use std::collections::{HashMap, HashSet};

use gm_capture::ParsedPacket;
use gm_parsers::{dnp3_function_code_name, DeepParseResult, Dnp3Role};

use crate::commands::{Dnp3Detail, Dnp3Relationship, FunctionCodeStat};
use crate::commands::protocol_handler::{ProcessorOutput, ProtocolHandler};

/// Accumulates DNP3 state per IP across all packets.
#[derive(Default)]
pub struct Dnp3Handler {
    fc_counts: HashMap<String, HashMap<u8, u64>>,
    addresses: HashMap<String, HashSet<u16>>,
    roles: HashMap<String, HashSet<String>>,
    unsolicited: HashMap<String, bool>,
    relationships: HashMap<String, HashMap<String, (String, u64)>>,
}

impl ProtocolHandler for Dnp3Handler {
    fn process(&mut self, packet: &ParsedPacket, deep_result: &DeepParseResult) {
        let info = match deep_result {
            DeepParseResult::Dnp3(i) => i,
            _ => return,
        };

        let ip = &packet.src_ip;

        if let Some(fc) = info.function_code {
            *self
                .fc_counts
                .entry(ip.clone())
                .or_default()
                .entry(fc)
                .or_insert(0) += 1;
        }

        self.addresses
            .entry(ip.clone())
            .or_default()
            .insert(info.source_address);

        let role_str = match info.role {
            Dnp3Role::Master => "master",
            Dnp3Role::Outstation => "outstation",
            Dnp3Role::Unknown => "unknown",
        };
        self.roles
            .entry(ip.clone())
            .or_default()
            .insert(role_str.to_string());

        if info.is_unsolicited {
            self.unsolicited.insert(ip.clone(), true);
        }

        let remote_role = match info.role {
            Dnp3Role::Master => "outstation",
            Dnp3Role::Outstation => "master",
            Dnp3Role::Unknown => "unknown",
        };
        let rel = self
            .relationships
            .entry(ip.clone())
            .or_default()
            .entry(packet.dst_ip.clone())
            .or_insert_with(|| (remote_role.to_string(), 0));
        rel.1 += 1;
    }

    fn finalize(&self, output: &mut ProcessorOutput) {
        let all_ips: HashSet<String> = self
            .fc_counts
            .keys()
            .chain(self.roles.keys())
            .cloned()
            .collect();

        for ip in &all_ips {
            let role = self
                .roles
                .get(ip)
                .map(|roles| {
                    if roles.contains("master") && roles.contains("outstation") {
                        "both"
                    } else if roles.contains("master") {
                        "master"
                    } else if roles.contains("outstation") {
                        "outstation"
                    } else {
                        "unknown"
                    }
                })
                .unwrap_or("unknown")
                .to_string();

            let mut addresses: Vec<u16> = self
                .addresses
                .get(ip)
                .map(|s| s.iter().copied().collect())
                .unwrap_or_default();
            addresses.sort();

            let function_codes: Vec<FunctionCodeStat> = self
                .fc_counts
                .get(ip)
                .map(|fc_map| {
                    let mut fcs: Vec<FunctionCodeStat> = fc_map
                        .iter()
                        .map(|(&code, &count)| FunctionCodeStat {
                            code,
                            name: dnp3_function_code_name(code).to_string(),
                            count,
                            is_write: matches!(code, 2..=6),
                        })
                        .collect();
                    fcs.sort_by(|a, b| b.count.cmp(&a.count));
                    fcs
                })
                .unwrap_or_default();

            let has_unsolicited = self.unsolicited.get(ip).copied().unwrap_or(false);

            let relationships: Vec<Dnp3Relationship> = self
                .relationships
                .get(ip)
                .map(|rel_map| {
                    rel_map
                        .iter()
                        .map(|(remote_ip, (remote_role, pkt_count))| Dnp3Relationship {
                            remote_ip: remote_ip.clone(),
                            remote_role: remote_role.clone(),
                            packet_count: *pkt_count,
                        })
                        .collect()
                })
                .unwrap_or_default();

            output.deep_parse.entry(ip.clone()).or_default().dnp3 = Some(Dnp3Detail {
                role,
                addresses,
                function_codes,
                has_unsolicited,
                relationships,
            });
        }
    }
}

//! Modbus protocol handler.

use std::collections::{HashMap, HashSet};

use gm_capture::ParsedPacket;
use gm_parsers::{modbus_function_code_name, DeepParseResult, ModbusRole};

use crate::commands::{
    FunctionCodeStat, ModbusDetail, ModbusDeviceIdInfo, ModbusRelationship,
    PollingInterval, RegisterRangeInfo,
};
use crate::commands::protocol_handler::{ProcessorOutput, ProtocolHandler};

/// Accumulates Modbus state per IP across all packets.
#[derive(Default)]
pub struct ModbusHandler {
    fc_counts: HashMap<String, HashMap<u8, u64>>,
    unit_ids: HashMap<String, HashSet<u8>>,
    #[allow(clippy::type_complexity)]
    register_ranges: HashMap<String, HashMap<(u16, u16, String), u64>>,
    roles: HashMap<String, HashSet<String>>,
    device_ids: HashMap<String, gm_parsers::ModbusDeviceId>,
    #[allow(clippy::type_complexity)]
    relationships: HashMap<String, HashMap<String, (String, HashSet<u8>, u64)>>,
    polling_timestamps: HashMap<(String, String, u8, u8), Vec<f64>>,
}

impl ProtocolHandler for ModbusHandler {
    fn process(&mut self, packet: &ParsedPacket, deep_result: &DeepParseResult) {
        let info = match deep_result {
            DeepParseResult::Modbus(i) => i,
            _ => return,
        };

        let ts_epoch = packet.timestamp.timestamp() as f64
            + packet.timestamp.timestamp_subsec_millis() as f64 / 1000.0;

        let ip_for_fc = &packet.src_ip;

        *self
            .fc_counts
            .entry(ip_for_fc.clone())
            .or_default()
            .entry(info.function_code)
            .or_insert(0) += 1;

        self.unit_ids
            .entry(ip_for_fc.clone())
            .or_default()
            .insert(info.unit_id);

        let role_str = match info.role {
            ModbusRole::Master => "master",
            ModbusRole::Slave => "slave",
            ModbusRole::Unknown => "unknown",
        };
        self.roles
            .entry(ip_for_fc.clone())
            .or_default()
            .insert(role_str.to_string());

        if let Some(ref range) = info.register_range {
            let reg_type = format!("{:?}", range.register_type).to_lowercase();
            *self
                .register_ranges
                .entry(ip_for_fc.clone())
                .or_default()
                .entry((range.start, range.count, reg_type))
                .or_insert(0) += 1;
        }

        if let Some(ref dev_id) = info.device_id {
            self.device_ids.insert(packet.src_ip.clone(), dev_id.clone());
        }

        let (local_ip, remote_ip, remote_role) = match info.role {
            ModbusRole::Master => (&packet.src_ip, &packet.dst_ip, "slave"),
            ModbusRole::Slave => (&packet.src_ip, &packet.dst_ip, "master"),
            ModbusRole::Unknown => (&packet.src_ip, &packet.dst_ip, "unknown"),
        };
        let rel = self
            .relationships
            .entry(local_ip.clone())
            .or_default()
            .entry(remote_ip.clone())
            .or_insert_with(|| (remote_role.to_string(), HashSet::new(), 0));
        rel.1.insert(info.unit_id);
        rel.2 += 1;

        if info.role == ModbusRole::Master && !info.is_exception {
            let key = (
                packet.src_ip.clone(),
                packet.dst_ip.clone(),
                info.function_code,
                info.unit_id,
            );
            self.polling_timestamps.entry(key).or_default().push(ts_epoch);
        }
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
                    if roles.contains("master") && roles.contains("slave") {
                        "both"
                    } else if roles.contains("master") {
                        "master"
                    } else if roles.contains("slave") {
                        "slave"
                    } else {
                        "unknown"
                    }
                })
                .unwrap_or("unknown")
                .to_string();

            let mut unit_ids: Vec<u8> = self
                .unit_ids
                .get(ip)
                .map(|s| s.iter().copied().collect())
                .unwrap_or_default();
            unit_ids.sort();

            let function_codes: Vec<FunctionCodeStat> = self
                .fc_counts
                .get(ip)
                .map(|fc_map| {
                    let mut fcs: Vec<FunctionCodeStat> = fc_map
                        .iter()
                        .map(|(&code, &count)| FunctionCodeStat {
                            code,
                            name: modbus_function_code_name(code).to_string(),
                            count,
                            is_write: matches!(code, 5 | 6 | 15 | 16 | 22 | 23),
                        })
                        .collect();
                    fcs.sort_by(|a, b| b.count.cmp(&a.count));
                    fcs
                })
                .unwrap_or_default();

            let register_ranges: Vec<RegisterRangeInfo> = self
                .register_ranges
                .get(ip)
                .map(|range_map| {
                    let mut ranges: Vec<RegisterRangeInfo> = range_map
                        .iter()
                        .map(|((start, count, reg_type), &access_count)| RegisterRangeInfo {
                            start: *start,
                            count: *count,
                            register_type: reg_type.clone(),
                            access_count,
                        })
                        .collect();
                    ranges.sort_by(|a, b| a.start.cmp(&b.start));
                    ranges
                })
                .unwrap_or_default();

            let device_id = self.device_ids.get(ip).map(|d| ModbusDeviceIdInfo {
                vendor_name: d.vendor_name.clone(),
                product_code: d.product_code.clone(),
                revision: d.revision.clone(),
                vendor_url: d.vendor_url.clone(),
                product_name: d.product_name.clone(),
                model_name: d.model_name.clone(),
            });

            let relationships: Vec<ModbusRelationship> = self
                .relationships
                .get(ip)
                .map(|rel_map| {
                    rel_map
                        .iter()
                        .map(|(remote_ip, (remote_role, unit_id_set, pkt_count))| {
                            let mut uids: Vec<u8> = unit_id_set.iter().copied().collect();
                            uids.sort();
                            ModbusRelationship {
                                remote_ip: remote_ip.clone(),
                                remote_role: remote_role.clone(),
                                unit_ids: uids,
                                packet_count: *pkt_count,
                            }
                        })
                        .collect()
                })
                .unwrap_or_default();

            let mut polling_intervals: Vec<PollingInterval> = Vec::new();
            for ((src, dst, fc, uid), timestamps) in &self.polling_timestamps {
                if src == ip && timestamps.len() >= 3 {
                    let mut sorted_ts = timestamps.clone();
                    sorted_ts
                        .sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

                    let intervals: Vec<f64> = sorted_ts
                        .windows(2)
                        .map(|w| (w[1] - w[0]) * 1000.0)
                        .filter(|&i| i > 0.0 && i < 60_000.0)
                        .collect();

                    if intervals.len() >= 2 {
                        let sum: f64 = intervals.iter().sum();
                        let avg = sum / intervals.len() as f64;
                        let min = intervals.iter().cloned().fold(f64::MAX, f64::min);
                        let max = intervals.iter().cloned().fold(f64::MIN, f64::max);

                        polling_intervals.push(PollingInterval {
                            remote_ip: dst.clone(),
                            unit_id: Some(*uid),
                            function_code: *fc,
                            avg_interval_ms: (avg * 10.0).round() / 10.0,
                            min_interval_ms: (min * 10.0).round() / 10.0,
                            max_interval_ms: (max * 10.0).round() / 10.0,
                            sample_count: intervals.len() as u64,
                        });
                    }
                }
            }

            output.deep_parse.entry(ip.clone()).or_default().modbus = Some(ModbusDetail {
                role,
                unit_ids,
                function_codes,
                register_ranges,
                device_id,
                relationships,
                polling_intervals,
            });
        }
    }
}

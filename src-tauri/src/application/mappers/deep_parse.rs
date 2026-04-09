//! Canonical mapper: `InventoryState` deep_parse_info → `DeepParseSnapshot` map.
//!
//! Single source of truth consumed by:
//!   - `build_analysis_input()` in commands/analysis_input_builder.rs
//!   - malware detection in commands/analysis.rs
//!
//! # Layering note
//! Borrows `InventoryState` from the commands module — see snapshots.rs for context.

use std::collections::HashMap;

use gm_analysis::{
    BacnetSnapshot, DeepParseSnapshot, Dnp3Snapshot, EnipSnapshot, FcSnapshot, Iec104Snapshot,
    ModbusSnapshot, PollingSnapshot, ProfinetDcpSnapshot, RelationshipSnapshot, S7Snapshot,
};

use crate::commands::InventoryState;

/// Build a `DeepParseSnapshot` map from all protocol deep-parse data in inventory.
///
/// Maps every IP in `inventory.deep_parse_info` to its full 7-protocol snapshot.
/// Returns an empty map when no deep-parse data has been collected.
pub fn build_deep_parse_snapshot_map(
    inventory: &InventoryState,
) -> HashMap<String, DeepParseSnapshot> {
    inventory
        .deep_parse_info
        .iter()
        .map(|(ip, dp)| {
            let modbus = dp.modbus.as_ref().map(|m| ModbusSnapshot {
                role: m.role.clone(),
                unit_ids: m.unit_ids.clone(),
                function_codes: m
                    .function_codes
                    .iter()
                    .map(|fc| FcSnapshot {
                        code: fc.code,
                        count: fc.count,
                        is_write: fc.is_write,
                    })
                    .collect(),
                relationships: m
                    .relationships
                    .iter()
                    .map(|r| RelationshipSnapshot {
                        remote_ip: r.remote_ip.clone(),
                        remote_role: r.remote_role.clone(),
                        packet_count: r.packet_count,
                    })
                    .collect(),
                polling_intervals: m
                    .polling_intervals
                    .iter()
                    .map(|pi| PollingSnapshot {
                        remote_ip: pi.remote_ip.clone(),
                        function_code: pi.function_code,
                        avg_interval_ms: pi.avg_interval_ms,
                        min_interval_ms: pi.min_interval_ms,
                        max_interval_ms: pi.max_interval_ms,
                        sample_count: pi.sample_count,
                    })
                    .collect(),
            });

            let dnp3 = dp.dnp3.as_ref().map(|d| Dnp3Snapshot {
                role: d.role.clone(),
                has_unsolicited: d.has_unsolicited,
                function_codes: d
                    .function_codes
                    .iter()
                    .map(|fc| FcSnapshot {
                        code: fc.code,
                        count: fc.count,
                        is_write: fc.is_write,
                    })
                    .collect(),
                relationships: d
                    .relationships
                    .iter()
                    .map(|r| RelationshipSnapshot {
                        remote_ip: r.remote_ip.clone(),
                        remote_role: r.remote_role.clone(),
                        packet_count: r.packet_count,
                    })
                    .collect(),
            });

            let enip = dp.enip.as_ref().map(|e| EnipSnapshot {
                role: e.role.clone(),
                cip_writes_to_assembly: e.cip_writes_to_assembly,
                cip_file_access: e.cip_file_access,
                list_identity_requests: e.list_identity_requests,
            });

            let s7 = dp.s7.as_ref().map(|s| S7Snapshot {
                role: s.role.clone(),
                functions_seen: s.functions_seen.clone(),
            });

            let bacnet = dp.bacnet.as_ref().map(|b| BacnetSnapshot {
                role: b.role.clone(),
                write_to_output: b.write_to_output,
                write_to_notification_class: b.write_to_notification_class,
                reinitialize_device: b.reinitialize_device,
                device_communication_control: b.device_communication_control,
            });

            let iec104 = dp.iec104.as_ref().map(|i| Iec104Snapshot {
                role: i.role.clone(),
                has_control_commands: i.has_control_commands,
                has_reset_process: i.has_reset_process,
                has_interrogation: i.has_interrogation,
            });

            let profinet_dcp = dp.profinet_dcp.as_ref().map(|p| ProfinetDcpSnapshot {
                role: p.role.clone(),
            });

            (
                ip.clone(),
                DeepParseSnapshot {
                    modbus,
                    dnp3,
                    enip,
                    s7,
                    bacnet,
                    iec104,
                    profinet_dcp,
                },
            )
        })
        .collect()
}

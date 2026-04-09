//! Deep-parse queries: per-connection packets, per-device deep parse info,
//! and aggregated function-code statistics.

use std::collections::HashMap;

use gm_parsers::{DeepParseInfo, FunctionCodeStat};
use gm_types::PacketSummary;

/// Get packet summaries for a specific connection (for the connection tree detail view).
///
/// Already capped at 1000 per connection during ingestion (see processor.rs).
pub fn get_connection_packets(
    connection_id: String,
    packet_summaries: &HashMap<String, Vec<PacketSummary>>,
) -> Result<Vec<PacketSummary>, String> {
    Ok(packet_summaries
        .get(&connection_id)
        .cloned()
        .unwrap_or_default())
}

/// Get deep parse information for a specific device by IP address.
///
/// Returns Modbus/DNP3 details including function codes, unit IDs,
/// register ranges, device identification, and polling intervals.
pub fn get_deep_parse_info(
    ip_address: String,
    deep_parse_info: &HashMap<String, DeepParseInfo>,
) -> Result<Option<DeepParseInfo>, String> {
    Ok(deep_parse_info.get(&ip_address).cloned())
}

/// Get function code distribution across all protocols.
///
/// Returns aggregated function code stats for the protocol stats view,
/// showing which function codes are most used across the network.
pub fn get_function_code_stats(
    deep_parse_info: &HashMap<String, DeepParseInfo>,
) -> Result<HashMap<String, Vec<FunctionCodeStat>>, String> {
    let mut modbus_fcs: HashMap<u8, u64> = HashMap::new();
    let mut dnp3_fcs: HashMap<u8, u64> = HashMap::new();

    for info in deep_parse_info.values() {
        if let Some(ref modbus) = info.modbus {
            for fc in &modbus.function_codes {
                *modbus_fcs.entry(fc.code).or_insert(0) += fc.count;
            }
        }
        if let Some(ref dnp3) = info.dnp3 {
            for fc in &dnp3.function_codes {
                *dnp3_fcs.entry(fc.code).or_insert(0) += fc.count;
            }
        }
    }

    let mut result: HashMap<String, Vec<FunctionCodeStat>> = HashMap::new();

    if !modbus_fcs.is_empty() {
        let mut fcs: Vec<FunctionCodeStat> = modbus_fcs
            .into_iter()
            .map(|(code, count)| FunctionCodeStat {
                code,
                name: gm_parsers::modbus_function_code_name(code).to_string(),
                count,
                is_write: matches!(code, 5 | 6 | 15 | 16 | 22 | 23),
            })
            .collect();
        fcs.sort_by(|a, b| b.count.cmp(&a.count));
        result.insert("modbus".to_string(), fcs);
    }

    if !dnp3_fcs.is_empty() {
        let mut fcs: Vec<FunctionCodeStat> = dnp3_fcs
            .into_iter()
            .map(|(code, count)| FunctionCodeStat {
                code,
                name: gm_parsers::dnp3_function_code_name(code).to_string(),
                count,
                is_write: matches!(code, 2..=6),
            })
            .collect();
        fcs.sort_by(|a, b| b.count.cmp(&a.count));
        result.insert("dnp3".to_string(), fcs);
    }

    Ok(result)
}

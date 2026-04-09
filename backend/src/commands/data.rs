//! Adapter layer for data-query endpoints.
//!
//! Commands in this module own lock access and pass immutable snapshots into
//! application-layer query functions.

use std::collections::HashMap;

use gm_parsers::{DeepParseInfo, FunctionCodeStat};
use gm_topology::TopologyGraph;
use gm_types::{PacketSummary, ProtocolStatInfo};

use crate::application::queries::data;

use super::{support::read_state, AppState};

pub use data::{AssetPage, AssetSortBy, ConnectionPage, ConnectionSortBy, DataCounts, ProtocolStatsSortBy};
pub use crate::application::queries::data::timeline::TimelineRange;

pub fn get_topology(state: &AppState) -> Result<TopologyGraph, String> {
    let capture = read_state(&state.capture, "capture")?;
    data::get_topology(&capture.topology)
}

pub fn get_assets(
    state: &AppState,
    page: Option<usize>,
    page_size: Option<usize>,
    sort_by: Option<AssetSortBy>,
) -> Result<AssetPage, String> {
    let capture = read_state(&state.capture, "capture")?;
    let inventory = read_state(&state.inventory, "inventory")?;
    data::get_assets(&inventory.assets, &capture.connections, page, page_size, sort_by)
}

pub fn get_connections(
    state: &AppState,
    page: Option<usize>,
    page_size: Option<usize>,
    sort_by: Option<ConnectionSortBy>,
) -> Result<ConnectionPage, String> {
    let capture = read_state(&state.capture, "capture")?;
    data::get_connections(&capture.connections, page, page_size, sort_by)
}

pub fn get_data_counts(state: &AppState) -> Result<DataCounts, String> {
    let capture = read_state(&state.capture, "capture")?;
    let inventory = read_state(&state.inventory, "inventory")?;
    data::get_data_counts(&inventory.assets, &capture.connections)
}

pub fn get_protocol_stats(
    state: &AppState,
    sort_by: Option<ProtocolStatsSortBy>,
) -> Result<Vec<ProtocolStatInfo>, String> {
    let capture = read_state(&state.capture, "capture")?;
    data::get_protocol_stats(&capture.connections, sort_by)
}

pub fn get_connection_packets(
    connection_id: String,
    state: &AppState,
) -> Result<Vec<PacketSummary>, String> {
    let capture = read_state(&state.capture, "capture")?;
    data::get_connection_packets(connection_id, &capture.packet_summaries)
}

pub fn get_deep_parse_info(
    ip_address: String,
    state: &AppState,
) -> Result<Option<DeepParseInfo>, String> {
    let inventory = read_state(&state.inventory, "inventory")?;
    data::get_deep_parse_info(ip_address, &inventory.deep_parse_info)
}

pub fn get_function_code_stats(
    state: &AppState,
) -> Result<HashMap<String, Vec<FunctionCodeStat>>, String> {
    let inventory = read_state(&state.inventory, "inventory")?;
    data::get_function_code_stats(&inventory.deep_parse_info)
}

pub fn get_timeline_range(state: &AppState) -> Result<TimelineRange, String> {
    let capture = read_state(&state.capture, "capture")?;
    data::get_timeline_range(&capture.connections)
}

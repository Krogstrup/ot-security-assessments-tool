//! Data query use-cases: topology, paging, protocol stats, deep-parse, timeline.

pub mod deep_parse;
pub mod paging;
pub mod protocol_stats;
pub mod timeline;
pub mod topology;

// ─── Public re-exports ────────────────────────────────────────────────────────
pub use deep_parse::{get_connection_packets, get_deep_parse_info, get_function_code_stats};
pub use paging::{
    get_assets, get_connections, get_data_counts, AssetPage, AssetSortBy, ConnectionPage,
    ConnectionSortBy, DataCounts,
};
pub use protocol_stats::{
    get_protocol_stats, protocol_stats_from_connections, ProtocolStatsSortBy,
};
pub use timeline::get_timeline_range;
pub use topology::get_topology;

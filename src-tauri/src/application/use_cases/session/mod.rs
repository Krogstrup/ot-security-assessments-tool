//! Session use-case: save/load/list/delete sessions, asset updates,
//! ZIP archive import/export, and DB ↔ domain-type mappers.

pub mod archive;
pub mod asset_updates;
pub mod crud;
pub mod mappers;

// ─── Public re-exports ────────────────────────────────────────────────────────
pub use archive::{export_session_archive, import_session_archive};
pub use asset_updates::{bulk_update_assets, update_asset};
pub use crud::{delete_session, list_sessions, load_session, save_session};

// ─── Public types (re-exported from commands::session) ───────────────────────
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::commands::DeepParseInfo;

/// Session info returned to the frontend.
#[derive(Debug, Clone, Serialize)]
pub struct SessionInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
    pub asset_count: i64,
    pub connection_count: i64,
}

/// Partial updates for an asset (from the frontend edit form).
#[derive(Debug, Clone, Deserialize)]
pub struct AssetUpdate {
    pub device_type: Option<String>,
    pub hostname: Option<String>,
    pub notes: Option<String>,
    pub purdue_level: Option<u8>,
    pub tags: Option<Vec<String>>,
}

/// Session metadata stored as JSON in the database.
#[derive(Debug, Serialize, Deserialize)]
pub(super) struct SessionMetadata {
    pub(super) deep_parse_info: HashMap<String, DeepParseInfo>,
    pub(super) imported_files: Vec<String>,
}

// ─── Shared helpers (used by crud + archive) ──────────────────────────────────

use gm_db::{Database, SessionRow};
use gm_topology::{TopologyBuilder, TopologyGraph};

use crate::commands::{
    support::{mutex_state, write_state},
    AppState, AssetInfo, ConnectionInfo, SessionState,
};

pub(super) const DATABASE_NOT_AVAILABLE: &str = "Database not available";

pub(super) fn db_from_session(session: &SessionState) -> Result<&Database, String> {
    session
        .db
        .as_ref()
        .ok_or_else(|| DATABASE_NOT_AVAILABLE.to_string())
}

pub(super) fn parse_session_metadata(metadata: &str) -> SessionMetadata {
    serde_json::from_str(metadata).unwrap_or(SessionMetadata {
        deep_parse_info: HashMap::new(),
        imported_files: Vec::new(),
    })
}

pub(super) fn build_topology_from_connections(connections: &[ConnectionInfo]) -> TopologyGraph {
    let mut topo_builder = TopologyBuilder::new();
    for conn in connections {
        let protocol = gm_parsers::IcsProtocol::from_name(&conn.protocol);
        topo_builder.add_connection(
            &conn.src_ip,
            &conn.dst_ip,
            conn.src_mac.as_deref(),
            conn.dst_mac.as_deref(),
            protocol,
            conn.byte_count,
        );
    }
    topo_builder.snapshot()
}

pub(super) fn apply_loaded_session_state(
    state: &AppState,
    session_id: String,
    session_name: String,
    topology: TopologyGraph,
    connections: Vec<ConnectionInfo>,
    assets: Vec<AssetInfo>,
    metadata: SessionMetadata,
) -> Result<(), String> {
    {
        let mut cap = write_state(&state.capture, "capture")?;
        cap.topology = topology;
        cap.connections = connections;
        cap.packet_summaries = HashMap::new();
        cap.imported_files = metadata.imported_files;
    }
    {
        let mut inv = write_state(&state.inventory, "inventory")?;
        inv.assets = assets;
        inv.deep_parse_info = metadata.deep_parse_info;
    }
    {
        let mut sess = mutex_state(&state.session, "session")?;
        sess.current_session_id = Some(session_id);
        sess.current_session_name = Some(session_name);
    }
    Ok(())
}

pub(super) fn session_info_from_row(row: SessionRow) -> SessionInfo {
    SessionInfo {
        id: row.id,
        name: row.name,
        description: row.description,
        created_at: row.created_at,
        updated_at: row.updated_at,
        asset_count: row.asset_count,
        connection_count: row.connection_count,
    }
}

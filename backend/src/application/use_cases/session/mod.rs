//! Session use-case: save/load/list/delete sessions, asset updates,
//! ZIP archive import/export, and DB ↔ domain-type mappers.

use std::fmt;

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

use gm_parsers::DeepParseInfo;

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
pub struct SessionMetadata {
    pub deep_parse_info: HashMap<String, DeepParseInfo>,
    pub imported_files: Vec<String>,
}

// ─── Shared helpers (used by crud + archive) ──────────────────────────────────

use gm_db::{Database, SessionRow};
use gm_topology::{TopologyBuilder, TopologyGraph};
use gm_types::{AssetInfo, ConnectionInfo};

pub(super) const DATABASE_NOT_AVAILABLE: &str = "Database not available";

#[derive(Debug)]
pub enum SessionUseCaseError {
    DatabaseNotAvailable,
    Database(gm_db::DbError),
    Io(std::io::Error),
    Serialization(serde_json::Error),
    Archive(zip::result::ZipError),
    InvalidInput(String),
}

impl SessionUseCaseError {
    pub fn invalid_input(message: impl Into<String>) -> Self {
        SessionUseCaseError::InvalidInput(message.into())
    }
}

impl fmt::Display for SessionUseCaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SessionUseCaseError::DatabaseNotAvailable => write!(f, "{DATABASE_NOT_AVAILABLE}"),
            SessionUseCaseError::Database(err) => write!(f, "{err}"),
            SessionUseCaseError::Io(err) => write!(f, "{err}"),
            SessionUseCaseError::Serialization(err) => write!(f, "{err}"),
            SessionUseCaseError::Archive(err) => write!(f, "{err}"),
            SessionUseCaseError::InvalidInput(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for SessionUseCaseError {}

impl From<gm_db::DbError> for SessionUseCaseError {
    fn from(value: gm_db::DbError) -> Self {
        SessionUseCaseError::Database(value)
    }
}

impl From<std::io::Error> for SessionUseCaseError {
    fn from(value: std::io::Error) -> Self {
        SessionUseCaseError::Io(value)
    }
}

impl From<serde_json::Error> for SessionUseCaseError {
    fn from(value: serde_json::Error) -> Self {
        SessionUseCaseError::Serialization(value)
    }
}

impl From<zip::result::ZipError> for SessionUseCaseError {
    fn from(value: zip::result::ZipError) -> Self {
        SessionUseCaseError::Archive(value)
    }
}

pub(super) fn db_or_error(db: Option<&Database>) -> Result<&Database, SessionUseCaseError> {
    db.ok_or(SessionUseCaseError::DatabaseNotAvailable)
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

/// Fully materialized session payload for adapter-owned state application.
pub struct LoadedSessionData {
    pub session_id: String,
    pub session_name: String,
    pub topology: TopologyGraph,
    pub connections: Vec<ConnectionInfo>,
    pub assets: Vec<AssetInfo>,
    pub metadata: SessionMetadata,
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

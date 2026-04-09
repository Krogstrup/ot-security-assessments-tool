//! Session CRUD: save, load, list, delete.

use std::collections::HashMap;

use gm_db::Database;
use gm_parsers::DeepParseInfo;
use gm_types::{AssetInfo, ConnectionInfo};

use super::{
    build_topology_from_connections, db_or_error,
    mappers::{asset_info_to_row, connection_info_to_row, row_to_asset_info, row_to_connection_info},
    parse_session_metadata, session_info_from_row, LoadedSessionData, SessionInfo, SessionMetadata,
};

pub struct LoadSessionResult {
    pub info: SessionInfo,
    pub data: LoadedSessionData,
}

/// Save the current state as a named session.
pub fn save_session(
    name: String,
    description: Option<String>,
    db: Option<&Database>,
    current_project_id: Option<i64>,
    connections: &[ConnectionInfo],
    assets: &[AssetInfo],
    deep_parse_info: &HashMap<String, DeepParseInfo>,
    imported_files: &[String],
) -> Result<SessionInfo, String> {
    let db = db_or_error(db)?;

    let session_id = uuid::Uuid::new_v4().to_string();
    let desc = description.unwrap_or_default();
    let metadata = SessionMetadata {
        deep_parse_info: deep_parse_info.clone(),
        imported_files: imported_files.to_vec(),
    };
    let metadata_json = serde_json::to_string(&metadata).map_err(|e| e.to_string())?;

    let session_row = db
        .create_session(&session_id, &name, &desc, &metadata_json)
        .map_err(|e| e.to_string())?;

    for asset in assets {
        let row = asset_info_to_row(asset, &session_id);
        db.insert_asset(&row).map_err(|e| e.to_string())?;
    }

    for conn in connections {
        let row = connection_info_to_row(conn, &session_id);
        db.insert_connection(&row).map_err(|e| e.to_string())?;
    }

    db.update_session_counts(&session_id, assets.len() as i64, connections.len() as i64)
        .map_err(|e| e.to_string())?;

    if let Some(project_id) = current_project_id {
        db.assign_session_to_project(&session_id, project_id)
            .map_err(|e| e.to_string())?;
    }

    let mut info = session_info_from_row(session_row);
    info.asset_count = assets.len() as i64;
    info.connection_count = connections.len() as i64;
    Ok(info)
}

/// Load a session by ID and return runtime payload for adapter application.
pub fn load_session(session_id: String, db: Option<&Database>) -> Result<LoadSessionResult, String> {
    let db = db_or_error(db)?;

    let session_row = db.get_session(&session_id).map_err(|e| e.to_string())?;
    let metadata = parse_session_metadata(&session_row.metadata);
    let assets: Vec<_> = db
        .list_assets(&session_id)
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(row_to_asset_info)
        .collect();
    let connections: Vec<_> = db
        .list_connections(&session_id)
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(row_to_connection_info)
        .collect();
    let topology = build_topology_from_connections(&connections);
    let session_name = session_row.name.clone();
    let info = session_info_from_row(session_row);
    let data = LoadedSessionData {
        session_id,
        session_name,
        topology,
        connections,
        assets,
        metadata,
    };
    Ok(LoadSessionResult { info, data })
}

/// List saved sessions. When a project is active, returns only that project's sessions.
pub fn list_sessions(
    db: Option<&Database>,
    current_project_id: Option<i64>,
) -> Result<Vec<SessionInfo>, String> {
    let db = db_or_error(db)?;
    let rows = match current_project_id {
        Some(project_id) => db
            .list_sessions_for_project(project_id)
            .map_err(|e| e.to_string())?,
        None => db.list_sessions().map_err(|e| e.to_string())?,
    };
    Ok(rows.into_iter().map(session_info_from_row).collect())
}

/// Delete a session by ID.
pub fn delete_session(session_id: String, db: Option<&Database>) -> Result<(), String> {
    let db = db_or_error(db)?;
    db.delete_session(&session_id).map_err(|e| e.to_string())
}

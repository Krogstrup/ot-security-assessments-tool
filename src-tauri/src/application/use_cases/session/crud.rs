//! Session CRUD: save, load, list, delete.

use crate::commands::{support::mutex_state, AppState};

use super::{
    apply_loaded_session_state, build_topology_from_connections, db_from_session,
    mappers::{asset_info_to_row, connection_info_to_row, row_to_asset_info, row_to_connection_info},
    parse_session_metadata, session_info_from_row, SessionInfo, SessionMetadata,
};

/// Save the current state as a named session.
pub async fn save_session(
    name: String,
    description: Option<String>,
    state: &AppState,
) -> Result<SessionInfo, String> {
    use crate::commands::support::read_state;

    let (connections_snap, assets_snap, deep_parse_snap, imported_files_snap) = {
        let cap = read_state(&state.capture, "capture")?;
        let inv = read_state(&state.inventory, "inventory")?;
        (
            cap.connections.clone(),
            inv.assets.clone(),
            inv.deep_parse_info.clone(),
            cap.imported_files.clone(),
        )
    };

    let session_id = uuid::Uuid::new_v4().to_string();
    let desc = description.unwrap_or_default();
    let metadata = SessionMetadata {
        deep_parse_info: deep_parse_snap,
        imported_files: imported_files_snap,
    };
    let metadata_json = serde_json::to_string(&metadata).map_err(|e| e.to_string())?;

    let sess = mutex_state(&state.session, "session")?;
    let db = db_from_session(&sess)?;

    let session_row = db
        .create_session(&session_id, &name, &desc, &metadata_json)
        .map_err(|e| e.to_string())?;

    for asset in &assets_snap {
        let row = asset_info_to_row(asset, &session_id);
        db.insert_asset(&row).map_err(|e| e.to_string())?;
    }

    for conn in &connections_snap {
        let row = connection_info_to_row(conn, &session_id);
        db.insert_connection(&row).map_err(|e| e.to_string())?;
    }

    db.update_session_counts(
        &session_id,
        assets_snap.len() as i64,
        connections_snap.len() as i64,
    )
    .map_err(|e| e.to_string())?;

    if let Some(project_id) = sess.current_project_id {
        db.assign_session_to_project(&session_id, project_id)
            .map_err(|e| e.to_string())?;
    }

    log::info!(
        "Saved session '{}' ({}) with {} assets, {} connections",
        name,
        session_id,
        assets_snap.len(),
        connections_snap.len()
    );

    let mut info = session_info_from_row(session_row);
    info.asset_count = assets_snap.len() as i64;
    info.connection_count = connections_snap.len() as i64;
    Ok(info)
}

/// Load a session by ID, replacing the current state.
pub async fn load_session(session_id: String, state: &AppState) -> Result<SessionInfo, String> {
    let (session_row, metadata, assets, connections) = {
        let sess = mutex_state(&state.session, "session")?;
        let db = db_from_session(&sess)?;

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
        (session_row, metadata, assets, connections)
    };

    let topology = build_topology_from_connections(&connections);
    apply_loaded_session_state(
        state,
        session_id.clone(),
        session_row.name.clone(),
        topology,
        connections,
        assets,
        metadata,
    )?;

    log::info!("Loaded session '{}' ({})", session_row.name, session_id);
    Ok(session_info_from_row(session_row))
}

/// List saved sessions. When a project is active, returns only that project's sessions.
pub async fn list_sessions(state: &AppState) -> Result<Vec<SessionInfo>, String> {
    let sess = mutex_state(&state.session, "session")?;
    let db = db_from_session(&sess)?;

    let rows = match sess.current_project_id {
        Some(project_id) => db
            .list_sessions_for_project(project_id)
            .map_err(|e| e.to_string())?,
        None => db.list_sessions().map_err(|e| e.to_string())?,
    };

    Ok(rows.into_iter().map(session_info_from_row).collect())
}

/// Delete a session by ID.
pub async fn delete_session(session_id: String, state: &AppState) -> Result<(), String> {
    let sess = mutex_state(&state.session, "session")?;
    let db = db_from_session(&sess)?;
    db.delete_session(&session_id).map_err(|e| e.to_string())?;
    log::info!("Deleted session {}", session_id);
    Ok(())
}

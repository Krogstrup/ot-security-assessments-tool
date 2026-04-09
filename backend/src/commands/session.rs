//! Adapter layer for session use-cases.

use crate::application::use_cases::session as use_case;

use super::{
    support::{mutex_state, read_state, write_state},
    AppState, AssetInfo, ConnectionInfo,
};

pub use use_case::AssetUpdate;
pub use use_case::SessionInfo;

fn apply_loaded_session_state(
    state: &AppState,
    data: use_case::LoadedSessionData,
) -> Result<(), String> {
    {
        let mut cap = write_state(&state.capture, "capture")?;
        cap.topology = data.topology;
        cap.connections = data.connections;
        cap.packet_summaries = std::collections::HashMap::new();
        cap.imported_files = data.metadata.imported_files;
    }
    {
        let mut inv = write_state(&state.inventory, "inventory")?;
        inv.assets = data.assets;
        inv.deep_parse_info = data.metadata.deep_parse_info;
    }
    {
        let mut sess = mutex_state(&state.session, "session")?;
        sess.current_session_id = Some(data.session_id);
        sess.current_session_name = Some(data.session_name);
    }
    Ok(())
}

/// Save the current state as a named session.
pub async fn save_session(
    name: String,
    description: Option<String>,
    state: &AppState,
) -> Result<SessionInfo, String> {
    let (connections, assets, deep_parse_info, imported_files): (
        Vec<ConnectionInfo>,
        Vec<AssetInfo>,
        std::collections::HashMap<String, gm_parsers::DeepParseInfo>,
        Vec<String>,
    ) = {
        let cap = read_state(&state.capture, "capture")?;
        let inv = read_state(&state.inventory, "inventory")?;
        (
            cap.connections.clone(),
            inv.assets.clone(),
            inv.deep_parse_info.clone(),
            cap.imported_files.clone(),
        )
    };

    let sess = mutex_state(&state.session, "session")?;
    use_case::save_session(
        name,
        description,
        sess.db.as_ref(),
        sess.current_project_id,
        &connections,
        &assets,
        &deep_parse_info,
        &imported_files,
    )
}

/// Load a session by ID and replace runtime state.
pub async fn load_session(session_id: String, state: &AppState) -> Result<SessionInfo, String> {
    let result = {
        let sess = mutex_state(&state.session, "session")?;
        use_case::load_session(session_id.clone(), sess.db.as_ref())?
    };
    apply_loaded_session_state(state, result.data)?;
    Ok(result.info)
}

/// List saved sessions, optionally scoped to active project.
pub async fn list_sessions(state: &AppState) -> Result<Vec<SessionInfo>, String> {
    let sess = mutex_state(&state.session, "session")?;
    use_case::list_sessions(sess.db.as_ref(), sess.current_project_id)
}

/// Delete a session by ID.
pub async fn delete_session(session_id: String, state: &AppState) -> Result<(), String> {
    let sess = mutex_state(&state.session, "session")?;
    use_case::delete_session(session_id, sess.db.as_ref())
}

/// Export a session to `.kkj` archive.
pub async fn export_session_archive(
    session_id: String,
    output_path: String,
    state: &AppState,
) -> Result<String, String> {
    let sess = mutex_state(&state.session, "session")?;
    use_case::export_session_archive(session_id, output_path, sess.db.as_ref())
}

/// Import a `.kkj` archive and replace runtime state with imported session.
pub async fn import_session_archive(
    archive_path: String,
    state: &AppState,
) -> Result<SessionInfo, String> {
    let result = {
        let sess = mutex_state(&state.session, "session")?;
        use_case::import_session_archive(archive_path, sess.db.as_ref())?
    };
    apply_loaded_session_state(state, result.data)?;
    Ok(result.info)
}

/// Update one asset's editable fields.
pub async fn update_asset(
    asset_id: String,
    updates: AssetUpdate,
    state: &AppState,
) -> Result<AssetInfo, String> {
    let mut inv = write_state(&state.inventory, "inventory")?;
    let sess = mutex_state(&state.session, "session")?;
    use_case::update_asset(
        asset_id,
        updates,
        inv.assets.as_mut_slice(),
        sess.db.as_ref(),
        sess.current_session_id.is_some(),
    )
}

/// Bulk-update many assets.
pub async fn bulk_update_assets(
    asset_ids: Vec<String>,
    updates: AssetUpdate,
    state: &AppState,
) -> Result<usize, String> {
    let mut inv = write_state(&state.inventory, "inventory")?;
    let sess = mutex_state(&state.session, "session")?;
    use_case::bulk_update_assets(
        asset_ids,
        updates,
        inv.assets.as_mut_slice(),
        sess.db.as_ref(),
        sess.current_session_id.is_some(),
    )
}

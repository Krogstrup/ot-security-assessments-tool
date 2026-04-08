//! Session management commands: save, load, list, delete sessions,
//! update and bulk update assets, export/import ZIP archives.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use gm_db::{AssetRow, ConnectionRow, Database, SessionRow};
use gm_topology::{TopologyBuilder, TopologyGraph};

use super::{
    support::{mutex_state, read_state, write_state},
    AppState, AssetInfo, ConnectionInfo, DeepParseInfo, SessionState,
};

const DATABASE_NOT_AVAILABLE: &str = "Database not available";

// ─── Types ──────────────────────────────────────────────────

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
struct SessionMetadata {
    deep_parse_info: HashMap<String, DeepParseInfo>,
    imported_files: Vec<String>,
}

fn db_from_session(session: &SessionState) -> Result<&Database, String> {
    session
        .db
        .as_ref()
        .ok_or_else(|| DATABASE_NOT_AVAILABLE.to_string())
}

fn parse_session_metadata(metadata: &str) -> SessionMetadata {
    serde_json::from_str(metadata).unwrap_or(SessionMetadata {
        deep_parse_info: HashMap::new(),
        imported_files: Vec::new(),
    })
}

fn build_topology_from_connections(connections: &[ConnectionInfo]) -> TopologyGraph {
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

fn apply_loaded_session_state(
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

fn session_info_from_row(row: SessionRow) -> SessionInfo {
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

fn normalize_hostname(hostname: &str) -> Option<String> {
    if hostname.is_empty() {
        None
    } else {
        Some(hostname.to_string())
    }
}

fn normalize_purdue_level(level: u8) -> Option<u8> {
    if level > 5 {
        None
    } else {
        Some(level)
    }
}

fn apply_asset_update(asset: &mut AssetInfo, updates: &AssetUpdate) {
    if let Some(ref dt) = updates.device_type {
        asset.device_type = dt.clone();
    }
    if let Some(ref hostname) = updates.hostname {
        asset.hostname = normalize_hostname(hostname);
    }
    if let Some(ref notes) = updates.notes {
        asset.notes = notes.clone();
    }
    if let Some(level) = updates.purdue_level {
        asset.purdue_level = normalize_purdue_level(level);
    }
    if let Some(ref tags) = updates.tags {
        asset.tags = tags.clone();
    }
}

// ─── Session Commands ───────────────────────────────────────

/// Save the current state as a named session.
pub async fn save_session(
    name: String,
    description: Option<String>,
    state: &AppState,
) -> Result<SessionInfo, String> {
    // Snapshot capture + inventory data before locking the DB (lock order: capture → inventory)
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
    // Step 1: load all data from DB (session domain only)
    let (session_row, metadata, assets, connections) = {
        let sess = mutex_state(&state.session, "session")?;
        let db = db_from_session(&sess)?;

        let session_row = db.get_session(&session_id).map_err(|e| e.to_string())?;
        let metadata = parse_session_metadata(&session_row.metadata);
        let assets: Vec<AssetInfo> = db
            .list_assets(&session_id)
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(row_to_asset_info)
            .collect();
        let connections: Vec<ConnectionInfo> = db
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

// ─── Asset Update Commands ──────────────────────────────────

/// Update a single asset's editable fields.
pub async fn update_asset(
    asset_id: String,
    updates: AssetUpdate,
    state: &AppState,
) -> Result<AssetInfo, String> {
    // Step 1: update asset in inventory domain
    let updated = {
        let mut inv = write_state(&state.inventory, "inventory")?;
        let asset = inv
            .assets
            .iter_mut()
            .find(|a| a.id == asset_id)
            .ok_or_else(|| format!("Asset {} not found", asset_id))?;
        apply_asset_update(asset, &updates);
        asset.clone()
    };

    // Step 2: persist to DB if a session is loaded (session domain)
    let sess = mutex_state(&state.session, "session")?;
    if let (Some(ref db), Some(ref _session_id)) = (&sess.db, &sess.current_session_id) {
        if let Some(ref dt) = updates.device_type {
            let _ = db.update_asset_field(&asset_id, "device_type", dt);
        }
        if let Some(ref hostname) = updates.hostname {
            let _ = db.update_asset_field(&asset_id, "hostname", hostname);
        }
        if let Some(ref notes) = updates.notes {
            let _ = db.update_asset_field(&asset_id, "notes", notes);
        }
        if let Some(level) = updates.purdue_level {
            let _ = db.update_asset_field(&asset_id, "purdue_level", &level.to_string());
        }
        if let Some(ref tags) = updates.tags {
            let tags_json = serde_json::to_string(tags).unwrap_or_else(|_| "[]".to_string());
            let _ = db.update_asset_field(&asset_id, "tags", &tags_json);
        }
    }

    Ok(updated)
}

/// Bulk update assets (same field on multiple assets).
pub async fn bulk_update_assets(
    asset_ids: Vec<String>,
    updates: AssetUpdate,
    state: &AppState,
) -> Result<usize, String> {
    let asset_id_set: std::collections::HashSet<&str> =
        asset_ids.iter().map(String::as_str).collect();

    // Step 1: update assets in inventory domain
    let count = {
        let mut inv = write_state(&state.inventory, "inventory")?;
        let mut count = 0;
        for asset in &mut inv.assets {
            if asset_id_set.contains(asset.id.as_str()) {
                apply_asset_update(asset, &updates);
                count += 1;
            }
        }
        count
    };

    // Step 2: persist to DB if session is loaded (session domain)
    let sess = mutex_state(&state.session, "session")?;
    if let (Some(ref db), Some(ref _session_id)) = (&sess.db, &sess.current_session_id) {
        if let Some(ref dt) = updates.device_type {
            let _ = db.bulk_update_asset_field(&asset_ids, "device_type", dt);
        }
        if let Some(ref notes) = updates.notes {
            let _ = db.bulk_update_asset_field(&asset_ids, "notes", notes);
        }
    }

    Ok(count)
}

// ─── Session Archive (ZIP) ──────────────────────────────────

/// Export a session to a .kkj ZIP archive.
pub async fn export_session_archive(
    session_id: String,
    output_path: String,
    state: &AppState,
) -> Result<String, String> {
    let sess = mutex_state(&state.session, "session")?;
    let db = db_from_session(&sess)?;

    // Load session data from DB
    let session = db.get_session(&session_id).map_err(|e| e.to_string())?;
    let assets = db.list_assets(&session_id).map_err(|e| e.to_string())?;
    let connections = db
        .list_connections(&session_id)
        .map_err(|e| e.to_string())?;

    // Build the session data JSON
    let session_data = serde_json::json!({
        "session": {
            "id": session.id,
            "name": session.name,
            "description": session.description,
            "created_at": session.created_at,
            "updated_at": session.updated_at,
        },
        "metadata": session.metadata,
        "assets": assets,
        "connections": connections,
    });

    let manifest = serde_json::json!({
        "version": "1.0",
        "app_version": env!("CARGO_PKG_VERSION"),
        "created_at": chrono::Utc::now().to_rfc3339(),
        "asset_count": assets.len(),
        "connection_count": connections.len(),
    });

    // Create ZIP file
    let file = std::fs::File::create(&output_path).map_err(|e| e.to_string())?;
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    // Write manifest.json
    zip.start_file("manifest.json", options)
        .map_err(|e| e.to_string())?;
    std::io::Write::write_all(
        &mut zip,
        serde_json::to_string_pretty(&manifest)
            .map_err(|e| e.to_string())?
            .as_bytes(),
    )
    .map_err(|e| e.to_string())?;

    // Write session.json
    zip.start_file("session.json", options)
        .map_err(|e| e.to_string())?;
    std::io::Write::write_all(
        &mut zip,
        serde_json::to_string_pretty(&session_data)
            .map_err(|e| e.to_string())?
            .as_bytes(),
    )
    .map_err(|e| e.to_string())?;

    zip.finish().map_err(|e| e.to_string())?;

    log::info!("Exported session archive to {}", output_path);
    Ok(output_path)
}

/// Import a session from a .kkj ZIP archive.
pub async fn import_session_archive(
    archive_path: String,
    state: &AppState,
) -> Result<SessionInfo, String> {
    // Read the ZIP file
    let file = std::fs::File::open(&archive_path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;

    // Read session.json
    let session_json: serde_json::Value = {
        let entry = archive.by_name("session.json").map_err(|e| e.to_string())?;
        serde_json::from_reader(entry).map_err(|e| e.to_string())?
    };

    // Parse session data
    let session_name = session_json["session"]["name"]
        .as_str()
        .unwrap_or("Imported Session")
        .to_string();
    let session_desc = session_json["session"]["description"]
        .as_str()
        .unwrap_or("")
        .to_string();
    let metadata_str = session_json
        .get("metadata")
        .map(|m| m.to_string())
        .unwrap_or_else(|| "{}".to_string());

    let assets: Vec<AssetRow> =
        serde_json::from_value(session_json["assets"].clone()).unwrap_or_default();

    let connections: Vec<ConnectionRow> =
        serde_json::from_value(session_json["connections"].clone()).unwrap_or_default();

    // Save to database and load data back (session domain only)
    let (new_session_id, session_row, asset_count, conn_count, metadata, assets_vec, conns_vec) = {
        let sess = mutex_state(&state.session, "session")?;
        let db = db_from_session(&sess)?;

        let new_session_id = uuid::Uuid::new_v4().to_string();
        db.create_session(&new_session_id, &session_name, &session_desc, &metadata_str)
            .map_err(|e| e.to_string())?;

        for mut asset in assets {
            asset.session_id = new_session_id.clone();
            db.insert_asset(&asset).map_err(|e| e.to_string())?;
        }

        for mut conn in connections {
            conn.session_id = new_session_id.clone();
            db.insert_connection(&conn).map_err(|e| e.to_string())?;
        }

        let session_row = db.get_session(&new_session_id).map_err(|e| e.to_string())?;
        let loaded_assets = db.list_assets(&new_session_id).map_err(|e| e.to_string())?;
        let loaded_conns = db
            .list_connections(&new_session_id)
            .map_err(|e| e.to_string())?;
        let asset_count = loaded_assets.len() as i64;
        let conn_count = loaded_conns.len() as i64;

        db.update_session_counts(&new_session_id, asset_count, conn_count)
            .map_err(|e| e.to_string())?;

        let metadata = parse_session_metadata(&metadata_str);
        let assets_vec: Vec<AssetInfo> = loaded_assets.into_iter().map(row_to_asset_info).collect();
        let conns_vec: Vec<ConnectionInfo> = loaded_conns
            .into_iter()
            .map(row_to_connection_info)
            .collect();
        (
            new_session_id,
            session_row,
            asset_count,
            conn_count,
            metadata,
            assets_vec,
            conns_vec,
        )
    };

    let topology = build_topology_from_connections(&conns_vec);
    apply_loaded_session_state(
        state,
        new_session_id,
        session_name.clone(),
        topology,
        conns_vec,
        assets_vec,
        metadata,
    )?;

    log::info!(
        "Imported session archive '{}' from {}",
        session_name,
        archive_path
    );

    let mut info = session_info_from_row(session_row);
    info.asset_count = asset_count;
    info.connection_count = conn_count;
    Ok(info)
}

// ─── Conversion Helpers ─────────────────────────────────────

fn asset_info_to_row(asset: &AssetInfo, session_id: &str) -> AssetRow {
    AssetRow {
        id: asset.id.clone(),
        session_id: session_id.to_string(),
        ip_address: asset.ip_address.clone(),
        mac_address: asset.mac_address.clone(),
        hostname: asset.hostname.clone(),
        device_type: asset.device_type.clone(),
        vendor: asset.vendor.clone(),
        product_family: asset.product_family.clone(),
        protocols: serde_json::to_string(&asset.protocols).unwrap_or_else(|_| "[]".to_string()),
        confidence: asset.confidence as i64,
        purdue_level: asset.purdue_level.map(|l| l as i64),
        tags: serde_json::to_string(&asset.tags).unwrap_or_else(|_| "[]".to_string()),
        notes: asset.notes.clone(),
        packet_count: asset.packet_count as i64,
        signature_matches: serde_json::to_string(&asset.signature_matches)
            .unwrap_or_else(|_| "[]".to_string()),
        oui_vendor: asset.oui_vendor.clone(),
        country: asset.country.clone(),
        is_public_ip: asset.is_public_ip,
        first_seen: asset.first_seen.clone(),
        last_seen: asset.last_seen.clone(),
    }
}

fn connection_info_to_row(conn: &ConnectionInfo, session_id: &str) -> ConnectionRow {
    ConnectionRow {
        id: conn.id.clone(),
        session_id: session_id.to_string(),
        src_ip: conn.src_ip.clone(),
        src_port: conn.src_port as i64,
        src_mac: conn.src_mac.clone(),
        dst_ip: conn.dst_ip.clone(),
        dst_port: conn.dst_port as i64,
        dst_mac: conn.dst_mac.clone(),
        protocol: conn.protocol.clone(),
        transport: conn.transport.clone(),
        packet_count: conn.packet_count as i64,
        byte_count: conn.byte_count as i64,
        first_seen: conn.first_seen.clone(),
        last_seen: conn.last_seen.clone(),
        origin_files: serde_json::to_string(&conn.origin_files)
            .unwrap_or_else(|_| "[]".to_string()),
    }
}

fn row_to_asset_info(row: AssetRow) -> AssetInfo {
    let protocols: Vec<String> = serde_json::from_str(&row.protocols).unwrap_or_default();
    let tags: Vec<String> = serde_json::from_str(&row.tags).unwrap_or_default();
    let signature_matches = serde_json::from_str(&row.signature_matches).unwrap_or_default();

    AssetInfo {
        id: row.id,
        ip_address: row.ip_address,
        mac_address: row.mac_address,
        hostname: row.hostname,
        device_type: row.device_type,
        vendor: row.vendor,
        protocols,
        first_seen: row.first_seen,
        last_seen: row.last_seen,
        notes: row.notes,
        purdue_level: row.purdue_level.map(|l| l as u8),
        tags,
        packet_count: row.packet_count as u64,
        confidence: row.confidence as u8,
        product_family: row.product_family,
        signature_matches,
        oui_vendor: row.oui_vendor,
        country: row.country,
        is_public_ip: row.is_public_ip,
    }
}

fn row_to_connection_info(row: ConnectionRow) -> ConnectionInfo {
    let origin_files: Vec<String> = serde_json::from_str(&row.origin_files).unwrap_or_default();

    ConnectionInfo {
        id: row.id,
        src_ip: row.src_ip,
        src_port: row.src_port as u16,
        src_mac: row.src_mac,
        dst_ip: row.dst_ip,
        dst_port: row.dst_port as u16,
        dst_mac: row.dst_mac,
        protocol: row.protocol,
        transport: row.transport,
        packet_count: row.packet_count as u64,
        byte_count: row.byte_count as u64,
        first_seen: row.first_seen,
        last_seen: row.last_seen,
        origin_files,
    }
}

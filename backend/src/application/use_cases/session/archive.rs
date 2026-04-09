//! Session archive: export to / import from a `.kkj` ZIP file.

use gm_db::{AssetRow, ConnectionRow, Database};

use super::{
    build_topology_from_connections, db_or_error,
    mappers::{row_to_asset_info, row_to_connection_info},
    parse_session_metadata, session_info_from_row, LoadedSessionData, SessionInfo,
    SessionUseCaseError,
};

pub struct ImportSessionArchiveResult {
    pub info: SessionInfo,
    pub data: LoadedSessionData,
}

/// Export a session to a `.kkj` ZIP archive.
pub fn export_session_archive(
    session_id: String,
    output_path: String,
    db: Option<&Database>,
) -> Result<String, SessionUseCaseError> {
    let db = db_or_error(db)?;

    let session = db.get_session(&session_id)?;
    let assets = db.list_assets(&session_id)?;
    let connections = db.list_connections(&session_id)?;

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

    let file = std::fs::File::create(&output_path)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    zip.start_file("manifest.json", options)?;
    std::io::Write::write_all(
        &mut zip,
        serde_json::to_string_pretty(&manifest)?.as_bytes(),
    )
    .map_err(SessionUseCaseError::from)?;

    zip.start_file("session.json", options)?;
    std::io::Write::write_all(
        &mut zip,
        serde_json::to_string_pretty(&session_data)?.as_bytes(),
    )
    .map_err(SessionUseCaseError::from)?;

    zip.finish()?;
    Ok(output_path)
}

/// Import a session from a `.kkj` ZIP archive.
pub fn import_session_archive(
    archive_path: String,
    db: Option<&Database>,
) -> Result<ImportSessionArchiveResult, SessionUseCaseError> {
    let db = db_or_error(db)?;

    let file = std::fs::File::open(&archive_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    let session_json: serde_json::Value = {
        let entry = archive.by_name("session.json")?;
        serde_json::from_reader(entry)?
    };

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

    let new_session_id = uuid::Uuid::new_v4().to_string();
    db.create_session(&new_session_id, &session_name, &session_desc, &metadata_str)
        .map_err(SessionUseCaseError::from)?;

    for mut asset in assets {
        asset.session_id = new_session_id.clone();
        db.insert_asset(&asset).map_err(SessionUseCaseError::from)?;
    }

    for mut conn in connections {
        conn.session_id = new_session_id.clone();
        db.insert_connection(&conn)
            .map_err(SessionUseCaseError::from)?;
    }

    let session_row = db.get_session(&new_session_id)?;
    let loaded_assets = db.list_assets(&new_session_id)?;
    let loaded_conns = db.list_connections(&new_session_id)?;
    let asset_count = loaded_assets.len() as i64;
    let conn_count = loaded_conns.len() as i64;
    db.update_session_counts(&new_session_id, asset_count, conn_count)
        .map_err(SessionUseCaseError::from)?;

    let metadata = parse_session_metadata(&metadata_str);
    let assets_vec: Vec<_> = loaded_assets.into_iter().map(row_to_asset_info).collect();
    let conns_vec: Vec<_> = loaded_conns
        .into_iter()
        .map(row_to_connection_info)
        .collect();
    let topology = build_topology_from_connections(&conns_vec);
    let session_name = session_row.name.clone();
    let mut info = session_info_from_row(session_row);
    info.asset_count = asset_count;
    info.connection_count = conn_count;

    Ok(ImportSessionArchiveResult {
        info,
        data: LoadedSessionData {
            session_id: new_session_id,
            session_name,
            topology,
            connections: conns_vec,
            assets: assets_vec,
            metadata,
        },
    })
}

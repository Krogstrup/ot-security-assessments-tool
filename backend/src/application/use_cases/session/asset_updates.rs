//! Asset update use-cases: single and bulk field updates.

use gm_db::Database;
use gm_types::AssetInfo;

use super::AssetUpdate;

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

/// Update a single asset's editable fields.
pub fn update_asset(
    asset_id: String,
    updates: AssetUpdate,
    assets: &mut [AssetInfo],
    db: Option<&Database>,
    has_active_session: bool,
) -> Result<AssetInfo, String> {
    let asset = assets
        .iter_mut()
        .find(|a| a.id == asset_id)
        .ok_or_else(|| format!("Asset {} not found", asset_id))?;
    apply_asset_update(asset, &updates);
    let updated = asset.clone();

    if let (Some(db), true) = (db, has_active_session) {
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
pub fn bulk_update_assets(
    asset_ids: Vec<String>,
    updates: AssetUpdate,
    assets: &mut [AssetInfo],
    db: Option<&Database>,
    has_active_session: bool,
) -> Result<usize, String> {
    let asset_id_set: std::collections::HashSet<&str> =
        asset_ids.iter().map(String::as_str).collect();

    let mut count = 0;
    for asset in assets.iter_mut() {
        if asset_id_set.contains(asset.id.as_str()) {
            apply_asset_update(asset, &updates);
            count += 1;
        }
    }

    if let (Some(db), true) = (db, has_active_session) {
        if let Some(ref dt) = updates.device_type {
            let _ = db.bulk_update_asset_field(&asset_ids, "device_type", dt);
        }
        if let Some(ref notes) = updates.notes {
            let _ = db.bulk_update_asset_field(&asset_ids, "notes", notes);
        }
    }

    Ok(count)
}

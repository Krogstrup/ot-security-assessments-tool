use super::support::{app_data_dir, create_dir_all, read_text_file, write_text_file};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// List all available network interfaces.
pub fn list_interfaces() -> Result<Vec<gm_capture::NetworkInterface>, String> {
    gm_capture::list_interfaces().map_err(|e| e.to_string())
}

#[derive(Serialize)]
pub struct AppInfo {
    version: String,
    rust_version: String,
}

/// Get application version info.
pub fn get_app_info() -> AppInfo {
    AppInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        rust_version: format!("rustc {}", env!("CARGO_PKG_RUST_VERSION")),
    }
}

// ─── Settings Persistence (Phase 11) ────────────────────────

/// Persistent user settings stored as JSON at ~/.kusanaginokajiki/settings.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    /// Theme mode: "dark", "light", or "system"
    #[serde(default = "default_theme")]
    pub theme: String,
}

fn default_theme() -> String {
    "dark".to_string()
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            theme: default_theme(),
        }
    }
}

/// Get the settings file path.
fn settings_path() -> Result<PathBuf, String> {
    Ok(app_data_dir()?.join("settings.json"))
}

/// Load user settings from disk. Returns defaults if file doesn't exist.
pub fn get_settings() -> Result<UserSettings, String> {
    let path = settings_path()?;
    if !path.exists() {
        return Ok(UserSettings::default());
    }
    let content = read_text_file(&path)?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

/// Save user settings to disk.
pub fn save_settings(settings: UserSettings) -> Result<(), String> {
    let path = settings_path()?;
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    write_text_file(&path, &content)?;
    log::info!("Settings saved to {}", path.display());
    Ok(())
}

// ─── Plugin Discovery (Phase 11) ────────────────────────────

/// A plugin manifest describing a plugin pack.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    /// Plugin type: "signature", "importer", "exporter", "analyzer"
    pub plugin_type: String,
    pub description: String,
    pub author: Option<String>,
}

/// List plugins found in the plugins directory.
///
/// Scans ~/.kusanaginokajiki/plugins/ for manifest.json files.
pub fn list_plugins() -> Result<Vec<PluginManifest>, String> {
    let plugins_dir = app_data_dir()?.join("plugins");

    if !plugins_dir.exists() {
        // Create the directory so users know where to put plugins
        let _ = create_dir_all(&plugins_dir);
        return Ok(Vec::new());
    }

    let mut plugins = Vec::new();
    let entries = std::fs::read_dir(&plugins_dir)
        .map_err(|e| format!("read_dir {}: {e}", plugins_dir.display()))?;

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let manifest_path = path.join("manifest.json");
        if manifest_path.exists() {
            match read_text_file(&manifest_path) {
                Ok(content) => match serde_json::from_str::<PluginManifest>(&content) {
                    Ok(manifest) => plugins.push(manifest),
                    Err(e) => {
                        log::warn!(
                            "Invalid plugin manifest at {}: {}",
                            manifest_path.display(),
                            e
                        );
                    }
                },
                Err(e) => {
                    log::warn!("Failed to read {}: {}", manifest_path.display(), e);
                }
            }
        }
    }

    Ok(plugins)
}

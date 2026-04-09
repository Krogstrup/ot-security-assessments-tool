//! HTTP types and import/export path support for the web API.

use std::path::{Path as StdPath, PathBuf};
use std::sync::OnceLock;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use serde_json::json;

use crate::commands;

// ── HTTP response types ───────────────────────────────────────────────────────

/// A typed API error that converts directly into an Axum HTTP response.
#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    message: String,
}

impl ApiError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: message.into(),
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.into(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status, Json(json!({ "error": self.message }))).into_response()
    }
}

/// One file entry in an import directory listing response.
#[derive(Debug, Serialize)]
pub struct ImportPcapFileEntry {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
}

/// Response body for the list-import-files endpoints.
#[derive(Debug, Serialize)]
pub struct ImportPcapFilesResponse {
    pub kind: String,
    pub base_dir: String,
    pub files: Vec<ImportPcapFileEntry>,
    pub list_limit: usize,
    pub truncated: bool,
}

// ── ImportKind ────────────────────────────────────────────────────────────────

const DEFAULT_IMPORT_FILE_LIST_LIMIT: usize = 500;

/// Discriminator for all supported import file types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportKind {
    Pcap,
    PhysicalConfig,
    PhysicalMac,
    PhysicalNeighbor,
    PhysicalArp,
    Zeek,
    Suricata,
    Nmap,
    Masscan,
    Wazuh,
    Sinema,
    Tia,
    SessionArchive,
}

impl ImportKind {
    const ALL: [Self; 13] = [
        ImportKind::Pcap,
        ImportKind::PhysicalConfig,
        ImportKind::PhysicalMac,
        ImportKind::PhysicalNeighbor,
        ImportKind::PhysicalArp,
        ImportKind::Zeek,
        ImportKind::Suricata,
        ImportKind::Nmap,
        ImportKind::Masscan,
        ImportKind::Wazuh,
        ImportKind::Sinema,
        ImportKind::Tia,
        ImportKind::SessionArchive,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            ImportKind::Pcap => "pcap",
            ImportKind::PhysicalConfig => "physical_config",
            ImportKind::PhysicalMac => "physical_mac",
            ImportKind::PhysicalNeighbor => "physical_neighbor",
            ImportKind::PhysicalArp => "physical_arp",
            ImportKind::Zeek => "zeek",
            ImportKind::Suricata => "suricata",
            ImportKind::Nmap => "nmap",
            ImportKind::Masscan => "masscan",
            ImportKind::Wazuh => "wazuh",
            ImportKind::Sinema => "sinema",
            ImportKind::Tia => "tia",
            ImportKind::SessionArchive => "session_archive",
        }
    }

    fn extensions(self) -> &'static [&'static str] {
        match self {
            ImportKind::Pcap => &["pcap", "pcapng", "cap"],
            ImportKind::PhysicalConfig => &["cfg", "conf", "txt", "log"],
            ImportKind::PhysicalMac => &["txt", "log", "csv"],
            ImportKind::PhysicalNeighbor => &["txt", "log", "csv"],
            ImportKind::PhysicalArp => &["txt", "log", "csv"],
            ImportKind::Zeek => &["log", "tsv"],
            ImportKind::Suricata => &["json", "jsonl", "ndjson"],
            ImportKind::Nmap => &["xml"],
            ImportKind::Masscan => &["json"],
            ImportKind::Wazuh => &["json", "jsonl", "ndjson"],
            ImportKind::Sinema => &["csv", "txt"],
            ImportKind::Tia => &["xml"],
            ImportKind::SessionArchive => &["kkj"],
        }
    }

    pub fn supported_values_csv() -> String {
        ImportKind::ALL
            .iter()
            .map(|kind| kind.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl std::str::FromStr for ImportKind {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        ImportKind::ALL
            .iter()
            .copied()
            .find(|kind| kind.as_str() == value)
            .ok_or(())
    }
}

// ── Runtime config ────────────────────────────────────────────────────────────

#[derive(Debug)]
struct HeadlessRuntimeConfig {
    import_root_dir: PathBuf,
    export_dir: PathBuf,
    import_list_limit: usize,
    legacy_pcap_import_dir: Option<PathBuf>,
}

impl HeadlessRuntimeConfig {
    fn from_env() -> Self {
        let app_data_dir = match dirs::home_dir() {
            Some(home) => home.join(commands::support::APP_DATA_DIR_NAME),
            None => PathBuf::from(format!("./{}", commands::support::APP_DATA_DIR_NAME)),
        };
        let import_root_dir = std::env::var("KK_HEADLESS_IMPORTS_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|_| app_data_dir.join("imports"));
        let export_dir = std::env::var("KK_HEADLESS_EXPORT_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| app_data_dir.join("export"));
        let import_list_limit = std::env::var("KK_HEADLESS_IMPORT_LIST_LIMIT")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .filter(|v| *v > 0)
            .unwrap_or(DEFAULT_IMPORT_FILE_LIST_LIMIT);
        let legacy_pcap_import_dir = std::env::var("KK_HEADLESS_IMPORT_DIR")
            .ok()
            .map(PathBuf::from);

        Self {
            import_root_dir,
            export_dir,
            import_list_limit,
            legacy_pcap_import_dir,
        }
    }

    fn import_dir(&self, kind: ImportKind) -> PathBuf {
        if kind == ImportKind::Pcap {
            if let Some(path) = &self.legacy_pcap_import_dir {
                return path.clone();
            }
        }
        self.import_root_dir.join(kind.as_str())
    }
}

static RUNTIME_CONFIG: OnceLock<HeadlessRuntimeConfig> = OnceLock::new();

fn runtime_config() -> &'static HeadlessRuntimeConfig {
    RUNTIME_CONFIG.get_or_init(HeadlessRuntimeConfig::from_env)
}

// ── Public path helpers ───────────────────────────────────────────────────────

/// Resolve the frontend dist directory from a CLI arg, env var, or well-known fallback.
pub fn resolve_frontend_dist(arg_dist: Option<PathBuf>) -> PathBuf {
    if let Some(dist) = arg_dist {
        return dist;
    }
    if let Ok(env_dist) = std::env::var("KK_FRONTEND_DIST") {
        let path = PathBuf::from(env_dist);
        if path.join("index.html").exists() {
            return path;
        }
    }
    for candidate in [PathBuf::from("build"), PathBuf::from("../build")] {
        if candidate.join("index.html").exists() {
            return candidate;
        }
    }
    PathBuf::from("build")
}

/// Resolve a caller-supplied export path to an absolute path under the export dir.
pub fn resolve_export_output_path(raw_path: &str, fallback_name: &str) -> Result<String, ApiError> {
    let export_dir = runtime_config().export_dir.clone();
    if !export_dir.exists() {
        std::fs::create_dir_all(&export_dir).map_err(|e| {
            ApiError::bad_request(format!(
                "failed to create export directory {}: {}",
                export_dir.display(),
                e
            ))
        })?;
    }
    let file_name = StdPath::new(raw_path)
        .file_name()
        .and_then(|v| v.to_str())
        .filter(|v| !v.trim().is_empty())
        .unwrap_or(fallback_name);
    Ok(export_dir.join(file_name).to_string_lossy().to_string())
}

/// Resolve a caller-supplied import path to an absolute path under the correct import dir.
pub fn resolve_import_input_path(raw_path: &str, kind: ImportKind) -> Result<String, ApiError> {
    let import_dir = runtime_config().import_dir(kind);
    if !import_dir.exists() {
        std::fs::create_dir_all(&import_dir).map_err(|e| {
            ApiError::bad_request(format!(
                "failed to create import directory {}: {}",
                import_dir.display(),
                e
            ))
        })?;
    }
    let file_name = StdPath::new(raw_path)
        .file_name()
        .and_then(|v| v.to_str())
        .filter(|v| !v.trim().is_empty())
        .ok_or_else(|| ApiError::bad_request(format!("invalid import path: {}", raw_path)))?;
    Ok(import_dir.join(file_name).to_string_lossy().to_string())
}

/// List all importable files of the given kind from its configured directory.
pub fn list_import_files_for_kind(kind: ImportKind) -> Result<ImportPcapFilesResponse, ApiError> {
    let base_dir = runtime_config().import_dir(kind);
    let list_limit = runtime_config().import_list_limit;

    if !base_dir.exists() {
        std::fs::create_dir_all(&base_dir).map_err(|e| {
            ApiError::bad_request(format!(
                "import directory does not exist and could not be created: {} ({})",
                base_dir.display(),
                e
            ))
        })?;
    }
    if !base_dir.is_dir() {
        return Err(ApiError::bad_request(format!(
            "import path is not a directory: {}",
            base_dir.display()
        )));
    }

    let mut files = Vec::new();
    let mut truncated = false;
    let allowed_extensions = kind.extensions();

    let entries =
        std::fs::read_dir(&base_dir).map_err(|e| ApiError::internal(format!("read_dir: {}", e)))?;

    for entry in entries {
        let entry = match entry {
            Ok(v) => v,
            Err(e) => {
                log::warn!("Skipping unreadable import-dir entry: {}", e);
                continue;
            }
        };
        let name = match entry.file_name().to_str() {
            Some(v) => v.to_string(),
            None => continue,
        };
        if !has_allowed_extension(&name, allowed_extensions) {
            continue;
        }
        if files.len() >= list_limit {
            truncated = true;
            break;
        }
        let file_type = match entry.file_type() {
            Ok(v) => v,
            Err(e) => {
                log::warn!("Skipping import-dir entry with unreadable file type: {}", e);
                continue;
            }
        };
        if !file_type.is_file() {
            continue;
        }
        let path = entry.path();
        let size_bytes = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
        files.push(ImportPcapFileEntry {
            name,
            path: path.to_string_lossy().to_string(),
            size_bytes,
        });
    }

    files.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(ImportPcapFilesResponse {
        kind: kind.as_str().to_string(),
        base_dir: base_dir.to_string_lossy().to_string(),
        files,
        list_limit,
        truncated,
    })
}

fn has_allowed_extension(name: &str, allowed: &[&str]) -> bool {
    let ext = match name.rsplit_once('.') {
        Some((_, ext)) => ext,
        None => return false,
    };
    allowed
        .iter()
        .any(|candidate| ext.eq_ignore_ascii_case(candidate))
}

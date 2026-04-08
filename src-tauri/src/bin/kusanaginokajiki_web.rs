use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use clap::Parser;
use gm_capture::{LiveCaptureConfig, ParsedPacket, PcapReader};
use gm_topology::TopologyGraph;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::convert::Infallible;
use std::path::{Path as StdPath, PathBuf};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::{Stream, StreamExt as _};
use tower_http::services::{ServeDir, ServeFile};

#[path = "../commands/mod.rs"]
mod commands;

use commands::capture::{FileImportResult, ImportResult};
use commands::data::{AssetPage, ConnectionPage, DataCounts};
use commands::processor::PacketProcessor;
use commands::AppState;
use commands::ProtocolStatInfo;

const DEFAULT_IMPORT_FILE_LIST_LIMIT: usize = 500;

type SharedState = Arc<AppState>;

#[derive(Parser, Debug)]
#[command(
    name = "kusanaginokajiki_web",
    about = "Headless HTTP API + static frontend server"
)]
struct Cli {
    /// Bind host (use 0.0.0.0 for remote access).
    #[arg(long, default_value = "0.0.0.0")]
    host: String,
    /// Bind port.
    #[arg(long, default_value_t = 4173)]
    port: u16,
    /// Path to built frontend dist directory (default: auto-detect build/).
    #[arg(long)]
    frontend_dist: Option<PathBuf>,
}

#[derive(Debug)]
struct ApiError {
    status: StatusCode,
    message: String,
}

impl ApiError {
    fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: message.into(),
        }
    }

    fn internal(message: impl Into<String>) -> Self {
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

#[derive(Debug, Deserialize)]
struct ImportPcapRequest {
    paths: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateProjectRequest {
    name: String,
    client_name: Option<String>,
    site_name: Option<String>,
    assessor_name: Option<String>,
    engagement_start: Option<String>,
    engagement_end: Option<String>,
    notes: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateProjectRequest {
    name: String,
    client_name: Option<String>,
    site_name: Option<String>,
    assessor_name: Option<String>,
    engagement_start: Option<String>,
    engagement_end: Option<String>,
    notes: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SetActiveProjectRequest {
    id: i64,
}

#[derive(Debug, Deserialize)]
struct PagingQuery {
    page: Option<usize>,
    #[serde(alias = "pageSize")]
    page_size: Option<usize>,
    #[serde(alias = "sortBy")]
    sort_by: Option<String>,
}

#[derive(Debug, Serialize)]
struct ImportPcapFileEntry {
    name: String,
    path: String,
    size_bytes: u64,
}

#[derive(Debug, Serialize)]
struct ImportPcapFilesResponse {
    kind: String,
    base_dir: String,
    files: Vec<ImportPcapFileEntry>,
    list_limit: usize,
    truncated: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ImportKind {
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

    fn as_str(self) -> &'static str {
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

    fn supported_values_csv() -> String {
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

fn build_shared_state() -> SharedState {
    let (event_tx, _) = broadcast::channel::<(String, serde_json::Value)>(256);
    let mut app_state = AppState::new(commands::resource_paths::ResourcePaths::from_env());
    app_state.event_tx = Some(event_tx);
    Arc::new(app_state)
}

fn build_api_router() -> Router<SharedState> {
    Router::new()
        .route("/health", get(health))
        .route("/system/app-info", get(get_app_info))
        .route("/system/interfaces", get(get_interfaces))
        .route("/system/import-pcap-files", get(list_import_pcap_files))
        .route("/system/import-files/{kind}", get(list_import_files))
        .route("/capture/import-pcap", post(import_pcap))
        .route("/invoke/{command}", post(invoke_command))
        .route("/data/topology", get(get_topology))
        .route("/data/assets", get(get_assets))
        .route("/data/connections", get(get_connections))
        .route("/data/counts", get(get_counts))
        .route("/data/protocol-stats", get(get_protocol_stats))
        .route(
            "/data/connection-packets/{connection_id}",
            get(get_connection_packets),
        )
        // v1 resource endpoints — projects
        // NOTE: /v1/projects/active must be registered before /v1/projects/{id}
        .route(
            "/v1/projects",
            get(list_projects_handler).post(create_project_handler),
        )
        .route(
            "/v1/projects/active",
            put(set_active_project_handler).delete(clear_active_project_handler),
        )
        .route(
            "/v1/projects/{id}",
            get(get_project_handler)
                .put(update_project_handler)
                .delete(delete_project_handler),
        )
        // v1 resource endpoints — sessions
        // NOTE: static sub-paths (/import, /compare) must be registered before /sessions/{id}
        .route(
            "/v1/sessions",
            get(list_sessions_handler).post(save_session_handler),
        )
        .route("/v1/sessions/import", post(import_session_handler))
        .route("/v1/sessions/compare", post(compare_sessions_handler))
        .route("/v1/sessions/{id}/load", post(load_session_handler))
        .route("/v1/sessions/{id}/export", post(export_session_handler))
        .route("/v1/sessions/{id}", delete(delete_session_handler))
        // v1 resource endpoints — analysis
        .route("/v1/analysis/run", post(run_analysis_handler))
        .route("/v1/analysis/findings", get(get_findings_handler))
        .route("/v1/analysis/purdue", get(get_purdue_handler))
        .route("/v1/analysis/anomalies", get(get_anomalies_handler))
        .route("/v1/analysis/credentials", get(get_credentials_handler))
        .route("/v1/analysis/criticality", get(get_criticality_handler))
        .route(
            "/v1/analysis/naming-suggestions",
            get(get_naming_suggestions_handler),
        )
        .route("/v1/analysis/malware", get(get_malware_handler))
        .route(
            "/v1/analysis/switch-security",
            get(get_switch_security_handler),
        )
        .route("/v1/analysis/compliance", get(get_compliance_handler))
        .route("/v1/analysis/cve", get(get_cve_handler))
        // v1 SSE event stream
        .route("/v1/events", get(events_handler))
}

fn build_http_app(frontend_dist: &StdPath, state: SharedState) -> Router {
    let index_path = frontend_dist.join("index.html");
    let static_files = ServeDir::new(frontend_dist).not_found_service(ServeFile::new(index_path));

    Router::new()
        .nest("/api", build_api_router())
        .fallback_service(static_files)
        .with_state(state)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let cli = Cli::parse();
    let frontend_dist = resolve_frontend_dist(cli.frontend_dist);
    let state = build_shared_state();
    let app = build_http_app(frontend_dist.as_path(), state);

    let addr = format!("{}:{}", cli.host, cli.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    log::info!("Headless server listening on http://{}", addr);
    log::info!("Serving frontend from {}", frontend_dist.display());
    axum::serve(listener, app).await?;
    Ok(())
}

fn resolve_frontend_dist(arg_dist: Option<PathBuf>) -> PathBuf {
    if let Some(dist) = arg_dist {
        return dist;
    }

    if let Ok(env_dist) = std::env::var("KK_FRONTEND_DIST") {
        let path = PathBuf::from(env_dist);
        if has_frontend_index(path.as_path()) {
            return path;
        }
    }

    let candidates = [PathBuf::from("build"), PathBuf::from("../build")];

    for candidate in candidates {
        if has_frontend_index(candidate.as_path()) {
            return candidate;
        }
    }

    PathBuf::from("build")
}

fn has_frontend_index(path: &StdPath) -> bool {
    path.join("index.html").exists()
}

fn resolve_app_data_dir() -> PathBuf {
    if let Some(home) = dirs::home_dir() {
        return home.join(".kusanaginokajiki");
    }
    PathBuf::from("./.kusanaginokajiki")
}

fn resolve_import_root_dir() -> PathBuf {
    if let Ok(path) = std::env::var("KK_HEADLESS_IMPORTS_ROOT") {
        return PathBuf::from(path);
    }
    resolve_app_data_dir().join("imports")
}

fn resolve_export_dir() -> PathBuf {
    if let Ok(path) = std::env::var("KK_HEADLESS_EXPORT_DIR") {
        return PathBuf::from(path);
    }
    resolve_app_data_dir().join("export")
}

fn resolve_import_dir(kind: ImportKind) -> PathBuf {
    // Backward compatibility for old single-folder PCAP env setting.
    if kind == ImportKind::Pcap {
        if let Ok(path) = std::env::var("KK_HEADLESS_IMPORT_DIR") {
            return PathBuf::from(path);
        }
    }
    resolve_import_root_dir().join(kind.as_str())
}

fn resolve_import_list_limit() -> usize {
    std::env::var("KK_HEADLESS_IMPORT_LIST_LIMIT")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(DEFAULT_IMPORT_FILE_LIST_LIMIT)
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

fn resolve_export_output_path(raw_path: &str, fallback_name: &str) -> Result<String, ApiError> {
    let export_dir = resolve_export_dir();
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
    let resolved = export_dir.join(file_name);
    Ok(resolved.to_string_lossy().to_string())
}

fn resolve_import_input_path(raw_path: &str, kind: ImportKind) -> Result<String, ApiError> {
    let import_dir = resolve_import_dir(kind);
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

    let resolved = import_dir.join(file_name);
    Ok(resolved.to_string_lossy().to_string())
}

fn list_import_files_for_kind(kind: ImportKind) -> Result<ImportPcapFilesResponse, ApiError> {
    let base_dir = resolve_import_dir(kind);
    let list_limit = resolve_import_list_limit();
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
    let entries =
        std::fs::read_dir(&base_dir).map_err(|e| ApiError::internal(format!("read_dir: {}", e)))?;
    let allowed_extensions = kind.extensions();

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

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "ok": true }))
}

async fn get_app_info() -> Result<Json<Value>, ApiError> {
    to_json(commands::system::get_app_info())
}

async fn get_interfaces() -> Result<Json<Vec<gm_capture::NetworkInterface>>, ApiError> {
    let interfaces = commands::system::list_interfaces().map_err(ApiError::bad_request)?;
    Ok(Json(interfaces))
}

async fn list_import_pcap_files() -> Result<Json<ImportPcapFilesResponse>, ApiError> {
    Ok(Json(list_import_files_for_kind(ImportKind::Pcap)?))
}

async fn list_import_files(
    Path(kind): Path<String>,
) -> Result<Json<ImportPcapFilesResponse>, ApiError> {
    let kind = kind.parse::<ImportKind>().map_err(|_| {
        ApiError::bad_request(format!(
            "unsupported import kind '{}'. expected one of: {}",
            kind,
            ImportKind::supported_values_csv()
        ))
    })?;
    Ok(Json(list_import_files_for_kind(kind)?))
}

async fn import_pcap(
    State(state): State<SharedState>,
    Json(request): Json<ImportPcapRequest>,
) -> Result<Json<ImportResult>, ApiError> {
    if request.paths.is_empty() {
        return Err(ApiError::bad_request(
            "paths must contain at least one PCAP path",
        ));
    }

    let start = Instant::now();
    let reader = PcapReader::new();
    let mut processor = PacketProcessor::new();
    let mut per_file_results: Vec<FileImportResult> = Vec::new();

    let paths: Vec<String> = request
        .paths
        .iter()
        .map(|path| resolve_import_input_path(path, ImportKind::Pcap))
        .collect::<Result<Vec<_>, _>>()?;

    for path in &paths {
        let filename = std::path::Path::new(path)
            .file_name()
            .map(|f| f.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.clone());

        match reader.read_file(path) {
            Ok(packets) => {
                for packet in &packets {
                    processor.process_packet(packet);
                }
                per_file_results.push(FileImportResult {
                    filename: filename.clone(),
                    packet_count: packets.len(),
                    status: "ok".to_string(),
                });
                if let Some(tx) = &state.event_tx {
                    let file_index = per_file_results.len() - 1;
                    let file_count = paths.len();
                    let progress = (file_index + 1) as f64 / file_count as f64 * 100.0;
                    let _ = tx.send((
                        "import_progress".to_string(),
                        json!({
                            "current_file": filename,
                            "file_index": file_index,
                            "file_count": file_count,
                            "packets_processed": packets.len(),
                            "bytes_processed": 0,
                            "file_size": 0,
                            "progress_percent": progress,
                            "elapsed_secs": start.elapsed().as_secs_f64(),
                        }),
                    ));
                }
            }
            Err(e) => {
                per_file_results.push(FileImportResult {
                    filename,
                    packet_count: 0,
                    status: format!("error: {}", e),
                });
            }
        }
    }

    let total_packet_count: usize = per_file_results.iter().map(|r| r.packet_count).sum();
    if total_packet_count == 0 && !per_file_results.iter().any(|r| r.status == "ok") {
        return Err(ApiError::bad_request(
            "No packets could be parsed from the provided files",
        ));
    }

    let deep_parse_info = processor.build_deep_parse_info();
    let (assets, sig_results) = {
        let sigs = state
            .signatures
            .read()
            .map_err(|e| ApiError::internal(e.to_string()))?;
        let inv = state
            .inventory
            .read()
            .map_err(|e| ApiError::internal(e.to_string()))?;
        processor.build_assets(
            &sigs.signature_engine,
            &deep_parse_info,
            &inv.oui_lookup,
            &inv.geoip_lookup,
        )
    };

    let mut topology = processor.topo_builder.snapshot();
    for node in &mut topology.nodes {
        if let Some(sig_matches) = sig_results.get(&node.ip_address) {
            if let Some(best) = sig_matches.first() {
                if let Some(ref v) = best.vendor {
                    node.vendor = Some(v.clone());
                }
                if let Some(ref dt) = best.device_type {
                    if best.confidence >= 3 {
                        node.device_type = dt.clone();
                    }
                }
            }
        }
    }

    let connection_list = processor.get_connections();
    let packet_summaries = processor.get_packet_summaries();
    let (connection_stats, pattern_anomalies) = processor.build_pattern_results();
    let redundancy_protocols = processor.build_redundancy_info();
    let asset_count = assets.len();
    let connection_count = connection_list.len();
    let protocols_detected = processor.get_protocols_detected();

    let imported_files: Vec<String> = per_file_results
        .iter()
        .filter(|f| f.status == "ok")
        .map(|f| f.filename.clone())
        .collect();

    {
        let mut cap = state
            .capture
            .write()
            .map_err(|e| ApiError::internal(e.to_string()))?;
        cap.topology = topology;
        cap.connections = connection_list;
        cap.packet_summaries = packet_summaries;
        cap.redundancy_protocols = redundancy_protocols;
        cap.imported_files.extend(imported_files);
        cap.imported_files.sort();
        cap.imported_files.dedup();
    }
    {
        let mut inv = state
            .inventory
            .write()
            .map_err(|e| ApiError::internal(e.to_string()))?;
        inv.assets = assets;
        inv.deep_parse_info = deep_parse_info;
    }
    {
        let mut analysis = state
            .analysis
            .write()
            .map_err(|e| ApiError::internal(e.to_string()))?;
        analysis.connection_stats = connection_stats;
        analysis.pattern_anomalies = pattern_anomalies;
    }

    let duration_ms = start.elapsed().as_millis() as u64;

    Ok(Json(ImportResult {
        file_count: request.paths.len(),
        packet_count: total_packet_count,
        connection_count,
        asset_count,
        protocols_detected,
        duration_ms,
        per_file: per_file_results,
    }))
}

async fn get_topology(State(state): State<SharedState>) -> Result<Json<TopologyGraph>, ApiError> {
    let topology = commands::data::get_topology(state.as_ref()).map_err(ApiError::bad_request)?;
    Ok(Json(topology))
}

async fn get_assets(
    State(state): State<SharedState>,
    Query(query): Query<PagingQuery>,
) -> Result<Json<AssetPage>, ApiError> {
    let page =
        commands::data::get_assets(state.as_ref(), query.page, query.page_size, query.sort_by)
            .map_err(ApiError::bad_request)?;
    Ok(Json(page))
}

async fn get_connections(
    State(state): State<SharedState>,
    Query(query): Query<PagingQuery>,
) -> Result<Json<ConnectionPage>, ApiError> {
    let page =
        commands::data::get_connections(state.as_ref(), query.page, query.page_size, query.sort_by)
            .map_err(ApiError::bad_request)?;
    Ok(Json(page))
}

async fn get_counts(State(state): State<SharedState>) -> Result<Json<DataCounts>, ApiError> {
    let counts = commands::data::get_data_counts(state.as_ref()).map_err(ApiError::bad_request)?;
    Ok(Json(counts))
}

async fn get_protocol_stats(
    State(state): State<SharedState>,
) -> Result<Json<Vec<ProtocolStatInfo>>, ApiError> {
    let stats =
        commands::data::get_protocol_stats(state.as_ref()).map_err(ApiError::bad_request)?;
    Ok(Json(stats))
}

async fn get_connection_packets(
    State(state): State<SharedState>,
    Path(connection_id): Path<String>,
) -> Result<Json<Vec<commands::PacketSummary>>, ApiError> {
    let packets = commands::data::get_connection_packets(connection_id, state.as_ref())
        .map_err(ApiError::bad_request)?;
    Ok(Json(packets))
}

fn to_json<T: Serialize>(value: T) -> Result<Json<Value>, ApiError> {
    serde_json::to_value(value)
        .map(Json)
        .map_err(|e| ApiError::internal(e.to_string()))
}

fn arg<T: DeserializeOwned>(payload: &Value, names: &[&str]) -> Result<T, ApiError> {
    for name in names {
        if let Some(raw) = payload.get(name) {
            if raw.is_null() {
                continue;
            }
            return serde_json::from_value(raw.clone())
                .map_err(|e| ApiError::bad_request(format!("invalid argument '{name}': {}", e)));
        }
    }
    Err(ApiError::bad_request(format!(
        "missing argument '{}'",
        names.join("' or '")
    )))
}

fn arg_opt<T: DeserializeOwned>(payload: &Value, names: &[&str]) -> Result<Option<T>, ApiError> {
    for name in names {
        if let Some(raw) = payload.get(name) {
            if raw.is_null() {
                return Ok(None);
            }
            let parsed = serde_json::from_value(raw.clone())
                .map_err(|e| ApiError::bad_request(format!("invalid argument '{name}': {}", e)))?;
            return Ok(Some(parsed));
        }
    }
    Ok(None)
}

fn spawn_processing_thread_headless(
    rx: mpsc::Receiver<ParsedPacket>,
    state: SharedState,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut processor = PacketProcessor::new();
        let mut batch: Vec<ParsedPacket> = Vec::new();
        let mut last_flush = Instant::now();
        let flush_interval = Duration::from_millis(250);

        loop {
            match rx.recv_timeout(Duration::from_millis(50)) {
                Ok(packet) => {
                    batch.push(packet);
                    if batch.len() >= 500 || last_flush.elapsed() >= flush_interval {
                        flush_batch_headless(&state, &mut processor, &mut batch);
                        last_flush = Instant::now();
                    }
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    if !batch.is_empty() && last_flush.elapsed() >= flush_interval {
                        flush_batch_headless(&state, &mut processor, &mut batch);
                        last_flush = Instant::now();
                    }
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    if !batch.is_empty() {
                        flush_batch_headless(&state, &mut processor, &mut batch);
                    }
                    break;
                }
            }
        }
    })
}

fn flush_batch_headless(
    state: &SharedState,
    processor: &mut PacketProcessor,
    batch: &mut Vec<ParsedPacket>,
) {
    for packet in batch.drain(..) {
        processor.process_packet(&packet);
    }

    let deep_parse_info = processor.build_deep_parse_info();
    let (assets, sig_results) = {
        let sigs = match state.signatures.read() {
            Ok(v) => v,
            Err(e) => {
                log::error!("capture flush signatures lock error: {}", e);
                return;
            }
        };
        let inv = match state.inventory.read() {
            Ok(v) => v,
            Err(e) => {
                log::error!("capture flush inventory lock error: {}", e);
                return;
            }
        };
        processor.build_assets(
            &sigs.signature_engine,
            &deep_parse_info,
            &inv.oui_lookup,
            &inv.geoip_lookup,
        )
    };

    let mut topology = processor.topo_builder.snapshot();
    for node in &mut topology.nodes {
        if let Some(sig_matches) = sig_results.get(&node.ip_address) {
            if let Some(best) = sig_matches.first() {
                if let Some(ref v) = best.vendor {
                    node.vendor = Some(v.clone());
                }
                if let Some(ref dt) = best.device_type {
                    if best.confidence >= 3 {
                        node.device_type = dt.clone();
                    }
                }
            }
        }
    }

    let connections = processor.get_connections();
    let packet_summaries = processor.get_packet_summaries();
    let (connection_stats, pattern_anomalies) = processor.build_pattern_results();
    let redundancy_protocols = processor.build_redundancy_info();

    if let Ok(mut cap) = state.capture.write() {
        cap.topology = topology;
        cap.connections = connections;
        cap.packet_summaries = packet_summaries;
        cap.redundancy_protocols = redundancy_protocols;
    }
    if let Ok(mut inv) = state.inventory.write() {
        inv.assets = assets;
        inv.deep_parse_info = deep_parse_info;
    }
    if let Ok(mut analysis) = state.analysis.write() {
        analysis.connection_stats = connection_stats;
        analysis.pattern_anomalies = pattern_anomalies;
    }

    // Emit capture_stats event for SSE subscribers
    if let Some(tx) = &state.event_tx {
        let asset_count = state
            .inventory
            .read()
            .map(|inv| inv.assets.len())
            .unwrap_or(0);
        let connection_count = state
            .capture
            .read()
            .map(|cap| cap.connections.len())
            .unwrap_or(0);
        let _ = tx.send((
            "capture_stats".to_string(),
            json!({
                "packets_captured": 0,
                "packets_per_second": 0,
                "bytes_captured": 0,
                "active_connections": connection_count,
                "asset_count": asset_count,
                "elapsed_seconds": 0.0,
            }),
        ));
    }
}

async fn start_capture_headless(
    state: SharedState,
    interface_name: String,
    bpf_filter: Option<String>,
) -> Result<(), ApiError> {
    {
        let cap = state
            .capture
            .read()
            .map_err(|e| ApiError::internal(e.to_string()))?;
        if cap.live_capture.is_some() {
            return Err(ApiError::bad_request(
                "A capture is already running. Stop it first.",
            ));
        }
    }

    let config = LiveCaptureConfig {
        interface_name: interface_name.clone(),
        bpf_filter: bpf_filter.clone(),
        promiscuous: true,
        ring_buffer_size: 1_000_000,
        snaplen: 65_535,
    };

    let (handle, rx) = gm_capture::LiveCaptureHandle::start(config)
        .map_err(|e| ApiError::bad_request(e.to_string()))?;
    let processing = spawn_processing_thread_headless(rx, state.clone());

    let mut cap = state
        .capture
        .write()
        .map_err(|e| ApiError::internal(e.to_string()))?;
    cap.live_capture = Some(handle);
    cap.processing_thread = Some(processing);

    log::info!(
        "Headless live capture started on {} (filter: {:?})",
        interface_name,
        bpf_filter
    );
    Ok(())
}

// ── /api/v1/projects ─────────────────────────────────────────────────────────

async fn list_projects_handler(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    let projects = commands::projects::list_projects(state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    to_json(projects)
}

async fn create_project_handler(
    State(state): State<SharedState>,
    Json(body): Json<CreateProjectRequest>,
) -> Result<Json<Value>, ApiError> {
    let project = commands::projects::create_project(
        state.as_ref(),
        body.name,
        body.client_name,
        body.site_name,
        body.assessor_name,
        body.engagement_start,
        body.engagement_end,
        body.notes,
    )
    .await
    .map_err(ApiError::bad_request)?;
    to_json(project)
}

async fn get_project_handler(
    Path(id): Path<i64>,
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    let project = commands::projects::get_project(state.as_ref(), id)
        .await
        .map_err(ApiError::bad_request)?;
    to_json(project)
}

async fn update_project_handler(
    Path(id): Path<i64>,
    State(state): State<SharedState>,
    Json(body): Json<UpdateProjectRequest>,
) -> Result<Json<Value>, ApiError> {
    let project = commands::projects::update_project(
        state.as_ref(),
        id,
        body.name,
        body.client_name,
        body.site_name,
        body.assessor_name,
        body.engagement_start,
        body.engagement_end,
        body.notes,
    )
    .await
    .map_err(ApiError::bad_request)?;
    to_json(project)
}

async fn delete_project_handler(
    Path(id): Path<i64>,
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    commands::projects::delete_project(state.as_ref(), id)
        .await
        .map_err(ApiError::bad_request)?;
    Ok(Json(json!({})))
}

async fn set_active_project_handler(
    State(state): State<SharedState>,
    Json(body): Json<SetActiveProjectRequest>,
) -> Result<Json<Value>, ApiError> {
    let project = commands::projects::set_active_project(state.as_ref(), body.id)
        .await
        .map_err(ApiError::bad_request)?;
    to_json(project)
}

async fn clear_active_project_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    commands::projects::clear_active_project(state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    Ok(Json(json!({})))
}

// ── /api/v1/sessions ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SaveSessionRequest {
    name: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExportSessionRequest {
    output_path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImportSessionRequest {
    archive_path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CompareSessionsRequest {
    baseline_session_id: String,
}

async fn list_sessions_handler(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    let sessions = commands::session::list_sessions(state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    to_json(sessions)
}

async fn save_session_handler(
    State(state): State<SharedState>,
    Json(body): Json<SaveSessionRequest>,
) -> Result<Json<Value>, ApiError> {
    let session = commands::session::save_session(body.name, body.description, state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    to_json(session)
}

async fn load_session_handler(
    Path(id): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    let session = commands::session::load_session(id, state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    to_json(session)
}

async fn delete_session_handler(
    Path(id): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    commands::session::delete_session(id, state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    Ok(Json(json!({})))
}

async fn export_session_handler(
    Path(id): Path<String>,
    State(state): State<SharedState>,
    Json(body): Json<ExportSessionRequest>,
) -> Result<Json<Value>, ApiError> {
    let output_path = resolve_export_output_path(&body.output_path, "session.kkj")?;
    let path = commands::session::export_session_archive(id, output_path, state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    to_json(path)
}

async fn import_session_handler(
    State(state): State<SharedState>,
    Json(body): Json<ImportSessionRequest>,
) -> Result<Json<Value>, ApiError> {
    let archive_path = resolve_import_input_path(&body.archive_path, ImportKind::SessionArchive)?;
    let session = commands::session::import_session_archive(archive_path, state.as_ref())
        .await
        .map_err(ApiError::bad_request)?;
    to_json(session)
}

async fn compare_sessions_handler(
    State(state): State<SharedState>,
    Json(body): Json<CompareSessionsRequest>,
) -> Result<Json<Value>, ApiError> {
    let diff = commands::baseline::compare_sessions(body.baseline_session_id, state.as_ref())
        .map_err(ApiError::bad_request)?;
    to_json(diff)
}

// ── /api/v1/analysis ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct ComplianceQuery {
    framework: String,
}

#[derive(Debug, Deserialize)]
struct CveQuery {
    ip: String,
}

async fn run_analysis_handler(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    let result = commands::analysis::run_analysis(state.as_ref()).map_err(ApiError::bad_request)?;
    to_json(result)
}

async fn get_findings_handler(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    to_json(commands::analysis::get_findings(state.as_ref()).map_err(ApiError::bad_request)?)
}

async fn get_purdue_handler(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::analysis::get_purdue_assignments(state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

async fn get_anomalies_handler(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    to_json(commands::analysis::get_anomalies(state.as_ref()).map_err(ApiError::bad_request)?)
}

async fn get_credentials_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::analysis::get_credential_warnings(state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

async fn get_criticality_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(commands::analysis::get_criticality(state.as_ref()).map_err(ApiError::bad_request)?)
}

async fn get_naming_suggestions_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::analysis::get_naming_suggestions(state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

async fn get_malware_handler(State(state): State<SharedState>) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::analysis::get_malware_findings(state.as_ref()).map_err(ApiError::bad_request)?,
    )
}

async fn get_switch_security_handler(
    State(state): State<SharedState>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::analysis::get_switch_security_findings(state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

async fn get_compliance_handler(
    State(state): State<SharedState>,
    Query(query): Query<ComplianceQuery>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::analysis::get_compliance_report(state.as_ref(), query.framework)
            .map_err(ApiError::bad_request)?,
    )
}

async fn get_cve_handler(
    State(state): State<SharedState>,
    Query(query): Query<CveQuery>,
) -> Result<Json<Value>, ApiError> {
    to_json(
        commands::analysis::get_cve_warnings(query.ip, state.as_ref())
            .map_err(ApiError::bad_request)?,
    )
}

// ── /api/v1/events (SSE) ─────────────────────────────────────────────────────

/// Server-Sent Events stream for real-time capture and import progress updates.
///
/// Each message is an SSE event with:
///   - `event:` field set to the event type (`capture_stats`, `import_progress`)
///   - `data:` field containing JSON-serialized payload matching the TypeScript interface
async fn events_handler(
    State(state): State<SharedState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state
        .event_tx
        .as_ref()
        .expect("event_tx always set in web mode")
        .subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|msg| match msg {
        Ok((event_type, data)) => {
            let data_str = serde_json::to_string(&data).ok()?;
            Some(Ok(Event::default().event(event_type).data(data_str)))
        }
        Err(_) => None, // lagged receiver — drop silently
    });
    Sse::new(stream).keep_alive(KeepAlive::default())
}

async fn invoke_command(
    Path(command): Path<String>,
    State(state): State<SharedState>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, ApiError> {
    let payload = if payload.is_object() {
        payload
    } else {
        json!({})
    };

    let out = match command.as_str() {
        "cancel_import" => to_json(
            commands::capture::cancel_import(state.as_ref())
                .await
                .map_err(ApiError::bad_request)?,
        )?,
        "get_signatures" => to_json(
            commands::signatures::get_signatures(state.as_ref()).map_err(ApiError::bad_request)?,
        )?,
        "reload_signatures" => to_json(
            commands::signatures::reload_signatures(state.as_ref())
                .map_err(ApiError::bad_request)?,
        )?,
        "test_signature" => {
            let yaml: String = arg(&payload, &["yaml"])?;
            to_json(
                commands::signatures::test_signature(yaml, state.as_ref())
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "get_deep_parse_info" => {
            let ip: String = arg(&payload, &["ipAddress", "ip_address"])?;
            to_json(
                commands::data::get_deep_parse_info(ip, state.as_ref())
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "get_function_code_stats" => to_json(
            commands::data::get_function_code_stats(state.as_ref())
                .map_err(ApiError::bad_request)?,
        )?,
        "start_capture" => {
            let interface_name: String = arg(&payload, &["interfaceName", "interface_name"])?;
            let bpf_filter: Option<String> = arg_opt(&payload, &["bpfFilter", "bpf_filter"])?;
            start_capture_headless(state.clone(), interface_name, bpf_filter).await?;
            to_json(())?
        }
        "stop_capture" => {
            let save_path: Option<String> = arg_opt(&payload, &["savePath", "save_path"])?;
            let save_path = save_path
                .as_deref()
                .map(|v| resolve_export_output_path(v, "capture.pcap"))
                .transpose()?;
            to_json(
                commands::capture::stop_capture(save_path, state.as_ref())
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "pause_capture" => to_json(
            commands::capture::pause_capture(state.as_ref())
                .await
                .map_err(ApiError::bad_request)?,
        )?,
        "resume_capture" => to_json(
            commands::capture::resume_capture(state.as_ref())
                .await
                .map_err(ApiError::bad_request)?,
        )?,
        "get_capture_status" => to_json(
            commands::capture::get_capture_status(state.as_ref())
                .await
                .map_err(ApiError::bad_request)?,
        )?,
        "update_asset" => {
            let asset_id: String = arg(&payload, &["assetId", "asset_id"])?;
            let updates: commands::session::AssetUpdate = arg(&payload, &["updates"])?;
            to_json(
                commands::session::update_asset(asset_id, updates, state.as_ref())
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "bulk_update_assets" => {
            let asset_ids: Vec<String> = arg(&payload, &["assetIds", "asset_ids"])?;
            let updates: commands::session::AssetUpdate = arg(&payload, &["updates"])?;
            to_json(
                commands::session::bulk_update_assets(asset_ids, updates, state.as_ref())
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "import_cisco_config" => {
            let path: String = arg(&payload, &["path"])?;
            let path = resolve_import_input_path(&path, ImportKind::PhysicalConfig)?;
            to_json(
                commands::physical::import_cisco_config(path, state.as_ref())
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "import_mac_table" => {
            let path: String = arg(&payload, &["path"])?;
            let switch_hostname: String = arg(&payload, &["switchHostname", "switch_hostname"])?;
            let path = resolve_import_input_path(&path, ImportKind::PhysicalMac)?;
            to_json(
                commands::physical::import_mac_table(path, switch_hostname, state.as_ref())
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "import_cdp_neighbors" => {
            let path: String = arg(&payload, &["path"])?;
            let switch_hostname: String = arg(&payload, &["switchHostname", "switch_hostname"])?;
            let path = resolve_import_input_path(&path, ImportKind::PhysicalNeighbor)?;
            to_json(
                commands::physical::import_cdp_neighbors(path, switch_hostname, state.as_ref())
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "import_arp_table" => {
            let path: String = arg(&payload, &["path"])?;
            let path = resolve_import_input_path(&path, ImportKind::PhysicalArp)?;
            to_json(
                commands::physical::import_arp_table(path, state.as_ref())
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "get_physical_topology" => to_json(
            commands::physical::get_physical_topology(state.as_ref())
                .map_err(ApiError::bad_request)?,
        )?,
        "clear_physical_topology" => to_json(
            commands::physical::clear_physical_topology(state.as_ref())
                .map_err(ApiError::bad_request)?,
        )?,
        "import_network_config" => {
            let path: String = arg(&payload, &["path"])?;
            let path = resolve_import_input_path(&path, ImportKind::PhysicalConfig)?;
            to_json(
                commands::physical::import_network_config(path, state.as_ref())
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "import_mac_table_auto" => {
            let path: String = arg(&payload, &["path"])?;
            let switch_hostname: String = arg(&payload, &["switchHostname", "switch_hostname"])?;
            let path = resolve_import_input_path(&path, ImportKind::PhysicalMac)?;
            to_json(
                commands::physical::import_mac_table_auto(path, switch_hostname, state.as_ref())
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "import_neighbor_table" => {
            let path: String = arg(&payload, &["path"])?;
            let switch_hostname: String = arg(&payload, &["switchHostname", "switch_hostname"])?;
            let path = resolve_import_input_path(&path, ImportKind::PhysicalNeighbor)?;
            to_json(
                commands::physical::import_neighbor_table(path, switch_hostname, state.as_ref())
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "run_topology_inference" => to_json(
            commands::physical::run_topology_inference(state.as_ref())
                .map_err(ApiError::bad_request)?,
        )?,
        "get_inferred_topology" => to_json(
            commands::physical::get_inferred_topology(state.as_ref())
                .map_err(ApiError::bad_request)?,
        )?,
        "import_zeek_logs" => {
            let paths: Vec<String> = arg(&payload, &["paths"])?;
            let paths = paths
                .iter()
                .map(|path| resolve_import_input_path(path, ImportKind::Zeek))
                .collect::<Result<Vec<_>, _>>()?;
            to_json(
                commands::ingest::import_zeek_logs(paths, state.as_ref())
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "import_suricata_eve" => {
            let path: String = arg(&payload, &["path"])?;
            let path = resolve_import_input_path(&path, ImportKind::Suricata)?;
            to_json(
                commands::ingest::import_suricata_eve(path, state.as_ref())
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "import_nmap_xml" => {
            let path: String = arg(&payload, &["path"])?;
            let path = resolve_import_input_path(&path, ImportKind::Nmap)?;
            to_json(
                commands::ingest::import_nmap_xml(path, state.as_ref())
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "import_masscan_json" => {
            let path: String = arg(&payload, &["path"])?;
            let path = resolve_import_input_path(&path, ImportKind::Masscan)?;
            to_json(
                commands::ingest::import_masscan_json(path, state.as_ref())
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "import_wazuh_alerts" => {
            let path: String = arg(&payload, &["path"])?;
            let path = resolve_import_input_path(&path, ImportKind::Wazuh)?;
            to_json(
                commands::ingest::import_wazuh_alerts(path, state.as_ref())
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "import_sinema_csv" => {
            let path: String = arg(&payload, &["path"])?;
            let path = resolve_import_input_path(&path, ImportKind::Sinema)?;
            to_json(
                commands::ingest::import_sinema_csv(path, state.as_ref())
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "import_tia_xml" => {
            let path: String = arg(&payload, &["path"])?;
            let path = resolve_import_input_path(&path, ImportKind::Tia)?;
            to_json(
                commands::ingest::import_tia_xml(path, state.as_ref())
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "get_device_zeek_events" => {
            let device_ip: String = arg(&payload, &["deviceIp", "device_ip"])?;
            to_json(
                commands::ingest::get_device_zeek_events(device_ip, state.as_ref())
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "detect_wireshark" => to_json(
            commands::wireshark::detect_wireshark()
                .await
                .map_err(ApiError::bad_request)?,
        )?,
        "open_in_wireshark" => {
            let connection_id: String = arg(&payload, &["connectionId", "connection_id"])?;
            to_json(
                commands::wireshark::open_in_wireshark(connection_id, state.as_ref())
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "open_wireshark_for_node" => {
            let ip_address: String = arg(&payload, &["ipAddress", "ip_address"])?;
            to_json(
                commands::wireshark::open_wireshark_for_node(ip_address)
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "get_connection_frames" => {
            let connection_id: String = arg(&payload, &["connectionId", "connection_id"])?;
            to_json(
                commands::wireshark::get_connection_frames(connection_id, state.as_ref())
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "export_frames_csv" => {
            let connection_id: String = arg(&payload, &["connectionId", "connection_id"])?;
            to_json(
                commands::wireshark::export_frames_csv(connection_id, state.as_ref())
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "save_frames_csv" => {
            let connection_id: String = arg(&payload, &["connectionId", "connection_id"])?;
            let output_path: String = arg(&payload, &["outputPath", "output_path"])?;
            let output_path = resolve_export_output_path(&output_path, "frames.csv")?;
            to_json(
                commands::wireshark::save_frames_csv(connection_id, output_path, state.as_ref())
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "export_assets_csv" => {
            let output_path: String = arg(&payload, &["outputPath", "output_path"])?;
            let output_path = resolve_export_output_path(&output_path, "assets.csv")?;
            to_json(
                commands::export::export_assets_csv(output_path, state.as_ref())
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "export_connections_csv" => {
            let output_path: String = arg(&payload, &["outputPath", "output_path"])?;
            let output_path = resolve_export_output_path(&output_path, "connections.csv")?;
            to_json(
                commands::export::export_connections_csv(output_path, state.as_ref())
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "export_topology_json" => {
            let output_path: String = arg(&payload, &["outputPath", "output_path"])?;
            let output_path = resolve_export_output_path(&output_path, "topology.json")?;
            to_json(
                commands::export::export_topology_json(output_path, state.as_ref())
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "export_assets_json" => {
            let output_path: String = arg(&payload, &["outputPath", "output_path"])?;
            let output_path = resolve_export_output_path(&output_path, "assets.json")?;
            to_json(
                commands::export::export_assets_json(output_path, state.as_ref())
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "generate_pdf_report" => {
            let config: commands::export::ReportConfigInput = arg(&payload, &["config"])?;
            let output_path: String = arg(&payload, &["outputPath", "output_path"])?;
            let output_path = resolve_export_output_path(&output_path, "assessment_report.pdf")?;
            to_json(
                commands::export::generate_pdf_report(config, output_path, state.as_ref())
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "export_sbom" => {
            let format: String = arg(&payload, &["format"])?;
            let output_path: String = arg(&payload, &["outputPath", "output_path"])?;
            let output_path = resolve_export_output_path(&output_path, "sbom.json")?;
            to_json(
                commands::export::export_sbom(format, output_path, state.as_ref())
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "export_stix_bundle" => {
            let output_path: String = arg(&payload, &["outputPath", "output_path"])?;
            let output_path = resolve_export_output_path(&output_path, "stix_bundle.json")?;
            to_json(
                commands::export::export_stix_bundle(output_path, state.as_ref())
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "save_topology_image" => {
            let image_data: String = arg(&payload, &["imageData", "image_data"])?;
            let output_path: String = arg(&payload, &["outputPath", "output_path"])?;
            let output_path = resolve_export_output_path(&output_path, "topology.png")?;
            to_json(
                commands::export::save_topology_image(image_data, output_path)
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "export_filtered_pcap" => {
            let filter_ips: Vec<String> = arg(&payload, &["filterIps", "filter_ips"])?;
            let filter_ports: Vec<u16> = arg(&payload, &["filterPorts", "filter_ports"])?;
            let output_path: String = arg(&payload, &["outputPath", "output_path"])?;
            let output_path = resolve_export_output_path(&output_path, "filtered.pcap")?;
            to_json(
                commands::export::export_filtered_pcap(
                    filter_ips,
                    filter_ports,
                    output_path,
                    state.as_ref(),
                )
                .await
                .map_err(ApiError::bad_request)?,
            )?
        }
        "get_settings" => {
            to_json(commands::system::get_settings().map_err(ApiError::bad_request)?)?
        }
        "save_settings" => {
            let settings: commands::system::UserSettings = arg(&payload, &["settings"])?;
            to_json(commands::system::save_settings(settings).map_err(ApiError::bad_request)?)?
        }
        "get_timeline_range" => to_json(
            commands::data::get_timeline_range(state.as_ref()).map_err(ApiError::bad_request)?,
        )?,
        "list_plugins" => {
            to_json(commands::system::list_plugins().map_err(ApiError::bad_request)?)?
        }
        "get_connection_stats" => to_json(
            commands::patterns::get_connection_stats(state.as_ref())
                .map_err(ApiError::bad_request)?,
        )?,
        "get_pattern_anomalies" => to_json(
            commands::patterns::get_pattern_anomalies(state.as_ref())
                .map_err(ApiError::bad_request)?,
        )?,
        "get_redundancy_protocols" => to_json(
            commands::patterns::get_redundancy_protocols(state.as_ref())
                .map_err(ApiError::bad_request)?,
        )?,
        "get_correlated_alerts" => to_json(
            commands::correlation::get_correlated_alerts(state.as_ref())
                .await
                .map_err(ApiError::bad_request)?,
        )?,
        "get_alerts_for_ip" => {
            let ip: String = arg(&payload, &["ip"])?;
            to_json(
                commands::correlation::get_alerts_for_ip(ip, state.as_ref())
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "clear_alerts" => to_json(
            commands::correlation::clear_alerts(state.as_ref())
                .await
                .map_err(ApiError::bad_request)?,
        )?,
        "generate_communication_allowlist" => to_json(
            commands::export::generate_communication_allowlist(state.as_ref())
                .await
                .map_err(ApiError::bad_request)?,
        )?,
        "export_allowlist_csv" => {
            let output_path: String = arg(&payload, &["outputPath", "output_path"])?;
            let output_path = resolve_export_output_path(&output_path, "allowlist.csv")?;
            to_json(
                commands::export::export_allowlist_csv(output_path, state.as_ref())
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "export_firewall_rules" => {
            let output_path: String = arg(&payload, &["outputPath", "output_path"])?;
            let output_path = resolve_export_output_path(&output_path, "firewall_rules.txt")?;
            to_json(
                commands::export::export_firewall_rules(output_path, state.as_ref())
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "run_segmentation" => to_json(
            commands::segmentation::run_segmentation(state.as_ref())
                .map_err(ApiError::bad_request)?,
        )?,
        "export_enforcement_config" => {
            let format: String = arg(&payload, &["format"])?;
            to_json(
                commands::segmentation::export_enforcement_config(format, state.as_ref())
                    .map_err(ApiError::bad_request)?,
            )?
        }
        other => {
            return Err(ApiError::bad_request(format!(
                "unsupported command in headless mode: {}",
                other
            )));
        }
    };

    Ok(out)
}

#![allow(dead_code)]

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use clap::Parser;
use gm_capture::{list_interfaces, LiveCaptureConfig, ParsedPacket, PcapReader};
use gm_topology::TopologyGraph;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::path::{Path as StdPath, PathBuf};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use tower_http::services::{ServeDir, ServeFile};

#[path = "../commands/mod.rs"]
mod commands;

use commands::capture::{FileImportResult, ImportResult};
use commands::data::{AssetPage, ConnectionPage, DataCounts};
use commands::processor::PacketProcessor;
use commands::AppState;
use commands::ProtocolStatInfo;

const MAX_TOPOLOGY_NODES: usize = 5_000;
const MAX_TOPOLOGY_EDGES: usize = 20_000;
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
struct PagingQuery {
    page: Option<usize>,
    #[serde(alias = "pageSize")]
    page_size: Option<usize>,
    #[serde(alias = "sortBy")]
    sort_by: Option<String>,
}

#[derive(Debug, Serialize)]
struct AppInfo {
    version: String,
    rust_version: String,
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

    fn from_str(value: &str) -> Option<Self> {
        match value {
            "pcap" => Some(ImportKind::Pcap),
            "physical_config" => Some(ImportKind::PhysicalConfig),
            "physical_mac" => Some(ImportKind::PhysicalMac),
            "physical_neighbor" => Some(ImportKind::PhysicalNeighbor),
            "physical_arp" => Some(ImportKind::PhysicalArp),
            "zeek" => Some(ImportKind::Zeek),
            "suricata" => Some(ImportKind::Suricata),
            "nmap" => Some(ImportKind::Nmap),
            "masscan" => Some(ImportKind::Masscan),
            "wazuh" => Some(ImportKind::Wazuh),
            "sinema" => Some(ImportKind::Sinema),
            "tia" => Some(ImportKind::Tia),
            "session_archive" => Some(ImportKind::SessionArchive),
            _ => None,
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let cli = Cli::parse();
    let frontend_dist = resolve_frontend_dist(cli.frontend_dist);
    let index_path = frontend_dist.join("index.html");

    let state: SharedState = Arc::new(AppState::new(commands::resource_paths::ResourcePaths::from_env()));

    let api = Router::new()
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
        );

    let static_files = ServeDir::new(&frontend_dist).not_found_service(ServeFile::new(index_path));

    let app = Router::new()
        .nest("/api", api)
        .fallback_service(static_files)
        .with_state(state);

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
        if path.exists() {
            return path;
        }
    }

    let candidates = [
        PathBuf::from("build"),
        PathBuf::from("../build"),
        PathBuf::from("./build"),
    ];

    for candidate in candidates {
        if candidate.join("index.html").exists() {
            return candidate;
        }
    }

    PathBuf::from("build")
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
    allowed.iter().any(|candidate| ext.eq_ignore_ascii_case(candidate))
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

async fn get_app_info() -> Json<AppInfo> {
    Json(AppInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        rust_version: format!("rustc {}", env!("CARGO_PKG_RUST_VERSION")),
    })
}

async fn get_interfaces() -> Result<Json<Vec<gm_capture::NetworkInterface>>, ApiError> {
    let interfaces = list_interfaces().map_err(|e| ApiError::internal(e.to_string()))?;
    Ok(Json(interfaces))
}

async fn list_import_pcap_files() -> Result<Json<ImportPcapFilesResponse>, ApiError> {
    Ok(Json(list_import_files_for_kind(ImportKind::Pcap)?))
}

async fn list_import_files(
    Path(kind): Path<String>,
) -> Result<Json<ImportPcapFilesResponse>, ApiError> {
    let kind = ImportKind::from_str(&kind).ok_or_else(|| {
        ApiError::bad_request(format!(
            "unsupported import kind '{}'. expected one of: pcap, physical_config, physical_mac, physical_neighbor, physical_arp, zeek, suricata, nmap, masscan, wazuh, sinema, tia, session_archive",
            kind
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
                    filename,
                    packet_count: packets.len(),
                    status: "ok".to_string(),
                });
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
        let state_inner = state
            .inner
            .lock()
            .map_err(|e| ApiError::internal(e.to_string()))?;
        processor.build_assets(
            &state_inner.signature_engine,
            &deep_parse_info,
            &state_inner.oui_lookup,
            &state_inner.geoip_lookup,
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
        let mut state_inner = state
            .inner
            .lock()
            .map_err(|e| ApiError::internal(e.to_string()))?;
        state_inner.topology = topology;
        state_inner.assets = assets;
        state_inner.connections = connection_list;
        state_inner.packet_summaries = packet_summaries;
        state_inner.deep_parse_info = deep_parse_info;
        state_inner.connection_stats = connection_stats;
        state_inner.pattern_anomalies = pattern_anomalies;
        state_inner.redundancy_protocols = redundancy_protocols;
        state_inner.imported_files.extend(imported_files);
        state_inner.imported_files.sort();
        state_inner.imported_files.dedup();
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
    let state_inner = state
        .inner
        .lock()
        .map_err(|e| ApiError::internal(e.to_string()))?;
    let topo = &state_inner.topology;

    if topo.nodes.len() <= MAX_TOPOLOGY_NODES && topo.edges.len() <= MAX_TOPOLOGY_EDGES {
        return Ok(Json(topo.clone()));
    }

    let mut nodes = topo.nodes.clone();
    nodes.sort_by(|a, b| b.packet_count.cmp(&a.packet_count));
    nodes.truncate(MAX_TOPOLOGY_NODES);

    let retained: HashSet<&str> = nodes.iter().map(|n| n.id.as_str()).collect();
    let mut edges: Vec<_> = topo
        .edges
        .iter()
        .filter(|e| retained.contains(e.source.as_str()) && retained.contains(e.target.as_str()))
        .cloned()
        .collect();
    edges.sort_by(|a, b| b.packet_count.cmp(&a.packet_count));
    edges.truncate(MAX_TOPOLOGY_EDGES);

    Ok(Json(TopologyGraph { nodes, edges }))
}

async fn get_assets(
    State(state): State<SharedState>,
    Query(query): Query<PagingQuery>,
) -> Result<Json<AssetPage>, ApiError> {
    let state_inner = state
        .inner
        .lock()
        .map_err(|e| ApiError::internal(e.to_string()))?;

    let page = query.page.unwrap_or(0);
    let page_size = query.page_size.unwrap_or(200);

    let mut all_assets = state_inner.assets.clone();
    let total = all_assets.len();

    match query.sort_by.as_deref() {
        Some("ip") => all_assets.sort_by(|a, b| a.ip_address.cmp(&b.ip_address)),
        Some("packets") => all_assets.sort_by(|a, b| b.packet_count.cmp(&a.packet_count)),
        Some("protocol") => {
            all_assets.sort_by(|a, b| {
                let ap = a.protocols.first().map(|s| s.as_str()).unwrap_or("");
                let bp = b.protocols.first().map(|s| s.as_str()).unwrap_or("");
                ap.cmp(bp)
            });
        }
        _ => {}
    }

    let start = page * page_size;
    let assets = if start < total {
        all_assets.into_iter().skip(start).take(page_size).collect()
    } else {
        Vec::new()
    };
    let has_more = start + page_size < total;

    Ok(Json(AssetPage {
        assets,
        total,
        page,
        page_size,
        has_more,
    }))
}

async fn get_connections(
    State(state): State<SharedState>,
    Query(query): Query<PagingQuery>,
) -> Result<Json<ConnectionPage>, ApiError> {
    let state_inner = state
        .inner
        .lock()
        .map_err(|e| ApiError::internal(e.to_string()))?;

    let page = query.page.unwrap_or(0);
    let page_size = query.page_size.unwrap_or(500);

    let mut all_connections = state_inner.connections.clone();
    let total = all_connections.len();

    match query.sort_by.as_deref() {
        Some("packets") => {
            all_connections.sort_by(|a, b| b.packet_count.cmp(&a.packet_count));
        }
        Some("bytes") => {
            all_connections.sort_by(|a, b| b.byte_count.cmp(&a.byte_count));
        }
        _ => {}
    }

    let start = page * page_size;
    let connections = if start < total {
        all_connections
            .into_iter()
            .skip(start)
            .take(page_size)
            .collect()
    } else {
        Vec::new()
    };
    let has_more = start + page_size < total;

    Ok(Json(ConnectionPage {
        connections,
        total,
        page,
        page_size,
        has_more,
    }))
}

async fn get_counts(State(state): State<SharedState>) -> Result<Json<DataCounts>, ApiError> {
    let state_inner = state
        .inner
        .lock()
        .map_err(|e| ApiError::internal(e.to_string()))?;
    Ok(Json(DataCounts {
        asset_count: state_inner.assets.len(),
        connection_count: state_inner.connections.len(),
    }))
}

async fn get_protocol_stats(
    State(state): State<SharedState>,
) -> Result<Json<Vec<ProtocolStatInfo>>, ApiError> {
    let state_inner = state
        .inner
        .lock()
        .map_err(|e| ApiError::internal(e.to_string()))?;

    let mut stats: HashMap<String, ProtocolStatInfo> = HashMap::new();
    let mut devices_per_proto: HashMap<String, HashSet<String>> = HashMap::new();

    for conn in &state_inner.connections {
        let entry = stats
            .entry(conn.protocol.clone())
            .or_insert_with(|| ProtocolStatInfo {
                protocol: conn.protocol.clone(),
                packet_count: 0,
                byte_count: 0,
                connection_count: 0,
                unique_devices: 0,
            });
        entry.packet_count += conn.packet_count;
        entry.byte_count += conn.byte_count;
        entry.connection_count += 1;

        let dev = devices_per_proto.entry(conn.protocol.clone()).or_default();
        dev.insert(conn.src_ip.clone());
        dev.insert(conn.dst_ip.clone());
    }

    for (proto, dev_set) in &devices_per_proto {
        if let Some(stat) = stats.get_mut(proto) {
            stat.unique_devices = dev_set.len() as u64;
        }
    }

    let mut result: Vec<ProtocolStatInfo> = stats.into_values().collect();
    result.sort_by(|a, b| b.packet_count.cmp(&a.packet_count));

    Ok(Json(result))
}

async fn get_connection_packets(
    State(state): State<SharedState>,
    Path(connection_id): Path<String>,
) -> Result<Json<Vec<commands::PacketSummary>>, ApiError> {
    let state_inner = state
        .inner
        .lock()
        .map_err(|e| ApiError::internal(e.to_string()))?;
    let packets = state_inner
        .packet_summaries
        .get(&connection_id)
        .cloned()
        .unwrap_or_default();
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

fn state_ref(state: &SharedState) -> tauri::State<'_, AppState> {
    // SAFETY: tauri::State is a transparent newtype over `&T` in tauri 2.x.
    unsafe { std::mem::transmute::<&AppState, tauri::State<'_, AppState>>(state.as_ref()) }
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
        let state_inner = match state.inner.lock() {
            Ok(v) => v,
            Err(e) => {
                log::error!("capture flush lock error: {}", e);
                return;
            }
        };
        processor.build_assets(
            &state_inner.signature_engine,
            &deep_parse_info,
            &state_inner.oui_lookup,
            &state_inner.geoip_lookup,
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

    if let Ok(mut state_inner) = state.inner.lock() {
        state_inner.topology = topology;
        state_inner.assets = assets;
        state_inner.connections = connections;
        state_inner.packet_summaries = packet_summaries;
        state_inner.deep_parse_info = deep_parse_info;
        state_inner.connection_stats = connection_stats;
        state_inner.pattern_anomalies = pattern_anomalies;
        state_inner.redundancy_protocols = redundancy_protocols;
    }
}

async fn start_capture_headless(
    state: SharedState,
    interface_name: String,
    bpf_filter: Option<String>,
) -> Result<(), ApiError> {
    {
        let inner = state
            .inner
            .lock()
            .map_err(|e| ApiError::internal(e.to_string()))?;
        if inner.live_capture.is_some() {
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

    let mut inner = state
        .inner
        .lock()
        .map_err(|e| ApiError::internal(e.to_string()))?;
    inner.live_capture = Some(handle);
    inner.processing_thread = Some(processing);

    log::info!(
        "Headless live capture started on {} (filter: {:?})",
        interface_name,
        bpf_filter
    );
    Ok(())
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
            commands::capture::cancel_import(state_ref(&state))
                .await
                .map_err(ApiError::bad_request)?,
        )?,
        "get_signatures" => to_json(
            commands::signatures::get_signatures(state_ref(&state))
                .map_err(ApiError::bad_request)?,
        )?,
        "reload_signatures" => to_json(
            commands::signatures::reload_signatures(state_ref(&state))
                .map_err(ApiError::bad_request)?,
        )?,
        "test_signature" => {
            let yaml: String = arg(&payload, &["yaml"])?;
            to_json(
                commands::signatures::test_signature(yaml, state_ref(&state))
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "get_deep_parse_info" => {
            let ip: String = arg(&payload, &["ipAddress", "ip_address"])?;
            to_json(
                commands::data::get_deep_parse_info(ip, state_ref(&state))
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "get_function_code_stats" => to_json(
            commands::data::get_function_code_stats(state_ref(&state))
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
                commands::capture::stop_capture(save_path, state_ref(&state))
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "pause_capture" => to_json(
            commands::capture::pause_capture(state_ref(&state))
                .await
                .map_err(ApiError::bad_request)?,
        )?,
        "resume_capture" => to_json(
            commands::capture::resume_capture(state_ref(&state))
                .await
                .map_err(ApiError::bad_request)?,
        )?,
        "get_capture_status" => to_json(
            commands::capture::get_capture_status(state_ref(&state))
                .await
                .map_err(ApiError::bad_request)?,
        )?,
        "save_session" => {
            let name: String = arg(&payload, &["name"])?;
            let description: Option<String> = arg_opt(&payload, &["description"])?;
            to_json(
                commands::session::save_session(name, description, state_ref(&state))
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "load_session" => {
            let session_id: String = arg(&payload, &["sessionId", "session_id"])?;
            to_json(
                commands::session::load_session(session_id, state_ref(&state))
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "list_sessions" => to_json(
            commands::session::list_sessions(state_ref(&state))
                .await
                .map_err(ApiError::bad_request)?,
        )?,
        "delete_session" => {
            let session_id: String = arg(&payload, &["sessionId", "session_id"])?;
            to_json(
                commands::session::delete_session(session_id, state_ref(&state))
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "update_asset" => {
            let asset_id: String = arg(&payload, &["assetId", "asset_id"])?;
            let updates: commands::session::AssetUpdate = arg(&payload, &["updates"])?;
            to_json(
                commands::session::update_asset(asset_id, updates, state_ref(&state))
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "bulk_update_assets" => {
            let asset_ids: Vec<String> = arg(&payload, &["assetIds", "asset_ids"])?;
            let updates: commands::session::AssetUpdate = arg(&payload, &["updates"])?;
            to_json(
                commands::session::bulk_update_assets(asset_ids, updates, state_ref(&state))
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "export_session_archive" => {
            let session_id: String = arg(&payload, &["sessionId", "session_id"])?;
            let output_path: String = arg(&payload, &["outputPath", "output_path"])?;
            let output_path = resolve_export_output_path(&output_path, "session.kkj")?;
            to_json(
                commands::session::export_session_archive(
                    session_id,
                    output_path,
                    state_ref(&state),
                )
                .await
                .map_err(ApiError::bad_request)?,
            )?
        }
        "import_session_archive" => {
            let archive_path: String = arg(&payload, &["archivePath", "archive_path"])?;
            let archive_path = resolve_import_input_path(&archive_path, ImportKind::SessionArchive)?;
            to_json(
                commands::session::import_session_archive(archive_path, state_ref(&state))
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "compare_sessions" => {
            let baseline_session_id: String =
                arg(&payload, &["baselineSessionId", "baseline_session_id"])?;
            to_json(
                commands::baseline::compare_sessions(baseline_session_id, state_ref(&state))
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "import_cisco_config" => {
            let path: String = arg(&payload, &["path"])?;
            let path = resolve_import_input_path(&path, ImportKind::PhysicalConfig)?;
            to_json(
                commands::physical::import_cisco_config(path, state_ref(&state))
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "import_mac_table" => {
            let path: String = arg(&payload, &["path"])?;
            let switch_hostname: String = arg(&payload, &["switchHostname", "switch_hostname"])?;
            let path = resolve_import_input_path(&path, ImportKind::PhysicalMac)?;
            to_json(
                commands::physical::import_mac_table(path, switch_hostname, state_ref(&state))
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "import_cdp_neighbors" => {
            let path: String = arg(&payload, &["path"])?;
            let switch_hostname: String = arg(&payload, &["switchHostname", "switch_hostname"])?;
            let path = resolve_import_input_path(&path, ImportKind::PhysicalNeighbor)?;
            to_json(
                commands::physical::import_cdp_neighbors(path, switch_hostname, state_ref(&state))
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "import_arp_table" => {
            let path: String = arg(&payload, &["path"])?;
            let path = resolve_import_input_path(&path, ImportKind::PhysicalArp)?;
            to_json(
                commands::physical::import_arp_table(path, state_ref(&state))
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "get_physical_topology" => to_json(
            commands::physical::get_physical_topology(state_ref(&state))
                .map_err(ApiError::bad_request)?,
        )?,
        "clear_physical_topology" => to_json(
            commands::physical::clear_physical_topology(state_ref(&state))
                .map_err(ApiError::bad_request)?,
        )?,
        "import_network_config" => {
            let path: String = arg(&payload, &["path"])?;
            let path = resolve_import_input_path(&path, ImportKind::PhysicalConfig)?;
            to_json(
                commands::physical::import_network_config(path, state_ref(&state))
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "import_mac_table_auto" => {
            let path: String = arg(&payload, &["path"])?;
            let switch_hostname: String = arg(&payload, &["switchHostname", "switch_hostname"])?;
            let path = resolve_import_input_path(&path, ImportKind::PhysicalMac)?;
            to_json(
                commands::physical::import_mac_table_auto(path, switch_hostname, state_ref(&state))
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "import_neighbor_table" => {
            let path: String = arg(&payload, &["path"])?;
            let switch_hostname: String = arg(&payload, &["switchHostname", "switch_hostname"])?;
            let path = resolve_import_input_path(&path, ImportKind::PhysicalNeighbor)?;
            to_json(
                commands::physical::import_neighbor_table(path, switch_hostname, state_ref(&state))
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "run_topology_inference" => to_json(
            commands::physical::run_topology_inference(state_ref(&state))
                .map_err(ApiError::bad_request)?,
        )?,
        "get_inferred_topology" => to_json(
            commands::physical::get_inferred_topology(state_ref(&state))
                .map_err(ApiError::bad_request)?,
        )?,
        "import_zeek_logs" => {
            let paths: Vec<String> = arg(&payload, &["paths"])?;
            let paths = paths
                .iter()
                .map(|path| resolve_import_input_path(path, ImportKind::Zeek))
                .collect::<Result<Vec<_>, _>>()?;
            to_json(
                commands::ingest::import_zeek_logs(paths, state_ref(&state))
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "import_suricata_eve" => {
            let path: String = arg(&payload, &["path"])?;
            let path = resolve_import_input_path(&path, ImportKind::Suricata)?;
            to_json(
                commands::ingest::import_suricata_eve(path, state_ref(&state))
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "import_nmap_xml" => {
            let path: String = arg(&payload, &["path"])?;
            let path = resolve_import_input_path(&path, ImportKind::Nmap)?;
            to_json(
                commands::ingest::import_nmap_xml(path, state_ref(&state))
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "import_masscan_json" => {
            let path: String = arg(&payload, &["path"])?;
            let path = resolve_import_input_path(&path, ImportKind::Masscan)?;
            to_json(
                commands::ingest::import_masscan_json(path, state_ref(&state))
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "import_wazuh_alerts" => {
            let path: String = arg(&payload, &["path"])?;
            let path = resolve_import_input_path(&path, ImportKind::Wazuh)?;
            to_json(
                commands::ingest::import_wazuh_alerts(path, state_ref(&state))
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "import_sinema_csv" => {
            let path: String = arg(&payload, &["path"])?;
            let path = resolve_import_input_path(&path, ImportKind::Sinema)?;
            to_json(
                commands::ingest::import_sinema_csv(path, state_ref(&state))
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "import_tia_xml" => {
            let path: String = arg(&payload, &["path"])?;
            let path = resolve_import_input_path(&path, ImportKind::Tia)?;
            to_json(
                commands::ingest::import_tia_xml(path, state_ref(&state))
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "get_device_zeek_events" => {
            let device_ip: String = arg(&payload, &["deviceIp", "device_ip"])?;
            to_json(
                commands::ingest::get_device_zeek_events(device_ip, state_ref(&state))
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
                commands::wireshark::open_in_wireshark(connection_id, state_ref(&state))
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
                commands::wireshark::get_connection_frames(connection_id, state_ref(&state))
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "export_frames_csv" => {
            let connection_id: String = arg(&payload, &["connectionId", "connection_id"])?;
            to_json(
                commands::wireshark::export_frames_csv(connection_id, state_ref(&state))
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "save_frames_csv" => {
            let connection_id: String = arg(&payload, &["connectionId", "connection_id"])?;
            let output_path: String = arg(&payload, &["outputPath", "output_path"])?;
            let output_path = resolve_export_output_path(&output_path, "frames.csv")?;
            to_json(
                commands::wireshark::save_frames_csv(connection_id, output_path, state_ref(&state))
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "export_assets_csv" => {
            let output_path: String = arg(&payload, &["outputPath", "output_path"])?;
            let output_path = resolve_export_output_path(&output_path, "assets.csv")?;
            to_json(
                commands::export::export_assets_csv(output_path, state_ref(&state))
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "export_connections_csv" => {
            let output_path: String = arg(&payload, &["outputPath", "output_path"])?;
            let output_path = resolve_export_output_path(&output_path, "connections.csv")?;
            to_json(
                commands::export::export_connections_csv(output_path, state_ref(&state))
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "export_topology_json" => {
            let output_path: String = arg(&payload, &["outputPath", "output_path"])?;
            let output_path = resolve_export_output_path(&output_path, "topology.json")?;
            to_json(
                commands::export::export_topology_json(output_path, state_ref(&state))
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "export_assets_json" => {
            let output_path: String = arg(&payload, &["outputPath", "output_path"])?;
            let output_path = resolve_export_output_path(&output_path, "assets.json")?;
            to_json(
                commands::export::export_assets_json(output_path, state_ref(&state))
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "generate_pdf_report" => {
            let config: commands::export::ReportConfigInput = arg(&payload, &["config"])?;
            let output_path: String = arg(&payload, &["outputPath", "output_path"])?;
            let output_path = resolve_export_output_path(&output_path, "assessment_report.pdf")?;
            to_json(
                commands::export::generate_pdf_report(config, output_path, state_ref(&state))
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "export_sbom" => {
            let format: String = arg(&payload, &["format"])?;
            let output_path: String = arg(&payload, &["outputPath", "output_path"])?;
            let output_path = resolve_export_output_path(&output_path, "sbom.json")?;
            to_json(
                commands::export::export_sbom(format, output_path, state_ref(&state))
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "export_stix_bundle" => {
            let output_path: String = arg(&payload, &["outputPath", "output_path"])?;
            let output_path = resolve_export_output_path(&output_path, "stix_bundle.json")?;
            to_json(
                commands::export::export_stix_bundle(output_path, state_ref(&state))
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
                    state_ref(&state),
                )
                .await
                .map_err(ApiError::bad_request)?,
            )?
        }
        "run_analysis" => to_json(
            commands::analysis::run_analysis(state_ref(&state)).map_err(ApiError::bad_request)?,
        )?,
        "get_findings" => to_json(
            commands::analysis::get_findings(state_ref(&state)).map_err(ApiError::bad_request)?,
        )?,
        "get_purdue_assignments" => to_json(
            commands::analysis::get_purdue_assignments(state_ref(&state))
                .map_err(ApiError::bad_request)?,
        )?,
        "get_anomalies" => to_json(
            commands::analysis::get_anomalies(state_ref(&state)).map_err(ApiError::bad_request)?,
        )?,
        "get_credential_warnings" => to_json(
            commands::analysis::get_credential_warnings(state_ref(&state))
                .map_err(ApiError::bad_request)?,
        )?,
        "get_criticality" => to_json(
            commands::analysis::get_criticality(state_ref(&state))
                .map_err(ApiError::bad_request)?,
        )?,
        "get_naming_suggestions" => to_json(
            commands::analysis::get_naming_suggestions(state_ref(&state))
                .map_err(ApiError::bad_request)?,
        )?,
        "get_switch_security_findings" => to_json(
            commands::analysis::get_switch_security_findings(state_ref(&state))
                .map_err(ApiError::bad_request)?,
        )?,
        "get_malware_findings" => to_json(
            commands::analysis::get_malware_findings(state_ref(&state))
                .map_err(ApiError::bad_request)?,
        )?,
        "get_compliance_report" => {
            let framework: String = arg(&payload, &["framework"])?;
            to_json(
                commands::analysis::get_compliance_report(state_ref(&state), framework)
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "get_cve_warnings" => {
            let ip: String = arg(&payload, &["ip"])?;
            to_json(
                commands::analysis::get_cve_warnings(ip, state_ref(&state))
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
            commands::data::get_timeline_range(state_ref(&state)).map_err(ApiError::bad_request)?,
        )?,
        "list_plugins" => {
            to_json(commands::system::list_plugins().map_err(ApiError::bad_request)?)?
        }
        "get_connection_stats" => to_json(
            commands::patterns::get_connection_stats(state_ref(&state))
                .map_err(ApiError::bad_request)?,
        )?,
        "get_pattern_anomalies" => to_json(
            commands::patterns::get_pattern_anomalies(state_ref(&state))
                .map_err(ApiError::bad_request)?,
        )?,
        "get_redundancy_protocols" => to_json(
            commands::patterns::get_redundancy_protocols(state_ref(&state))
                .map_err(ApiError::bad_request)?,
        )?,
        "create_project" => {
            let name: String = arg(&payload, &["name"])?;
            let client_name: Option<String> = arg_opt(&payload, &["clientName", "client_name"])?;
            let site_name: Option<String> = arg_opt(&payload, &["siteName", "site_name"])?;
            let assessor_name: Option<String> =
                arg_opt(&payload, &["assessorName", "assessor_name"])?;
            let engagement_start: Option<String> =
                arg_opt(&payload, &["engagementStart", "engagement_start"])?;
            let engagement_end: Option<String> =
                arg_opt(&payload, &["engagementEnd", "engagement_end"])?;
            let notes: Option<String> = arg_opt(&payload, &["notes"])?;
            to_json(
                commands::projects::create_project(
                    state_ref(&state),
                    name,
                    client_name,
                    site_name,
                    assessor_name,
                    engagement_start,
                    engagement_end,
                    notes,
                )
                .await
                .map_err(ApiError::bad_request)?,
            )?
        }
        "list_projects" => to_json(
            commands::projects::list_projects(state_ref(&state))
                .await
                .map_err(ApiError::bad_request)?,
        )?,
        "get_project" => {
            let id: i64 = arg(&payload, &["id"])?;
            to_json(
                commands::projects::get_project(state_ref(&state), id)
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "update_project" => {
            let id: i64 = arg(&payload, &["id"])?;
            let name: String = arg(&payload, &["name"])?;
            let client_name: Option<String> = arg_opt(&payload, &["clientName", "client_name"])?;
            let site_name: Option<String> = arg_opt(&payload, &["siteName", "site_name"])?;
            let assessor_name: Option<String> =
                arg_opt(&payload, &["assessorName", "assessor_name"])?;
            let engagement_start: Option<String> =
                arg_opt(&payload, &["engagementStart", "engagement_start"])?;
            let engagement_end: Option<String> =
                arg_opt(&payload, &["engagementEnd", "engagement_end"])?;
            let notes: Option<String> = arg_opt(&payload, &["notes"])?;
            to_json(
                commands::projects::update_project(
                    state_ref(&state),
                    id,
                    name,
                    client_name,
                    site_name,
                    assessor_name,
                    engagement_start,
                    engagement_end,
                    notes,
                )
                .await
                .map_err(ApiError::bad_request)?,
            )?
        }
        "delete_project" => {
            let id: i64 = arg(&payload, &["id"])?;
            to_json(
                commands::projects::delete_project(state_ref(&state), id)
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "set_active_project" => {
            let id: i64 = arg(&payload, &["id"])?;
            to_json(
                commands::projects::set_active_project(state_ref(&state), id)
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "clear_active_project" => to_json(
            commands::projects::clear_active_project(state_ref(&state))
                .await
                .map_err(ApiError::bad_request)?,
        )?,
        "get_correlated_alerts" => to_json(
            commands::correlation::get_correlated_alerts(state_ref(&state))
                .await
                .map_err(ApiError::bad_request)?,
        )?,
        "get_alerts_for_ip" => {
            let ip: String = arg(&payload, &["ip"])?;
            to_json(
                commands::correlation::get_alerts_for_ip(ip, state_ref(&state))
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "clear_alerts" => to_json(
            commands::correlation::clear_alerts(state_ref(&state))
                .await
                .map_err(ApiError::bad_request)?,
        )?,
        "generate_communication_allowlist" => to_json(
            commands::export::generate_communication_allowlist(state_ref(&state))
                .await
                .map_err(ApiError::bad_request)?,
        )?,
        "export_allowlist_csv" => {
            let output_path: String = arg(&payload, &["outputPath", "output_path"])?;
            let output_path = resolve_export_output_path(&output_path, "allowlist.csv")?;
            to_json(
                commands::export::export_allowlist_csv(output_path, state_ref(&state))
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "export_firewall_rules" => {
            let output_path: String = arg(&payload, &["outputPath", "output_path"])?;
            let output_path = resolve_export_output_path(&output_path, "firewall_rules.txt")?;
            to_json(
                commands::export::export_firewall_rules(output_path, state_ref(&state))
                    .await
                    .map_err(ApiError::bad_request)?,
            )?
        }
        "run_segmentation" => to_json(
            commands::segmentation::run_segmentation(state_ref(&state))
                .map_err(ApiError::bad_request)?,
        )?,
        "export_enforcement_config" => {
            let format: String = arg(&payload, &["format"])?;
            to_json(
                commands::segmentation::export_enforcement_config(format, state_ref(&state))
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

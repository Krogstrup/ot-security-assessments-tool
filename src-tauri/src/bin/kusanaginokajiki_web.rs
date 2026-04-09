use axum::extract::State;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::Router;
use clap::Parser;
use std::convert::Infallible;
use std::path::{Path as StdPath, PathBuf};
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::{Stream, StreamExt as _};
use tower_http::services::{ServeDir, ServeFile};

#[path = "../application/mod.rs"]
mod application;
#[path = "../commands/mod.rs"]
mod commands;
#[path = "kusanaginokajiki_web/web_api_paths.rs"]
mod web_api_paths;
#[path = "kusanaginokajiki_web/web_routes.rs"]
mod web_routes;
#[path = "kusanaginokajiki_web/web_runtime.rs"]
mod web_runtime;
#[path = "kusanaginokajiki_web/web_support.rs"]
mod web_support;

use commands::AppState;
use web_support::resolve_frontend_dist;

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

fn build_shared_state() -> SharedState {
    let (event_tx, _) = broadcast::channel::<(String, serde_json::Value)>(256);
    let mut app_state = AppState::new(commands::resource_paths::ResourcePaths::from_env());
    app_state.event_tx = Some(event_tx);
    Arc::new(app_state)
}

fn build_api_router() -> Router<SharedState> {
    web_routes::build_api_router()
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

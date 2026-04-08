//! Project management commands: create, list, get, update, delete, set active.

use gm_db::{Project, ProjectInput, ProjectSummary};

use super::{support::mutex_state, AppState, SessionState};

const DATABASE_NOT_AVAILABLE: &str = "Database not available";

fn project_input(
    name: String,
    client_name: Option<String>,
    site_name: Option<String>,
    assessor_name: Option<String>,
    engagement_start: Option<String>,
    engagement_end: Option<String>,
    notes: Option<String>,
) -> ProjectInput {
    ProjectInput {
        name,
        client_name: client_name.unwrap_or_default(),
        site_name: site_name.unwrap_or_default(),
        assessor_name: assessor_name.unwrap_or_default(),
        engagement_start: engagement_start.unwrap_or_default(),
        engagement_end: engagement_end.unwrap_or_default(),
        notes: notes.unwrap_or_default(),
    }
}

fn db_from_session(session: &SessionState) -> Result<&gm_db::Database, String> {
    session
        .db
        .as_ref()
        .ok_or_else(|| DATABASE_NOT_AVAILABLE.to_string())
}

/// Create a new project.
#[allow(clippy::too_many_arguments)]
pub async fn create_project(
    state: &AppState,
    name: String,
    client_name: Option<String>,
    site_name: Option<String>,
    assessor_name: Option<String>,
    engagement_start: Option<String>,
    engagement_end: Option<String>,
    notes: Option<String>,
) -> Result<Project, String> {
    let inner = mutex_state(&state.session, "session")?;
    let db = db_from_session(&inner)?;
    let input = project_input(
        name,
        client_name,
        site_name,
        assessor_name,
        engagement_start,
        engagement_end,
        notes,
    );
    db.create_project(&input).map_err(|e| e.to_string())
}

/// List all projects with session counts.
pub async fn list_projects(state: &AppState) -> Result<Vec<ProjectSummary>, String> {
    let inner = mutex_state(&state.session, "session")?;
    let db = db_from_session(&inner)?;
    db.list_projects().map_err(|e| e.to_string())
}

/// Get a single project by ID.
pub async fn get_project(state: &AppState, id: i64) -> Result<Project, String> {
    let inner = mutex_state(&state.session, "session")?;
    let db = db_from_session(&inner)?;
    db.get_project(id).map_err(|e| e.to_string())
}

/// Update a project's metadata.
#[allow(clippy::too_many_arguments)]
pub async fn update_project(
    state: &AppState,
    id: i64,
    name: String,
    client_name: Option<String>,
    site_name: Option<String>,
    assessor_name: Option<String>,
    engagement_start: Option<String>,
    engagement_end: Option<String>,
    notes: Option<String>,
) -> Result<Project, String> {
    let inner = mutex_state(&state.session, "session")?;
    let db = db_from_session(&inner)?;
    let input = project_input(
        name,
        client_name,
        site_name,
        assessor_name,
        engagement_start,
        engagement_end,
        notes,
    );
    db.update_project(id, &input).map_err(|e| e.to_string())
}

/// Delete a project (and cascade to all its sessions).
pub async fn delete_project(state: &AppState, id: i64) -> Result<(), String> {
    let mut inner = mutex_state(&state.session, "session")?;
    let db = db_from_session(&inner)?;
    db.delete_project(id).map_err(|e| e.to_string())?;
    // Clear active project if it was the one deleted
    if inner.current_project_id == Some(id) {
        inner.current_project_id = None;
    }
    log::info!("Deleted project {}", id);
    Ok(())
}

/// Set the active project. All subsequent save_session / list_sessions calls
/// will be scoped to this project.
pub async fn set_active_project(state: &AppState, id: i64) -> Result<Project, String> {
    let mut inner = mutex_state(&state.session, "session")?;
    let db = db_from_session(&inner)?;
    let project = db.get_project(id).map_err(|e| e.to_string())?;
    inner.current_project_id = Some(id);
    log::info!("Active project set to '{}' ({})", project.name, id);
    Ok(project)
}

/// Clear the active project (return to project selection view).
pub async fn clear_active_project(state: &AppState) -> Result<(), String> {
    let mut inner = mutex_state(&state.session, "session")?;
    inner.current_project_id = None;
    Ok(())
}

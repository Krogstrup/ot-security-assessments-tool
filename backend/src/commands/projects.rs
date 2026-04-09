//! Project management commands: create, list, get, update, delete, set active.

use gm_db::{Project, ProjectInput, ProjectSummary};

use super::{error::AppError, support::mutex_state, AppState, SessionState};

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

fn db_from_session(session: &SessionState) -> Result<&gm_db::Database, AppError> {
    session
        .db
        .as_ref()
        .ok_or_else(|| AppError::invalid_input(DATABASE_NOT_AVAILABLE))
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
) -> Result<Project, AppError> {
    let inner = mutex_state(&state.session, "session").map_err(AppError::state_lock)?;
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
    db.create_project(&input).map_err(AppError::from)
}

/// List all projects with session counts.
pub async fn list_projects(state: &AppState) -> Result<Vec<ProjectSummary>, AppError> {
    let inner = mutex_state(&state.session, "session").map_err(AppError::state_lock)?;
    let db = db_from_session(&inner)?;
    db.list_projects().map_err(AppError::from)
}

/// Get a single project by ID.
pub async fn get_project(state: &AppState, id: i64) -> Result<Project, AppError> {
    let inner = mutex_state(&state.session, "session").map_err(AppError::state_lock)?;
    let db = db_from_session(&inner)?;
    db.get_project(id).map_err(AppError::from)
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
) -> Result<Project, AppError> {
    let inner = mutex_state(&state.session, "session").map_err(AppError::state_lock)?;
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
    db.update_project(id, &input).map_err(AppError::from)
}

/// Delete a project (and cascade to all its sessions).
pub async fn delete_project(state: &AppState, id: i64) -> Result<(), AppError> {
    let mut inner = mutex_state(&state.session, "session").map_err(AppError::state_lock)?;
    let db = db_from_session(&inner)?;
    db.delete_project(id).map_err(AppError::from)?;
    // Clear active project if it was the one deleted
    if inner.current_project_id == Some(id) {
        inner.current_project_id = None;
    }
    log::info!("Deleted project {}", id);
    Ok(())
}

/// Set the active project. All subsequent save_session / list_sessions calls
/// will be scoped to this project.
pub async fn set_active_project(state: &AppState, id: i64) -> Result<Project, AppError> {
    let mut inner = mutex_state(&state.session, "session").map_err(AppError::state_lock)?;
    let db = db_from_session(&inner)?;
    let project = db.get_project(id).map_err(AppError::from)?;
    inner.current_project_id = Some(id);
    log::info!("Active project set to '{}' ({})", project.name, id);
    Ok(project)
}

/// Clear the active project (return to project selection view).
pub async fn clear_active_project(state: &AppState) -> Result<(), AppError> {
    let mut inner = mutex_state(&state.session, "session").map_err(AppError::state_lock)?;
    inner.current_project_id = None;
    Ok(())
}

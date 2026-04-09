//! Project management commands: create, list, get, update, delete, set active.

use gm_db::{Project, ProjectSummary};

use crate::application::use_cases::projects as use_case;

use super::{error::AppError, support::mutex_state, AppState};

fn project_input_args(
    name: String,
    client_name: Option<String>,
    site_name: Option<String>,
    assessor_name: Option<String>,
    engagement_start: Option<String>,
    engagement_end: Option<String>,
    notes: Option<String>,
) -> use_case::ProjectInputArgs {
    use_case::ProjectInputArgs {
        name,
        client_name,
        site_name,
        assessor_name,
        engagement_start,
        engagement_end,
        notes,
    }
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
    let input = project_input_args(
        name,
        client_name,
        site_name,
        assessor_name,
        engagement_start,
        engagement_end,
        notes,
    );
    use_case::create_project(inner.db.as_ref(), input).map_err(AppError::from)
}

/// List all projects with session counts.
pub async fn list_projects(state: &AppState) -> Result<Vec<ProjectSummary>, AppError> {
    let inner = mutex_state(&state.session, "session").map_err(AppError::state_lock)?;
    use_case::list_projects(inner.db.as_ref()).map_err(AppError::from)
}

/// Get a single project by ID.
pub async fn get_project(state: &AppState, id: i64) -> Result<Project, AppError> {
    let inner = mutex_state(&state.session, "session").map_err(AppError::state_lock)?;
    use_case::get_project(inner.db.as_ref(), id).map_err(AppError::from)
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
    let input = project_input_args(
        name,
        client_name,
        site_name,
        assessor_name,
        engagement_start,
        engagement_end,
        notes,
    );
    use_case::update_project(inner.db.as_ref(), id, input).map_err(AppError::from)
}

/// Delete a project (and cascade to all its sessions).
pub async fn delete_project(state: &AppState, id: i64) -> Result<(), AppError> {
    let mut inner = mutex_state(&state.session, "session").map_err(AppError::state_lock)?;
    inner.current_project_id =
        use_case::delete_project(inner.db.as_ref(), id, inner.current_project_id)
            .map_err(AppError::from)?;
    log::info!("Deleted project {}", id);
    Ok(())
}

/// Set the active project. All subsequent save_session / list_sessions calls
/// will be scoped to this project.
pub async fn set_active_project(state: &AppState, id: i64) -> Result<Project, AppError> {
    let mut inner = mutex_state(&state.session, "session").map_err(AppError::state_lock)?;
    let project = use_case::set_active_project(inner.db.as_ref(), id).map_err(AppError::from)?;
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

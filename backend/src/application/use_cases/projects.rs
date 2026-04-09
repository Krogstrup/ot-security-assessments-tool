//! Project CRUD use-cases.

use std::fmt;

use gm_db::{Database, DbError, Project, ProjectInput, ProjectSummary};

const DATABASE_NOT_AVAILABLE: &str = "Database not available";

#[derive(Debug)]
pub enum ProjectUseCaseError {
    DatabaseNotAvailable,
    Database(DbError),
}

impl fmt::Display for ProjectUseCaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectUseCaseError::DatabaseNotAvailable => write!(f, "{DATABASE_NOT_AVAILABLE}"),
            ProjectUseCaseError::Database(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for ProjectUseCaseError {}

impl From<DbError> for ProjectUseCaseError {
    fn from(value: DbError) -> Self {
        ProjectUseCaseError::Database(value)
    }
}

#[derive(Debug)]
pub struct ProjectInputArgs {
    pub name: String,
    pub client_name: Option<String>,
    pub site_name: Option<String>,
    pub assessor_name: Option<String>,
    pub engagement_start: Option<String>,
    pub engagement_end: Option<String>,
    pub notes: Option<String>,
}

impl ProjectInputArgs {
    fn into_project_input(self) -> ProjectInput {
        ProjectInput {
            name: self.name,
            client_name: self.client_name.unwrap_or_default(),
            site_name: self.site_name.unwrap_or_default(),
            assessor_name: self.assessor_name.unwrap_or_default(),
            engagement_start: self.engagement_start.unwrap_or_default(),
            engagement_end: self.engagement_end.unwrap_or_default(),
            notes: self.notes.unwrap_or_default(),
        }
    }
}

fn db_or_error(db: Option<&Database>) -> Result<&Database, ProjectUseCaseError> {
    db.ok_or(ProjectUseCaseError::DatabaseNotAvailable)
}

pub fn create_project(
    db: Option<&Database>,
    input: ProjectInputArgs,
) -> Result<Project, ProjectUseCaseError> {
    let db = db_or_error(db)?;
    db.create_project(&input.into_project_input())
        .map_err(ProjectUseCaseError::from)
}

pub fn list_projects(db: Option<&Database>) -> Result<Vec<ProjectSummary>, ProjectUseCaseError> {
    let db = db_or_error(db)?;
    db.list_projects().map_err(ProjectUseCaseError::from)
}

pub fn get_project(db: Option<&Database>, id: i64) -> Result<Project, ProjectUseCaseError> {
    let db = db_or_error(db)?;
    db.get_project(id).map_err(ProjectUseCaseError::from)
}

pub fn update_project(
    db: Option<&Database>,
    id: i64,
    input: ProjectInputArgs,
) -> Result<Project, ProjectUseCaseError> {
    let db = db_or_error(db)?;
    db.update_project(id, &input.into_project_input())
        .map_err(ProjectUseCaseError::from)
}

pub fn delete_project(
    db: Option<&Database>,
    id: i64,
    current_project_id: Option<i64>,
) -> Result<Option<i64>, ProjectUseCaseError> {
    let db = db_or_error(db)?;
    db.delete_project(id).map_err(ProjectUseCaseError::from)?;
    Ok(if current_project_id == Some(id) {
        None
    } else {
        current_project_id
    })
}

pub fn set_active_project(db: Option<&Database>, id: i64) -> Result<Project, ProjectUseCaseError> {
    let db = db_or_error(db)?;
    db.get_project(id).map_err(ProjectUseCaseError::from)
}

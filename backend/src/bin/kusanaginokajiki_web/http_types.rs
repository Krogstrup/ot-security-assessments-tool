//! HTTP response and error envelope types for the web API.

use crate::commands::error::AppError as CommandError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use serde_json::json;

/// A typed API error that converts directly into an Axum HTTP response.
#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    code: &'static str,
    message: String,
}

impl ApiError {
    fn with_code(status: StatusCode, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status,
            code,
            message: message.into(),
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::with_code(StatusCode::BAD_REQUEST, "bad_request", message)
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::with_code(StatusCode::INTERNAL_SERVER_ERROR, "internal_error", message)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(json!({ "code": self.code, "message": self.message })),
        )
            .into_response()
    }
}

impl From<CommandError> for ApiError {
    fn from(value: CommandError) -> Self {
        match value {
            CommandError::StateLock(message) => {
                ApiError::with_code(StatusCode::INTERNAL_SERVER_ERROR, "state_lock", message)
            }
            CommandError::NoSession => {
                ApiError::with_code(StatusCode::BAD_REQUEST, "no_session", "No active session")
            }
            CommandError::NoProject => {
                ApiError::with_code(StatusCode::BAD_REQUEST, "no_project", "No active project")
            }
            CommandError::InvalidInput(message) => {
                ApiError::with_code(StatusCode::BAD_REQUEST, "invalid_input", message)
            }
            CommandError::ParseFailure(message) => {
                ApiError::with_code(StatusCode::BAD_REQUEST, "parse_failure", message)
            }
            CommandError::DbError(message) => {
                ApiError::with_code(StatusCode::INTERNAL_SERVER_ERROR, "db_error", message)
            }
            CommandError::IoError(message) => {
                ApiError::with_code(StatusCode::INTERNAL_SERVER_ERROR, "io_error", message)
            }
            CommandError::NoCaptureRunning => ApiError::with_code(
                StatusCode::BAD_REQUEST,
                "no_capture_running",
                "No capture running",
            ),
            CommandError::ExternalProcess(message) => {
                ApiError::with_code(StatusCode::BAD_REQUEST, "external_process", message)
            }
        }
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

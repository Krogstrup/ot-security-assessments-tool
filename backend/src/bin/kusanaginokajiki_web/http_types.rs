//! HTTP response and error envelope types for the web API.

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
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code: "bad_request",
            message: message.into(),
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code: "internal_error",
            message: message.into(),
        }
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

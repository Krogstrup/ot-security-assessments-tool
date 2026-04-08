//! Structured error type for backend command handlers.
//!
//! All commands return `Result<T, AppError>`. The HTTP adapter serialises
//! `AppError` as:
//!
//! ```json
//! { "code": "no_session", "message": "No active session" }
//! ```
//!
//! The frontend can discriminate on `code` to decide how to handle the error
//! (e.g. show "open a session first" vs. a generic error toast).

use std::fmt;

use serde::Serialize;

/// Machine-readable error variants returned by all command handlers.
///
/// Serialised as a tagged JSON object so the frontend can switch on `code`.
/// Ready for use — migrate command return types from `String` to `AppError`
/// module by module.
#[allow(dead_code)]
#[derive(Debug, Serialize)]
#[serde(tag = "code", content = "message", rename_all = "snake_case")]
pub enum AppError {
    /// The global application state Mutex was poisoned.
    StateLock(String),
    /// A command that requires an active session was called without one.
    NoSession,
    /// A command that requires an active project was called without one.
    NoProject,
    /// The caller supplied an argument that failed validation.
    InvalidInput(String),
    /// A file or network payload could not be parsed.
    ParseFailure(String),
    /// A database operation failed.
    DbError(String),
    /// A filesystem I/O operation failed.
    IoError(String),
    /// An operation that requires a live capture found none running.
    NoCaptureRunning,
    /// An external process (e.g. Wireshark) could not be found or launched.
    ExternalProcess(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::StateLock(msg) => write!(f, "State lock error: {msg}"),
            AppError::NoSession => write!(f, "No active session"),
            AppError::NoProject => write!(f, "No active project"),
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {msg}"),
            AppError::ParseFailure(msg) => write!(f, "Parse failure: {msg}"),
            AppError::DbError(msg) => write!(f, "Database error: {msg}"),
            AppError::IoError(msg) => write!(f, "I/O error: {msg}"),
            AppError::NoCaptureRunning => write!(f, "No capture running"),
            AppError::ExternalProcess(msg) => write!(f, "External process error: {msg}"),
        }
    }
}

impl std::error::Error for AppError {}

// ── Convenience From impls ────────────────────────────────────────────────────

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::IoError(e.to_string())
    }
}

impl From<gm_db::DbError> for AppError {
    fn from(e: gm_db::DbError) -> Self {
        AppError::DbError(e.to_string())
    }
}

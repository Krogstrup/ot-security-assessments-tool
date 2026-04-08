//! Session management adapter layer.
//!
//! Public types and function signatures are preserved here so that web
//! handlers resolving `commands::session::*` compile unchanged.

// ─── Public types (defined in use-case, re-exported here) ────────────────────
pub use crate::application::use_cases::session::AssetUpdate;

// ─── Public commands (thin re-exports) ───────────────────────────────────────
pub use crate::application::use_cases::session::{
    bulk_update_assets, delete_session, export_session_archive, import_session_archive,
    list_sessions, load_session, save_session, update_asset,
};

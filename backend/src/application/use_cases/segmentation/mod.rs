//! Microsegmentation use-cases.

pub mod input_builder;
pub mod runner;

// ─── Public re-exports ────────────────────────────────────────────────────────
pub use runner::{export_enforcement_config, run_segmentation, SegmentationUseCaseError};

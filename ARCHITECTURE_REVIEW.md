<!-- markdownlint-disable MD013 MD029 MD032 -->

# Backend Architecture Review (Current Codebase)

Last verified against code on **2026-04-09**.

## 1. Runtime and Transport

- Runtime binary: `kusanaginokajiki_web` (`backend/Cargo.toml` `default-run`).
- Server entrypoint: `backend/src/bin/kusanaginokajiki_web.rs`.
- Route composition is explicit in `build_api_router()` by chaining:
  - `api_core.rs`
  - `api_capture_data.rs`
  - `api_physical_ingest_wireshark.rs`
  - `api_patterns_exports.rs`
  - `api_projects_sessions_analysis.rs`
- Public route constants live in `backend/src/bin/kusanaginokajiki_web/web_api_paths.rs`.
- Web support files are split by responsibility:
  - `http_types.rs`: HTTP envelope types (`ApiError`, import listing DTOs)
  - `import_support.rs`: import/export path resolution and import listing helpers
  - `web_support.rs`: thin compatibility re-export shim only

## 2. Layering Status

Current dependency direction is now enforced in code:

- Transport: `backend/src/bin/kusanaginokajiki_web/api_*.rs`
- Adapter: `backend/src/commands/*` (state access, lock choreography, adapter mapping)
- Application: `backend/src/application/{queries,use_cases,services,mappers}`
- Domain/infra libraries: `backend/crates/gm-*`

## 3. Boundary Evidence

Architecture guardrail scans now pass:

- `backend/src/application/**` has no imports of:
  - `crate::commands::*`
  - `commands::support::*`
  - `AppState`
  - command state structs (`CaptureState`, `InventoryState`, `AnalysisState`, `SessionState`)
- `backend/src/application/**` has no lock calls (`.lock()`, `.read()`, `.write()`).
- `backend/crates/gm-*` has no upward dependency on `application` or `commands` and no Axum/Tauri transport coupling.

This means application modules are now detached from runtime state containers and command modules.

## 4. Commands Layer Status

`commands/*` is now structurally adapter-first:

- New adapter-focused modules fronting application modules:
  - `commands/data.rs`
  - `commands/export.rs`
  - `commands/session.rs`
  - `commands/segmentation.rs`
- Existing adapters were updated to use application DTOs and services:
  - `commands/analysis.rs`
  - `commands/capture.rs`
  - `commands/ingest.rs`

Hotspots remain large and should continue to be reduced, but boundary direction is now correct.

Current hotspot sizes:

- `capture.rs`: 429 lines
- `physical.rs`: 374 lines
- `baseline.rs`: 363 lines
- `analysis.rs`: 344 lines
- `wireshark.rs`: 285 lines
- `projects.rs`: 132 lines

## 5. Application Layer Status

Application modules are now shaped around detached use-cases, mappers, and services:

- Data queries: `application/queries/data/*`
- Segmentation use-cases: `application/use_cases/segmentation/*`
- Session use-cases: `application/use_cases/session/*`
- Export use-cases: `application/use_cases/export/*`
- Ingest use-cases: `application/use_cases/ingest/*`
- Capture pipeline service: `application/services/capture_pipeline_commit.rs`
- Shared mappers: `application/mappers/*`

Use-cases consume DTOs/snapshots/ports and do not accept `&AppState`.

## 6. Contract and Error Status

- HTTP error envelope is now structured in web transport (`http_types::ApiError`) with:
  - `code`
  - `message`
- Frontend API core now parses structured error envelope and normalizes fallback behavior.
- Runtime validation coverage was expanded across high-traffic API modules (analysis/projects/session/data/capture/export/ingest/physical and related modules).

## 7. Remaining Backend Risks

1. Some command modules are still large and should be further thinned to reduce orchestration concentration.
2. A few command paths still use stringly domain/library errors and can be migrated further to structured app-level errors.
3. Build warnings remain for a small number of unused re-exports/helpers and should be cleaned up.

## 8. Incremental Next Steps

1. Continue extracting orchestration from `capture.rs`, `physical.rs`, and `baseline.rs` into application use-cases/services.
2. Complete structured error migration for remaining high-traffic command paths that still surface plain `String` errors.
3. Add CI guardrail checks for `application/**` import rules (`AppState`/`crate::commands::*`) and no upward dependencies from `gm-*` crates.

<!-- markdownlint-disable MD013 MD029 MD032 -->

# Backend Architecture Review (Current Codebase)

Last verified against code on **2026-04-09**.

## 1. Runtime Entry and Route Composition

- Runtime binary is web-first: `backend/Cargo.toml` sets `default-run = "kusanaginokajiki_web"`.
- Server entrypoint is `backend/src/bin/kusanaginokajiki_web.rs`.
- API route composition is centralized in `build_api_router()` in the entrypoint and delegates to module-level `add_routes()` functions:
  - `api_core.rs`
  - `api_capture_data.rs`
  - `api_physical_ingest_wireshark.rs`
  - `api_patterns_exports.rs`
  - `api_projects_sessions_analysis.rs`
- `backend/src/bin/kusanaginokajiki_web/web_routes.rs` is not present; composition currently lives in `kusanaginokajiki_web.rs`.
- Public path constants are in `backend/src/bin/kusanaginokajiki_web/web_api_paths.rs`.

## 2. Layer Boundaries (As Implemented)

Current effective layering:

- Transport: `backend/src/bin/kusanaginokajiki_web/*`
- Adapter/runtime state: `backend/src/commands/*`
- Application: `backend/src/application/{mappers,queries,services,use_cases}`
- Domain/infra: workspace crates under `backend/crates/gm-*`

Application extraction is explicit in `backend/src/application/mod.rs` and includes:

- `queries::data`
- `services::{asset_inventory,capture_pipeline_commit}`
- `use_cases::{analysis,baseline,capture,correlation,export,ingest,physical,projects,segmentation,session,wireshark}`

## 3. Verified Boundary Checks

Audit checks confirm the architecture direction is currently enforced:

- `backend/src/application/**` has no imports of `crate::commands::*`, `commands::support::*`, `AppState`, or command-state structs.
- `backend/src/application/**` has no direct lock operations (`.lock()`, `.read()`, `.write()`).
- `backend/crates/**` has no `crate::application::*` or `crate::commands::*` imports.

This keeps application modules runtime-container independent.

## 4. Command Layer Status

`backend/src/commands` now splits into two roles:

- Thin adapters over application use-cases/queries (`analysis`, `capture`, `data`, `export`, `ingest`, `physical`, `projects`, `segmentation`, `session`, `wireshark`, etc.)
- Runtime infrastructure (`processor`, `protocol_handler`, `handlers/*`, state model in `mod.rs`, lock/file helpers in `support.rs`)

Current command-layer hotspots (`wc -l`):

- `capture.rs`: 389
- `protocol_handler.rs`: 327
- `mod.rs`: 250
- `handlers/modbus.rs`: 247
- `analysis.rs`: 215
- `ingest.rs`: 210

These are now mostly runtime mechanics and adapter plumbing, not clear signs of business-use-case ownership leaks.

## 5. Application Hotspots

Large application files now hold substantial use-case/service logic (`wc -l`):

- `application/use_cases/baseline.rs`: 311
- `application/services/asset_inventory.rs`: 254
- `application/use_cases/physical.rs`: 247
- `application/use_cases/segmentation/input_builder.rs`: 236
- `application/use_cases/ingest/mod.rs`: 221
- `application/use_cases/analysis.rs`: 217

This is consistent with migration away from command-level orchestration.

## 6. Error and Contract State

- Structured command error type exists at `commands/error.rs` (`AppError` with `code`/`message` serialization shape).
- HTTP transport wraps errors via `http_types::ApiError`.
- Remaining inconsistency: several command modules still return raw `String` results (`system.rs`, `signatures.rs`, `correlation.rs`) and are mapped as generic bad-request at transport boundaries.

## 7. Risks and Drift Points

1. Transport is split well by endpoint domain, but route registration still lives in `kusanaginokajiki_web.rs`; there is no dedicated route-composition module.
2. `web_runtime.rs` directly bridges both application service APIs and command-layer runtime types/helpers, creating a mixed adapter/runtime seam inside transport code.
3. Command error typing is not yet fully standardized on `AppError`.

## 8. Incremental Backend Roadmap

1. Standardize remaining `Result<_, String>` command adapters to `Result<_, AppError>` (`system`, `signatures`, `correlation`) and keep HTTP envelope stable.
2. Extract route assembly from `kusanaginokajiki_web.rs` into a dedicated router composition module (without endpoint contract changes).
3. Introduce a small live-capture runtime service boundary so `web_runtime.rs` depends on one runtime interface instead of mixed `commands` + `application` details.
4. Add CI guard checks for import boundaries (application not importing commands/state containers, crates not importing app/commands).

<!-- markdownlint-disable MD013 MD029 MD032 -->

# Backend Architecture Review (Current Codebase)

Last verified against code on **2026-04-09**.

## 1. Current Backend Shape

### Runtime entrypoint and transport

- Runtime binary: `kusanaginokajiki_web` (`backend/Cargo.toml` `default-run`)
- HTTP server: `backend/src/bin/kusanaginokajiki_web.rs`
- API composition: `backend/src/bin/kusanaginokajiki_web/web_routes.rs`
- Route groups:
  - `api_core.rs`
  - `api_capture_data.rs`
  - `api_physical_ingest_wireshark.rs`
  - `api_patterns_exports.rs`
  - `api_projects_sessions_analysis.rs`
- API path constants: `backend/src/bin/kusanaginokajiki_web/web_api_paths.rs`

### State model

- Shared state: `Arc<AppState>`
- State root: `backend/src/commands/mod.rs`
- Domain slices in `AppState`:
  - `capture`, `inventory`, `analysis`, `session`, `physical`, `segmentation`, `signatures`
- Lock order is explicitly documented and enforced by convention in `commands/mod.rs`.

### Layering as implemented today

#### Interface layer (HTTP handlers)

- `backend/src/bin/kusanaginokajiki_web/api_*.rs`
- Responsibilities:
  - parse HTTP body/query/path
  - normalize payload names
  - map string errors into HTTP `ApiError` (`{ "error": "..." }`)
  - call adapter functions in `crate::commands::*`

#### Adapter layer (`commands/*`)

- `backend/src/commands/*.rs`
- Mixed role today:
  - part adapter
  - part application orchestration
  - part state/persistence manipulation

#### Application modules (extracted and actively used)

- `backend/src/application/queries/data/*`
- `backend/src/application/use_cases/export/*`
- `backend/src/application/use_cases/session/*`
- `backend/src/application/use_cases/segmentation/*`
- `backend/src/application/use_cases/ingest/*`
- `backend/src/application/services/capture_pipeline_commit.rs`
- `backend/src/application/mappers/*`

#### Domain + infrastructure crates

- `gm-analysis`, `gm-capture`, `gm-db`, `gm-ingest`, `gm-parsers`, `gm-topology`, `gm-signatures`, `gm-segmentation`, `gm-report`, `gm-physical`, `gm-types`

## 2. What Is Already in Good Shape

### 2.1 Transport is HTTP-first and explicit

- Routes are explicit resource-style paths (`/api/data/*`, `/api/v1/*`), not a generic command-dispatch endpoint.
- Route grouping by domain area is clear in `web_routes.rs`.

### 2.2 Significant extraction from `commands` already happened

- Data queries are re-exported from application query modules via `commands::data`.
- Export/session/segmentation are re-exported from application use-cases via `commands::{export,session,segmentation}`.
- Ingest adapters call one consolidated use-case: `application::use_cases::ingest::run_ingest`.
- Capture import commit logic moved to `application::services::capture_pipeline_commit`.

### 2.3 Backend module ownership is understandable

- Workspace crates map cleanly to technical domains.
- `web_api_paths.rs` centralizes public HTTP route names.

## 3. Current Architectural Gaps

### 3.1 `commands` still contains heavy orchestration hotspots

Large modules still own business flow and state choreography directly:

- `commands/physical.rs` (~374 LOC)
- `commands/baseline.rs` (~363 LOC)
- `commands/capture.rs` (~334 LOC)
- `commands/protocol_handler.rs` (~327 LOC)
- `commands/processor.rs` (~313 LOC)
- `commands/analysis.rs` (~301 LOC)
- `commands/wireshark.rs` (~285 LOC)

Impact:

- harder isolated testing
- harder ownership boundaries
- higher merge-conflict probability in common files

### 3.2 Application layer still depends on `commands` internals

Several extracted application modules import `crate::commands::*` types/helpers, for example:

- `application/queries/data/*` depends on `commands::{AppState, AssetInfo, ConnectionInfo, support::*}`
- `application/use_cases/session/*` depends on `commands` types and lock helpers
- `application/services/capture_pipeline_commit.rs` depends on `commands::processor::PacketProcessor`

This blocks true inward dependency direction (`interface -> application -> domain`).

### 3.3 Error model is inconsistent across layers

- HTTP adapters currently return `ApiError` with shape `{ "error": string }`.
- `commands/error.rs` defines structured `AppError` (`code` + message), but most command APIs still return `Result<T, String>`.
- Frontend cannot rely on stable machine-readable backend error codes across most endpoints.

### 3.4 Locking policy is explicit but leaks everywhere

- Lock ordering is documented, which is good.
- But many call paths still do lock choreography directly.
- No application-owned repository/port boundary hides lock semantics yet.

## 4. Recommended Target (Incremental, Based on Existing Structure)

Keep the current folder roots and move behavior progressively:

- Keep HTTP handlers as thin interface adapters in `src/bin/kusanaginokajiki_web/api_*.rs`.
- Keep `commands/*` as compatibility adapter surface, but reduce to thin delegators.
- Expand `application/{queries,use_cases,services}` as the primary behavior location.
- Push reusable domain logic into `gm-*` crates.

Target dependency direction:

- `api_*.rs` -> `commands` adapter facade
- `commands` facade -> `application` modules
- `application` -> `gm-*` crates + abstracted state access
- No new business logic in HTTP handlers or adapter wrappers

## 5. Backend Refactoring Roadmap

### Step 1: Formalize application-facing state access

- Create narrow state-access services (or traits) in `application` for common operations.
- Stop importing `commands::support::*` directly from new application modules.

### Step 2: Move `analysis.rs` orchestration into an application use-case

- Keep command function names stable.
- Move analysis input building + result persistence orchestration into `application/use_cases/analysis/*`.

### Step 3: Move remaining capture workflow pieces into application modules

- Keep `commands/capture.rs` as adapter + transport DTOs only.
- Move packet import/live capture orchestration where practical under `application/use_cases/capture/*`.

### Step 4: Move project CRUD into application use-cases

- Extract DB/session coupling from `commands/projects.rs` into `application/use_cases/projects/*`.

### Step 5: Standardize structured backend errors

- Migrate high-traffic command functions from `String` errors to structured `AppError`.
- Update HTTP adapter mapping so frontend can receive stable `code` fields.

### Step 6: Add architecture guardrails

- Add tests or lint checks that block new cross-layer regressions (for example, application modules importing adapter helpers).

## 6. Suggested Issue Backlog

1. `backend: introduce application state access facade`
- Scope: `application/*` + lock helper usage
- Outcome: no new `commands::support` imports from extracted application modules

2. `backend: extract analysis run orchestration into application use-case`
- Scope: `commands/analysis.rs` + new `application/use_cases/analysis/*`
- Outcome: command file reduced to adapter responsibilities

3. `backend: extract projects CRUD use-case module`
- Scope: `commands/projects.rs` + new application module
- Outcome: DB/session orchestration no longer embedded in command adapter

4. `backend: migrate command error contract from String to AppError`
- Scope: selected high-traffic commands + HTTP mapping
- Outcome: stable machine-readable error codes on core endpoints

5. `backend: add dependency-boundary checks`
- Scope: CI/test scripts
- Outcome: prevent new layer violations in future changes

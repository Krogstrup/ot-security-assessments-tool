<!-- markdownlint-disable MD013 MD029 MD032 -->

# System Architecture Review (Backend + Frontend)

Last verified against code on **2026-04-09**.

## 1. System Runtime Topology

## 1.1 Deployment model in code today

- Frontend: SvelteKit/Vite app under `src/`
- Backend: Axum server binary `kusanaginokajiki_web`
- Combined runtime: backend serves static frontend build + `/api/*` endpoints
  - Static serving: `backend/src/bin/kusanaginokajiki_web.rs`
  - API mount: `.nest("/api", build_api_router())`

## 1.2 API surface organization

Route groups in `backend/src/bin/kusanaginokajiki_web/web_routes.rs`:

- `core` (health, app info, interface listing, import file discovery)
- `capture_data` (capture lifecycle + data queries + asset updates)
- `physical_ingest_wireshark` (physical imports, external ingest, Wireshark integration)
- `patterns_exports` (patterns, signatures, correlation, exports, segmentation)
- `projects_sessions_analysis` (projects, sessions, analysis, SSE events)

## 2. Backend Architecture Status

## 2.1 Layering currently visible in code

- Interface layer: `backend/src/bin/kusanaginokajiki_web/api_*.rs`
- Adapter/state layer: `backend/src/commands/*`
- Application extraction layer: `backend/src/application/{queries,use_cases,services,mappers}`
- Domain/infrastructure crates: `backend/crates/gm-*`

## 2.2 Areas already extracted to application modules

- Data queries: `application/queries/data/*`
- Session workflows: `application/use_cases/session/*`
- Export workflows: `application/use_cases/export/*`
- Segmentation workflows: `application/use_cases/segmentation/*`
- Ingest merge workflow: `application/use_cases/ingest/run_ingest`
- Capture commit orchestration: `application/services/capture_pipeline_commit`

## 2.3 Remaining backend concentration points

- Command modules still hold significant orchestration and direct lock choreography (`analysis.rs`, `capture.rs`, `physical.rs`, `projects.rs`, `wireshark.rs`, `baseline.rs`).
- Application modules still import several adapter-layer types/helpers from `commands`.
- Error contracts are mixed (`String` command errors + HTTP `{error}` envelope, while structured `AppError` exists but is not the default path yet).

## 3. Frontend Architecture Status

## 3.1 Structure in code today

- Feature/component organization: `src/lib/components/*`
- API modules: `src/lib/api/*`
- State stores: `src/lib/stores/*`
- Root route orchestrator: `src/routes/+page.svelte`

## 3.2 Current orchestration distribution

More extracted:

- Capture flow modules (`capture/sessionFlows.ts`, `capture/pcapImportFlow.ts`, `capture/externalImportTasks.ts`, `capture/captureListeners.ts`)
- Export flow modules (`export/exportFlows.ts`)

Still mostly in view components:

- `AnalysisView.svelte`
- `ProjectsView.svelte`
- `SegmentationView.svelte`
- `ConnectionTree.svelte`
- Top-level routing/tab composition in `+page.svelte`

## 3.3 API contract handling status

- Transport is centralized through `httpJson` and `httpValidated` in `src/lib/api/core.ts`.
- Zod runtime validation is actively used in:
  - `api/analysis.ts`
  - `api/projects.ts`
  - `api/session.ts`
- Many other API modules still use unvalidated `httpJson` responses.

## 4. Frontend–Backend Contract Status

## 4.1 What is consistent

- Endpoint paths are explicit and mostly stable via `web_api_paths.rs` + `src/lib/api/*` wrappers.
- Request payload casing mismatches are normalized at adapters (for example query aliases such as `pageSize`, `sortBy`).

## 4.2 What is inconsistent

- Error contract is not yet a fully typed system-wide envelope.
- Validation coverage is partial (strong in analysis/projects/session, weaker elsewhere).
- A few comments/docs still describe older contract assumptions and should continue to be cleaned up as modules are touched.

## 5. Highest-Value System Risks

1. Layer leakage from application modules into adapter/state internals
- Slows extraction and keeps lock policy spread across modules.

2. Large orchestration in UI and command hotspots
- Increases regression risk and review complexity.

3. Inconsistent error and validation behavior
- Makes frontend resilience uneven across features.

4. Contract drift risk in unvalidated API modules
- Schema mismatches can surface late at runtime.

## 6. Pragmatic System Roadmap

### Phase 1: Contract and boundary hardening

1. Standardize backend error envelope for high-traffic endpoints.
2. Expand `httpValidated` coverage to data/capture/export/ingest endpoints with stable schemas.
3. Add guardrail docs and lightweight dependency checks for backend and frontend boundaries.

### Phase 2: Orchestration extraction

1. Backend: move analysis and projects orchestration into application use-case modules.
2. Frontend: extract `AnalysisView`, `ProjectsView`, `SegmentationView`, and `ConnectionTree` async flows into feature modules.
3. Keep public endpoint/function names stable while moving internals.

### Phase 3: State and dependency cleanup

1. Introduce application-owned state access abstractions to reduce direct lock choreography.
2. Introduce feature-level state actions/selectors in frontend to reduce broad direct writable mutations.
3. Add CI checks that prevent new cross-layer dependency regressions.

## 7. Suggested Cross-Layer Issue Set

1. `system: standardize error envelope and frontend error parsing`
- Outcome: stable machine-readable error handling end-to-end.

2. `system: expand runtime response validation coverage`
- Outcome: typed validation on high-traffic API paths beyond analysis/projects/session.

3. `backend: continue command-to-application extraction`
- Outcome: thinner adapters and better isolation for tests.

4. `frontend: extract orchestration from top-level views`
- Outcome: smaller view scripts and reusable application flows.

5. `system: enforce architecture boundaries in CI`
- Outcome: reduced architecture drift during future feature work.

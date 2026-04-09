<!-- markdownlint-disable MD013 MD029 MD032 -->

# System Architecture Review (Backend + Frontend)

Last verified against code on **2026-04-09**.

## 1. Runtime Topology

- Frontend runtime: SvelteKit app under `src/`, served as static assets.
- Backend runtime: Axum binary `kusanaginokajiki_web` under `backend/`.
- Unified deployment path in `backend/src/bin/kusanaginokajiki_web.rs`:
  - `Router::nest("/api", build_api_router())` for API
  - static fallback for frontend files
- Real-time updates flow through SSE endpoint `/api/v1/events`.

## 2. Route and Contract Flow

Route surfaces are grouped by transport modules:

- core/system/import: `api_core.rs`
- capture/data/assets: `api_capture_data.rs`
- physical/ingest/wireshark: `api_physical_ingest_wireshark.rs`
- signatures/patterns/exports/segmentation/correlation: `api_patterns_exports.rs`
- projects/sessions/analysis: `api_projects_sessions_analysis.rs`

Path constants are centralized in `web_api_paths.rs`, but versioning is currently mixed:

- unversioned endpoints: `/api/system/*`, `/api/data/*`, `/api/capture/import-pcap`
- versioned endpoints: `/api/v1/*`

## 3. Layer Boundaries Across the Stack

- Backend layering is now explicit and mostly clean:
  - transport (`bin/kusanaginokajiki_web/*`)
  - adapter/runtime (`commands/*`)
  - application (`application/*`)
  - domain/infra (`crates/gm-*`)
- Verified by grep checks:
  - `application/**` does not depend on `commands` or runtime state structs
  - `application/**` does not call lock primitives directly
  - `backend/crates/**` does not depend upward on `application` or `commands`

## 4. Cross-Layer Contract Status

### Error envelope

- HTTP layer exposes structured `{ code, message }` via `ApiError`.
- Frontend transport (`src/lib/api/core.ts`) parses `code` and `message` consistently.

### Runtime validation

- All API modules call `httpValidated(...)`.
- No direct `httpJson(...)` usage in `src/lib/api/*.ts` call sites.
- Validation depth is uneven: many endpoints still use `z.unknown()` then cast.
  - Current `z.unknown()` hotspots include `connections.ts`, `capture.ts`, `assets.ts`, `system.ts`, `wireshark.ts`, `correlation.ts`, `signatures.ts`.

### Error typing at adapter boundary

- Most command modules return `AppError`.
- Some still return raw `String` errors (`commands/system.rs`, `commands/signatures.rs`, `commands/correlation.rs`), producing less precise HTTP mapping.

## 5. Frontend-Backend Coupling Observations

1. `src/routes/+page.svelte` still owns global tab/split-layout/topology-tab orchestration.
2. Frontend has meaningful extraction into feature modules (`components/capture/*.ts`, `components/export/*.ts`, etc.), but container views like `AnalysisView.svelte`, `ProjectsView.svelte`, and `SegmentationView.svelte` still coordinate async workflow directly.
3. Backend transport `web_runtime.rs` mixes application service usage and command runtime types/helpers, creating a broader seam than ideal.

## 6. Current System Hotspots

Backend transport hotspots (`wc -l`):

- `api_physical_ingest_wireshark.rs`: 384
- `api_patterns_exports.rs`: 368
- `api_projects_sessions_analysis.rs`: 328
- `import_support.rs`: 300
- `web_runtime.rs`: 252

Frontend container/shell hotspots (`wc -l`):

- `AnalysisView.svelte`: 249
- `ConnectionTree.svelte`: 245
- `+page.svelte`: 237
- `SegmentationView.svelte`: 236
- `ProjectsView.svelte`: 209
- `ExportView.svelte`: 202

## 7. Pragmatic Roadmap

### Phase 1: Contract consistency (low-risk)

1. Standardize remaining command `String` errors to `AppError`.
2. Replace high-traffic `z.unknown()` API schemas with concrete Zod schemas.
3. Keep endpoint names and payload shapes stable while tightening validation.

### Phase 2: Orchestration seam cleanup

1. Move route assembly into a dedicated router composition module.
2. Introduce a narrower live-capture runtime service API so `web_runtime.rs` stops directly coordinating command-runtime details.

### Phase 3: View/container thinning

1. Continue moving async workflows out of large Svelte containers into feature flow modules.
2. Keep `+page.svelte` focused on shell/routing decisions only.

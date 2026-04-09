<!-- markdownlint-disable MD013 MD029 MD032 -->

# System Architecture Review (Backend + Frontend)

Last verified against code on **2026-04-09**.

## 1. Runtime Topology

- Frontend: SvelteKit/Vite app in `src/`.
- Backend: Axum binary `kusanaginokajiki_web` in `backend/`.
- Combined deployment path:
  - backend serves static frontend build
  - backend exposes `/api/*` HTTP endpoints
- Entrypoint and server wiring: `backend/src/bin/kusanaginokajiki_web.rs`.

## 2. Backend Layering (Current)

- Transport handlers: `backend/src/bin/kusanaginokajiki_web/api_*.rs`
- Adapter layer: `backend/src/commands/*`
- Application layer: `backend/src/application/{mappers,queries,use_cases,services}`
- Domain/infra crates: `backend/crates/gm-*`

Boundary status now validated:

- `application/**` has no `AppState`, no `crate::commands::*`, and no direct lock calls.
- `gm-*` crates have no upward dependency on `application`/`commands`.

## 3. Transport/Contract Status

- Route paths are centralized in `web_api_paths.rs`.
- HTTP envelope is structured via `http_types::ApiError` (`code`, `message`).
- Web support is now intentionally split:
  - `http_types.rs`
  - `import_support.rs`
  - `web_support.rs` (thin shim)

## 4. Frontend Structure and Contract Handling

- App shell/route composition: `src/routes/+page.svelte`.
- API transport core: `src/lib/api/core.ts`.
- Runtime schemas: `src/lib/schemas.ts`.
- API modules now use validated responses broadly (`httpValidated`) across:
  - analysis
  - projects
  - session
  - data/assets/connections
  - capture
  - export
  - ingest
  - physical
  - wireshark/system/signatures/correlation

## 5. System Risks Still Worth Addressing

1. Command hotspots are still large (`capture.rs`, `physical.rs`, `baseline.rs`, `analysis.rs`).
2. Some endpoints still return loosely typed payloads parsed with `z.unknown()` and can be tightened.
3. Container Svelte views still host substantial orchestration logic and can be further extracted.

## 6. Pragmatic Roadmap

1. Continue command-thinning by migrating remaining orchestration to `application/use_cases/*` and `application/services/*`.
2. Replace `z.unknown()` schemas on high-traffic responses with concrete schemas where backend payloads are stable.
3. Extract additional frontend view orchestration into feature flow modules while preserving endpoint contracts.
4. Add CI-level architecture checks for boundary regressions (`application` import rules and crate dependency direction).

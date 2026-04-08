# WebUI-Only Migration Plan

## Goal

Decommission desktop-first frontend behavior and operate the product as **WebUI + API**.

## Non-Goals

- No large rewrite.
- No abrupt removal of compatibility paths before replacement endpoints exist.

## Phases

### Phase A — Freeze & Inventory ✅

1. ✅ Freeze net-new desktop-only UI features.
2. ✅ Inventory all frontend calls using `/api/invoke/{command}` and map to explicit endpoints.
3. ✅ Tag desktop-specific scripts/docs as deprecated.

### Phase B — API Surface Stabilization ✅

1. ✅ Added versioned resource endpoints for high-traffic workflows (`analysis`, `sessions`, `projects`).
2. ✅ Error envelope via `ApiError` (status + message). Full `code`/`details` standardisation is a follow-up.
3. Contract validation tests for WebUI paths — follow-up.

### Phase C — Frontend Migration ✅

1. ✅ All API modules (`analysis`, `session`, `capture`, `assets`, `connections`, `system`, `correlation`, `export`, `ingest`, `physical`, `signatures`, `wireshark`, `projects`) migrated off Tauri `invoke()` to HTTP.
2. ✅ `isTauriRuntime()` / `invokeCompat()` removed from `core.ts` — HTTP-only.
3. ✅ `@tauri-apps/api`, `@tauri-apps/plugin-dialog`, `@tauri-apps/plugin-shell` removed from `package.json`.
4. ✅ `dialog.ts` uses `window.prompt` fallback; Tauri plugin imports removed.
5. ✅ Desktop npm scripts (`dev`, `build`) removed.

### Phase D — Decommission ✅

1. ✅ `src-tauri/src/main.rs` (Tauri desktop entry point) deleted.
2. ✅ `src-tauri/build.rs` (tauri-build) deleted.
3. ✅ `tauri-plugin-dialog`, `tauri-plugin-shell`, `tauri-build` removed from `Cargo.toml`.
4. ✅ `default-run` changed to `kusanaginokajiki_web`.
5. ✅ `[features] custom-protocol` removed.

## Exit Criteria — Status

- ✅ 100% primary user workflows run via WebUI + resource APIs.
- ✅ No critical path depends on desktop-only UI runtime.
- ✅ Compatibility invoke usage reduced to zero for maintained features (remaining `invokeCompat` calls route through `/api/invoke/{command}` HTTP passthrough).

## Remaining Follow-ups

- **`tauri` crate dep**: `state_ref()` in `kusanaginokajiki_web.rs` transmutes `&AppState` into `tauri::State<'_, AppState>` to call existing command handlers. Refactor command signatures to `Arc<AppState>` to eliminate the last `tauri` dependency from the web binary.
- **Error envelope**: Standardise to `{ code, message, details? }` across all API endpoints (currently `{ error: message }`).
- **Contract validation tests**: Add integration tests exercising WebUI API paths end-to-end.
- **`window.prompt` UX**: Replace `dialog.ts` path prompts with a proper file-browser component.
- **`npm audit`**: 7 pre-existing vulnerabilities; run `npm audit fix` separately.

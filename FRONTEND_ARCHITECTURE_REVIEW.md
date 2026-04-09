<!-- markdownlint-disable MD013 MD029 MD032 -->

# Frontend Architecture Review (Svelte + TypeScript)

Last verified against code on **2026-04-09**.

## 1. Current Frontend Structure

- Route shell: `src/routes/+page.svelte`
- Feature containers: `src/lib/components/*View.svelte`
- Feature helpers/flows: `src/lib/components/<feature>/*.ts`
- API layer: `src/lib/api/*`
- Store layer: `src/lib/stores/*`
- Runtime validation schemas: `src/lib/schemas.ts`

## 2. API and Validation Status

Transport and error handling are centralized in `src/lib/api/core.ts`:

- `httpJson` handles fetch + backend error envelope parsing (`code`, `message`).
- `httpValidated` is the default call path for API modules.

Validation coverage is now broad across high-traffic modules:

- analysis
- projects
- session
- assets/connections/data queries
- capture
- export
- ingest
- physical
- wireshark
- system
- signatures
- correlation

`rg` shows no remaining `httpJson(` direct usage inside `src/lib/api/*.ts`.

## 3. Runtime Schema Coverage

`src/lib/schemas.ts` now includes concrete schemas for key flows, including:

- core entities (`AssetInfo`, `ConnectionInfo`, findings, sessions, projects)
- data pages/counts/protocol stats/timeline
- capture import/status/stop responses
- ingest import responses and Zeek device events
- export allowlist and filtered pcap responses
- wireshark frame/info responses
- physical and inferred topology payloads

## 4. Frontend Gaps Still Present

1. Several API endpoints still use `z.unknown()` where concrete schema tightening is possible.
2. Large container views still carry orchestration logic that could be extracted further (`AnalysisView`, `ProjectsView`, `SegmentationView`, `ConnectionTree`).
3. App-shell behavior in `+page.svelte` remains coupled to tab/topology orchestration.

## 5. Incremental Improvement Plan

1. Replace `z.unknown()` responses with dedicated schemas on high-traffic endpoints.
2. Continue extracting async workflow/state mutation logic from large `.svelte` containers into feature flow modules.
3. Introduce feature action helpers for shared store updates to reduce direct writable mutations in view components.
4. Keep endpoint paths and payload contracts stable while iterating on internals.

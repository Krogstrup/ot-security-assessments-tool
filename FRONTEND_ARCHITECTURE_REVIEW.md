<!-- markdownlint-disable MD013 MD029 MD032 -->

# Frontend Architecture Review (Svelte + TypeScript)

Last verified against code on **2026-04-09**.

## 1. Current Frontend Architecture

## 1.1 Project layout in use

- Route entrypoint: `src/routes/+page.svelte`
- Feature views: `src/lib/components/*View.svelte`
- Feature subcomponents: `src/lib/components/<feature>/*`
- API layer: `src/lib/api/*`
- Shared state: `src/lib/stores/*`
- Runtime schemas: `src/lib/schemas.ts`

The implementation is currently **feature-grouped by folders**, with a **hybrid orchestration model**: some features use extracted flow modules, while others still run workflows directly in view components.

## 1.2 Root-level orchestration

`src/routes/+page.svelte` currently owns:

- top-level tab selection
- topology sub-tab routing logic
- connection-tree panel visibility state
- initial interface bootstrap (`listInterfaces`)

This works, but keeps app-shell routing concerns tightly coupled to presentation.

## 1.3 Feature orchestration distribution

### More extracted

- Capture feature:
  - `capture/sessionFlows.ts`
  - `capture/pcapImportFlow.ts`
  - `capture/externalImportTasks.ts`
  - `capture/captureListeners.ts`
- Export feature:
  - `export/exportFlows.ts`

### Still view-centric

- `AnalysisView.svelte` (analysis run, section loading, refresh flows)
- `ProjectsView.svelte` (CRUD + modal form state + active-project navigation)
- `SegmentationView.svelte` (run/export orchestration + loading stage lifecycle)
- `ConnectionTree.svelte` (packet/frame fetch and interaction workflows)

## 1.4 API layer status

Transport utilities are centralized:

- `httpJson` and `httpValidated` in `src/lib/api/core.ts`

Validation coverage today:

- Validated (`httpValidated` + Zod):
  - `api/analysis.ts`
  - `api/projects.ts`
  - `api/session.ts`
- Mostly unvalidated (`httpJson`):
  - `api/assets.ts`, `connections.ts`, `capture.ts`, `export.ts`, `ingest.ts`, `physical.ts`, `correlation.ts`, `system.ts`, `wireshark.ts`, `signatures.ts`

## 1.5 State model

Domain stores are clear and discoverable (`analysis.ts`, `capture.ts`, `core.ts`, `navigation.ts`, `physical.ts`, `session.ts`, `timeline.ts`, `topology-tabs.ts`), but many UI containers still mutate shared writable stores directly.

## 2. What Is Working Well

1. API calls are centralized in `src/lib/api/*` rather than scattered `fetch` calls in components.
2. Runtime validation pattern is established and already used on key modules.
3. Capture and export features demonstrate a reusable extracted-flow pattern that can be copied by other features.
4. Components are grouped by domain area, which keeps navigation and ownership understandable.

## 3. Current Frontend Gaps

### 3.1 Large container components still hold business flow

High-churn view components still mix:

- async workflows
- API orchestration
- shared-store mutation
- UI state and rendering

This increases regression risk and makes reuse difficult.

### 3.2 App-shell concerns are not isolated

- `+page.svelte` contains both shell routing and topology-tab behavior.
- Harder to test shell behavior independently from view rendering.

### 3.3 Validation coverage is uneven

- Core analysis/projects/session paths are protected.
- Several high-traffic data paths (assets, connections, capture, export, ingest) are still unvalidated.

### 3.4 Store mutation discipline is inconsistent

- Direct writable access across many components makes state transitions implicit.
- Feature actions/selectors are not yet the primary write/read interface.

## 4. Recommended Target (Incremental, No Big-Bang Rewrite)

Keep the existing directory strategy and evolve in-place:

- Keep `src/lib/components/<feature>` grouping.
- Add/expand per-feature `*Flows.ts` or `*Actions.ts` modules for orchestration.
- Keep `src/lib/api/*` as infra layer; do not move API calls into presentational components.
- Move shell/tab orchestration from `+page.svelte` into a focused app-shell module over time.

Target dependency direction inside frontend features:

- Presentational UI components -> props/events only
- Containers/views -> feature flow modules + state actions
- Flow modules -> API layer + store actions
- API layer -> `httpJson/httpValidated`

## 5. Frontend Refactoring Roadmap

### Step 1: Extract analysis orchestration module

- Move `AnalysisView.svelte` workflow logic (run/load section refresh/cross-store updates) into `analysis` flow module(s).
- Keep component focused on composition + event wiring.

### Step 2: Extract projects orchestration module

- Move list/create/update/delete/activate logic and form-state transitions from `ProjectsView.svelte` into reusable flow helpers.

### Step 3: Extract segmentation orchestration module

- Move run/export/loading-stage state machine from `SegmentationView.svelte` into feature-level flow utilities.

### Step 4: Extract connection tree interaction flows

- Move packet/frame fetch, cache lifecycle, and Wireshark actions from `ConnectionTree.svelte` into a focused module.

### Step 5: Broaden runtime validation coverage

- Add Zod schemas for high-traffic unvalidated responses.
- Prioritize assets, connections, capture status/results, and export/ingest responses.

### Step 6: Introduce store action/selectors for top-level features

- Start with analysis, inventory/core, and topology-tab state transitions.
- Reduce direct writable mutations in large view scripts.

### Step 7: App-shell extraction from `+page.svelte`

- Move topology subtab and panel-toggle orchestration into dedicated shell module(s).
- Keep `+page.svelte` as a slim composition entrypoint.

## 6. Suggested Issue Backlog

1. `frontend: extract analysis view workflows`
- Outcome: smaller `AnalysisView.svelte`, reusable analysis orchestration.

2. `frontend: extract projects CRUD and modal orchestration`
- Outcome: leaner projects container and easier form-state testing.

3. `frontend: extract segmentation run/export state machine`
- Outcome: improved segmentation workflow testability and readability.

4. `frontend: extract connection tree interaction logic`
- Outcome: reusable interaction flows and reduced component complexity.

5. `frontend: expand zod validation across api modules`
- Outcome: earlier detection of backend/frontend contract drift.

6. `frontend: introduce store action/selectors for core flows`
- Outcome: explicit state transitions and lower coupling.

7. `frontend: split app shell orchestration from +page`
- Outcome: clearer routing/shell behavior ownership.

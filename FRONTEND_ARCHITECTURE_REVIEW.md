# Frontend Architecture & Modularization Review (Svelte + TypeScript)

## 1. Current Architecture Overview

The frontend currently follows a **mixed architecture** with useful domain grouping but inconsistent layering discipline.

### Current structure (what works)

- **UI by feature area** under `src/lib/components/*` (capture, inventory, analysis, export, physical, etc.).
- **Global stores by domain** under `src/lib/stores/*` (core, capture, analysis, navigation, session, physical).
- **API access centralized** in `src/lib/api/*`, with runtime adaptation for Tauri IPC vs headless HTTP.
- **Types centralized** in `src/lib/types/*` and runtime schema definitions in `src/lib/schemas.ts`.

This is a good foundation and indicates an intent toward modularity.

### Implicit architectural pattern

The codebase exhibits a **feature-oriented UI layer with partial service extraction**:

- Some features externalize orchestration into `.ts` modules (`capture/sessionFlows.ts`, `capture/pcapImportFlow.ts`, `capture/externalImportTasks.ts`).
- Other features keep orchestration directly inside top-level `.svelte` view components (`AnalysisView.svelte`, `ProjectsView.svelte`, `SegmentationView.svelte`, `ExportView.svelte`, `ConnectionTree.svelte`).

So the architecture is currently **hybrid**: partially layered, partially view-centric.

### Where boundaries are inconsistent

1. **UI + application orchestration mixed in major views** in multiple areas.
2. **Application logic placement is inconsistent** between features (capture has extracted flow modules; analysis/projects/segmentation mostly do not).
3. **Store usage is broad/global**, and components frequently mutate stores directly rather than through feature-specific state services.
4. **Data validation infrastructure exists but is not integrated into API calls** (`invokeValidated` and schemas are largely unused).

---

## 2. Key Problems (categorized)

### A) Component design issues

1. **Top-level views carry too much responsibility**
   - Routing concerns, API calls, state transitions, error handling, and UI rendering live in one file in major views.
   - This increases cognitive load and reduces reuse of orchestration logic.

2. **Container/presentational split is inconsistent**
   - Some feature trees have clear containers + child sections; others are still “smart components everywhere”.

3. **Cross-cutting UI helpers are duplicated**
   - Formatting and feedback/status patterns are re-implemented in multiple views rather than standardized in shared application utilities.

### B) State management issues

1. **Global stores are used as writable integration surface**
   - Many views import shared stores directly and mutate them in-place, making data flow less explicit.

2. **Feature state lifecycle logic is scattered**
   - Load/reset/refresh rules vary by component, making behavior less predictable for maintainers.

3. **No unified feature state adapters**
   - Stores expose raw writable primitives, but many features would benefit from “domain store APIs” (intent-based actions/selectors).

### C) Data access issues

1. **API calls are frequently executed directly inside components**
   - This tightly couples UI with data access timing, retries, and fallback behavior.

2. **Runtime validation layer is underutilized**
   - `invokeValidated` + Zod schemas exist, but command callers mostly use `invokeCompat` directly.

3. **Transport behavior and endpoint concerns leak upward**
   - While API modules are centralized, feature components still make low-level decisions that should be abstracted into application services.

### D) Reusability / coupling problems

1. **Feature orchestration patterns are not standardized**
   - Capture feature has reusable flow modules; other features duplicate orchestration shape in component scripts.

2. **Strong coupling between view components and global state modules**
   - Makes independent feature extraction and testability harder.

3. **Large root page orchestration**
   - `+page.svelte` contains substantial tab/layout/render branching logic and topology subtab control.

### E) Naming and structure inconsistencies

1. **Mixed naming conventions for orchestration files**
   - `*Flow`, `*Handlers`, `*Utils`, `*Controller`, and ad hoc component-local async handlers coexist without clear boundary rules.

2. **Type import strategy partly migrated but still mixed**
   - A deprecated barrel (`types/index.ts`) coexists with domain-type imports, creating transition ambiguity.

### F) Separation-of-concerns violations

1. **Business/application decisions inside UI components**
   - Run analysis workflows, asset refresh rules, segmentation flow staging, and project CRUD orchestration are often embedded in `.svelte`.

2. **UI components handling infrastructure concerns**
   - Components sometimes decide how/when to call lower-level APIs, rather than delegating to application services.

---

## 3. Target Modular Architecture

### Recommended architecture style: **Feature-first + layered within each feature**

Keep feature folders (good for discoverability), but enforce strict sub-layer boundaries inside each feature.

### Proposed folder shape

```text
src/lib/
  app/
    router/                 # tab/view routing orchestration
    shell/                  # app-level layout shell state and actions

  features/
    analysis/
      ui/                   # presentational components
      containers/           # smart feature entry components
      application/          # use-cases/orchestrators (runAnalysis, loadSections)
      state/                # feature stores + selectors + actions
      domain/               # feature rules/transformers
      infra/                # analysis-specific API adapters (optional thin wrappers)
      types/

    capture/
      ui/
      containers/
      application/
      state/
      domain/
      infra/
      types/

    inventory/
    export/
    projects/
    physical/
    segmentation/
    topology/

  shared/
    ui/                     # reusable dumb components
    application/            # shared workflows (status handling, async guards)
    state/                  # cross-feature state atoms only when truly global
    domain/                 # pure formatters/validators not tied to API
    infra/
      api/                  # existing API clients (refined)
      dialog/
    types/
```

### Component classification model

1. **Presentational components (`ui/`)**
   - Receive data via props, emit events.
   - No direct API calls.
   - No direct writes to global stores.

2. **Container components (`containers/`)**
   - Bind feature state to UI components.
   - Call application use-cases.
   - Minimal conditional orchestration only.

### Placement rules

- **Stores**: `features/*/state` with actions/selectors; avoid exposing raw writable stores broadly.
- **API clients**: keep centralized in `shared/infra/api` (current `src/lib/api` can evolve into this).
- **Domain logic**: pure transformations/formatters/validators in `features/*/domain` or `shared/domain`.
- **Application orchestration**: async workflows in `features/*/application`.

### Dependency direction (must enforce)

- `ui -> containers/state selectors`
- `containers -> application + state`
- `application -> domain + infra(api) + state actions`
- `domain -> (pure, no UI/store/api)`
- `infra -> transport/runtime libraries`

Forbidden:

- `ui -> api`
- `ui -> raw global stores`
- `domain -> api/stores/svelte runtime`

---

## 4. Refactoring Roadmap

### Step 1 — Define frontend layering conventions and ADR
- **Description:** Add architecture doc + folder conventions + dependency rules for components/stores/api orchestration.
- **Why it matters:** Creates team-wide consistency before moving files.
- **Expected impact:** Prevents further pattern divergence.
- **Risk:** Low.
- **Dependencies:** None.

### Step 2 — Introduce feature application services for high-churn views
- **Description:** Extract orchestration from `AnalysisView`, `ProjectsView`, `SegmentationView`, `ExportView`, and `ConnectionTree` into `application/*` modules (no behavior change).
- **Why it matters:** Largest SRP win with minimal UX risk.
- **Expected impact:** Leaner components, easier testing.
- **Risk:** Medium.
- **Dependencies:** Step 1.

### Step 3 — Wrap stores with feature state APIs
- **Description:** Replace direct broad store writes with feature actions/selectors (e.g., `analysisState.run()`, `inventoryState.applyBulkEdit()` patterns).
- **Why it matters:** Makes state transitions explicit and traceable.
- **Expected impact:** Lower accidental coupling and easier onboarding.
- **Risk:** Medium.
- **Dependencies:** Step 2.

### Step 4 — Standardize async workflow utilities
- **Description:** Create shared async helpers for loading/error/status/retry/cancel conventions used across views.
- **Why it matters:** Eliminates duplicated status logic and inconsistent error UX.
- **Expected impact:** Better consistency + reduced code size.
- **Risk:** Low.
- **Dependencies:** Step 2.

### Step 5 — Integrate runtime response validation into API clients
- **Description:** Adopt `invokeValidated` for prioritized high-traffic endpoints first (assets/connections/findings/analysis summary).
- **Why it matters:** Catches backend/frontend contract drift early.
- **Expected impact:** Fewer silent runtime mismatches.
- **Risk:** Low/Medium.
- **Dependencies:** Step 1.

### Step 6 — Split root page orchestration into app shell modules
- **Description:** Move large branching logic from `+page.svelte` into app shell router/container components.
- **Why it matters:** Improves top-level readability and enables isolated view composition.
- **Expected impact:** Cleaner entrypoint and easier future navigation changes.
- **Risk:** Medium.
- **Dependencies:** Steps 1–3.

### Step 7 — Normalize naming conventions and file taxonomy
- **Description:** Standardize `application/*` naming (`useCases`, `actions`, `selectors`, `mappers`) and phase out ad hoc names.
- **Why it matters:** Improves discoverability for new maintainers.
- **Expected impact:** Faster onboarding and lower architectural drift.
- **Risk:** Low.
- **Dependencies:** Steps 1–4.

### Step 8 — Add architecture fitness checks
- **Description:** Add lint/import rules to enforce boundaries (e.g., no `api` imports inside `ui/` components).
- **Why it matters:** Sustains architecture over time.
- **Expected impact:** Prevents regressions automatically.
- **Risk:** Low.
- **Dependencies:** Steps 1–7.

---

## 5. GitHub Issues List

### Issue 1
- **Title:** Define frontend layered architecture conventions (ADR + dependency rules)
- **Description:** Document and ratify feature-first layered conventions and allowed dependency directions.
- **Acceptance criteria:**
  - ADR committed.
  - Folder/layer naming conventions documented.
  - Dependency rule examples included.
- **Labels:** `frontend`, `architecture`, `tech-debt`
- **Priority:** High

### Issue 2
- **Title:** Extract AnalysisView orchestration into analysis application service
- **Description:** Move analysis run/load/section refresh logic out of `AnalysisView.svelte` into `features/analysis/application`.
- **Acceptance criteria:**
  - `AnalysisView.svelte` becomes container + rendering logic only.
  - Existing UI behavior unchanged.
  - Unit tests cover orchestration module.
- **Labels:** `frontend`, `refactor`, `analysis`
- **Priority:** High

### Issue 3
- **Title:** Extract ProjectsView CRUD orchestration into projects application service
- **Description:** Relocate loading/form/save/delete/open logic into reusable projects service.
- **Acceptance criteria:**
  - Component script reduced substantially.
  - No regression in create/edit/delete/open workflows.
  - Error handling centralized.
- **Labels:** `frontend`, `refactor`, `projects`
- **Priority:** High

### Issue 4
- **Title:** Extract SegmentationView async workflow/state transitions into application module
- **Description:** Move run/export/stage progression/error orchestration out of component.
- **Acceptance criteria:**
  - Component no longer manages stage timer internals directly.
  - Workflow module tested for run/export transitions.
- **Labels:** `frontend`, `refactor`, `segmentation`
- **Priority:** Medium

### Issue 5
- **Title:** Introduce feature store action/selectors API and reduce direct writable mutations
- **Description:** Wrap core stores with intent-driven actions and read selectors.
- **Acceptance criteria:**
  - At least analysis + inventory features migrated.
  - Direct writable mutations in views reduced.
  - State transition paths documented.
- **Labels:** `frontend`, `state-management`, `architecture`
- **Priority:** High

### Issue 6
- **Title:** Adopt invokeValidated + Zod schemas in high-traffic API paths
- **Description:** Incrementally switch key API functions to runtime-validated responses.
- **Acceptance criteria:**
  - Assets, connections, analysis result endpoints validated.
  - Validation failures surface typed errors.
  - Existing success behavior unchanged.
- **Labels:** `frontend`, `api`, `reliability`
- **Priority:** Medium

### Issue 7
- **Title:** Refactor +page.svelte into app-shell router/container modules
- **Description:** Move top-level tab and topology subtab orchestration into app shell layer.
- **Acceptance criteria:**
  - `+page.svelte` reduced to compositional entrypoint.
  - Navigation behavior unchanged.
  - Subtab close/select logic covered by tests.
- **Labels:** `frontend`, `refactor`, `architecture`
- **Priority:** Medium

### Issue 8
- **Title:** Add lint import-boundary rules for frontend architecture
- **Description:** Configure tooling to prevent forbidden cross-layer imports.
- **Acceptance criteria:**
  - Rule examples: `ui/` cannot import API directly.
  - CI fails on violations.
  - Developer docs updated with remediation examples.
- **Labels:** `frontend`, `quality`, `ci`, `architecture`
- **Priority:** Medium

---

## 6. Quick Wins

1. **Pilot extraction in one high-value file**
   - Start with `AnalysisView.svelte` orchestration extraction into `analysis/application` service.

2. **Use existing capture pattern as template**
   - Reuse the proven `capture/*Flow.ts` extraction style as standard for other features.

3. **Standardize status/error helper now**
   - Introduce a shared async status utility and apply to projects/analysis/export quickly.

4. **Document naming conventions immediately**
   - Define when to use `*Flow`, `*Actions`, `*Selectors`, `*Mapper` and enforce in new files.

5. **Remove new usage of deprecated type barrel**
   - Enforce direct domain imports from `types/*` in all new code.

6. **Validate at least one endpoint path end-to-end**
   - Wire `invokeValidated` for assets or findings to establish the pattern.

---

## 7. Long-Term Strategic Improvements

1. **Strategic change: full feature-first layered module architecture**
   - Transition from mixed structure to consistent `features/<feature>/{ui,containers,application,state,domain,infra}`.

2. **Strategic change: app-shell/navigation module**
   - Introduce dedicated app shell with explicit route-state orchestration and feature mounting contracts.

3. **Strategic change: design-system style shared UI kit**
   - Consolidate reusable presentational patterns (tables, banners, dialogs, status blocks, section cards).

4. **Strategic change: domain-centric frontend model layer**
   - Add pure domain mappers/normalizers for backend payloads before entering stores/UI.

5. **Strategic change: architecture governance in CI**
   - Enforce import boundaries, component size thresholds, and orchestration-location rules.


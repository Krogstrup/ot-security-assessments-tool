# System Architecture & Modularization Review (Backend + Frontend)

## 1. Current System Architecture Overview

> **Direction update:** product should converge to **WebUI + API only**. Desktop UI is deprecated and should receive only maintenance-level changes during transition.

### Backend architecture

The backend has a strong foundational decomposition at the **workspace crate level** (`gm-*` crates), but the application shell still centralizes orchestration in transport-facing modules.

- Rust workspace spans domain-focused crates (`gm-analysis`, `gm-capture`, `gm-db`, `gm-ingest`, `gm-report`, etc.).
- Tauri app exposes a large command surface via `commands::*` handlers.
- Headless Axum mode reuses the same `AppState` and command modules.

`AppState` is partitioned into domain slices (capture/inventory/analysis/session/physical/segmentation/signatures) with explicit lock order guidance, which is operationally sound but also creates a broad mutable integration hub used across many command handlers.

### Frontend architecture

The frontend is organized by broad concerns (`components`, `stores`, `api`, `types`) with feature-clustered component directories.

- `src/lib/api` centralizes transport adaptation and endpoint access.
- `src/lib/stores` holds app-global writable state and derived views.
- Large feature views combine rendering and orchestration (`AnalysisView`, `ProjectsView`, `SegmentationView`, `ExportView`, `ConnectionTree`, root `+page`).
- Some features (capture) already extracted orchestration into reusable flow modules.

### Frontend ↔ backend interaction

Interaction model is dual-mode:

1. **Desktop mode:** frontend uses Tauri command names (`invoke`).
2. **Headless mode:** frontend calls HTTP routes (some direct data routes, plus `/api/invoke/{command}` compatibility endpoint).

This design enables mode portability, but introduces command-string coupling and a large command-dispatch mapping in headless mode.

### Implicit patterns and inconsistencies

- Strong modular intent at package level, but **layer boundaries are inconsistent inside app code**.
- Controllers/transport adapters are not thin; they often own orchestration.
- Frontend feature modularization exists unevenly (capture more modular than other features).
- Contract validation primitives exist (`Zod`, `invokeValidated`) but are not consistently applied.

---

## 2. Key Problems (Backend / Frontend / Cross-System)

### Backend Issues

#### Structural problems

1. **Command handlers are multi-responsibility units** (input parsing, orchestration, state mutation, mapping, side effects).
2. **AppState is a central mutable bus** with many direct cross-domain mutations.
3. **Headless and desktop adapters duplicate orchestration paths** (command registration + invoke dispatch + route handlers).

#### Dependency issues

1. High coupling to lock-aware shared state contracts.
2. Workflow correctness depends on update ordering across multiple state slices.
3. Commands layer depends directly on concrete infrastructure concerns (paths, files, transport-specific argument normalization).

#### SOLID violations

1. **SRP**: command modules do too much.
2. **OCP pressure**: feature changes frequently require editing large command files.
3. **DIP**: high-level use cases often depend directly on concrete state/infrastructure types.

### Frontend Issues

#### Component design problems

1. Top-level views are orchestration-heavy and “smart”.
2. Presentational vs container boundaries are inconsistent.
3. Root page (`+page.svelte`) contains large branching/layout orchestration.

#### State management issues

1. Stores are widely mutable from many components.
2. Feature state actions/selectors are not standardized.
3. Lifecycle behavior (load/reset/refresh) varies by feature and component.

#### Data handling inconsistencies

1. API calls often occur directly in components.
2. Error/status patterns are duplicated across features.
3. Runtime response validation is underused despite available schemas/utilities.

### Cross-System Issues

#### API design / contract problems

1. **String-based command coupling** between frontend and backend command names.
2. Mixed API styles (resource-like routes + invoke passthrough) complicate long-term API governance.
3. Headless invoke endpoint has very large command dispatch surface, making compatibility maintenance expensive.

#### Duplication and coupling

1. Similar workflow logic appears in backend command paths and frontend view orchestration.
2. Cross-layer data contracts are mostly “implicit by convention”; not versioned formally.
3. UI behavior can become coupled to backend command naming and parameter aliasing.

#### Contract definition risks

1. Zod schemas exist but are not broadly enforced, reducing protection against drift.
2. Error modeling across boundaries is inconsistent (string errors/common wrapper but not uniformly typed end-to-end).

---

## 3. Target System Architecture

### Backend target architecture (Clean Architecture)

#### Layers

1. **Interface/Transport** (`tauri`, `http` adapters)
2. **Application Services / Use Cases**
3. **Domain** (core policies/models)
4. **Infrastructure** (DB, filesystem, ingest connectors, external tooling)

#### Suggested structure

```text
src-tauri/src/
  interface/
    tauri/
    http/
  application/
    use_cases/
    services/
    ports/
    dto/
  domain/
    discovery/
    analysis/
    session/
    topology/
  infrastructure/
    persistence/
    ingest/
    reporting/
    runtime/
```

#### Dependency direction

- `interface -> application`
- `application -> domain + ports`
- `infrastructure -> ports + domain`
- `domain` depends on nothing outside itself

### Frontend target architecture (Layered Feature Modules)

#### Layers per feature

1. **UI** (presentational components)
2. **Application** (orchestrators/use-cases)
3. **Domain** (pure rules, transforms)
4. **Infrastructure** (API adapters)
5. **State** (actions/selectors/store facades)

#### Suggested structure

```text
src/lib/
  app/{shell,router}
  features/<feature>/{ui,containers,application,state,domain,infra,types}
  shared/{ui,application,domain,infra,types}
```

#### Component hierarchy strategy

- Entry container per feature tab/view.
- UI sections are presentational.
- Containers call feature application services only.
- State updates routed through feature state actions, not direct scattered writable mutations.

### API layer target design

#### Contract principles

1. Prefer **resource-oriented HTTP APIs** for headless mode.
2. Keep Tauri command adapters as transport wrappers over shared application use-cases.
3. Define explicit request/response DTOs with stable naming.
4. Limit aliasing and normalize at adapter boundary only.

#### Error handling strategy

- Backend: stable structured error envelope (`code`, `message`, `details?`).
- Frontend: parse into typed AppError union; no raw stringly flows.

#### Versioning approach

- Introduce API contract versioning for headless endpoints (e.g., `/api/v1/...`).
- Maintain command compatibility matrix for Tauri/headless while migrating.

### Shared concepts (what to share vs separate)

#### Share

- Contract schemas (JSON Schema/Zod/OpenAPI-derived)
- Error code taxonomy
- Key DTO semantics (assets, connections, findings pages, pagination envelopes)

#### Keep separate

- Backend domain internals and persistence models
- Frontend view models/UI-specific state shape
- Transport-specific adapter logic

---

## 4. Refactoring Roadmap (Phased)

## Phase 1: Stabilization (low-risk improvements)

### Step 1 — Define architecture contracts and ADRs
- **Description:** Document backend/frontend layering rules, dependency constraints, naming conventions, and migration guardrails.
- **Scope:** Both
- **Why it matters:** Aligns implementation direction before structural changes.
- **Expected impact:** Reduces architecture drift.
- **Risk:** Low
- **Dependencies:** None

### Step 2 — Standardize error envelope and frontend error typing
- **Description:** Introduce consistent backend error DTO and frontend typed parser utilities.
- **Scope:** Both
- **Why it matters:** Improves observability and debuggability.
- **Expected impact:** Safer cross-layer error handling.
- **Risk:** Low
- **Dependencies:** Step 1

### Step 3 — Activate runtime validation for highest-traffic APIs
- **Description:** Apply `invokeValidated` + schemas to core reads/writes (assets, connections, analysis summary).
- **Scope:** Frontend
- **Why it matters:** Detects contract drift early.
- **Expected impact:** Fewer runtime data-shape failures.
- **Risk:** Low/Medium
- **Dependencies:** Step 1

## Phase 2: Modularization (clear boundaries)

### Step 4 — Extract backend use-cases from command handlers
- **Description:** Move orchestration from `commands/*` into `application/use_cases/*`, keep adapters thin.
- **Scope:** Backend
- **Why it matters:** Enforces controller/service separation.
- **Expected impact:** Better testability, lower command complexity.
- **Risk:** Medium
- **Dependencies:** Phase 1

### Step 5 — Extract frontend feature orchestration services
- **Description:** Move async workflows from large view components into feature `application/*` modules.
- **Scope:** Frontend
- **Why it matters:** Reduces smart-component bloat.
- **Expected impact:** Reusable orchestration and easier maintenance.
- **Risk:** Medium
- **Dependencies:** Phase 1

### Step 6 — Introduce feature state APIs (actions/selectors)
- **Description:** Wrap writable stores behind intent-based actions and selectors.
- **Scope:** Frontend
- **Why it matters:** Clarifies data flow and state ownership.
- **Expected impact:** Lower coupling and improved onboarding.
- **Risk:** Medium
- **Dependencies:** Step 5

## Phase 3: Decoupling (reduce dependencies)

### Step 7 — Unify backend transport adapters on shared application facade
- **Description:** Route both Tauri and Axum adapters through common use-case interfaces.
- **Scope:** Backend
- **Why it matters:** Eliminates drift between desktop and headless behavior.
- **Expected impact:** Single source of behavior truth.
- **Risk:** Medium
- **Dependencies:** Step 4

### Step 8 — Replace broad `/invoke/{command}` usage with resource endpoints for headless mode
- **Description:** Keep invoke for compatibility, but migrate frontend headless calls to explicit versioned endpoints.
- **Scope:** Both
- **Why it matters:** Improves API clarity and contract governance.
- **Expected impact:** Less string-command coupling.
- **Risk:** Medium
- **Dependencies:** Step 7

### Step 9 — Introduce backend repository ports over AppState/DB access
- **Description:** Encapsulate lock ordering and state mutation behind application repository interfaces.
- **Scope:** Backend
- **Why it matters:** Reduces lock and state coupling leakage.
- **Expected impact:** Safer composability for new use-cases.
- **Risk:** Medium/High
- **Dependencies:** Steps 4,7

## Phase 4: Optimization (refinement and scaling)

### Step 10 — Add architecture fitness tests and import-boundary linting
- **Description:** Enforce layer rules in CI (backend module dependencies + frontend import boundaries).
- **Scope:** Both
- **Why it matters:** Prevents regression.
- **Expected impact:** Sustained architecture quality.
- **Risk:** Low
- **Dependencies:** Phase 2/3 progress

### Step 11 — Introduce contract-generation pipeline
- **Description:** Generate typed contracts (OpenAPI/JSON Schema/Zod/TS types) from source-of-truth DTO schemas.
- **Scope:** Both
- **Why it matters:** Strengthens cross-system consistency.
- **Expected impact:** Reduced contract drift and duplication.
- **Risk:** Medium
- **Dependencies:** Step 8

### Step 12 — Performance and scaling cleanup
- **Description:** Profile major workflows; optimize payload sizes, pagination defaults, and rendering pressure points.
- **Scope:** Both
- **Why it matters:** Improves large-dataset usability and operational scalability.
- **Expected impact:** Better responsiveness under heavy captures.
- **Risk:** Medium
- **Dependencies:** Prior phases

---

## 5. GitHub Issues List (Grouped by phase)

## Phase 1

### Issue 1: Define architecture ADRs and dependency rules
- **Description:** Create backend/frontend/system ADR docs with allowed/forbidden dependency directions.
- **Acceptance criteria:** ADRs merged; contributor guide updated; examples included.
- **Labels:** `architecture`, `backend`, `frontend`, `tech-debt`
- **Priority:** High

### Issue 2: Standardize API error envelope and frontend error parser
- **Description:** Introduce unified error schema and typed parsing helper.
- **Acceptance criteria:** Core endpoints return structured errors; frontend maps to typed union.
- **Labels:** `backend`, `frontend`, `refactor`, `architecture`
- **Priority:** High

### Issue 3: Enable runtime validation on core frontend API calls
- **Description:** Migrate key API calls to `invokeValidated` with schemas.
- **Acceptance criteria:** assets/connections/analysis responses validated; failures surfaced clearly.
- **Labels:** `frontend`, `architecture`, `reliability`
- **Priority:** High

## Phase 2

### Issue 4: Extract backend import/session/analysis use-cases from command modules
- **Description:** Move orchestration into application use-case modules.
- **Acceptance criteria:** command handlers become thin adapters; use-case tests added.
- **Labels:** `backend`, `refactor`, `architecture`
- **Priority:** High

### Issue 5: Extract frontend orchestration from AnalysisView/ProjectsView/SegmentationView/ExportView
- **Description:** Create feature application services and slim components.
- **Acceptance criteria:** component scripts reduced; behavior unchanged; service tests added.
- **Labels:** `frontend`, `refactor`, `architecture`
- **Priority:** High

### Issue 6: Introduce feature state action/selectors and reduce direct store writes
- **Description:** Add store facades for analysis/inventory/capture first.
- **Acceptance criteria:** direct writable mutations in top-level views reduced significantly.
- **Labels:** `frontend`, `state-management`, `refactor`
- **Priority:** High

## Phase 3

### Issue 7: Unify Tauri and Axum adapters through shared application facade
- **Description:** Ensure both transports call same use-case interfaces.
- **Acceptance criteria:** parity tests pass between transports for selected workflows.
- **Labels:** `backend`, `architecture`, `refactor`
- **Priority:** Medium

### Issue 8: Introduce versioned headless resource APIs and de-emphasize invoke passthrough
- **Description:** Add `/api/v1/*` endpoints and migrate frontend headless clients incrementally.
- **Acceptance criteria:** top workflows migrated; compatibility layer maintained.
- **Labels:** `backend`, `frontend`, `api`, `architecture`
- **Priority:** Medium

### Issue 9: Add repository ports for AppState-backed operations
- **Description:** Encapsulate multi-slice lock operations behind domain/application interfaces.
- **Acceptance criteria:** core use-cases no longer lock AppState directly.
- **Labels:** `backend`, `architecture`, `concurrency`, `tech-debt`
- **Priority:** Medium

## Phase 4

### Issue 10: Add architecture fitness checks in CI
- **Description:** Add lint/rules for layer boundaries and forbidden dependencies.
- **Acceptance criteria:** CI fails on violations; docs include remediation guidance.
- **Labels:** `backend`, `frontend`, `architecture`, `ci`
- **Priority:** Medium

### Issue 11: Implement contract-generation pipeline
- **Description:** Generate shared schemas/types from source DTO definitions.
- **Acceptance criteria:** build pipeline emits contracts; frontend consumes generated types/schemas.
- **Labels:** `backend`, `frontend`, `architecture`, `api`
- **Priority:** Medium

### Issue 12: Optimize heavy data paths and rendering/pagination strategy
- **Description:** Profile and optimize large dataset flows.
- **Acceptance criteria:** measurable improvement targets defined and met.
- **Labels:** `backend`, `frontend`, `performance`, `refactor`
- **Priority:** Medium

---

## 6. Quick Wins

1. Freeze net-new desktop UI features immediately.
2. Default all new frontend capabilities to WebUI routes/components.
3. Add a compatibility matrix: desktop command -> web endpoint.
4. Mark desktop-specific scripts/docs as deprecated.
5. Start moving headless clients from `/api/invoke/{command}` to explicit resource endpoints.

- Add architecture README/ADRs with dependency rules immediately.
- Pilot `invokeValidated` on one high-traffic API (`get_assets`) and expand.
- Extract one frontend orchestration module from `AnalysisView` as pattern template.
- Introduce shared async-status helper to remove repeated loading/error boilerplate.
- Add “adapter-only” comments/docs to backend command modules.
- Establish naming convention (`useCase`, `actions`, `selectors`, `mapper`) and enforce for new files.
- Stop new imports from deprecated type barrel and prefer domain type paths.

---

## 7. Long-Term Strategic Improvements

1. **Strategic initiative:** Retire desktop UI runtime and operate WebUI + API only (keep optional thin desktop shell only if required for packaging).
2. **Strategic initiative:** Full transition to explicit application-service architecture on backend and feature-layered architecture on frontend.
3. **Strategic initiative:** API contract-first development with generated schemas and compatibility tests.
4. **Strategic initiative:** Replace compatibility invoke dependence with stable versioned resource APIs in headless mode.
5. **Strategic initiative:** Event-driven internal workflow pipeline for long-running operations (capture/analysis/export).
6. **Strategic initiative:** Design-system and shared UI primitives for consistent section/table/form patterns.
7. **Strategic initiative:** Architecture governance tooling in CI (dependency graph checks, import boundaries, complexity thresholds).


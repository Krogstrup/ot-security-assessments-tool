# Backend Architecture & Modularization Review

## 1. Current Architecture Overview

### What exists today

The backend is implemented as:

- A **Rust workspace** with domain crates (`gm-analysis`, `gm-capture`, `gm-db`, `gm-ingest`, etc.) coordinated by a top-level Tauri crate.  
- A **Tauri command layer** (`src-tauri/src/commands/*`) that acts as API endpoints for desktop mode.  
- A **headless Axum HTTP binary** (`src-tauri/src/bin/kusanaginokajiki_web.rs`) that reuses the same command/app state model in web/server mode.  
- A **TypeScript frontend API layer** (`src/lib/api/*`) with runtime switching (`invokeCompat`) between Tauri IPC and HTTP.

This is effectively a **shared-core + dual-delivery** architecture (desktop IPC and HTTP). It has many good foundations: separated Rust crates for major concerns, and a stable API surface from TS via command names.

### Implicit architectural pattern

The command layer currently blends three roles:

1. **Controller/API adapter** (`#[tauri::command]` functions)
2. **Application service orchestration** (workflow, lock coordination, merge semantics)
3. **Data mapping/assembly** (DTO shaping and snapshot conversion)

`AppState` is organized by domain slices and explicit lock order, which is a strong operational control, but it also acts as a central shared data hub that many commands mutate directly.

### Boundary violations / unclear boundaries

- Controllers and use-case logic are tightly co-located in `commands/*.rs`.  
- Persistence logic is mostly isolated in `gm-db`, but transaction boundaries and domain invariants live in command handlers rather than explicit application services.  
- Domain crates are not uniformly “pure domain”; some are domain+application hybrids (e.g., analysis crate exports both models and executable policies).  
- Headless HTTP and Tauri command paths duplicate orchestration patterns instead of sharing explicit use-case interfaces.

---

## 2. Key Problems (categorized)

### A) Structural issues

1. **God module tendency in command layer**  
   Commands such as capture/analysis/session/ingest own large workflows directly, making them hard to test and evolve independently.

2. **`AppState` as a broad mutable integration hub**  
   It is cleanly partitioned by locks but still encourages cross-domain mutations from many entrypoints.

3. **Transport and application logic are interwoven**  
   Tauri/Axum endpoint concerns and business orchestration are mixed in the same functions, reducing reusability.

4. **Inconsistent layering depth across domains**  
   Some domains have stronger crate boundaries (e.g., parser/report engines), while others rely heavily on command-layer orchestration.

### B) Dependency / coupling issues

1. **High coupling to shared mutable state contracts**  
   Feature flows depend on lock ordering and multi-slice write sequencing, which increases incidental complexity.

2. **Potential temporal coupling**  
   Correctness of outcomes relies on call order and “remembering to update related slices” (capture/inventory/analysis/session consistency).

3. **Dual transport duplication risk**  
   Desktop and headless routes are aligned conceptually but not fully unified behind one application-service API.

### C) Naming / readability issues

1. **`commands` namespace overloaded conceptually**  
   It implies thin API handlers but contains substantial business workflows.

2. **Mixed vocabulary: command/data/service semantics**  
   Some files are named by transport intent (`commands/data.rs`) and others by domain intent (`commands/analysis.rs`), which can blur ownership.

3. **In TS, API modules are clear, but orchestration helpers are spread across components/utils/stores**  
   The frontend integration surface is usable but can feel fragmented for onboarding.

### D) SOLID principle violations

1. **Single Responsibility Principle (SRP)** violations in larger command handlers.  
2. **Open/Closed Principle (OCP)** pressure: adding behavior often means editing large command files rather than composing new use-case handlers.  
3. **Dependency Inversion Principle (DIP)** partially violated at app level: high-level workflows directly depend on concrete state and concrete storage adapters.

### E) Scalability / maintainability risks

1. **Feature growth concentrates complexity in command modules** (likely merge conflicts and regression risk).  
2. **Testing becomes integration-heavy** due to lack of narrow application-service seams.  
3. **Onboarding cost increases** because developers must reason about lock ordering, state slice interplay, and transport details simultaneously.

---

## 3. Target Modular Architecture

### Proposed target layers

#### 1) Interface Layer (outer)

- `interface/tauri/*` → Tauri command adapters (deserialize, call use case, serialize)
- `interface/http/*` → Axum handlers (same contract)
- No business rules, no direct state mutation.

#### 2) Application Layer

- `application/use_cases/*` (e.g., `ImportPcapUseCase`, `RunAnalysisUseCase`, `SaveSessionUseCase`)
- `application/services/*` for orchestration helpers
- Explicit input/output DTOs and result types
- Coordinates transactions/unit-of-work and domain services.

#### 3) Domain Layer

- Domain entities/value objects/policies by bounded context:
  - `discovery` (assets, connections, protocol evidence)
  - `analysis` (findings, risk scoring, purdue/compliance)
  - `session` (session lifecycle, archive semantics)
  - `topology` (graph semantics)
- Domain logic is framework-agnostic and persistence-agnostic.

#### 4) Infrastructure Layer (outer)

- DB repositories (`gm-db` adapters)
- File/pcap readers, external ingest parsers, report exporters
- Signature/OUI/GeoIP providers
- Runtime adapters (Tauri events, filesystem path resolution)

### Suggested folder/package structure

- `src-tauri/src/interface/tauri/*`
- `src-tauri/src/interface/http/*`
- `src-tauri/src/application/use_cases/*`
- `src-tauri/src/application/ports/*` (traits: repos/services/events)
- `src-tauri/src/application/dto/*`
- `src-tauri/src/domain/*` (or keep in workspace crates and formalize boundaries)
- `src-tauri/src/infrastructure/*` (adapters wiring to crates)

### Allowed dependency rules

- `interface -> application`
- `application -> domain + application ports`
- `infrastructure -> application ports + domain`
- `domain -> (none of above)`

Not allowed:

- `domain -> infrastructure`
- `application -> tauri/axum directly`
- `interface -> direct DB logic`

### What to split/merge/extract

- **Extract** use-case orchestration from `commands/*.rs` into `application/use_cases/*`.
- **Extract** mapping code (state → snapshots/DTOs) into dedicated assemblers/mappers.
- **Split** `AppState` access behind domain repositories/services interfaces.
- **Merge** duplicated transport logic into shared application contracts used by both Tauri and Axum adapters.

---

## 4. Refactoring Roadmap (step-by-step)

### Step 1 — Establish architecture seams (ports + DTO contracts)
- **Description:** Define application-level traits for repositories/services/events and standardized DTOs for major flows (import, analysis, session, ingest).  
- **Why it matters:** Creates stable contracts before moving logic.  
- **Expected impact:** Enables incremental extraction with minimal behavior change.  
- **Risk:** Low.  
- **Dependencies:** None.

### Step 2 — Introduce use-case modules for highest-value flows
- **Description:** Create use cases for `import_pcap`, `run_analysis`, `save_session/load_session`, `merge_ingest_result` while keeping existing command endpoints as wrappers.  
- **Why it matters:** Removes business logic from controllers first where complexity is highest.  
- **Expected impact:** Better testability and clearer ownership.  
- **Risk:** Medium.  
- **Dependencies:** Step 1.

### Step 3 — Convert command handlers into thin interface adapters
- **Description:** Refactor `commands/*.rs` to validate inputs, call use case, return response; no direct state orchestration.  
- **Why it matters:** Enforces controller/application separation.  
- **Expected impact:** Cleaner API layer and lower coupling to state internals.  
- **Risk:** Medium.  
- **Dependencies:** Step 2.

### Step 4 — Encapsulate `AppState` behind repository adapters
- **Description:** Create application repositories backed by `AppState` slices and lock handling internally; expose intention-level operations.  
- **Why it matters:** Reduces lock-order leakage and temporal coupling.  
- **Expected impact:** safer concurrency model, improved maintainability.  
- **Risk:** Medium.  
- **Dependencies:** Steps 1–3.

### Step 5 — Unify Tauri + Axum pathways through shared application API
- **Description:** Move shared route behavior to common interface/application wiring so desktop and headless modes differ only at transport adapter layer.  
- **Why it matters:** Eliminates drift and duplicate logic risk.  
- **Expected impact:** Higher consistency and reduced bug surface.  
- **Risk:** Low/Medium.  
- **Dependencies:** Step 3.

### Step 6 — Standardize domain boundaries per bounded context
- **Description:** Explicitly map crate/module ownership for Discovery, Analysis, Session, Physical, Segmentation; remove cross-context leakage from controllers.  
- **Why it matters:** Supports parallel team ownership and predictable change impact.  
- **Expected impact:** Better scalability of feature work.  
- **Risk:** Medium.  
- **Dependencies:** Step 4.

### Step 7 — Add architectural fitness tests
- **Description:** Add tests/lints that enforce dependency directions (e.g., no `tauri` usage outside interface layer, no infra imports in domain).  
- **Why it matters:** Prevents architecture regression.  
- **Expected impact:** Long-term structural integrity.  
- **Risk:** Low.  
- **Dependencies:** Steps 3–6.

### Step 8 — Frontend integration boundary cleanup
- **Description:** Define frontend-side application client facade (`lib/application-client/*`) that wraps `lib/api/*` and centralizes orchestration currently scattered across components/stores/utils.  
- **Why it matters:** Improves onboarding and cross-feature consistency in TS layer.  
- **Expected impact:** Easier feature delivery and fewer duplicated UI-side workflows.  
- **Risk:** Low.  
- **Dependencies:** Optional; can start after Step 2.

---

## 5. GitHub Issues List

### Issue 1
- **Title:** Define application ports and DTO contracts for backend use-cases  
- **Description:** Introduce `application/ports` and `application/dto` for import, analysis, session, ingest flows.  
- **Acceptance criteria:**
  - Ports exist for state, persistence, and event emission.
  - DTOs defined and used in at least one flow.
  - No behavior change in runtime output.
- **Labels:** `architecture`, `refactor`, `tech-debt`  
- **Priority:** High

### Issue 2
- **Title:** Extract Import PCAP workflow into `ImportPcapUseCase`  
- **Description:** Move orchestration from `commands/capture.rs` into application layer and keep command as adapter.  
- **Acceptance criteria:**
  - Command function is thin wrapper.
  - Use case unit tests cover success/error/cancel.
  - Existing API contract unchanged.
- **Labels:** `refactor`, `backend`, `capture`  
- **Priority:** High

### Issue 3
- **Title:** Extract security analysis orchestration into dedicated use-cases  
- **Description:** Move snapshot building and analysis execution from `commands/analysis.rs` into application module.  
- **Acceptance criteria:**
  - Analysis command contains no domain mapping loops.
  - Use-case tests validate data mapping and result persistence.
- **Labels:** `refactor`, `architecture`, `analysis`  
- **Priority:** High

### Issue 4
- **Title:** Refactor session save/load flows into session application service  
- **Description:** Move DB orchestration and topology rebuild workflow out of `commands/session.rs`.  
- **Acceptance criteria:**
  - Session commands reduced to adapter logic.
  - Session service has integration tests.
  - Lock ordering details hidden behind repository interface.
- **Labels:** `refactor`, `session`, `tech-debt`  
- **Priority:** High

### Issue 5
- **Title:** Introduce AppState repository adapters with encapsulated lock strategy  
- **Description:** Wrap direct lock usage in repositories to prevent lock-order leakage.  
- **Acceptance criteria:**
  - No direct lock acquisition in command adapters.
  - Repository APIs express intent-level operations.
- **Labels:** `architecture`, `concurrency`, `refactor`  
- **Priority:** Medium

### Issue 6
- **Title:** Unify Tauri and Axum endpoint execution through shared application facade  
- **Description:** Route both interface adapters through common use-case contracts.  
- **Acceptance criteria:**
  - Shared execution path for at least import + data queries.
  - Snapshot regression tests between transports pass.
- **Labels:** `architecture`, `backend`, `api`  
- **Priority:** Medium

### Issue 7
- **Title:** Enforce dependency rules with architecture tests/lints  
- **Description:** Add checks for layer rule violations.  
- **Acceptance criteria:**
  - CI fails on prohibited imports/usages.
  - Documentation describes allowed dependencies.
- **Labels:** `architecture`, `quality`, `ci`  
- **Priority:** Medium

### Issue 8
- **Title:** Create frontend application client facade over runtime API modules  
- **Description:** Consolidate TS-side orchestration into a clean application client layer.  
- **Acceptance criteria:**
  - At least two major views consume new facade.
  - Component-level API calls reduced.
  - Store update orchestration centralized.
- **Labels:** `frontend`, `refactor`, `architecture`  
- **Priority:** Medium

---

## 6. Quick Wins

1. **Rename/clarify command-layer intent**  
   Add module-level docs: “interface adapters only” goal and migration status.

2. **Create mapping helpers immediately**  
   Move large conversion blocks (e.g., analysis snapshot assembly) into dedicated mapper files without behavior changes.

3. **Introduce use-case for one pilot flow first**  
   Start with `run_analysis` or `save_session` to prove pattern before broad rollout.

4. **Add architecture decision record (ADR)**  
   Capture target layering and dependency rules to guide contributors.

5. **Frontend quick cleanup**  
   Centralize repeated `onDataChanged`/refresh orchestration patterns into a small app-client utility to reduce component coupling.

---

## 7. Long-Term Strategic Improvements

1. **Strategic change: Bounded-context workspace split with explicit APIs**  
   Reorganize around context-level crates/modules (Discovery, Analysis, Session, Reporting, Integration) with versioned internal contracts.

2. **Strategic change: Event-driven internal pipeline**  
   Introduce domain events (`CaptureImported`, `AssetsEnriched`, `AnalysisCompleted`) to decouple sequential workflows and support async/background processing.

3. **Strategic change: Unit-of-Work + transactional application services**  
   Formalize atomic operations for multi-slice updates and persistence consistency.

4. **Strategic change: Plugin-capable application services**  
   Make ingest/analysis/reporting extension points explicit via ports so new integrations do not require command-layer modifications.

5. **Strategic change: API contract versioning for desktop/headless parity**  
   Define shared contract schemas and compatibility tests to prevent drift across IPC and HTTP transports.


# CLAUDE.md — Execution & Architecture Workflow Guide

> Canonical agent policy is defined in `AGENTS.md`.
> This file defines **how Claude plans, executes, and verifies work in this repository**.

---

# 1) Mission

Operate on Kusanagi Kajiki with a **WebUI + API-first architecture**.

For every task:

1. Preserve or improve architecture boundaries
2. Prefer small, safe, reviewable changes
3. Validate against real code (never assume plan == reality)
4. Keep responses concise unless depth is requested

---

# 2) Source of Truth (Priority Order)

1. `AGENTS.md` → rules, constraints, execution behavior
2. `SYSTEM_ARCHITECTURE_REVIEW.md` → system-level direction
3. `ARCHITECTURE_REVIEW.md` → backend structure
4. `FRONTEND_ARCHITECTURE_REVIEW.md` → frontend structure
5. Migration plans (if applicable)


---

# 3) Execution Model (MANDATORY)

Claude must follow this loop:

### 1. Inspect

* Check real files, imports, usage
* Validate assumptions before acting

### 2. Decide

* Apply step
* Adapt step
* Skip step

### 3. Execute

* Make minimal, scoped changes

### 4. Verify

* Run relevant checks (`cargo check`, etc.)
* Confirm behavior unchanged (unless requested)

---

## Hard rules

* Do NOT blindly follow plans
* Do NOT delete before successful compile
* Do NOT apply global replacements without verification
* If mismatch is found → adapt, not abort

---

# 4) Architecture Model (Enforced)

## Backend

Layers:

* **interface** → `api_*.rs` (HTTP only)
* **adapter** → `commands/*` (thin boundary target)
* **application** → `application/*` (use-cases, orchestration)
* **domain** → `gm-*` (pure logic)

### Rules

* Commands must shrink over time (not grow)
* Application owns workflows
* Domain is pure and dependency-free

Correct direction:

```text
api → commands → application → gm-*
```

Forbidden:

```text
gm-* → commands
application → commands
```

---

## Frontend

Layers:

* ui → `.svelte`
* application → `*Flows.ts`
* state → stores/actions
* infra → API layer

### Rules

* No orchestration in large views
* Flows own async workflows
* API calls stay centralized

---

# 5) Operating Modes

---

## A) Planning Mode

Use when:

* task is large
* architecture unclear
* multiple files involved

Output:

1. Objective
2. Current state (based on inspection)
3. Risks
4. Incremental plan (3–8 steps)
5. Validation strategy

---

## B) Coding Mode

Use for implementation.

### Required behavior

* Verify file exists before modifying
* Check usage before deleting or moving
* Keep diffs small and reversible
* Prefer extraction over expansion

### If task is large

* Deliver first safe slice
* Explicitly list follow-up slices

---

## C) Review Mode

Use for audits / PR review.

Output:

1. Correctness
2. Architecture alignment
3. Coupling risks
4. Contract/API risks
5. Smallest high-impact fix

Severity:

* Critical
* Major
* Minor

---

# 6) Refactor Discipline (Important for this repo)

This repo is actively reducing:

* micro-files
* micro-crates
* adapter-layer overload

### Therefore:

When refactoring:

* Merge trivial files when tightly coupled
* Remove indirection with no abstraction value
* Keep domain logic untouched
* Move orchestration → application layer

---

## Safe Refactor Pattern

1. Locate usage (`rg`)
2. Move logic
3. Update call sites
4. Compile
5. Remove old code

Never skip compile step.

---

# 7) Backend-Specific Rules

* Do not add new orchestration to `commands/*`
* Prefer adding use-cases in `application/*`
* Keep DTOs explicit and typed
* Preserve AppState lock ordering (critical)

Lock order must not change.

---

# 8) Frontend-Specific Rules

* Extract flows from large components
* Avoid direct writable store mutation for complex flows
* Expand runtime validation on critical API paths

---

# 9) Contract Discipline

When backend changes affect API:

* Update DTOs
* Update frontend types
* Update validation schemas
* Maintain error format:

```json
{ "code": "...", "message": "...", "details": {} }
```

---

# 10) Execution Checks

Run only what is relevant:

### Backend

```bash
cargo check
cargo test (if needed)
```

### Frontend

```bash
npm run check
```

Report succinctly:

* ✅ success
* ❌ failure + reason

---

# 11) Safe Change Strategy

For medium/large work:

1. Add abstraction
2. Move logic
3. Update callers
4. Compile
5. Remove old path

Avoid big-bang changes.

---

# 12) WebUI-First Constraint

* Prefer HTTP endpoints over command-style calls
* Keep compatibility paths only when necessary
* Mark deprecated paths clearly

---

# 13) Done Definition

A task is complete when:

* behavior implemented correctly
* architecture boundaries respected
* code compiles cleanly
* risks are documented
* changes are reviewable and minimal

---

# 14) Mental Model (Quick Reference)

* **Domain (`gm-*`)** → capabilities (pure logic)
* **Application** → use-cases (orchestration)
* **Commands** → adapter (thin boundary)

---

# 15) Anti-Patterns (Avoid)

* Growing `commands/*`
* Logic inside HTTP handlers
* Blind refactors without inspection
* Deleting before compile passes
* Cross-layer dependency leaks

---

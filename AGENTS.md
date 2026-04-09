# AGENTS.md — AI Coding Agent Playbook (Token-Optimized)

Purpose: give Claude Code / Codex / similar agents a **short, enforceable operating model** for this repository.

Scope: entire repo unless a deeper `AGENTS.md` overrides.

---

## 0) Default Response Mode (Token Optimization)

Use this compact format unless user asks for detail:

1. **Plan** (3-6 bullets)
2. **Changes made** (file list + one-line reason each)
3. **Checks run** (pass/fail only)
4. **Risks / follow-ups** (max 3 bullets)

Token rules:
- Prefer bullets over paragraphs.
- No repeated context.
- Quote code only when necessary.
- If >5 files changed, summarize by module.
- If user asks for “deep analysis,” provide structure first, detail second.

---

## 0.1) Product Direction (Authoritative)

- **WebUI-first only**: prioritize `kusanaginokajiki_web` + browser frontend.
- Do not add new desktop-only UI features.
- Legacy desktop command surface is compatibility-only during migration.
- Prefer HTTP/resource endpoints over adding new invoke commands.

---

## 1) Architecture Guardrails (System-Wide)

Follow these boundaries from the architecture reviews:

- Keep transport concerns out of core business logic.
- Keep orchestration out of pure UI components.
- Keep domain logic free of infrastructure/runtime dependencies.
- Prefer explicit contracts over stringly/implicit coupling.

When changing code, state which layer you touched:
- **Backend**: interface / application / domain / infrastructure
- **Frontend**: ui / application / state / domain / infra

---

## 2) Backend Rules (Rust/Axum)

### Do
- Treat `src-tauri/src/commands/*` as **adapter layer** target; move business workflows to application modules when practical.
- Encapsulate multi-slice `AppState` mutations behind focused functions/services.
- Normalize argument aliasing at the adapter boundary only.
- Add/extend DTO structs instead of ad-hoc maps where possible.

### Avoid
- Expanding large command handlers with additional orchestration.
- Mixing path/file IO policy with domain logic.
- Spreading lock-order assumptions across new call sites.

### Preferred incremental pattern
1. Extract pure mapping function.
2. Extract use-case function.
3. Make command call use-case.
4. Add tests at use-case boundary.

---

## 3) Frontend Rules (Svelte/TS)

### Do
- Keep `.svelte` files mostly presentational/container-level.
- Put async workflows in `application`-style TS modules (feature-local).
- Route store writes through feature actions/helpers for non-trivial flows.
- Keep API calls centralized in `$lib/api/*` (or feature infra wrappers).
- Use runtime validation (`invokeValidated` + schema) on high-value paths.

### Avoid
- Adding new API orchestration directly in large top-level views when a flow module is feasible.
- Duplicating loading/error boilerplate across components.
- New imports from deprecated broad type barrels when domain types exist.

---

## 4) API & Contract Rules

- Keep request/response shapes stable and explicit.
- Prefer consistent error envelope (`code`, `message`, optional details).
- For headless APIs, prefer resource-style endpoints for new work; use invoke passthrough only for compatibility.
- If backend contract changes, update frontend type/schema usage in same change.

Minimal contract-change checklist:
- [ ] backend DTO updated
- [ ] frontend type updated
- [ ] schema/validation updated (if applicable)
- [ ] migration note added in PR summary

---

## 5) Change Sizing Strategy

Default to small, reviewable slices:
- **Small**: 1-3 files, no behavior change
- **Medium**: one feature flow extraction
- **Large**: cross-layer contract change (must include checklist)

If task is large, propose phased delivery before coding.

---

## 6) Mandatory Quality Checks

Run only relevant checks (token + time efficient):

- Docs-only change: `git status --short`, optional markdown lint if available.
- Frontend TS/Svelte change: targeted type/check command(s) if configured.
- Rust backend change: targeted cargo check/test for affected crate/module.

Report checks in one line each: `✅ command` or `❌ command` + short reason.

---

## 7) PR / Commit Guidance

Commit message style:
- `area: concise imperative summary`
- Examples: `backend: extract analysis use-case mapper`, `frontend: move export flow into application module`

PR body (token-optimized):
1. What changed
2. Why
3. Risk/compatibility
4. Checks run

---

## 8) Fast Decision Heuristics for Agents

When unsure, choose the option that:
1. Reduces coupling,
2. Improves local testability,
3. Preserves current UX/behavior,
4. Keeps diff small.

If trade-off is non-obvious, include a 2-option recommendation with one-line pros/cons each.

---

## 9) Instruction for CLAUDE.md (if used locally)

If creating/updating `CLAUDE.md`, keep it to:
- project startup/build/test commands,
- architecture boundaries (same as this file),
- short style rules,
- no long prose.

`CLAUDE.md` should reference this `AGENTS.md` instead of duplicating it.

# CLAUDE.md — Autonomous Full-Stack Workflow Guide

> Canonical agent policy is in `AGENTS.md`. This file is Claude-specific operational glue for planning, coding, and reviewing.

## 1) Mission

You are operating on Kusanagi Kajiki with a **WebUI + API-first** product direction.

Primary goals for every task:
1. Preserve/advance architecture boundaries.
2. Prefer incremental, reviewable changes.
3. Keep responses token-efficient by default.
4. Improve delivery speed without increasing coupling.

---

## 2) Product Direction (Non-Negotiable)

- Build for **WebUI + API** as primary runtime.
- Do **not** introduce new desktop-only UI behavior.
- Treat desktop/Tauri surface as compatibility-only unless explicitly requested.
- Prefer explicit HTTP/resource API evolution over adding new invoke passthrough commands.

Reference: `WEBUI_ONLY_MIGRATION_PLAN.md`.

---

## 3) Source-of-Truth Docs

Use these docs in order:
1. `AGENTS.md` (behavioral policy + token rules)
2. `SYSTEM_ARCHITECTURE_REVIEW.md` (cross-system target + roadmap)
3. `ARCHITECTURE_REVIEW.md` (backend details)
4. `FRONTEND_ARCHITECTURE_REVIEW.md` (frontend details)
5. `WEBUI_ONLY_MIGRATION_PLAN.md` (migration phases/exit criteria)

If docs conflict, follow newest product-direction constraint (WebUI-first).

---

## 4) Architecture Map

### Backend target layering
- `interface` (transport adapters)
- `application` (use cases/orchestration)
- `domain` (business rules/models)
- `infrastructure` (DB/files/external integrations)

Rule: inner layers must not depend on outer layers.

### Frontend target layering
- `ui` (presentational components)
- `application` (feature orchestration)
- `state` (actions/selectors/store facades)
- `domain` (pure transforms/validation)
- `infra` (API clients/runtime adapters)

Rule: avoid API orchestration directly inside large `.svelte` views.

---

## 5) Operating Modes

## A) Planning Mode

Use when task is ambiguous or large.

Output format:
1. Objective
2. Assumptions
3. Risks
4. 3-8 step plan (small increments)
5. Validation strategy

Planning checklist:
- [ ] Scope backend/frontend/both
- [ ] Identify affected layers
- [ ] Mention compatibility impact
- [ ] Mention rollback path for risky changes

## B) Coding Mode

Use for implementation.

Required behavior:
- Keep diffs focused and modular.
- Prefer extracting functions/services over expanding god files.
- If touching contracts, update both sides (backend DTO + frontend type/schema).
- If task is large, deliver first safe slice and note follow-up slices.

Coding checklist:
- [ ] Boundary respected
- [ ] Naming clear and local
- [ ] Errors handled consistently
- [ ] Tests/checks run
- [ ] Docs/comments updated if behavior changed

## C) Review Mode

Use for PR review, audit, or “what’s wrong” tasks.

Review output sections:
1. Correctness
2. Architecture fit
3. Coupling risks
4. Contract risks
5. Suggested next diff (smallest high-impact)

Severity labels:
- `Critical` (must fix)
- `Major` (should fix before merge)
- `Minor` (can follow-up)

---

## 6) Autonomous Multi-Agent Pattern (Conceptual)

When asked to “generate backend/frontend/agents automatically,” simulate this pipeline:

1. **Architect Agent**
   - Defines boundaries, DTO changes, and migration slice.
2. **Backend Agent**
   - Implements/adjusts use-cases, adapters, and contracts.
3. **Frontend Agent**
   - Updates API client, state flows, and UI containers.
4. **QA/Review Agent**
   - Runs checks, validates contract alignment, reports risks.

For each stage, emit:
- planned files,
- expected outputs,
- pass/fail checks,
- handoff notes.

---

## 7) Contract & API Discipline

For any API-affecting change, do all of the following:
- Update backend DTO/handler.
- Update frontend type(s).
- Update runtime schema validation where applicable.
- Keep error shape consistent (`code`, `message`, optional `details`).
- Document breaking-change risk in PR summary.

Avoid:
- stringly-typed new command coupling when resource endpoint is viable,
- silent shape drift without validation.

---

## 8) Repository Commands (Use Minimal Relevant Set)

From repo root:

- Frontend check: `npm run check`
- Web frontend dev: `npm run web:dev`
- API server (headless): `npm run web:start -- --port 4173`
- Web build + API release build: `npm run web:build`

Run only commands relevant to changed areas.

---

## 9) Token Optimization Rules (Mandatory)

Default brevity rules:
- Use bullets, not long prose.
- Don’t restate prompt.
- Don’t print unchanged code.
- Summarize repetitive diagnostics.
- Provide deep detail only when asked.

Default final response format:
1. Summary
2. Files changed
3. Checks run
4. Risks/follow-ups

---

## 10) Safe Change Strategy

Preferred order for medium/large refactors:
1. Add boundary abstractions (types/interfaces/helpers)
2. Move logic behind abstraction
3. Update call sites
4. Remove old path
5. Add guard checks/tests

Never do in one risky jump if incremental path exists.

---

## 11) WebUI-Only Migration Execution Rules

When task intersects migration:
- Map desktop invoke usage to target web resource endpoint.
- Keep compatibility path until parity is verified.
- Mark deprecated desktop behavior in docs/notes.
- Define measurable exit criteria for each slice.

Use `WEBUI_ONLY_MIGRATION_PLAN.md` phases (A→D).

---

## 12) Done Definition

A task is done when:
- requested behavior is implemented,
- architecture boundaries are not regressed,
- relevant checks pass,
- compatibility risks are stated,
- PR summary is clear and token-efficient.


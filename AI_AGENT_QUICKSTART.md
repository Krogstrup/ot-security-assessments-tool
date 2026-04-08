# AI Agent Quickstart (How to Use Correctly)

This project now has two instruction layers:

1. `AGENTS.md` (repo-wide policy for any coding agent)
2. `CLAUDE.md` (Claude-specific workflow for planning/coding/review)

Use both in this order.

## 1) Start Every Task

- Read `AGENTS.md` first.
- Read `CLAUDE.md` second.
- For architecture tasks, also open:
  - `SYSTEM_ARCHITECTURE_REVIEW.md`
  - `ARCHITECTURE_REVIEW.md`
  - `FRONTEND_ARCHITECTURE_REVIEW.md`
  - `WEBUI_ONLY_MIGRATION_PLAN.md`

## 2) Choose Mode

From `CLAUDE.md`:

- Planning Mode: for ambiguous/large tasks.
- Coding Mode: for implementation.
- Review Mode: for PR analysis and audit.

## 3) Follow Product Direction

Use WebUI + API as primary target:

- No net-new desktop-only UI features.
- Treat desktop/Tauri as compatibility-only unless explicitly requested.

## 4) Keep Responses Token-Efficient

Default output:

1. Summary
2. Files changed
3. Checks run
4. Risks/follow-ups

Use bullets and avoid repeating context.

## 5) If You Change Contracts

Update all of these in the same task:

- backend DTO/handler,
- frontend type(s),
- runtime schema validation,
- PR note for compatibility impact.

## 6) Run Minimal Relevant Checks

- Docs-only: `git status --short`
- Frontend: `npm run check`
- Web/API run: `npm run web:dev`, `npm run web:start -- --port 4173`

## 7) Migration Workflow (WebUI-Only)

Execute in phases from `WEBUI_ONLY_MIGRATION_PLAN.md`:

- Phase A: Freeze + inventory
- Phase B: API stabilization
- Phase C: Frontend migration
- Phase D: decommission compatibility paths

## 8) Common Mistakes to Avoid

- Adding orchestration to large UI components when a flow/service file can be used.
- Adding new stringly invoke commands when a resource endpoint is viable.
- Updating backend contracts without frontend/type/schema alignment.
- Returning long narrative responses when short structured output is enough.
---
name: architecture-review-refresh
description: Refresh ARCHITECTURE_REVIEW.md, SYSTEM_ARCHITECTURE_REVIEW.md, and FRONTEND_ARCHITECTURE_REVIEW.md so they match the current repository architecture. Use when asked to update architecture review docs, sync docs with actual backend/frontend structure, or audit architecture drift before planning refactors.
---

<!-- markdownlint-disable MD013 MD029 MD032 -->

# Architecture Review Refresh

## Overview

Update the three architecture review docs from source code evidence, not from prior doc assumptions. Keep recommendations incremental and aligned with the current code layout.

## Workflow

1. Collect backend runtime evidence.
- Verify runtime entrypoint and route composition in:
  - `backend/Cargo.toml`
  - `backend/src/bin/kusanaginokajiki_web.rs`
  - `backend/src/bin/kusanaginokajiki_web/web_routes.rs`
  - `backend/src/bin/kusanaginokajiki_web/web_api_paths.rs`
- Verify current extraction level in:
  - `backend/src/application/mod.rs`
  - `backend/src/application/queries/*`
  - `backend/src/application/use_cases/*`
  - `backend/src/application/services/*`
  - `backend/src/commands/mod.rs`

2. Collect frontend structure evidence.
- Verify app shell and top-level routing in:
  - `src/routes/+page.svelte`
- Verify API/validation status in:
  - `src/lib/api/core.ts`
  - `src/lib/api/*.ts`
  - `src/lib/schemas.ts`
- Verify orchestration concentration and extraction in:
  - `src/lib/components/*View.svelte`
  - `src/lib/components/capture/*.ts`
  - `src/lib/components/export/*.ts`
  - `src/lib/stores/*.ts`

3. Quantify hotspots before writing conclusions.
- Use line counts for large command/view files (`wc -l`) to avoid vague claims.
- Keep findings tied to concrete file paths and current responsibilities.

4. Rewrite each doc for current-state truth.
- `ARCHITECTURE_REVIEW.md`:
  - Focus on backend layering as currently implemented.
  - Distinguish extracted application modules vs remaining command-heavy modules.
  - Avoid aspirational claims unless marked as target-state recommendations.
- `SYSTEM_ARCHITECTURE_REVIEW.md`:
  - Describe current frontend-backend runtime topology and contract flow.
  - Highlight cross-layer inconsistencies (error envelope, validation coverage, dependency leakage).
  - Provide phased roadmap with clear cross-layer outcomes.
- `FRONTEND_ARCHITECTURE_REVIEW.md`:
  - Map real orchestration placement (what is extracted vs still in view components).
  - Map API validation coverage by module.
  - Recommend incremental extraction plan without forcing a folder big-bang.

5. Keep recommendations pragmatic.
- Prefer in-place incremental refactors over full rewrites.
- Keep endpoint/function names stable unless the user requested contract breaks.
- Respect repository direction: web-first runtime and explicit HTTP endpoints.

6. Validate and report.
- Run markdown lint on modified markdown files.
- Run `git status --short` and include changed files in final summary.

## Quality Bar

- Do not copy stale claims from existing docs without re-checking code.
- Do not mention deprecated runtime paths unless they still exist in code.
- Tie every major finding to specific modules/files.
- Keep issue backlog scoped and actionable.

## References

Read `references/review-checklist.md` for reusable command patterns and section-by-section coverage checks.

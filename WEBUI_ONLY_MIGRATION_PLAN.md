# WebUI-Only Migration Plan

## Goal
Decommission desktop-first frontend behavior and operate the product as **WebUI + API**.

## Non-Goals
- No large rewrite.
- No abrupt removal of compatibility paths before replacement endpoints exist.

## Phases

### Phase A — Freeze & Inventory
1. Freeze net-new desktop-only UI features.
2. Inventory all frontend calls using `/api/invoke/{command}` and map to explicit endpoints.
3. Tag desktop-specific scripts/docs as deprecated.

### Phase B — API Surface Stabilization
1. Add versioned resource endpoints for high-traffic workflows (`assets`, `connections`, `analysis`, `sessions`, `projects`).
2. Standardize error envelope (`code`, `message`, `details?`).
3. Add contract validation tests for WebUI paths.

### Phase C — Frontend Migration
1. Move WebUI API modules from invoke passthrough to resource endpoints by feature slice.
2. Keep adapter fallback only behind explicit compatibility flags.
3. Remove desktop-runtime assumptions from top-level routing/state defaults.

### Phase D — Decommission
1. Remove unused desktop invoke mappings once parity is confirmed.
2. Keep optional thin shell only if packaging requirements remain.
3. Publish migration completion checklist and support window policy.

## Exit Criteria
- 100% primary user workflows run via WebUI + resource APIs.
- No critical path depends on desktop-only UI runtime.
- Compatibility invoke usage reduced to zero for maintained features.

## Suggested First 3 Issues
1. `api: add v1 resources for analysis + sessions`
2. `frontend: migrate analysis/projects modules off invoke passthrough`
3. `platform: mark desktop frontend paths deprecated and freeze feature additions`

<!-- markdownlint-disable MD013 MD029 MD032 -->

# Frontend Architecture Review (Svelte + TypeScript)

Last verified against code on **2026-04-09**.

## 1. Current Structure Map

- App shell / primary route: `src/routes/+page.svelte`
- Feature containers: `src/lib/components/*View.svelte`
- Feature submodules: `src/lib/components/<feature>/*`
- API client modules: `src/lib/api/*.ts`
- State modules: `src/lib/stores/*.ts`
- Runtime schemas: `src/lib/schemas.ts`

Feature folders under `src/lib/components` are now broad and explicit (`analysis`, `capture`, `export`, `physical`, `projects`, `segmentation`, `timeline`, etc.), indicating ongoing extraction from monolithic views.

## 2. Shell and Container Ownership

### App shell (`+page.svelte`, 237 lines)

`+page.svelte` still owns:

- top-level tab switching (`projects`, `topology`, `analysis`, etc.)
- topology sub-tab management (`topologyTabs`, `activeTopologyTabId`, close handlers)
- split layout behavior (connection tree show/hide)

This is appropriate for shell-level concerns, but it still mixes layout orchestration and some startup data bootstrap (`listInterfaces()` on mount).

### Container views

Current container hotspots (`wc -l`):

- `AnalysisView.svelte`: 249
- `ConnectionTree.svelte`: 245
- `SegmentationView.svelte`: 236
- `ProjectsView.svelte`: 209
- `ExportView.svelte`: 202
- `PhysicalView.svelte`: 188
- `InventoryView.svelte`: 205

Notable positive extraction:

- `CaptureView.svelte` is now thin (44 lines), delegating behavior to capture subcomponents and flow modules.
- `ExportView.svelte` delegates significant workflow logic into `components/export/exportFlows.ts`.

## 3. Orchestration Extraction Status

Large extracted TS workflow modules exist and are actively used:

- `components/capture/sessionFlows.ts` (186)
- `components/capture/externalImportTasks.ts` (170)
- `components/export/exportFlows.ts` (160)
- `components/export/exportUtils.ts` (107)

This confirms incremental extraction is real, not just planned.

Remaining orchestration-heavy container behavior is still visible in:

- `AnalysisView.svelte` (section-level async loading and coordination)
- `ProjectsView.svelte` (CRUD orchestration + modal form flow)
- `SegmentationView.svelte` (run pipeline + staged loading + export actions)

## 4. API and Validation Status

### Transport baseline

- API calls are centralized through `httpValidated` in `src/lib/api/core.ts`.
- `httpJson` exists as a lower-level helper but is not directly used by module call sites (`rg "httpJson\(" src/lib/api/*.ts` returns no hits).

### Validation coverage

- All API modules call `httpValidated(...)`.
- `src/lib/schemas.ts` contains broad schema coverage for core entities and many high-traffic responses.
- Validation is still mixed-depth: `z.unknown()` remains common in API modules where typed schemas are not yet attached.

Current `z.unknown()` concentration by module:

- `connections.ts`: 4
- `capture.ts`: 4
- `assets.ts`: 3
- `system.ts`: 3
- `wireshark.ts`: 3
- `correlation.ts`: 3
- `signatures.ts`: 2
- `projects.ts`: 2

## 5. Store Layer Status

Store modules are split by concern (`core`, `analysis`, `navigation`, `capture`, `physical`, `session`, `timeline`, `topology-tabs`).

- `stores/core.ts` is the largest (114 lines) and owns shared entity collections plus derived views like `connectionTree`.
- Most stores remain data-centric and avoid direct async side-effects.

## 6. Frontend Risks

1. Container orchestration remains uneven: some features are well-extracted (`capture`, `export`), while others still coordinate multi-step flows in `.svelte` containers.
2. `z.unknown()` usage still weakens runtime contract guarantees on several high-traffic endpoints.
3. `+page.svelte` remains a central coupling point for shell + topology-tab behavior.

## 7. Incremental Frontend Roadmap

1. Prioritize schema tightening for `z.unknown()` endpoints in `connections`, `capture`, `assets`, and `system` modules.
2. Continue container thinning by extracting async orchestrators from `AnalysisView`, `ProjectsView`, and `SegmentationView` into feature-local TS flow modules.
3. Keep `+page.svelte` as shell/router only; move non-shell bootstrap/orchestration into feature entry helpers.
4. Preserve existing endpoint paths and response shapes while tightening internal typing and flow boundaries.

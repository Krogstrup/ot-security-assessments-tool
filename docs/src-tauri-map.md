# `src-tauri` File Map

This document is the maintained map for everything under `src-tauri/`.

It answers, for each file:
- what it is for,
- what it does,
- how it is used by the rest of the code,
- how to maintain it safely.

Last reviewed: 2026-04-09.

---

## How To Maintain This Document

When `src-tauri/` changes, update this file in the same PR.

Required update checklist:
- Add/remove/rename file entries to match current tree.
- Update "Used in code" links when call paths change.
- Update "Maintain" notes when contracts or lock ordering change.
- If a backend DTO changes, verify related generated TS types section is updated.

Quick audit commands:

```bash
find src-tauri -type f | sort
rg -n "^pub mod|^mod |^pub use " src-tauri/src src-tauri/crates
rg -n "^pub fn|route\(" src-tauri/src
```

Ownership intent:
- `src-tauri/src/*`: adapter/runtime layer (HTTP + command wiring).
- `src-tauri/crates/gm-*`: domain/application libraries.
- `src-tauri/signatures/*` and `src-tauri/data/*`: runtime data inputs.
- `src-tauri/bindings/gen/types/*`: generated API contract surface for frontend.

---

## Architecture Wiring (How Files Connect)

Runtime flow:
1. `src-tauri/src/bin/kusanaginokajiki_web.rs` starts the web server and owns route registration.
2. Routes call `src-tauri/src/commands/*` functions.
3. Commands operate on `AppState` from `src-tauri/src/commands/mod.rs`.
4. Domain logic lives in `src-tauri/crates/gm-*` crates.
5. Results are serialized using shared models (`gm-types`) and generated TS types (`bindings/gen/types`).

Critical state rule:
- Lock order in `AppState` must stay: `capture -> inventory -> analysis -> session -> physical -> segmentation -> signatures`.

## Recent Refactor Notes (2026-04-09)

- `src-tauri/src/commands/analysis_builders.rs` was deleted and split into two focused modules:
  - `analysis_input_builder.rs` — `build_analysis_input()` only.
  - `capture_context_builder.rs` — `build_capture_context()` only.
- `src-tauri/src/application/mappers/deep_parse.rs` was added as the **canonical DeepParseSnapshot mapper** (all 7 protocols). The duplicate in `analysis.rs` (`build_malware_deep_parse`, Modbus+IEC104 only) was deleted and callers now use this single function.
- `src-tauri/src/application/use_cases/ingest/mod.rs` was thinned to an orchestrator. Extracted:
  - `asset_merge.rs` — `enrich_asset` / `create_asset_from_ingested` helpers.
  - `alert_index.rs` — `ingested_alert_to_stored` / `rebuild_zeek_device_events` / `classify_zeek_log_type`.
- `src-tauri/src/bin/kusanaginokajiki_web/web_support.rs` was thinned to a re-export shim. Extracted:
  - `http_types.rs` — `ApiError`, `ImportPcapFilesResponse`.
  - `import_support.rs` — `ImportKind`, `HeadlessRuntimeConfig`, all resolve/list path helpers.
- `gm-types/src/lib.rs` gained `MAX_FINDINGS` and 15 `DEVICE_TYPE_*` string constants.
- `gm-analysis/src/attack_codes.rs` was added with all 31 MITRE T-code constants. All literal T-code strings across the crate now reference this module.
- `gm-analysis/src/attack/test_utils.rs` was added with a shared `make_input()` helper. Eight duplicate copies removed from attack sub-modules.

## Recent Refactor Notes (2026-04-08)

- `src-tauri/src/bin/web_*.rs` helper files were moved under `src-tauri/src/bin/kusanaginokajiki_web/`.
  - Why: files directly under `src/bin/*.rs` are auto-discovered by Cargo as standalone binaries.
  - Maintain: keep helper modules under `src/bin/kusanaginokajiki_web/` and reference them from `kusanaginokajiki_web.rs` with explicit `#[path = "..."]` attributes.
- Route path literals are centralized in `src-tauri/src/bin/kusanaginokajiki_web/web_api_paths.rs`.
  - Maintain: add/update endpoint paths there first, then consume constants in `web_routes_*`.
- Shared command helpers are centralized in `src-tauri/src/commands/support.rs`.
  - Maintain: use `read_state` / `write_state` / `mutex_state` for lock access and `app_data_dir()` for `~/.kusanaginokajiki` paths.
- Sort contracts for data endpoints are now explicit enums in `src-tauri/src/commands/data.rs`:
  - `AssetSortBy`, `ConnectionSortBy`, `ProtocolStatsSortBy`.
  - Maintain: keep these enum variants aligned with frontend API constants in `src/lib/api/contracts.ts`.

---

## Root Files

### `src-tauri/Cargo.toml`
- For: workspace and package manifest for the backend runtime and all `gm-*` crates.
- Does: defines workspace members, dependency graph, release profile, and default binary (`kusanaginokajiki_web`).
- Used in code: drives all Cargo builds; every crate dependency edge resolves here.
- Maintain: keep workspace membership accurate, avoid duplicate dependency versions unless required, keep release profile intentional.

### `src-tauri/Cargo.lock`
- For: lockfile for reproducible backend dependency resolution.
- Does: pins exact crate versions and transitive graph.
- Used in code: consumed by Cargo at build/test time.
- Maintain: never hand-edit; regenerate with Cargo when dependencies change.

### `src-tauri/src/lib.rs`
- For: crate-level architecture documentation anchor.
- Does: documents top-level crate purpose and module boundaries.
- Used in code: doc surface only; no runtime logic.
- Maintain: keep comments aligned with actual architecture.

### `src-tauri/src/bin/kusanaginokajiki_web.rs`

- For: primary WebUI-first backend runtime (headless HTTP + static frontend server).
- Does: parses CLI args, creates `AppState`, registers `/api` routes, serves SSE events, keeps legacy invoke-compat endpoint.
- Used in code: central entrypoint for web mode; calls nearly all command modules.
- Maintain: add new functionality as resource-style routes first, keep invoke passthrough only for compatibility, preserve route ordering notes (`/active` before `/{id}`, etc).

### `src-tauri/src/bin/kusanaginokajiki_web/http_types.rs`

- For: HTTP-layer response types for the web binary.
- Does: defines `ApiError` (error envelope), `ImportPcapFileEntry`, and `ImportPcapFilesResponse`.
- Used in code: imported by handler modules and re-exported via `web_support.rs`.
- Maintain: keep `ApiError` shape (`code`, `message`) stable — frontend error handling depends on it.

### `src-tauri/src/bin/kusanaginokajiki_web/import_support.rs`

- For: import/export path resolution and file listing for web mode.
- Does: `ImportKind` enum, `HeadlessRuntimeConfig`, `RUNTIME_CONFIG` static, `resolve_frontend_dist`, `resolve_export_output_path`, `resolve_import_input_path`, `list_import_files_for_kind`.
- Used in code: import handlers and web runtime startup.
- Maintain: keep env var names stable (`KK_IMPORT_DIR`, `KK_EXPORT_DIR`); path resolution must be consistent across headless and dev modes.

### `src-tauri/src/bin/kusanaginokajiki_web/web_support.rs`

- For: backward-compatibility re-export shim for handler files.
- Does: re-exports `ApiError`, `ImportPcapFilesResponse` from `http_types` and all import helpers from `import_support`.
- Used in code: all 7 handler files (`web_handlers_*.rs`) import via `super::web_support::*`.
- Maintain: this is a thin shim — add new items to `http_types.rs` or `import_support.rs`, then re-export here.

---

## Generated Type Bindings (`src-tauri/bindings/gen/types`)

All files in this directory are generated by `ts-rs` from Rust structs.

Global maintenance rule:
- Do not edit manually.
- Update Rust model types in `gm-types`, `gm-parsers::models`, or `gm-ingest::models`.
- Regenerate bindings in test/build flow that emits `ts-rs` outputs.

### `src-tauri/bindings/gen/types/AssetInfo.ts`
- For: frontend contract for discovered assets.
- Does: mirrors Rust `AssetInfo` fields including confidence, vendor/product, geo/public flags.
- Used in code: consumed by frontend data tables/details views.
- Maintain: regenerate when Rust asset schema changes.

### `src-tauri/bindings/gen/types/AssetSignatureMatch.ts`
- For: frontend contract for signature match metadata per asset.
- Does: exposes signature name, confidence, vendor/product/device hints.
- Used in code: asset details and signature confidence UI.
- Maintain: regenerate on Rust struct changes.

### `src-tauri/bindings/gen/types/BacnetDetail.ts`
- For: BACnet deep-parse detail payload type.
- Does: carries BACnet role and attack-relevant BACnet operations flags.
- Used in code: deep-parse panels and analysis evidence rendering.
- Maintain: regenerate on Rust detail changes.

### `src-tauri/bindings/gen/types/ConnectionInfo.ts`
- For: frontend contract for connection records.
- Does: mirrors src/dst, protocol, traffic counters, times, origin files.
- Used in code: connection list, topology edge detail, exports.
- Maintain: regenerate with Rust schema changes.

### `src-tauri/bindings/gen/types/DeepParseInfo.ts`
- For: top-level per-device deep protocol parse aggregate type.
- Does: groups optional protocol-specific detail structs for a single IP.
- Used in code: device details and analysis modules needing protocol context.
- Maintain: regenerate when any protocol detail schema changes.

### `src-tauri/bindings/gen/types/DeviceZeekEvents.ts`
- For: frontend contract for Zeek event aggregate by device.
- Does: summarizes Zeek-derived event counters and sample events.
- Used in code: ingest/correlation UI.
- Maintain: regenerate on model changes.

### `src-tauri/bindings/gen/types/Dnp3Detail.ts`
- For: DNP3 deep-parse detail payload.
- Does: includes role, addresses, function code stats, unsolicited behavior, relationships.
- Used in code: DNP3 detail cards and ATT&CK rules.
- Maintain: regenerate on Rust detail updates.

### `src-tauri/bindings/gen/types/Dnp3Relationship.ts`
- For: DNP3 peer relationship row type.
- Does: links remote IP/role and packet count.
- Used in code: DNP3 relationship tables.
- Maintain: regenerate on Rust relation struct changes.

### `src-tauri/bindings/gen/types/EnipDetail.ts`
- For: EtherNet/IP aggregate detail payload.
- Does: carries scanner/adapter role and CIP write/file-access/list-identity indicators.
- Used in code: EIP detail view and ATT&CK logic context.
- Maintain: regenerate on Rust detail changes.

### `src-tauri/bindings/gen/types/FunctionCodeStat.ts`
- For: protocol function-code histogram row type.
- Does: stores code, name, count, write/read classification.
- Used in code: Modbus/DNP3 stats visualizations.
- Maintain: regenerate on struct changes.

### `src-tauri/bindings/gen/types/Iec104Detail.ts`
- For: IEC-104 aggregate detail payload.
- Does: role and control/reset/interrogation flags.
- Used in code: IEC104 detail panel and ATT&CK rules.
- Maintain: regenerate on Rust changes.

### `src-tauri/bindings/gen/types/LldpDetail.ts`
- For: LLDP enrichment payload type.
- Does: exposes system/chassis/port/capability/vendor/model/firmware data.
- Used in code: infrastructure identification and device context UI.
- Maintain: regenerate on Rust changes.

### `src-tauri/bindings/gen/types/ModbusDetail.ts`
- For: Modbus aggregate detail payload.
- Does: role, function codes, register ranges, polling stats, device-id and peer relationships.
- Used in code: Modbus detail panel, analysis, segmentation input.
- Maintain: regenerate on Rust changes.

### `src-tauri/bindings/gen/types/ModbusDeviceIdInfo.ts`
- For: Modbus FC43/14 device identity sub-structure.
- Does: vendor/product/model/revision fields extracted from payload.
- Used in code: high-confidence device enrichment.
- Maintain: regenerate on Rust changes.

### `src-tauri/bindings/gen/types/ModbusRelationship.ts`
- For: Modbus peer relationship row.
- Does: remote role, unit IDs, packet count.
- Used in code: device relationship views and ATT&CK logic.
- Maintain: regenerate on Rust changes.

### `src-tauri/bindings/gen/types/PacketSummary.ts`
- For: lightweight frame list row type.
- Does: timestamp + addressing + protocol + length + origin file.
- Used in code: connection packet view and Wireshark CSV exports.
- Maintain: regenerate on Rust changes.

### `src-tauri/bindings/gen/types/PollingInterval.ts`
- For: periodicity stats row type.
- Does: remote/function-code interval statistics.
- Used in code: anomaly evidence and protocol detail UIs.
- Maintain: regenerate on Rust changes.

### `src-tauri/bindings/gen/types/ProfinetDcpDetail.ts`
- For: PROFINET DCP detail payload.
- Does: role and optional device name.
- Used in code: PROFINET detail views and segmentation role modeling.
- Maintain: regenerate on Rust changes.

### `src-tauri/bindings/gen/types/ProtocolStatInfo.ts`
- For: protocol aggregate statistics type.
- Does: per-protocol packet/byte/connection/device counts.
- Used in code: protocol distribution UI and reports.
- Maintain: regenerate on Rust changes.

### `src-tauri/bindings/gen/types/RegisterRangeInfo.ts`
- For: Modbus register range access row type.
- Does: start/count/type/access_count fields.
- Used in code: Modbus deep detail tables.
- Maintain: regenerate on Rust changes.

### `src-tauri/bindings/gen/types/S7Detail.ts`
- For: S7 aggregate detail payload.
- Does: role and S7 function list seen.
- Used in code: S7 detail panels and attack detections.
- Maintain: regenerate on Rust changes.

### `src-tauri/bindings/gen/types/SnmpDetail.ts`
- For: SNMP identity payload type.
- Does: sysDescr/sysName/sysLocation/objectID/vendor etc.
- Used in code: infrastructure classification and device metadata UI.
- Maintain: regenerate on Rust changes.

### `src-tauri/bindings/gen/types/StoredAlert.ts`
- For: normalized imported alert type.
- Does: Suricata/Wazuh alert fields for state storage.
- Used in code: correlation commands and alerts UI.
- Maintain: regenerate on Rust changes.

### `src-tauri/bindings/gen/types/ZeekEventSummary.ts`
- For: Zeek event display row type.
- Does: summarized event record fields.
- Used in code: per-device Zeek activity views.
- Maintain: regenerate on Rust changes.

## Command Adapter Layer (`src-tauri/src/commands`)

### `src-tauri/src/commands/mod.rs`
- For: command module index and global shared app state definitions.
- Does: defines `AppState` domain slices (`capture`, `inventory`, `analysis`, `session`, `physical`, `segmentation`, `signatures`) and lock-order contract.
- Used in code: imported by web binary and all command modules; re-exports common models and parser details.
- Maintain: treat lock-order comment as authoritative; if adding state, document lock position and update all multi-lock call sites.

### `src-tauri/src/commands/resource_paths.rs`
- For: runtime resource path resolution for signatures and data files.
- Does: resolves `signatures_dir`, `oui_path`, `geoip_path` from app resource dir or environment/fallback probes.
- Used in code: `AppState::new` path input in web mode and legacy desktop mode.
- Maintain: keep env var names stable (`KK_SIGNATURES_DIR`, `KK_DATA_DIR`), avoid hard-coding only one run-layout.

### `src-tauri/src/commands/error.rs`
- For: structured command-level error envelope (`AppError`) migration target.
- Does: defines serializable machine-readable error variants and conversions.
- Used in code: currently mostly not wired (commands still return `String`), but intended for uniform API errors.
- Maintain: evolve additively; avoid breaking serialized `code` shape expected by frontend.

### `src-tauri/src/commands/protocol_handler.rs`
- For: packet processing core abstractions used by `PacketProcessor`.
- Does: owns `ProcessorCore` accumulators, shared packet pipeline logic, and `ProtocolHandler` trait for per-protocol modules.
- Used in code: called by `processor.rs`, handlers under `commands/handlers/*`, capture import/live flush.
- Maintain: preserve bounded memory behavior (caps on packet summaries and per-IP packet buffers), keep non-IP LLDP/redundancy handling first.

### `src-tauri/src/commands/processor.rs`
- For: high-level packet pipeline orchestrator.
- Does: wires core + protocol handlers, produces deep parse info, assets, connections, topology snapshots, pattern stats.
- Used in code: invoked by `capture.rs` and `kusanaginokajiki_web.rs` for both import and live capture.
- Maintain: register new protocol handlers here, keep enrichment precedence (signature/OUI/deep-parse/LLDP) consistent.

### `src-tauri/src/commands/capture.rs`
- For: import/live capture commands and lifecycle control.
- Does: imports PCAPs with progress + cancel support, starts/stops/pauses/resumes live capture, flushes batches, emits live attack alerts.
- Used in code: called via invoke compatibility and web route handlers.
- Maintain: keep background thread boundaries explicit; avoid holding state locks while heavy parsing runs.

### `src-tauri/src/commands/data.rs`
- For: read-side data query commands for topology/assets/connections/stats.
- Does: returns capped topology graph, paginated assets/connections, protocol stats, packet summaries, deep parse, timeline range.
- Used in code: frontend dashboards, tables, and detail views.
- Maintain: preserve pagination defaults/fields; topology cap logic must remain deterministic and safe for large graphs.

### `src-tauri/src/commands/analysis.rs`
- For: analysis command bridge to `gm-analysis`.
- Does: runs full analysis, stores findings/anomalies/assignments, serves derived analysis views (credentials, criticality, malware, CVE, compliance). Snapshot building is delegated to `analysis_input_builder`, `capture_context_builder`, and `application/mappers/deep_parse`.
- Used in code: `/v1/analysis/*` routes and invoke paths.
- Maintain: keep snapshot mapping complete when DTOs change; avoid direct domain mutation inside analysis crate.

### `src-tauri/src/commands/analysis_input_builder.rs`
- For: builds `AnalysisInput` from capture + inventory state.
- Does: calls `asset_snapshots`, `connection_snapshots`, and `build_deep_parse_snapshot_map` and assembles them into `AnalysisInput`.
- Used in code: `commands/analysis.rs::run_analysis` and related commands.
- Maintain: this is a thin orchestrator — logic stays in mapper modules.

### `src-tauri/src/commands/capture_context_builder.rs`
- For: builds `CaptureContext` from domain state for Phase 14C ATT&CK detections.
- Does: computes OT device IP sets, MAC/IP mappings, first/last-seen timestamps, write-rate counters, and read/write target sets.
- Used in code: `commands/analysis.rs::run_context_attacks`.
- Maintain: keep OT port lookup using `gm_types::OT_SERVER_PORTS`; do not duplicate port lists locally.

### `src-tauri/src/commands/patterns.rs`
- For: communication pattern and redundancy retrieval commands.
- Does: returns precomputed `ConnectionStats`, `PatternAnomaly`, and redundancy protocol observations.
- Used in code: analysis/pattern UI tabs.
- Maintain: treat as read-only over `analysis`/`capture` caches.

### `src-tauri/src/commands/ingest.rs`
- For: external-tool import command adapter.
- Does: imports Zeek/Suricata/Nmap/Masscan/Wazuh/SINEMA/TIA data via `gm-ingest`, merges into inventory/connections/alerts.
- Used in code: invoke compatibility and headless routes for ingest kinds.
- Maintain: merge behavior must be idempotent and non-destructive to passive discovery fields unless confidence improves.

### `src-tauri/src/commands/correlation.rs`
- For: alert-to-device enrichment commands.
- Does: correlates stored alerts with asset inventory metadata (hostname/device type/Purdue) and filtering by IP.
- Used in code: external alerts UI and analysis panels.
- Maintain: keep sort order stable (`severity`, then timestamp desc).

### `src-tauri/src/commands/baseline.rs`
- For: baseline drift comparison command.
- Does: diffs current in-memory state against a stored session for new/missing/changed assets and connections with drift score.
- Used in code: session comparison feature.
- Maintain: keep field-diff semantics explicit and backwards-compatible with stored DB row formats.

### `src-tauri/src/commands/physical.rs`
- For: physical topology command adapter.
- Does: imports network configs/tables (Cisco/Juniper/Aruba/generic), correlates ARP/MAC/CDP, and runs traffic-based inference.
- Used in code: physical topology workflows.
- Maintain: preserve parser auto-detect ordering and require switch existence checks when merging tables.

### `src-tauri/src/commands/projects.rs`
- For: project lifecycle commands.
- Does: CRUD for projects and active project selection used for session scoping.
- Used in code: `/v1/projects*` endpoints and project UI.
- Maintain: maintain `current_project_id` consistency when deleting/clearing.

### `src-tauri/src/commands/session.rs`
- For: session persistence and archive import/export commands.
- Does: save/load/list/delete sessions, update assets (single/bulk), zip archive export/import, DB row conversion.
- Used in code: session management UI and baseline workflows.
- Maintain: preserve metadata schema (`deep_parse_info`, `imported_files`), keep lock ordering capture->inventory->session in load/save flows.

### `src-tauri/src/commands/signatures.rs`
- For: runtime signature management commands.
- Does: list loaded signatures, reload from disk, test ad-hoc YAML signature against current connection-derived packet data.
- Used in code: signature diagnostics and tuning UI.
- Maintain: keep test behavior deterministic; do not regress read lock ordering assumptions.

### `src-tauri/src/commands/export.rs`
- For: export/report command adapter.
- Does: converts state to report DTOs and exports CSV/JSON/PDF/SBOM/STIX/images/allowlist/firewall rules/filtered PCAP.
- Used in code: export dialogs and reporting workflows.
- Maintain: keep output schemas stable; sanitize output path handling at adapter boundary.

### `src-tauri/src/commands/segmentation.rs`
- For: segmentation command bridge to `gm-segmentation`.
- Does: builds `SegmentationInput` from capture/inventory/analysis state, runs full segmentation pipeline, caches report, exports enforcement config.
- Used in code: segmentation analysis UI.
- Maintain: preserve lock order and input normalization for protocol names/roles.

### `src-tauri/src/commands/system.rs`
- For: system-level commands (interfaces/app metadata/settings/plugins).
- Does: lists interfaces, returns version, reads/writes user settings JSON, scans plugin manifests.
- Used in code: settings/system pages and startup checks.
- Maintain: keep settings file path stable (`~/.kusanaginokajiki/settings.json`), tolerate missing plugin dirs.

### `src-tauri/src/commands/wireshark.rs`
- For: Wireshark integration commands.
- Does: detects binary path, launches Wireshark with generated filters, returns/exports frame rows.
- Used in code: packet drill-down UX.
- Maintain: keep path detection cross-platform and avoid active capture actions.

## Protocol Handler Modules (`src-tauri/src/commands/handlers`)

### `src-tauri/src/commands/handlers/mod.rs`
- For: protocol handler registry module.
- Does: exposes handler submodules for packet processor registration.
- Used in code: imported by `processor.rs`.
- Maintain: add new handler modules here when extending deep parse coverage.

### `src-tauri/src/commands/handlers/modbus.rs`
- For: Modbus-specific packet accumulation.
- Does: tracks function codes, unit IDs, register ranges, device IDs, relationships, polling intervals.
- Used in code: `PacketProcessor::build_deep_parse_info` via handler finalize.
- Maintain: keep aggregation keys stable; avoid unbounded timestamp growth.

### `src-tauri/src/commands/handlers/dnp3.rs`
- For: DNP3-specific accumulation.
- Does: records function code stats, addresses, role combinations, unsolicited flags, peer relationships.
- Used in code: DNP3 deep parse output and analysis input.
- Maintain: preserve role resolution (`master/outstation/both`) semantics.

### `src-tauri/src/commands/handlers/enip.rs`
- For: EtherNet/IP-specific accumulation.
- Does: tracks scanner/adapter role, CIP writes to assembly, file access, ListIdentity requests.
- Used in code: ENIP detail/analysis.
- Maintain: keep CIP service/class checks aligned with parser enums.

### `src-tauri/src/commands/handlers/s7.rs`
- For: S7-specific accumulation.
- Does: tracks role and S7 functions seen.
- Used in code: S7 deep parse output and attack detections.
- Maintain: maintain canonical function name mapping strings.

### `src-tauri/src/commands/handlers/bacnet.rs`
- For: BACnet-specific accumulation.
- Does: tracks role and service/object combinations relevant to control operations.
- Used in code: BACnet detail output and ATT&CK checks.
- Maintain: keep service/object interpretation aligned with parser output.

### `src-tauri/src/commands/handlers/iec104.rs`
- For: IEC104-specific accumulation.
- Does: tracks role and command/reset/interrogation flags.
- Used in code: IEC104 detail output and ATT&CK checks.
- Maintain: retain simple, deterministic boolean feature extraction.

### `src-tauri/src/commands/handlers/profinet.rs`
- For: PROFINET DCP-specific accumulation.
- Does: tracks role and station/device name.
- Used in code: PROFINET device detail and segmentation role modeling.
- Maintain: keep unknown-role fallback behavior stable.

---

## Application Layer (`src-tauri/src/application`)

### `src-tauri/src/application/mappers/deep_parse.rs`

- For: canonical builder for `DeepParseSnapshot` maps fed into `gm-analysis`.
- Does: maps all 7 protocol accumulators (Modbus, DNP3, EtherNet/IP, S7, BACnet, IEC-104, PROFINET DCP) from `InventoryState` into `HashMap<String, DeepParseSnapshot>`.
- Used in code: `commands/analysis_input_builder.rs::build_analysis_input` and `commands/analysis.rs` malware path.
- Maintain: this is the **single source of truth** for DeepParse mapping — do not add partial copies elsewhere. Extend here when new protocols gain deep-parse fields.

### `src-tauri/src/application/use_cases/ingest/mod.rs`

- For: ingest use-case orchestrator.
- Does: `run_ingest()` — validates input, dispatches to `gm-ingest`, then merges results into `InventoryState` via `enrich_asset` / `create_asset_from_ingested` and rebuilds alert indexes.
- Used in code: `commands/ingest.rs` command adapter.
- Maintain: keep as thin orchestrator; logic lives in `asset_merge` and `alert_index` sub-modules. Merge behavior must be idempotent (improving confidence only).

### `src-tauri/src/application/use_cases/ingest/asset_merge.rs`

- For: per-asset enrichment and creation helpers for the ingest flow.
- Does: `enrich_asset()` — merges ingested fields (hostname, device type, vendor, Purdue level) into an existing `AssetInfo` using confidence rules. `create_asset_from_ingested()` — constructs a new `AssetInfo` from ingested data when no existing asset matches.
- Used in code: `ingest/mod.rs::run_ingest` only (`pub(super)`).
- Maintain: confidence rules must remain monotonically improving — never lower a higher-confidence existing value.

### `src-tauri/src/application/use_cases/ingest/alert_index.rs`

- For: alert storage conversion and Zeek event rebuilding.
- Does: `ingested_alert_to_stored()` — converts `gm-ingest` alert output to `StoredAlert`. `rebuild_zeek_device_events()` — rebuilds the per-device Zeek event index from all stored alerts. `classify_zeek_log_type()` — private helper mapping Zeek log names to event category strings.
- Used in code: `ingest/mod.rs::run_ingest` only (`pub(super)`).
- Maintain: `classify_zeek_log_type` string keys must stay aligned with Zeek log filenames produced by `gm-ingest`.

---

## Runtime Data Files

### `src-tauri/data/oui.tsv`
- For: OUI prefix to vendor lookup source for MAC enrichment.
- Does: maps `AA:BB:CC` prefixes to vendor names loaded by `gm-db::OuiLookup`.
- Used in code: asset enrichment in packet processor via inventory OUI lookup.
- Maintain: keep strict TSV format; comments/blank lines are supported.

## Signature Rule Files (`src-tauri/signatures`)

Global maintenance rules for all signature YAML files:
- Schema must match `gm-signatures::Signature` (`name`, `filters`, `confidence`, optional metadata).
- `confidence` must stay in the `1..=5` range or load will fail validation.
- Filters are AND-combined; adding a filter narrows matching and can remove previous hits.
- Prefer `tcp.*` / `udp.*` fields for transport-specific rules, and only use bare `dst_port` when protocol ambiguity is acceptable.
- Every new signature should be verified via `commands/signatures.rs` test path before release.

Global use in code:
- Loaded by `SignatureEngine::load_directory` at startup and via reload commands.
- Applied by packet processing/asset enrichment flows to set vendor/product/device-type/role hints.
- Exposed via signature list/reload/test APIs for diagnostics and tuning.

### `src-tauri/signatures/abb_mac.yaml`
- For: ABB vendor identification by IEEE OUI.
- Does: matches `mac.src_oui == 00:80:25` and sets ABB PLC-oriented metadata.
- Used in code: raises vendor confidence for ABB devices when protocol-only signals are weak.
- Maintain: keep OUI lowercase/colon-normalized to match engine OUI comparison behavior.

### `src-tauri/signatures/bacnet.yaml`
- For: high-confidence BACnet/IP fingerprint.
- Does: matches UDP/47808 plus BVLC start byte and NPDU version byte at fixed offsets.
- Used in code: confirms BACnet protocol identity for building-controller enrichment and protocol stats.
- Maintain: preserve byte offsets (`0` and `4`) unless parser/packet framing assumptions change.

### `src-tauri/signatures/bacnet_bvlc.yaml`
- For: BACnet BVLC header fingerprint.
- Does: matches UDP/47808 and payload byte `0x81` at offset `0`.
- Used in code: fast BACnet confirmation with fewer constraints than `bacnet.yaml`.
- Maintain: keep confidence below strict NPDU+BVLC rule if both remain present.

### `src-tauri/signatures/bacnet_generic.yaml`
- For: low-confidence generic BACnet detection.
- Does: matches only UDP destination port `47808`.
- Used in code: fallback classification when payload bytes are unavailable/truncated.
- Maintain: keep confidence low to avoid overriding stronger protocol/vendor evidence.

### `src-tauri/signatures/dnp3_generic.yaml`
- For: low-confidence generic DNP3 identification.
- Does: matches TCP destination port `20000` with default `role: slave`, `device_type: rtu`.
- Used in code: baseline DNP3 tagging before deeper byte-based confirmation.
- Maintain: treat role defaults as heuristic and avoid raising confidence without payload checks.

### `src-tauri/signatures/dnp3_start_bytes.yaml`
- For: high-confidence DNP3 frame fingerprint.
- Does: matches TCP/20000 plus DNP3 start bytes `0x05 0x64` at payload offset `0`.
- Used in code: confirms DNP3 traffic independent of deeper function-code parsing.
- Maintain: keep byte sequence exact; any broadened pattern increases false positives.

### `src-tauri/signatures/enip_cip.yaml`
- For: EtherNet/IP ListIdentity command detection.
- Does: matches TCP/44818 and encapsulation command bytes `0x63 0x00` at offset `0`.
- Used in code: identifies active CIP discovery/session behavior.
- Maintain: keep command bytes tied to ListIdentity semantics; do not repurpose this as a generic EIP rule.

### `src-tauri/signatures/ethernet_ip_generic.yaml`
- For: low-confidence generic EtherNet/IP identification.
- Does: matches TCP destination port `44818`.
- Used in code: fallback CIP classification when payload command bytes are absent.
- Maintain: keep confidence low and pair with stronger signatures for production-quality vendor inference.

### `src-tauri/signatures/ethernet_ip_register_session.yaml`
- For: high-confidence EtherNet/IP session setup detection.
- Does: matches TCP/44818 and RegisterSession bytes `0x00 0x65` at offset `0`.
- Used in code: identifies active CIP session establishment flows.
- Maintain: verify byte ordering against encapsulation format before editing.

### `src-tauri/signatures/ge_srtp_generic.yaml`
- For: generic GE SRTP protocol tagging.
- Does: matches TCP destination port `18245` and sets vendor hint to GE.
- Used in code: enriches GE PLC device inventory in utility/SCADA traffic.
- Maintain: keep confidence low unless adding SRTP payload markers.

### `src-tauri/signatures/hart_ip_generic.yaml`
- For: generic HART-IP protocol tagging.
- Does: matches TCP destination port `5094`.
- Used in code: marks process-instrument traffic as HART-IP when only port evidence exists.
- Maintain: preserve as low-confidence baseline unless handshake bytes are added.

### `src-tauri/signatures/iec104.yaml`
- For: high-confidence IEC-104 STARTDT-ACT handshake detection.
- Does: matches TCP/2404 plus full 6-byte U-frame `68 04 07 00 00 00` at offset `0`.
- Used in code: confirms IEC-104 telecontrol session startup in power-grid networks.
- Maintain: keep this exact sequence strict; use separate files for broader IEC-104 matching.

### `src-tauri/signatures/iec104_generic.yaml`
- For: low-confidence generic IEC-104 detection.
- Does: matches TCP destination port `2404`.
- Used in code: fallback IEC-104 classification for sparse captures.
- Maintain: keep confidence low to avoid overpowering strict IEC-104 byte signatures.

### `src-tauri/signatures/iec104_start_byte.yaml`
- For: high-confidence IEC-104 APCI start-byte fingerprint.
- Does: matches TCP/2404 and payload start byte `0x68` at offset `0`.
- Used in code: confirms IEC-104 APDU framing when full U-frame constants are not present.
- Maintain: keep layered with other IEC-104 rules; avoid making this rule the sole basis for vendor inference.

### `src-tauri/signatures/modbus_generic.yaml`
- For: low-confidence generic Modbus TCP tagging.
- Does: matches TCP destination port `502` with `role: slave` and `device_type: rtu`.
- Used in code: baseline Modbus classification before function-level details are parsed.
- Maintain: keep confidence `1` and ensure role/device defaults remain conservative.

### `src-tauri/signatures/modbus_server_response.yaml`
- For: Modbus server-response behavior detection.
- Does: matches source port `502` plus minimum payload length `7` (MBAP header threshold).
- Used in code: distinguishes active responders from clients scanning/talking to port 502.
- Maintain: do not lower `min_length` below MBAP minimum, or false positives rise.

### `src-tauri/signatures/mqtt_generic.yaml`
- For: low-confidence generic MQTT tagging.
- Does: matches TCP destination port `1883`.
- Used in code: identifies likely broker/client telemetry channels in IIoT networks.
- Maintain: keep confidence low unless adding CONNECT/PUBLISH payload-byte checks.

### `src-tauri/signatures/opc_ua_generic.yaml`
- For: low-confidence generic OPC UA tagging.
- Does: matches TCP destination port `4840`.
- Used in code: baseline OPC UA identification for endpoint/service inventory.
- Maintain: keep as fallback; rely on `opc_ua_hello.yaml` for stronger proof.

### `src-tauri/signatures/opc_ua_hello.yaml`
- For: high-confidence OPC UA binary-handshake fingerprint.
- Does: matches TCP/4840 and payload ASCII `HEL` (`0x48 0x45 0x4c`) at offset `0`.
- Used in code: confirms OPC UA endpoint handshake traffic.
- Maintain: preserve offset `0`; this rule is for binary transport header start only.

### `src-tauri/signatures/profinet_dcp.yaml`
- For: PROFINET DCP Identify request detection.
- Does: matches UDP/34964 and payload bytes `0x05 0x00` at offset `0`.
- Used in code: identifies active PROFINET discovery/configuration activity.
- Maintain: keep this scoped to DCP request semantics; do not broaden to generic PROFINET ports.

### `src-tauri/signatures/profinet_generic.yaml`
- For: low-confidence generic PROFINET tagging.
- Does: matches destination port `34962` for baseline protocol inference.
- Used in code: fallback PROFINET labeling when discovery/control payload markers are missing.
- Maintain: preserve low confidence and note it currently uses generic `dst_port` field.

### `src-tauri/signatures/rockwell_ethernet_ip.yaml`
- For: Rockwell vendor/product-family EtherNet/IP fingerprint.
- Does: matches TCP/44818 plus ASCII `Rockwell` bytes in CIP payload.
- Used in code: lifts vendor confidence to Rockwell Automation and product family hints.
- Maintain: keep byte pattern and encoding aligned with expected identity-response payloads.

### `src-tauri/signatures/rockwell_mac.yaml`
- For: Rockwell vendor identification by IEEE OUI.
- Does: matches `mac.src_oui == 00:00:bc` and tags PLC-oriented metadata.
- Used in code: enriches device inventory even when protocol payloads are limited.
- Maintain: keep OUI format normalized and avoid adding product-family claims without payload proof.

### `src-tauri/signatures/s7comm.yaml`
- For: Siemens S7 protocol-byte fingerprint.
- Does: matches TCP/102 and byte `0x32` at payload offset `7` (TPKT/COTP framed S7 PDU).
- Used in code: stronger S7comm confirmation than port-only detection.
- Maintain: keep offset tied to TPKT/COTP framing; update only with explicit parser evidence.

### `src-tauri/signatures/s7comm_cotp_connect.yaml`
- For: high-confidence Siemens ISO-TPKT connection fingerprint.
- Does: matches TCP/102 and TPKT header bytes `0x03 0x00` at payload offset `0`.
- Used in code: confirms ISO-transport wrapped S7 communication setup.
- Maintain: retain strict offset and combine with `s7comm.yaml` for stronger protocol confidence layering.

### `src-tauri/signatures/s7comm_generic.yaml`
- For: low-confidence generic S7comm tagging.
- Does: matches TCP destination port `102`.
- Used in code: fallback Siemens protocol classification when payload parsing is unavailable.
- Maintain: keep low confidence and avoid specific product claims.

### `src-tauri/signatures/schneider_mac.yaml`
- For: Schneider Electric vendor identification by IEEE OUI.
- Does: matches `mac.src_oui == 00:80:f4` and tags PLC device type.
- Used in code: vendor enrichment for Modicon-class endpoints before deeper payload signatures match.
- Maintain: keep OUI canonical and avoid over-scoping beyond vendor identity.

### `src-tauri/signatures/schneider_modbus.yaml`
- For: Schneider Modicon-specific Modbus payload fingerprint.
- Does: matches TCP/502 and ASCII `Schneider` bytes in payload, with vendor/product metadata.
- Used in code: promotes Schneider device family confidence above generic Modbus signals.
- Maintain: verify byte pattern assumptions against FC43/device-identification captures.

### `src-tauri/signatures/siemens_s7_mac.yaml`
- For: Siemens vendor identification by IEEE OUI.
- Does: matches `mac.src_oui == 00:0e:8c` and sets PLC device hints.
- Used in code: strengthens Siemens classification in mixed-protocol environments.
- Maintain: keep this vendor-level only; avoid assigning SCALANCE-specific family here.

### `src-tauri/signatures/siemens_scalance.yaml`
- For: Siemens SCALANCE infrastructure fingerprint.
- Does: combines Siemens OUI with PROFINET destination port evidence and sets `device_type: switch`.
- Used in code: distinguishes network infrastructure nodes from PLC endpoints.
- Maintain: keep dual-condition matching (MAC + port) to preserve specificity.

### `src-tauri/signatures/wonderware_suitelink.yaml`
- For: AVEVA/Wonderware SuiteLink server protocol tagging.
- Does: matches TCP destination port `5007` and sets server/scada metadata.
- Used in code: classifies HMI/historian side communications in OT app layers.
- Maintain: keep low confidence unless adding SuiteLink payload discriminators.

## Workspace Crates (`src-tauri/crates`)

## `gm-analysis`

### `src-tauri/crates/gm-analysis/Cargo.toml`
- For: analysis crate dependency/build manifest.
- Does: declares analysis-only dependencies and features.
- Used in code: compiled into command-layer analysis flows.
- Maintain: keep crate independent from runtime/transport-specific dependencies.

### `src-tauri/crates/gm-analysis/src/lib.rs`

- For: analysis domain API and shared analysis data types.
- Does: exports findings, anomalies, assignments, snapshots, and `run_full_analysis` orchestrator.
- Used in code: `commands/analysis.rs` builds snapshots and calls this crate.
- Maintain: treat as stable domain contract; add fields compatibly.

### `src-tauri/crates/gm-analysis/src/attack_codes.rs`

- For: single source of truth for all MITRE ATT&CK for ICS T-code string constants.
- Does: defines one `pub const T0XXX: &str = "T0XXX"` per technique used in the crate (~31 codes).
- Used in code: every `Finding` that sets `technique_id` must reference `crate::attack_codes::T0XXX`.
- Maintain: add a new constant here before adding a new detection rule. Never hard-code T-code strings in detection files.

### `src-tauri/crates/gm-analysis/src/helpers.rs`

- For: shared predicate helpers used across attack and context-attack modules.
- Does: `is_ot_device_type()` — returns `true` for device types classified as OT field equipment. `is_ot_protocol_name()` — returns `true` for OT protocol name strings.
- Used in code: `attack/`, `context_attacks/`, and `naming.rs`.
- Maintain: `is_ot_device_type` must stay in sync with `gm_types::DEVICE_TYPE_*` values and with how `AssetInfo.device_type` is populated. Do not inline device type checks elsewhere.

### `src-tauri/crates/gm-analysis/src/attack/` (module directory)

- For: ATT&CK-for-ICS rule detections.
- Does: sub-modules implement technique-specific detectors grouped by protocol or attack category. `mod.rs` orchestrates and returns all findings.
- Used in code: called by `run_full_analysis`.
- Maintain: every rule should emit concrete evidence and bounded logic. All T-code strings must use `crate::attack_codes::T0XXX`.

### `src-tauri/crates/gm-analysis/src/attack/test_utils.rs`

- For: shared test fixture factory for `gm-analysis` attack module tests (`#[cfg(test)]` only).
- Does: `make_input() -> AnalysisInput` — returns a default empty input, eliminating duplicate boilerplate.
- Used in code: all 8 attack sub-module test sections via `use crate::attack::test_utils::make_input`.
- Maintain: keep minimal; do not add production logic here.

### `src-tauri/crates/gm-analysis/src/context_attacks/` (module directory)

- For: context-rich ATT&CK detections requiring capture/session context.
- Does: sub-modules implement extended technique checks using per-device/temporal context maps.
- Used in code: `attack/mod.rs` delegates to `detect_context_attacks`.
- Maintain: keep context field names stable with `commands/capture_context_builder.rs`.

### `src-tauri/crates/gm-analysis/src/anomaly.rs`
- For: anomaly detection algorithms.
- Does: polling deviation, role reversal, unexpected public-IP anomaly detection.
- Used in code: `run_full_analysis` path.
- Maintain: threshold changes require test/evidence review to avoid noisy regressions.

### `src-tauri/crates/gm-analysis/src/comm_patterns.rs`
- For: packet timing statistics and pattern anomalies.
- Does: online accumulators (`Welford`) and anomaly labeling.
- Used in code: fed during packet processing, read by commands and segmentation.
- Maintain: preserve O(1) update complexity and deterministic sort behavior.

### `src-tauri/crates/gm-analysis/src/purdue.rs`
- For: Purdue model auto-assignment and cross-level policy checks.
- Does: assigns levels and flags suspicious cross-level communication.
- Used in code: `run_full_analysis`, plus outputs consumed by UI and segmentation.
- Maintain: preserve manual override semantics and level mapping consistency.

### `src-tauri/crates/gm-analysis/src/risk.rs`
- For: asset criticality scoring.
- Does: maps role/protocol/level hints to `CriticalityLevel`.
- Used in code: analysis endpoints and segmentation input enrichment.
- Maintain: keep precedence order explicit (safety/control/supervisory/etc).

### `src-tauri/crates/gm-analysis/src/naming.rs`
- For: hostname suggestion heuristics.
- Does: builds deterministic name suggestions from role/protocol/IP suffix.
- Used in code: analysis naming suggestions endpoint.
- Maintain: keep naming format stable for UX consistency.

### `src-tauri/crates/gm-analysis/src/default_creds.rs`
- For: default credential risk lookup.
- Does: loads embedded credential dataset and matches devices by vendor/product patterns.
- Used in code: credential warning analysis endpoint.
- Maintain: normalization and pattern matching should stay conservative.

### `src-tauri/crates/gm-analysis/src/compliance.rs`
- For: compliance mapping (IEC62443/NIST80082/NERCCIP).
- Does: evaluates requirement statuses from findings + topology/device context.
- Used in code: compliance endpoint in analysis commands.
- Maintain: keep `check_type` handlers synchronized with `data/compliance_mappings.json`.

### `src-tauri/crates/gm-analysis/src/cve_matcher.rs`
- For: OT infrastructure CVE matching.
- Does: matches discovered infra devices to curated CVE data.
- Used in code: CVE analysis endpoint.
- Maintain: update matching heuristics with product aliases and version handling carefully.

### `src-tauri/crates/gm-analysis/src/infrastructure.rs`
- For: infrastructure device-role classification.
- Does: classifies switches/routers/firewalls/APs etc from asset properties.
- Used in code: switch security and criticality/compliance context.
- Maintain: avoid over-classifying OT endpoints as infra.

### `src-tauri/crates/gm-analysis/src/malware_patterns.rs`
- For: behavior-based ICS malware signatures.
- Does: detects FrostyGoop/PIPEDREAM/Industroyer2 style traffic patterns.
- Used in code: malware findings endpoint.
- Maintain: tune for explainability and low false positives.

### `src-tauri/crates/gm-analysis/src/switch_security.rs`
- For: switch security posture findings.
- Does: evaluates flat network/default VLAN/redundancy/rogue switch/etc conditions.
- Used in code: switch-security findings endpoint.
- Maintain: keep remediation text actionable and tied to evidence.

### `src-tauri/crates/gm-analysis/src/allowlist.rs`
- For: communication allowlist generation and formatting.
- Does: classifies observed flows and renders allowlist/firewall rule exports.
- Used in code: `commands/export.rs` allowlist endpoints and segmentation references.
- Maintain: keep classification and direction logic stable.

### `src-tauri/crates/gm-analysis/src/error.rs`
- For: analysis error type definitions.
- Does: shared crate-specific error variants.
- Used in code: internal analysis flows.
- Maintain: keep conversion coverage and messages clear.

### `src-tauri/crates/gm-analysis/data/compliance_mappings.json`
- For: requirement-to-check mapping database.
- Does: drives framework-specific compliance row generation.
- Used in code: loaded in `compliance.rs` via `include_str!`.
- Maintain: preserve field names and supported framework IDs.

### `src-tauri/crates/gm-analysis/data/default_credentials.json`
- For: known-default credential catalog.
- Does: maps vendor/product patterns and protocols to default credential warnings.
- Used in code: loaded in `default_creds.rs`.
- Maintain: ensure entries are sourced and deduplicated.

### `src-tauri/crates/gm-analysis/data/ics_malware_patterns.json`
- For: malware pattern metadata.
- Does: names patterns, indicators, references used in malware findings.
- Used in code: loaded in `malware_patterns.rs`.
- Maintain: keep indicators aligned with implemented detectors.

### `src-tauri/crates/gm-analysis/data/ot_infra_cves.json`
- For: OT infra CVE dataset.
- Does: contains CVE metadata for infra product matching.
- Used in code: loaded in `cve_matcher.rs`.
- Maintain: keep versions/remediation text precise.

## `gm-capture`

### `src-tauri/crates/gm-capture/Cargo.toml`
- For: capture crate manifest.
- Does: declares pcap/packet parsing dependencies.
- Used in code: imported by command-layer capture and processor flows.
- Maintain: keep cross-platform capture compatibility in mind.

### `src-tauri/crates/gm-capture/src/lib.rs`
- For: capture crate API surface.
- Does: re-exports capture interfaces, models, reader/filter utilities.
- Used in code: command layer imports from this re-export hub.
- Maintain: keep public API stable for consumers.

### `src-tauri/crates/gm-capture/src/error.rs`
- For: capture error variants.
- Does: normalizes pcap/io/parse/cancel failures.
- Used in code: bubbled through capture commands.
- Maintain: keep variant semantics specific enough for UI mapping.

### `src-tauri/crates/gm-capture/src/interface.rs`
- For: network interface listing.
- Does: wraps libpcap device enumeration into serializable interface models.
- Used in code: `commands/system.rs::list_interfaces` and web `/system/interfaces`.
- Maintain: keep loopback detection and address formatting robust.

### `src-tauri/crates/gm-capture/src/live.rs`
- For: passive live capture engine.
- Does: runs capture thread, emits parsed packets, tracks stats, stores ring buffer for PCAP save.
- Used in code: `commands/capture.rs` and headless capture flow.
- Maintain: preserve stop/pause semantics and thread join safety.

### `src-tauri/crates/gm-capture/src/packet.rs`
- For: parsed packet domain type.
- Does: stores normalized L2-L4 fields and payload bytes.
- Used in code: parser, processor, handlers, signature matching input conversion.
- Maintain: field additions should remain backward-compatible where serialized.

### `src-tauri/crates/gm-capture/src/parsing.rs`
- For: raw Ethernet/IP/TCP/UDP parse helpers.
- Does: extracts packet info and special L2 packet types (LLDP/redundancy) from raw frames.
- Used in code: `pcap_reader.rs` and live capture path.
- Maintain: strict length/bounds checks before indexing.

### `src-tauri/crates/gm-capture/src/pcap_reader.rs`
- For: offline PCAP/PCAPNG reader.
- Does: read-all and streaming parse paths with progress reporting/cancel checks.
- Used in code: import commands and headless import route.
- Maintain: keep progress callback cadence and error tolerance predictable.

### `src-tauri/crates/gm-capture/src/pcap_filter.rs`
- For: filtered PCAP export utility.
- Does: reads source PCAPs and writes only packets matching IP/port filters.
- Used in code: `commands/export.rs::export_filtered_pcap`.
- Maintain: preserve OR-filter semantics and skip non-file origin tags (`[Suricata]`, etc).

## `gm-types`

### `src-tauri/crates/gm-types/Cargo.toml`
- For: shared types crate manifest (merged from former `gm-constants` + `gm-models`).
- Does: declares serde dependency; no runtime coupling.
- Used in code: imported by gm-analysis, command layer, processors, exports, bindings generation.
- Maintain: keep dependency footprint small; avoid app/runtime coupling.

### `src-tauri/crates/gm-types/src/lib.rs`
- For: shared OT/ICS constants, helpers, and canonical cross-crate DTOs — single source of truth for all backend modules.
- Does: defines `OT_SERVER_PORTS`, `CLEARTEXT_OT_PORTS`, `REMOTE_ACCESS_PORTS`, `MODBUS_WRITE_FCS`, `MAX_FINDINGS`, `MAX_ANOMALY_RESULTS`, `MAX_TOPOLOGY_NODES/EDGES`, `LIVE_CAPTURE_BATCH_SIZE`, `DEVICE_TYPE_*` string constants (15 types), port/protocol helper functions, and domain structs `AssetInfo`, `ConnectionInfo`, `PacketSummary`, `ProtocolStatInfo`, `AssetSignatureMatch`.
- Used in code: attack detection, server-role heuristics, protocol naming, analysis caps, ingest asset creation, state storage, command responses, TS generation.
- Maintain: add new constants here rather than inlining them in caller files; every `DEVICE_TYPE_*` string must stay in sync with the `AssetInfo.device_type` field values; evolve DTOs additively and regenerate TS.

## `gm-db`

### `src-tauri/crates/gm-db/Cargo.toml`
- For: DB crate manifest.
- Does: declares SQLite/serialization/geoip deps.
- Used in code: session/project/asset persistence.
- Maintain: keep rusqlite compatibility with schema/sql usage.

### `src-tauri/crates/gm-db/src/lib.rs`
- For: database façade API.
- Does: wraps `rusqlite::Connection` and exposes typed CRUD methods.
- Used in code: `commands/session.rs`, `commands/projects.rs`, baseline comparisons.
- Maintain: keep method behavior consistent; add functions instead of changing existing signatures when possible.

### `src-tauri/crates/gm-db/src/schema.rs`
- For: schema initialization SQL.
- Does: creates projects/sessions/assets/connections/history/findings tables and indexes.
- Used in code: `Database::open` initialization.
- Maintain: schema evolution should be additive/migration-safe.

### `src-tauri/crates/gm-db/src/sessions.rs`
- For: session table operations.
- Does: create/get/list/delete/update-counts for sessions.
- Used in code: session command save/load/list/delete paths.
- Maintain: preserve metadata column semantics and ordering.

### `src-tauri/crates/gm-db/src/projects.rs`
- For: project table operations and project-session relations.
- Does: CRUD projects and list sessions scoped to a project.
- Used in code: project commands and session scoping.
- Maintain: keep cascade/foreign-key expectations consistent.

### `src-tauri/crates/gm-db/src/assets.rs`
- For: asset table CRUD + change history.
- Does: insert/list/get/update field(s) and history reads.
- Used in code: session save/load and asset edit workflows.
- Maintain: JSON-in-text fields must stay valid serialized JSON arrays/objects.

### `src-tauri/crates/gm-db/src/connections.rs`
- For: connection table CRUD.
- Does: insert/list connection rows per session.
- Used in code: session persistence and baseline comparison.
- Maintain: keep row mapping and sort expectations stable.

### `src-tauri/crates/gm-db/src/oui.rs`
- For: MAC OUI vendor lookup implementation.
- Does: loads TSV file and resolves vendor from MAC prefix.
- Used in code: packet processor enrichment path.
- Maintain: preserve prefix normalization behavior.

### `src-tauri/crates/gm-db/src/geoip.rs`
- For: GeoIP country lookup + public-IP determination.
- Does: optionally loads MMDB and classifies private/special ranges.
- Used in code: asset geolocation/public-ip enrichment.
- Maintain: keep RFC range checks correct and complete.

### `src-tauri/crates/gm-db/src/error.rs`
- For: DB error type definitions.
- Does: wraps SQLite, serde, IO, lookup failures.
- Used in code: converted to string/app errors by command layer.
- Maintain: keep errors diagnostic but stable.

## `gm-ingest`

### `src-tauri/crates/gm-ingest/Cargo.toml`
- For: ingest crate manifest.
- Does: declares parser dependencies for JSON/XML/log ingestion.
- Used in code: imported by ingest command adapter.
- Maintain: keep parser dependency set minimal and secure.

### `src-tauri/crates/gm-ingest/src/lib.rs`
- For: ingest domain models and shared result types.
- Does: defines `IngestSource`, `IngestedAsset/Connection/Alert`, `IngestResult` and source metadata behavior.
- Used in code: `commands/ingest.rs` merge pipeline.
- Maintain: keep source tagging (`active vs passive`) accurate.

### `src-tauri/crates/gm-ingest/src/models.rs`
- For: TS-exported alert/Zeek view models.
- Does: defines `StoredAlert`, `ZeekEventSummary`, `DeviceZeekEvents`.
- Used in code: inventory alert storage and frontend bindings.
- Maintain: schema changes must regenerate `bindings/gen/types`.

### `src-tauri/crates/gm-ingest/src/zeek.rs`
- For: Zeek TSV parser.
- Does: parses conn/modbus/dnp3/s7comm style Zeek logs by header fields.
- Used in code: `import_zeek_logs` command.
- Maintain: preserve tolerant parsing for Zeek schema/version variance.

### `src-tauri/crates/gm-ingest/src/suricata.rs`
- For: Suricata eve.json parser.
- Does: extracts flow/alert/protocol events to ingested assets/connections/alerts.
- Used in code: `import_suricata_eve` command.
- Maintain: keep line-level error tolerance for large imperfect eve files.

### `src-tauri/crates/gm-ingest/src/wazuh.rs`
- For: Wazuh alert parser.
- Does: supports JSON array and line-delimited JSON formats.
- Used in code: `import_wazuh_alerts` command.
- Maintain: continue handling mixed field typing (`rule.id` string/number).

### `src-tauri/crates/gm-ingest/src/nmap.rs`
- For: Nmap XML import parser.
- Does: converts host/ports/service data to ingested assets.
- Used in code: `import_nmap_xml` command.
- Maintain: keep this import-only (never active scanning from app).

### `src-tauri/crates/gm-ingest/src/masscan.rs`
- For: Masscan JSON import parser.
- Does: parses and cleans Masscan JSON quirks, produces active-scan-tagged assets.
- Used in code: `import_masscan_json` command.
- Maintain: keep malformed JSON cleanup conservative and logged.

### `src-tauri/crates/gm-ingest/src/sinema.rs`
- For: SINEMA CSV and TIA XML import parsers.
- Does: imports inventory/config data from Siemens tooling exports.
- Used in code: `import_sinema_csv` and `import_tia_xml` commands.
- Maintain: keep header/column matching tolerant to exporter variants.

### `src-tauri/crates/gm-ingest/src/error.rs`
- For: ingest-specific error types.
- Does: wraps IO/JSON/XML/format/parse failure kinds.
- Used in code: command-layer error conversion.
- Maintain: keep granular error categories for actionable user feedback.

## `gm-parsers`

### `src-tauri/crates/gm-parsers/Cargo.toml`
- For: parser crate manifest.
- Does: declares protocol parsing dependencies.
- Used in code: called by packet processor for protocol identification/deep parse.
- Maintain: keep parser crate independent from state/transport concerns.

### `src-tauri/crates/gm-parsers/src/lib.rs`
- For: parser crate façade and exports.
- Does: re-exports protocol enums, parse functions, and deep parse models.
- Used in code: `commands/protocol_handler.rs`, handlers, and analysis snapshot build.
- Maintain: add new protocol modules + exports here as single integration point.

### `src-tauri/crates/gm-parsers/src/protocol.rs`
- For: protocol enum and identification helpers.
- Does: defines `IcsProtocol`, naming conversions, OT classification, port-based identification.
- Used in code: packet processor, topology builder, session load protocol reconstruction.
- Maintain: keep `from_name`/`to_name` compatible with persisted/serialized strings.

### `src-tauri/crates/gm-parsers/src/models.rs`
- For: deep parse aggregate data models.
- Does: defines per-protocol detail structs used by app state and TS bindings.
- Used in code: deep parse map storage and API responses.
- Maintain: coordinate with handlers and generated TS types.

### `src-tauri/crates/gm-parsers/src/modbus.rs`
- For: Modbus deep parser.
- Does: extracts roles, FCs, register ranges, exceptions, device IDs.
- Used in code: packet processor deep parse and Modbus handler accumulation.
- Maintain: preserve wire-level bounds checks and role heuristics.

### `src-tauri/crates/gm-parsers/src/dnp3.rs`
- For: DNP3 deep parser.
- Does: parses link/transport/application fields and role/function data.
- Used in code: DNP3 handler + analysis.
- Maintain: maintain start-byte checks and endian correctness.

### `src-tauri/crates/gm-parsers/src/enip.rs`
- For: EtherNet/IP + CIP parser.
- Does: parses ENIP encapsulation and CIP services/classes/identity hints.
- Used in code: ENIP handler and analysis logic.
- Maintain: keep command/service/class mappings aligned with ODVA specs.

### `src-tauri/crates/gm-parsers/src/s7comm.rs`
- For: Siemens S7 parser.
- Does: parses TPKT/COTP/S7 layers, role/function/rack-slot hints.
- Used in code: S7 handler and analysis.
- Maintain: offset math must remain exact; add tests for new function parsing.

### `src-tauri/crates/gm-parsers/src/bacnet.rs`
- For: BACnet/IP parser.
- Does: parses BVLCI/NPDU/APDU and extracts service/object/device info.
- Used in code: BACnet handler and analysis.
- Maintain: preserve APDU type/service decoding and object type handling.

### `src-tauri/crates/gm-parsers/src/iec104.rs`
- For: IEC60870-5-104 parser.
- Does: parses APCI/ASDU framing and command semantics.
- Used in code: IEC104 handler and ATT&CK detections.
- Maintain: maintain frame-type decoding and ASDU mappings.

### `src-tauri/crates/gm-parsers/src/profinet_dcp.rs`
- For: PROFINET DCP parser.
- Does: parses DCP service blocks and infers role/device naming hints.
- Used in code: PROFINET handler and segmentation profile building.
- Maintain: handle padding/alignment and block length validation carefully.

### `src-tauri/crates/gm-parsers/src/lldp.rs`
- For: LLDP parser.
- Does: parses TLVs to extract system identity/capabilities/vendor/model/firmware.
- Used in code: processor core LLDP enrichment.
- Maintain: robust TLV bounds checks and vendor heuristics.

### `src-tauri/crates/gm-parsers/src/snmp.rs`
- For: SNMP header/community parsing.
- Does: extracts SNMP version/community string and default-community risk flag.
- Used in code: SNMP enrichment and security findings context.
- Maintain: keep BER length parsing safe and conservative.

### `src-tauri/crates/gm-parsers/src/redundancy.rs`
- For: OT redundancy protocol detection/parsing.
- Does: detects/parses MRP/RSTP/HSR/PRP/DLR from L2 frames.
- Used in code: processor core and pattern/redundancy outputs.
- Maintain: preserve ethertype/destination MAC signatures and role inference.

### `src-tauri/crates/gm-parsers/src/vendor_tables.rs`
- For: static vendor ID lookup helpers.
- Does: maps CIP/PROFINET/BACnet vendor IDs to vendor/product names.
- Used in code: deep parsers for identity enrichment.
- Maintain: update tables from authoritative registries only.

## `gm-physical`

### `src-tauri/crates/gm-physical/Cargo.toml`
- For: physical topology crate manifest.
- Does: declares parser dependencies for vendor config text parsing.
- Used in code: physical topology commands.
- Maintain: keep regex usage manageable and tested.

### `src-tauri/crates/gm-physical/src/lib.rs`
- For: physical topology models and top-level exports.
- Does: defines switch/port/neighbor/ARP/MAC and inferred-topology structures.
- Used in code: `commands/physical.rs` state payloads.
- Maintain: keep model fields stable for frontend compatibility.

### `src-tauri/crates/gm-physical/src/cisco.rs`
- For: Cisco IOS text parser.
- Does: parses running config, MAC table, CDP neighbors, ARP table.
- Used in code: direct import commands and auto-detect path.
- Maintain: parser regexes should be resilient to IOS output variants.

### `src-tauri/crates/gm-physical/src/juniper.rs`
- For: JunOS text parser.
- Does: parses config/interfaces/MAC/LLDP/ARP style outputs.
- Used in code: auto-detected network-config import.
- Maintain: preserve `set ...` syntax handling and interface normalization.

### `src-tauri/crates/gm-physical/src/aruba.rs`
- For: HP/Aruba ProCurve parser.
- Does: parses running config, MAC tables, neighbors, ARP-like data.
- Used in code: auto-detected network-config import.
- Maintain: keep block parsing robust around indentation and VLAN blocks.

### `src-tauri/crates/gm-physical/src/generic.rs`
- For: vendor-neutral CSV/JSON import.
- Does: ingests flat device inventory formats into physical topology model.
- Used in code: fallback import paths and generic sources.
- Maintain: maintain tolerant field parsing without silently corrupting records.

### `src-tauri/crates/gm-physical/src/inference.rs`
- For: traffic-based topology inference.
- Does: infers subnets, gateways, switch candidates, broadcast domains from observed traffic snapshots.
- Used in code: `commands/physical.rs::run_topology_inference`.
- Maintain: keep heuristics explainable and deterministic.

### `src-tauri/crates/gm-physical/src/error.rs`
- For: physical parser/inference error types.
- Does: wraps IO/parse/vendor errors.
- Used in code: command-level error conversion.
- Maintain: keep failures explicit enough for import troubleshooting.

## `gm-report`

### `src-tauri/crates/gm-report/Cargo.toml`
- For: reporting/export crate manifest.
- Does: declares PDF/serialization/export support dependencies.
- Used in code: export command adapter.
- Maintain: keep optional-heavy dependencies pinned and tested.

### `src-tauri/crates/gm-report/src/lib.rs`
- For: report/export data contracts.
- Does: defines `ReportConfig`, `ReportData`, and export DTOs.
- Used in code: `commands/export.rs` conversion layer.
- Maintain: contract changes require frontend/export compatibility checks.

### `src-tauri/crates/gm-report/src/csv_export.rs`
- For: CSV rendering utilities.
- Does: serializes assets/connections into escaped CSV and file writes.
- Used in code: CSV export commands.
- Maintain: preserve RFC4180 escaping behavior.

### `src-tauri/crates/gm-report/src/json_export.rs`
- For: JSON export utilities.
- Does: emits topology/assets JSON bundles with metadata.
- Used in code: JSON export commands.
- Maintain: keep metadata schema stable.

### `src-tauri/crates/gm-report/src/pdf.rs`
- For: PDF assessment report generation.
- Does: builds multi-section PDF document from report config/data.
- Used in code: `generate_pdf_report` command.
- Maintain: keep font fallback paths and section toggles reliable.

### `src-tauri/crates/gm-report/src/sbom.rs`
- For: SBOM export generation.
- Does: maps assets to SBOM entries and renders CSV/JSON outputs.
- Used in code: `export_sbom` command.
- Maintain: keep Purdue mapping and field naming consistent.

### `src-tauri/crates/gm-report/src/stix.rs`
- For: STIX 2.1 bundle generation.
- Does: emits STIX objects/relationships from assets/connections/findings.
- Used in code: `export_stix_bundle` command.
- Maintain: preserve STIX object/reference validity and deterministic IDs.

### `src-tauri/crates/gm-report/src/error.rs`
- For: report crate error variants.
- Does: wraps IO/PDF/JSON/no-data conditions.
- Used in code: export command error conversion.
- Maintain: keep error messages user-actionable.

## `gm-segmentation`

### `src-tauri/crates/gm-segmentation/Cargo.toml`
- For: segmentation crate manifest.
- Does: declares segmentation pipeline dependencies.
- Used in code: segmentation command bridge.
- Maintain: keep crate state/runtime independent.

### `src-tauri/crates/gm-segmentation/src/lib.rs`
- For: segmentation domain contracts + phase orchestration.
- Does: defines all segmentation models and runs 15A-15E pipeline.
- Used in code: `commands/segmentation.rs`.
- Maintain: preserve additive evolution of report schema.

### `src-tauri/crates/gm-segmentation/src/identity_groups.rs`
- For: phase 15A identity grouping.
- Does: clusters assets by Purdue/role/vendor/community into policy groups.
- Used in code: segmentation orchestrator.
- Maintain: deterministic output ordering and threshold reasoning.

### `src-tauri/crates/gm-segmentation/src/zones.rs`
- For: phase 15B zone/conduit recommendation.
- Does: builds zone model, DMZ recommendations, conduit rules and scoring.
- Used in code: segmentation orchestrator.
- Maintain: keep zone-ID mapping and recommendation criteria explicit.

### `src-tauri/crates/gm-segmentation/src/matrix.rs`
- For: phase 15C communication matrix generation.
- Does: aggregates inter-zone observed flows into least-privilege policy rules.
- Used in code: segmentation orchestrator and enforcement export.
- Maintain: preserve grouping keys and coverage calculation semantics.

### `src-tauri/crates/gm-segmentation/src/enforcement.rs`
- For: phase 15D enforcement config export.
- Does: renders matrix to Cisco IOS/ASA, generic table, Suricata, JSON policy formats.
- Used in code: segmentation export command.
- Maintain: keep output syntax valid per target format and ACL name constraints.

### `src-tauri/crates/gm-segmentation/src/simulation.rs`
- For: phase 15E policy simulation.
- Does: replays observed traffic against proposed policy and scores impact.
- Used in code: segmentation orchestrator results.
- Maintain: preserve allow-match semantics and false-positive criteria.

### `src-tauri/crates/gm-segmentation/src/error.rs`
- For: segmentation-specific errors.
- Does: defines empty-input/config/serialization error cases.
- Used in code: segmentation phase wrappers.
- Maintain: keep errors narrow and meaningful.

## `gm-signatures`

### `src-tauri/crates/gm-signatures/Cargo.toml`
- For: signature engine crate manifest.
- Does: declares YAML/parsing dependencies.
- Used in code: loaded into AppState signature subsystem.
- Maintain: keep parser stack secure and deterministic.

### `src-tauri/crates/gm-signatures/src/lib.rs`
- For: signature engine public API surface.
- Does: re-exports signature model, engine, test result, and errors.
- Used in code: command layer for load/reload/test and processor for matching.
- Maintain: maintain stable exported types.

### `src-tauri/crates/gm-signatures/src/signature.rs`
- For: YAML signature schema definitions.
- Does: defines signature/filter/extractor/match structures.
- Used in code: engine parsing and command test endpoints.
- Maintain: keep schema backward-compatible for existing signature files.

### `src-tauri/crates/gm-signatures/src/engine.rs`
- For: signature loading/compilation/matching runtime.
- Does: loads YAML signatures, compiles filters, matches packet data per device, reloads directory.
- Used in code: AppState initialization and packet processor asset enrichment.
- Maintain: preserve match ordering/confidence behavior and reload safety.

### `src-tauri/crates/gm-signatures/src/error.rs`
- For: signature engine errors.
- Does: parse/validation/io error variants.
- Used in code: command-layer reload/test failures.
- Maintain: keep parse vs validation error distinction.

## `gm-topology`

### `src-tauri/crates/gm-topology/Cargo.toml`
- For: topology crate manifest.
- Does: defines minimal dependencies for graph modeling.
- Used in code: processor builds topology snapshots.
- Maintain: keep lightweight.

### `src-tauri/crates/gm-topology/src/lib.rs`
- For: topology graph data structures and builder.
- Does: builds node/edge graph from connections, tracks protocols/counts/subnets.
- Used in code: capture import/live flush and data/topology endpoints.
- Maintain: keep node/edge IDs and bidirectional detection logic stable.

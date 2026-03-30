# DEVELOPER GUIDE — Kusanagi Kajiki

This guide is code-driven and intended for maintainers.

Conventions used below:
- **Observed:** directly confirmed in repository files.
- **Inferred:** likely true from code shape, but not explicitly guaranteed.

---

## 1) Project Identity

### What the project does
**Observed:** Kusanagi Kajiki is a desktop OT/ICS analysis tool that imports PCAPs, supports live capture, builds topology, performs protocol/deep-parse analysis, runs security analytics, and exports reports via a Tauri + Svelte application. (`src-tauri/src/main.rs`, `src-tauri/src/commands/*.rs`, `src/lib/components/*.svelte`)

### Why it exists
**Observed:** README frames the problem as passive ICS/SCADA discovery for environments where active scanning is risky. (`README.md`)

### Intended users / operator persona
**Observed:** command and UI surface target assessors/operators who need:
- capture import/live capture,
- asset inventory + topology,
- ATT&CK/compliance findings,
- reporting/export,
- project/session management.

Evidence: command domains and component names (`capture`, `analysis`, `export`, `projects`, `session`, `physical`, `segmentation`).

### Comparison to GRASSMARLIN
**Observed:** README explicitly states this is a rewrite/successor to NSA GRASSMARLIN and lists a capability comparison table. Use README as the canonical statement; do not repeat claims not backed by code. (`README.md`)

---

## 2) Tech Stack

## Runtime and framework
- **Backend:** Rust 2021 + Tauri 2 app crate (`src-tauri/Cargo.toml`, `src-tauri/src/main.rs`).
- **Frontend:** SvelteKit + TypeScript (`src/routes`, `src/lib`, `package.json`).

## Workspace layout
**Observed:** `src-tauri/Cargo.toml` workspace includes 10 crates:
`gm-capture`, `gm-parsers`, `gm-signatures`, `gm-topology`, `gm-db`, `gm-physical`, `gm-ingest`, `gm-report`, `gm-analysis`, `gm-segmentation`.

## Key dependencies by responsibility (Observed)

| Responsibility | Evidence |
|---|---|
| Desktop shell + IPC | `tauri`, `tauri-plugin-dialog`, `tauri-plugin-shell` in `src-tauri/Cargo.toml` |
| Packet ingest/parsing | `pcap`, `etherparse` in `gm-capture/Cargo.toml` |
| Parsing + protocol model | `gm-parsers` depends on `gm-capture` |
| Fingerprinting | `serde_yaml`, `regex` in `gm-signatures/Cargo.toml` |
| Graph model | `petgraph` in `gm-topology/Cargo.toml` |
| Persistence | `rusqlite (bundled)` + `maxminddb` in `gm-db/Cargo.toml` |
| Reporting | `genpdf` in `gm-report/Cargo.toml` |
| Ingest importers | `quick-xml` in `gm-ingest/Cargo.toml` |
| Shared serialization | `serde`, `serde_json` across crates |
| Errors + logging | `thiserror`, `log`, `env_logger` |

---

## 3) Repository and Directory Map

## Top-level map (major boundaries)

```text
.
├── README.md                       # Product/docs entrypoint
├── DEVELOPER_GUIDE.md              # This file
├── package.json                    # Frontend scripts + deps
├── src/
│   ├── routes/                     # SvelteKit route shell
│   └── lib/
│       ├── components/             # UI views and feature panels
│       ├── stores/index.ts         # Global frontend state model
│       ├── types/index.ts          # TS IPC contract types
│       └── utils/tauri.ts          # Typed invoke()/event wrappers
└── src-tauri/
    ├── Cargo.toml                  # Tauri app + workspace manifest
    ├── src/main.rs                 # App bootstrap + command registration
    ├── src/commands/               # Command layer + AppState + pipeline orchestration
    ├── crates/                     # 10 backend domain crates
    ├── signatures/                 # YAML fingerprint files loaded at runtime
    └── data/oui.tsv                # OUI lookup data
```

## Command layer ownership (`src-tauri/src/commands`)
- `capture.rs`: PCAP import + live capture + progress events + cancel.
- `processor.rs`: shared packet pipeline used by import and live capture paths.
- `data.rs`: topology/assets/connections/deep parse retrieval.
- `analysis.rs`: transforms `AppState` -> `AnalysisInput` and invokes `gm-analysis`.
- `export.rs`: export/report/allowlist endpoints.
- `session.rs`: save/load/list/delete sessions and asset updates.
- `projects.rs`: project CRUD and active project selection.
- `physical.rs`: network-config and table imports for physical topology.
- `ingest.rs`: Zeek/Suricata/Nmap/Masscan/Wazuh/SINEMA/TIA ingestion.
- `segmentation.rs`: `AppState` -> `SegmentationInput` and segmentation execution.
- `wireshark.rs`: external Wireshark process integration.
- `correlation.rs`: enrich imported alerts with inventory context.
- `baseline.rs`: baseline drift comparison logic.
- `signatures.rs`, `system.rs`, `patterns.rs`: signature mgmt/system/pattern data endpoints.

## Frontend components (Observed file inventory)
`AnalysisView`, `BaselineDriftView`, `CaptureView`, `CommunicationPatterns`, `ConnectionTree`, `ExportView`, `FilteredView`, `InventoryView`, `LogicalView`, `MeshView`, `PhysicalView`, `ProjectsView`, `ProtocolStats`, `PurdueOverlay`, `SegmentationView`, `SettingsView`, `SignatureEditor`, `TimelineScrubber`, `TopologyView`, `WatchTab`.

---

## 4) End-to-End Data Pipeline

### Flow A: PCAP import -> UI

1. **Command entry:** `import_pcap(paths, state, app_handle)` in `capture.rs`.
2. **Blocking ingestion loop:** runs `PcapReader::stream_file(...)` in `spawn_blocking` and emits `import_progress` events.
3. **Per-packet processing:** closure calls `PacketProcessor::process_packet(packet)`.
4. **Deep parse materialization:** after ingest, `build_deep_parse_info()`.
5. **Asset build + fingerprinting:** `build_assets(signature_engine, oui_lookup, geoip_lookup)`.
6. **Topology snapshot:** `processor.topo_builder.snapshot()` then signature enrichment of node vendor/device_type.
7. **Pattern outputs:** `build_pattern_results()` and `build_redundancy_info()`.
8. **State commit:** lock `state.inner` once and write topology/assets/connections/summaries/deep_parse/stats.
9. **Read side:** frontend calls `get_topology/get_assets/get_connections/...` via `tauri.ts` wrappers.

### Flow B: live capture
**Observed:** same processor path is reused; live capture commands maintain `live_capture` and `processing_thread` in `AppStateInner`, emit packet/capture stats events, and stop/pause/resume via command endpoints.

### Flow C: export/report
1. Frontend invokes export command (`generate_pdf_report`, `export_assets_csv`, etc.).
2. `export.rs` converts `AppState` structs to `gm-report` data structures.
3. `gm-report` module-specific exporters produce files/strings returned by command.

### Crate handoff summary
`gm-capture` -> `commands/processor` (+ `gm-parsers`) -> `gm-signatures` + `gm-topology` + `gm-db` lookups -> persisted/queried via command layer -> `gm-analysis` / `gm-segmentation` / `gm-report` on demand -> TS wrappers -> stores/components.

---

## 5) Backend Crate Deep Dive

## `gm-capture`
- **Responsibility (Observed):** read PCAP/live traffic and normalize as `ParsedPacket`.
- **Key types:** `ParsedPacket`, `TransportProtocol`, `PcapReader`, `LiveCaptureHandle`, `CaptureStats`.
- **Public entry points:** `PcapReader::read_file/stream_file`, interface listing, live capture handles.
- **Input/Output:** bytes/interface traffic -> parsed packet records.
- **Dependencies:** `pcap`, `etherparse`, `chrono`, `serde`.
- **Extension points:** additional parsing metadata in `packet.rs` and `parsing.rs`.
- **Failure areas:** malformed frames, interface privilege issues, blocking I/O behavior.

## `gm-parsers`
- **Responsibility:** protocol identification and deep parsing dispatch.
- **Key types:** `IcsProtocol`, per-protocol `*Info` structs, `DeepParseResult`.
- **Entry points:** `identify_protocol`, `deep_parse`, parser functions (`parse_modbus`, `parse_dnp3`, ...).
- **Input/Output:** `ParsedPacket` + payload bytes -> protocol enum + protocol-specific semantic fields.
- **Dependencies:** depends on `gm-capture`.
- **Extension points:** add enum variant + port mapping + parser module + dispatcher export.
- **Failure areas:** misclassification by ports, payload length guards, role inference drift.

## `gm-signatures`
- **Responsibility:** YAML signature load/compile/match.
- **Key types:** `SignatureEngine`, `PacketData`, `SignatureMatch`, `SignatureFilter`.
- **Entry points:** `load_directory`, `reload`, `match_packet`, `match_device_packets`, `test_signature`.
- **Input/Output:** packet slices + compiled signatures -> confidence-scored matches.
- **Dependencies:** `serde_yaml`, `regex`, `gm-parsers`.
- **Extension points:** new filter types + extractor behavior.
- **Failure areas:** invalid YAML schema, low-specificity signatures producing noisy classifications.

## `gm-topology`
- **Responsibility:** build serializable topology graph.
- **Key types:** `TopologyBuilder`, `TopologyGraph`, `TopoNode`, `TopoEdge`.
- **Entry points:** `add_connection`, `snapshot`, `build`.
- **Input/Output:** directional packet/flow observations -> nodes/edges with counts.
- **Dependencies:** `gm-parsers::IcsProtocol`, `petgraph` dep present (currently builder uses HashMaps).
- **Extension points:** richer graph analytics or subnet/community strategies.
- **Failure areas:** graph size blow-up; command-side caps mitigate UI load (`data.rs`).

## `gm-db`
- **Responsibility:** SQLite persistence + OUI + GeoIP lookup.
- **Key types:** `Database`, `SessionRow`, `AssetRow`, `ConnectionRow`, `Project*`, `OuiLookup`, `GeoIpLookup`.
- **Entry points:** session/asset/connection/project CRUD methods on `Database`.
- **Input/Output:** App snapshot rows -> DB records and query results.
- **Dependencies:** `rusqlite`, `maxminddb`, `serde_json`.
- **Extension points:** schema migrations in `schema.rs`, new repository methods.
- **Failure areas:** schema drift, JSON metadata shape changes.

## `gm-analysis`
- **Responsibility:** ATT&CK detections + Purdue/anomaly/risk/CVE/malware/compliance/etc.
- **Key types:** `AnalysisInput`, `Finding`, `PurdueAssignment`, `AnomalyScore`, snapshot structs.
- **Entry points:** detection and assessment exports in `lib.rs` (`detect_attack_techniques`, `assign_purdue_levels`, etc.).
- **Input/Output:** immutable snapshot from command layer -> findings/assignments/scores.
- **Dependencies:** mostly serde/chrono/uuid; no direct Tauri coupling.
- **Extension points:** `attack.rs`/`context_attacks.rs`, matcher datasets under `data/`.
- **Failure areas:** false positives from heuristic thresholds; snapshot mapper mismatches in `commands/analysis.rs`.

## `gm-physical`
- **Responsibility:** parse vendor network config/table outputs and infer physical links.
- **Key types:** `PhysicalTopology`, `InferredTopology` and parser result structs.
- **Entry points:** parser functions per vendor module (`cisco`, `juniper`, `aruba`, `generic`) and inference module.
- **Input/Output:** text/CSV/JSON config artifacts -> switch/port/link model.
- **Dependencies:** `regex`, `serde`.
- **Extension points:** vendor-specific grammar support.
- **Failure areas:** CLI output format variance across device OS versions.

## `gm-ingest`
- **Responsibility:** ingest external tool outputs into shared asset/connection/alert model.
- **Key types:** `IngestResult`, `IngestedAsset`, `IngestedConnection`, `IngestedAlert`, `IngestSource`.
- **Entry points:** parser modules `zeek`, `suricata`, `nmap`, `masscan`, `wazuh`, `sinema`.
- **Input/Output:** external logs/results -> normalized ingest bundle.
- **Dependencies:** `gm-capture`, `gm-parsers`, `quick-xml`, `serde_json`.
- **Extension points:** new source parser module + command wiring.
- **Failure areas:** schema/version drift in upstream tools.

## `gm-report`
- **Responsibility:** render/export report artifacts.
- **Key types:** `ReportData`, `ReportConfig`, `ExportAsset`, `ExportConnection`, `ExportFinding`.
- **Entry points:** module-specific exporters (`pdf`, `csv_export`, `json_export`, `sbom`, `stix`).
- **Input/Output:** structured export model -> serialized report content/files.
- **Dependencies:** `genpdf`, `serde`.
- **Extension points:** new format modules and config fields.
- **Failure areas:** large dataset rendering cost, format-specific escaping/encoding issues.

## `gm-segmentation`
- **Responsibility:** build microsegmentation recommendations from observed state.
- **Key types:** `SegmentationInput`, `AssetProfile`, `ObservedConnection`, `SegmentationReport`.
- **Entry points:** `run_segmentation_analysis` orchestrating phases 15A–15E.
- **Input/Output:** enriched assets/connections/findings -> zones/matrix/enforcement/simulation report.
- **Dependencies:** serde/uuid/chrono only (no Tauri coupling).
- **Extension points:** phase modules (`identity_groups`, `zones`, `matrix`, `enforcement`, `simulation`).
- **Failure areas:** role/classification quality directly affects policy quality.

---

## 6) AppState and Backend Runtime Model

## AppState layout (Observed)
`src-tauri/src/commands/mod.rs` defines:
- `AppState { inner: Mutex<AppStateInner>, import_cancelled: Arc<AtomicBool> }`
- `AppStateInner` includes topology, assets, connections, packet summaries, imported files, signature engine, deep parse cache, live capture handles/thread, OUI/GeoIP lookup, optional DB/session/project context, physical and inferred topology, analysis outputs, pattern outputs, redundancy frames, imported alerts, Zeek per-device summaries, and cached segmentation report.

## Synchronization model
- **Observed:** Commands take `State<'_, AppState>` and lock `inner` to read/write shared state.
- **Observed:** cancellation flag is outside mutex for low-contention import cancel checks.
- **Observed:** import pipeline does expensive file IO + packet processing off async executor via `spawn_blocking`.

## Long-running coordination
- Import: `cancel_import` sets atomic bool; ingest loop checks it.
- Live capture: `live_capture` + `processing_thread` are stored in state and coordinated by start/stop/pause/resume commands.

**Maintenance note:** keep lock scopes narrow when adding heavy analysis to commands; prefer local snapshots then write back.

---

## 7) IPC Contract

## Registered command surface
**Observed:** `src-tauri/src/main.rs` `generate_handler!` registers **96** commands grouped by domain modules (`system`, `capture`, `data`, `analysis`, etc.).

## Domain summary (Observed)
- `system`: interface enumeration, app info, settings, plugin listing.
- `capture`: PCAP import/cancel + live capture lifecycle.
- `data`: topology/assets/connections/deep parse/protocol stats/timeline data queries.
- `signatures`: list/reload/test signature workflows.
- `session`/`projects`/`baseline`: persistence and organization features.
- `physical`/`ingest`/`correlation`/`wireshark`: integration paths.
- `analysis`/`patterns`/`segmentation`/`export`: analytics and output workflows.

## Rust <-> TypeScript mapping
- Rust command payload/return structs are `serde` serialized.
- Frontend defines corresponding TS interfaces in `src/lib/types/index.ts`.
- Wrapper functions in `src/lib/utils/tauri.ts` call `invoke('<command_name>', args)` and type the response.

## Coupling and brittle areas
- **Observed:** `tauri.ts` has wrappers for almost all command names; one registered command appears without a frontend wrapper: `get_function_code_stats`.
- **Observed:** field naming uses snake_case across Rust and TS; drift risk appears whenever Rust structs change but TS mirrors are not updated.
- **Inferred:** because there is no generated schema, contract verification is mostly compile-time plus runtime smoke checks.

---

## 8) Frontend Architecture

## Stores (`src/lib/stores/index.ts`)
Primary mirrors of backend state include:
- capture (`captureStatus`, `captureStats`)
- network data (`assets`, `connections`, `topology`, `protocolStats`, `assetCount`, `connectionCount`)
- session/project (`sessions`, `currentSession`, `activeProject`)
- analysis (`findings`, `purdueAssignments`, `anomalies`, `analysisSummary`)
- physical/topology submodes (`physicalTopology`, topology tab state)
- baseline/segmentation (`baselineDiff`, `segmentationReport`)
- UI filters/timeline/theme state.

## Component map (Observed)
- Topology: `TopologyView`, `LogicalView`, `MeshView`, `FilteredView`, `WatchTab`, `PurdueOverlay`, `TimelineScrubber`.
- Data ops: `CaptureView`, `InventoryView`, `ConnectionTree`, `ProtocolStats`.
- Security/analytics: `AnalysisView`, `CommunicationPatterns`, `SegmentationView`, `BaselineDriftView`.
- Ops/reporting: `ProjectsView`, `ExportView`, `SignatureEditor`, `PhysicalView`, `SettingsView`.

## IPC wrapper layer (`src/lib/utils/tauri.ts`)
- Single invoke/listen boundary; components call wrapper functions instead of direct IPC.
- Provides typed event subscriptions (`onPacketEvent`, `onCaptureStats`, `onImportProgress`).

## Drift risks
- Rust command rename without updating wrapper string literal.
- Rust struct field change without TS type mirror update.
- Added command not exposed in wrappers/stores (already visible for `get_function_code_stats`).

---

## 9) Change Recipes

## A) Add a new protocol parser
1. **Inspect first:** `gm-parsers/src/lib.rs`, existing parser template (`modbus.rs`), `commands/processor.rs`.
2. Add parser module file and export it from `gm-parsers/lib.rs`.
3. Extend `IcsProtocol` and detection path (`identify_protocol` and deep parse dispatch).
4. In `commands/processor.rs`, fold parsed output into device-level accumulators and `DeepParseInfo`.
5. Extend Rust `DeepParseInfo`/detail structs (`commands/mod.rs`) and TS mirrors (`src/lib/types/index.ts`).
6. Add or update data command exposure and frontend rendering.
7. Add parser tests near crate module + end-to-end smoke via import command.

**Pitfalls:** forgetting TS enum mirror, forgetting processor aggregation, returning parser details not serializable.

## B) Add a new YAML signature
1. **Inspect first:** `gm-signatures/src/signature.rs`, `engine.rs`, existing YAML files in `src-tauri/signatures/`.
2. Add YAML file with valid schema and confidence 1–5.
3. Reload via `reload_signatures` command; verify with `test_signature`.
4. Confirm match appears in asset `signature_matches` and vendor/device_type mapping logic.

**Pitfalls:** overly broad filters; invalid hex payload filters; confidence misuse.

## C) Add a new ATT&CK detection rule
1. **Inspect first:** `gm-analysis/src/attack.rs`, `context_attacks.rs`, `commands/analysis.rs` snapshot builder.
2. Add detection function returning `Vec<Finding>` with concrete evidence and optional technique ID.
3. Register function in `detect_attack_techniques` orchestration.
4. If rule needs additional fields, update snapshot structs and mapper in `commands/analysis.rs`.
5. Add tests using synthetic `AnalysisInput`.

**Pitfalls:** depending on data not populated in mapper; high false positive thresholds.

## D) Add a new frontend view + Tauri command
1. **Inspect first:** related command module, `src-tauri/src/main.rs`, `src/lib/utils/tauri.ts`, `src/lib/types/index.ts`, `src/lib/stores/index.ts`, and similar component.
2. Implement `#[tauri::command]` function in command module.
3. Register command in `generate_handler!`.
4. Add TS request/response types.
5. Add typed wrapper in `tauri.ts` and store wiring if persistent state is needed.
6. Add Svelte component and integrate tab/routing composition.
7. Run backend + frontend checks.

**Pitfalls:** command added but not registered; wrapper added with wrong command string; store type mismatch.

---

## 10) Testing and Validation

## How to run (Observed from repo docs/scripts)
- Backend tests: `cd src-tauri && cargo test --all`
- Frontend check: `npm run check`
- Full local dev smoke: `npm run tauri dev`

## Test organization patterns (Observed)
- Crate unit tests in module files (`#[cfg(test)]` blocks) for parsers/builders/report structs/db behavior.
- In-memory DB tests (`Database::open_in_memory`) for persistence flows.

## CI/CD expectations
**Inferred:** no CI workflow file was inspected in this pass, so treat local command checks as required pre-commit baseline.

## Pre-commit validation checklist (maintainer practice)
1. Run Rust tests for touched crates (or all when changing shared types).
2. Run `npm run check` when touching TS stores/types/components.
3. Manually verify IPC commands touched are:
   - registered in `main.rs`,
   - wrapped in `tauri.ts`,
   - typed in `types/index.ts`.
4. For parser/signature/analysis changes, test against representative PCAP/log inputs.

---

## 11) Dev Environment and Debugging

## Build/run
- `npm install`
- `npm run tauri dev`

## Logging
- Set `RUST_LOG=info` or `RUST_LOG=debug` before `npm run tauri dev` to inspect backend flow.

## Useful debug entry points
- Import path: `commands/capture.rs::import_pcap`.
- Processing path: `commands/processor.rs::process_packet` and builders.
- Analysis mapping: `commands/analysis.rs::build_analysis_input`.
- Contract layer: `src/lib/utils/tauri.ts` + `src/lib/types/index.ts`.

## Common setup issues
**Observed from project docs:** platform dependencies and Npcap/toolchain notes are documented in `README.md`; use that as the environment source of truth.

---

## Required-first file audit (completed)
The following files were inspected and used as primary evidence while drafting this guide:
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/commands/processor.rs`
- `src-tauri/crates/gm-parsers/src/modbus.rs`
- `src-tauri/crates/gm-analysis/src/attack.rs`
- `src-tauri/crates/gm-signatures/src/engine.rs`
- `src/lib/types/index.ts`
- `src/lib/stores/index.ts`
- `src-tauri/Cargo.toml`

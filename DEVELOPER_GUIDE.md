# Kusanagi Kajiki Developer Guide

> Audience: engineers onboarding to the repository who need a full-stack mental model (Rust crates, Tauri command layer, Svelte frontend, and operational workflows).

---

## 1) Project Identity

### What Kusanagi Kajiki is
Kusanagi Kajiki is a passive OT/ICS network discovery and analysis platform implemented as:
- **Rust + Tauri 2 backend** for packet processing, analysis, and persistence.
- **SvelteKit 5 frontend** for operator workflows and visualization.

The core value proposition is **zero active probing** in production OT environments: packet captures and imported logs are analyzed without transmitting packets on target networks.

### Why it exists
- Active scanning can destabilize PLC/RTU/process-control systems.
- OT security assessments still require asset inventory, communication mapping, and risk triage.
- Kusanagi Kajiki provides passive discovery plus advanced security analytics and reporting.

### Comparison to GRASSMARLIN
Kusanagi Kajiki is explicitly positioned as a modern successor to GRASSMARLIN:
- Rewritten from scratch in Rust/Svelte instead of Java Swing.
- Adds ATT&CK-for-ICS detections, CVE/compliance mapping, malware behavior detection, baseline drift, segmentation recommendations, and richer exports.
- Uses human-readable YAML signatures instead of legacy XML.

See canonical positioning and capability matrix in `README.md`. 

---

## 2) Tech Stack

## Backend (Rust/Tauri)
- **Tauri 2** desktop app shell + IPC command layer.
- **Workspace architecture** with 10 domain crates under `src-tauri/crates/*`.
- **Core deps** include:
  - `serde`, `serde_json`, `serde_yaml` (serialization + config/signatures)
  - `tokio` (async runtime in app setup/tasks)
  - `rusqlite` (in gm-db)
  - `pcap`/packet parsing ecosystem (in capture/parsers crates)
  - `env_logger`, `log` (observability)
  - `clap` (CLI switches for startup import/open)

## Frontend (SvelteKit/TypeScript)
- **SvelteKit + Svelte 5** application shell.
- **TypeScript IPC contract** in `src/lib/types/index.ts`.
- **Central stores** in `src/lib/stores/index.ts`.
- **Typed Tauri wrappers** in `src/lib/utils/tauri.ts`.
- Graph/UI utilities in Cytoscape-oriented components and helpers.

---

## 3) Directory Map (Annotated)

The repo is split between a webview frontend and a Rust workspace backend.

```text
.
├── README.md                     # Product overview, capability matrix, install/run/test quickstart
├── DEVELOPER_GUIDE.md            # This onboarding guide
├── package.json                  # Frontend scripts/dependencies
├── vite.config.ts                # Vite/SvelteKit bundling config
├── svelte.config.js              # SvelteKit adapter/preprocess config
├── src/
│   ├── routes/                   # App entry routes/layout wiring
│   │   ├── +page.svelte          # Main UI shell composition
│   │   ├── +layout.svelte        # Global layout wrapper
│   │   └── +layout.ts            # Route-level preload config
│   └── lib/
│       ├── components/           # 20 primary view components (tabs/subviews)
│       ├── stores/index.ts       # Global application state stores + derived stores
│       ├── types/index.ts        # Frontend↔backend data contract types
│       ├── utils/tauri.ts        # Typed invoke()/event wrappers for IPC
│       ├── utils/graph.ts        # Graph helper logic for topology rendering
│       └── layouts/purdueLayout.ts # Purdue model graph layout utility
├── src-tauri/
│   ├── Cargo.toml                # Tauri app crate + workspace membership + shared deps
│   ├── src/main.rs               # App bootstrap, state setup, 96-command invoke handler
│   ├── src/commands/             # Tauri command modules by domain
│   ├── crates/
│   │   ├── gm-capture/           # PCAP/live capture normalization to ParsedPacket
│   │   ├── gm-parsers/           # Protocol ID + deep parse modules
│   │   ├── gm-signatures/        # YAML signature engine + matcher
│   │   ├── gm-topology/          # Topology graph builder (nodes/edges)
│   │   ├── gm-db/                # SQLite sessions, assets, projects, OUI, GeoIP
│   │   ├── gm-analysis/          # ATT&CK/Purdue/anomaly/CVE/compliance/malware/etc.
│   │   ├── gm-physical/          # Vendor config parsing + physical topology inference
│   │   ├── gm-ingest/            # External tool importers (Zeek/Suricata/Nmap/...)
│   │   ├── gm-report/            # PDF/CSV/JSON/SBOM/STIX export models/generation
│   │   └── gm-segmentation/      # Zone/matrix/enforcement recommendation engine
│   ├── signatures/               # YAML fingerprint files (runtime-loaded)
│   ├── data/oui.tsv              # IEEE OUI lookup dataset
│   └── tauri.conf.json           # Tauri window/app packaging config
└── tests/pcaps/                  # Sample/import test data helpers
```

### `src/lib/components` file map (20)
1. `TopologyView.svelte` — logical topology orchestration shell.
2. `LogicalView.svelte` — node/edge logical graph rendering.
3. `MeshView.svelte` — matrix-like connectivity view.
4. `FilteredView.svelte` — subset graph tab with hidden-node controls.
5. `WatchTab.svelte` — focused neighborhood tracking by node/depth.
6. `PhysicalView.svelte` — physical/inferred topology visual layer.
7. `InventoryView.svelte` — asset table/details/editing.
8. `CaptureView.svelte` — import/live-capture control plane.
9. `ProtocolStats.svelte` — protocol distribution presentation.
10. `AnalysisView.svelte` — findings/anomalies/Purdue output.
11. `CommunicationPatterns.svelte` — periodicity/jitter anomaly views.
12. `SignatureEditor.svelte` — YAML signature editing/testing UX.
13. `ExportView.svelte` — report and data export UX.
14. `ProjectsView.svelte` — engagement/project management.
15. `BaselineDriftView.svelte` — baseline-vs-current drift display.
16. `SegmentationView.svelte` — microsegmentation recommendations.
17. `SettingsView.svelte` — user preferences.
18. `TimelineScrubber.svelte` — temporal filtering/playback controls.
19. `PurdueOverlay.svelte` — Purdue level visual bands/annotations.
20. `ConnectionTree.svelte` — per-node/connection packet summary explorer.

---

## 4) Core Data Pipeline (PCAP End-to-End)

A packet capture flows through the system in this order:

1. **Ingestion** (`gm-capture`)
   - Reads PCAP files or live interfaces.
   - Produces normalized `ParsedPacket` records (L2/L3/L4 metadata + payload + origin file).

2. **Protocol Identification + Deep Parse** (`gm-parsers`)
   - Fast protocol ID using ports/signatures.
   - Deep dissectors extract protocol-specific semantics (roles, FCs, object IDs, etc.) for 10 protocols (Modbus, DNP3, ENIP/CIP, S7, BACnet, IEC104, PROFINET DCP, LLDP, SNMP, redundancy protocols).

3. **Packet Processing Orchestration** (`commands/processor.rs`)
   - `PacketProcessor` accumulates:
     - connections, packet summaries, per-asset metadata,
     - deep-parse aggregates,
     - LLDP/redundancy/SNMP identity data,
     - communication timing stats and anomaly precursors.

4. **Fingerprinting** (`gm-signatures`)
   - `SignatureEngine` matches accumulated per-IP packet batches to YAML signatures.
   - Produces confidence-scored vendor/device-type/product hints.

5. **Graph Building** (`gm-topology`)
   - `TopologyBuilder` materializes `TopoNode`/`TopoEdge` graph snapshot.

6. **Enrichment + Persistence support** (`gm-db`)
   - OUI vendor and GeoIP lookup enrich assets.
   - Sessions/assets/connections/projects persisted in SQLite via command layer.

7. **Security Analytics** (`gm-analysis`)
   - ATT&CK detections + context attack logic.
   - Purdue assignments/violations, anomaly scoring, CVE matching, malware signatures, compliance mappings, allowlist outputs.

8. **Physical Context** (`gm-physical`)
   - Config and neighbor tables from Cisco/Juniper/Aruba/generic files augment physical topology.

9. **External Correlation** (`gm-ingest`)
   - Zeek/Suricata/Nmap/Masscan/Wazuh/SINEMA/TIA imports merge signals into same model.

10. **Export + Policy Design** (`gm-report`, `gm-segmentation`)
   - Exports: PDF/CSV/JSON/SBOM/STIX and filtered PCAP derivatives.
   - Segmentation engine builds identity groups, zones/conduits, comm matrix, enforcement configs, and simulation results.

---

## 5) Backend Crates (One Section Each)

## `gm-capture`
**Responsibility:** packet I/O and normalization.
- `PcapReader` for file import, with progress reporting.
- Live capture module with start/stop/pause/resume handles and stats.
- `ParsedPacket` is the canonical packet struct consumed downstream.

## `gm-parsers`
**Responsibility:** protocol identification and deep decode.
- `identify_protocol()` classifies packets.
- `deep_parse()` dispatches to protocol parsers.
- Includes modules for Modbus, DNP3, ENIP/CIP, S7, BACnet, IEC104, PROFINET DCP, LLDP, SNMP, redundancy.
- `modbus.rs` is a good template for adding new parser behavior.

## `gm-signatures`
**Responsibility:** YAML-based fingerprinting.
- Loads `*.yaml`/`*.yml` signatures.
- Compiles filter set (port/protocol/payload/OUI/min-len).
- Matches packets and returns sorted confidence results.
- Supports signature test workflows for editor UX.

## `gm-topology`
**Responsibility:** logical graph model.
- Maintains IP-keyed nodes and directional protocol edges.
- Tracks byte/packet counters and bidirectionality.
- Emits serializable `TopologyGraph` snapshots for frontend.

## `gm-db`
**Responsibility:** persistence + lookups.
- Session CRUD, asset/history CRUD, connection and project operations.
- OUI and GeoIP helper lookups.
- Central `Database` wrapper around `rusqlite::Connection`.

## `gm-analysis`
**Responsibility:** security and posture analytics.
- ATT&CK rule engine (`attack.rs` + context rules).
- Purdue assignment/violation logic.
- Anomaly and communication-pattern models.
- CVE matching, malware behavior matching, default creds, risk/criticality, compliance mapping, switch hardening checks.

## `gm-physical`
**Responsibility:** physical topology extraction.
- Parsers for Cisco/Juniper/Aruba/generic input formats.
- Link and location inference for switch-port-level maps.

## `gm-ingest`
**Responsibility:** external tool data import.
- Parses Zeek, Suricata, Nmap, Masscan, Wazuh, SINEMA CSV, TIA XML.
- Produces merged `IngestResult` with assets, connections, alerts.

## `gm-report`
**Responsibility:** output generation.
- Structured export models (`ReportData`, `ExportAsset`, etc.).
- PDF, CSV, JSON, SBOM, STIX output paths.

## `gm-segmentation`
**Responsibility:** microsegmentation recommendations.
- Policy groups (identity), zones/conduits, matrix, enforcement formats, simulation.
- Orchestrated by `run_segmentation_analysis` returning `SegmentationReport`.

---

## 6) AppState Deep Dive

`AppState` is the Tauri-managed singleton and has two-layer structure:

- `AppState { inner: Mutex<AppStateInner>, import_cancelled: Arc<AtomicBool> }`
- `AppStateInner` contains all mutable domain state (topology, assets, connections, parse caches, findings, session context, physical/inferred topology, ingest alerts, segmentation cache, etc.).

### Mutex pattern
- Command handlers lock `inner` for coherent updates.
- Heavy/long-running import pipeline keeps cancellation flag outside the mutex (`Arc<AtomicBool>`) to avoid lock contention and allow responsive cancel checks.

### Cancel flag pattern
- Import command resets flag, worker loop checks periodically.
- `cancel_import` toggles atomic bool.
- Import exits early and reports canceled status without requiring full state lock ownership.

---

## 7) IPC Contract (Tauri Commands)

### Reality check on command count
- **Current code registers 96 commands** in `generate_handler!` (not 93).
- If any docs/UI assume 93, treat that as stale.

### Domain grouping
- `system` (5): app info, settings, interface/plugin discovery.
- `capture` (7): PCAP import/cancel + live capture lifecycle.
- `data` (9): topology/assets/connections/counts/protocol/deep parse/timeline.
- `signatures` (3): list/reload/test.
- `session` (8): save/load/list/delete + asset update + archive import/export.
- `baseline` (1): compare sessions.
- `physical` (11): vendor imports, inferred topology, cleanup.
- `ingest` (8): external tool imports + Zeek event summary.
- `correlation` (3): alert correlation queries/clear.
- `wireshark` (6): detect/open/frame exports.
- `export` (12): CSV/JSON/PDF/SBOM/STIX/allowlist/firewall rules/filtered PCAP.
- `analysis` (11): run + findings + Purdue/anomaly/credentials/criticality/naming/switch/malware/compliance/CVE.
- `patterns` (3): connection stats, anomalies, redundancy protocols.
- `projects` (7): CRUD + active project selection.
- `segmentation` (2): run + enforcement export.

---

## 8) Frontend Architecture

## State model (`src/lib/stores/index.ts`)
- Stores mirror backend state domains: interfaces, assets, connections, topology, protocol stats, sessions, physical topology, findings, anomalies, segmentation report, capture stats/status, timeline controls, active tab/project, filters.
- Derived stores (`selectedAsset`, `filteredAssets`) provide UI projections.

## Component architecture (20 components)
- The app composes domain-specific tab views from `src/lib/components/*`.
- `TopologyView` controls logical/mesh/filtered/watch subtabs.
- Capture/import and analysis flows are tab-first and store-driven.

## IPC wrapper layer (`src/lib/utils/tauri.ts`)
- Single abstraction point for `invoke()` and event listeners.
- Strongly typed signatures prevent ad-hoc stringly backend calls in components.

## Type safety contract (`src/lib/types/index.ts`)
- Frontend interfaces intentionally mirror Rust serialization structs.
- Any Rust field/type change that crosses IPC boundary must be reflected here.
- Discipline: update Rust struct + command payload + TS type + wrapper + component usage together.

---

## 9) Adding New Features (Recipes)

## A) New protocol parser
1. Add parser module in `gm-parsers/src/<protocol>.rs`.
2. Extend `IcsProtocol` and port/protocol ID mapping.
3. Hook into `deep_parse()` dispatcher + expose parsed structs.
4. Extend `PacketProcessor` accumulators/builders for frontend-relevant summaries.
5. Add/adjust TS types + UI rendering (deep parse panel).
6. Add parser/unit tests with realistic payload samples.

## B) New YAML signature
1. Add YAML under `src-tauri/signatures/`.
2. Use existing filter primitives (port/protocol/payload/MAC OUI/min length).
3. Set confidence 1–5 based on specificity.
4. Reload signatures (`reload_signatures`) or restart app.
5. Validate in Signature Editor test flow.

## C) New ATT&CK detection rule
1. Implement detection function in `gm-analysis/src/attack.rs` (or `context_attacks.rs` for cross-state logic).
2. Emit `Finding::new(...)` with clear evidence and ATT&CK technique ID.
3. Wire into `detect_attack_techniques()` orchestration.
4. Add deterministic tests with representative synthetic snapshots.

## D) New frontend view + Tauri command
1. Add Rust `#[tauri::command]` in appropriate `src-tauri/src/commands/*.rs` module.
2. Register in `generate_handler!` in `main.rs`.
3. Add TS type(s) in `src/lib/types/index.ts`.
4. Add wrapper in `src/lib/utils/tauri.ts`.
5. Add store wiring if persistent UI state is needed.
6. Build Svelte component and integrate into main tab shell.

---

## 10) Testing, Patterns, and CI/CD

### Rust tests
- Workspace tests run from `src-tauri` with `cargo test --all`.
- Repo documentation and badge claim **356 tests** (use as current expected baseline).

### Frontend checks
- Type/lint checks from root via `npm run check` (and optional `npm run build` for bundle validation).

### Test design patterns
- Crate-level unit tests for parsers/engines/data transforms.
- Snapshot-like assertions for serialization and export models.
- In-memory DB tests (`Database::open_in_memory`) for persistence logic.

### CI/CD practical note
- Keep backend and frontend contract changes synchronized; most regressions are cross-boundary type drift.
- For PRs touching commands/types/stores, run both Rust and frontend checks before merge.

---

## 11) Developer Environment / Build / Debug

## Local setup
1. Install Node + Rust toolchains and platform system dependencies (see README matrix).
2. `npm install`
3. `npm run tauri dev`

## Useful debug switches
- Set `RUST_LOG=info` (or `debug`) before running to inspect command/capture pipeline behavior.
- Use CLI startup aids:
  - `--open <pcap-or-kkj>`
  - `--import-pcap <pcap>`

## Practical debugging workflow
- Reproduce with smallest PCAP or importer input.
- Trace through `commands/processor.rs` for packet-path bugs.
- Validate Rust↔TS payload compatibility via wrapper call sites and type definitions.

---

## Appendix: Critical Files (Read These First)

- `src-tauri/src/commands/mod.rs` — AppState model and shared command structs.
- `src-tauri/src/commands/processor.rs` — packet processing and state accumulation pipeline.
- `src-tauri/crates/gm-parsers/src/modbus.rs` — deep parser implementation pattern.
- `src-tauri/crates/gm-analysis/src/attack.rs` — ATT&CK rule patterns.
- `src-tauri/crates/gm-signatures/src/engine.rs` — signature lifecycle and matcher.
- `src/lib/types/index.ts` — IPC type contract.
- `src/lib/stores/index.ts` — frontend state source of truth.
- `src-tauri/Cargo.toml` — workspace and backend dependency graph.


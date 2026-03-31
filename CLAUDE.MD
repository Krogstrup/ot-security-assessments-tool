# CLAUDE.md — Kusanagi Kajiki (草薙カジキ)

## Project Identity

**Kusanagi Kajiki** (草薙の梶木) — modern ICS/SCADA passive network discovery and topology visualization tool. Ground-up Rust rewrite of NSA's archived [GRASSMARLIN](https://github.com/nsacyber/GRASSMARLIN). Display name: **Kusanagi Kajiki**. Binary/package: **`kusanaginokajiki`**.

**This is a functional tool for real OT security assessments — not a demo.** Correctness and reliability take priority over feature velocity. Passive only — **NEVER generate network traffic.**

---

## External Documentation (READ THESE BEFORE CODING)

|Document|Contents|
|--------|--------|
|`docs/ARCHITECTURE.md`|Crate structure, data pipeline, frontend component map|
|`docs/PROJECT-REQUIREMENTS.md`|Phase tracking, deliverables, test counts|
|`docs/PRODUCT-REQUIREMENTS.md`|Feature specs, ATT&CK detections, protocol matrix|
|`docs/PROTOCOL-DEEP-PARSE.md`|Wire formats, Rust structs, test payloads|
|`docs/CHANGELOG.md`|Phase-by-phase delivery history|

---

## Current State

**All phases complete (0–15E). 431 tests passing. 95 Tauri commands. 31 YAML signatures. 10 deep-parsed protocols. 28k+ lines across 11 Rust crates.**

The 11 crates are: `gm-capture`, `gm-parsers`, `gm-signatures`, `gm-topology`, `gm-db`, `gm-physical`, `gm-ingest`, `gm-report`, `gm-analysis`, `gm-segmentation`, and the root `kusanaginokajiki` binary.

---

## Phase Status

|Phase|Scope|Status|
|-----|-----|------|
|0|Foundation|✅|
|1|PCAP Import & Basic Parsing|✅|
|2|Topology Visualization|✅|
|3|Signature Engine|✅|
|4|Deep Protocol Parsing|✅|
|5|Live Capture|✅|
|6|Persistence & Asset Management|✅|
|7|Physical Topology|✅|
|8|External Tool Integration|✅|
|9|Export & Reporting|✅|
|10|Security Analysis (ATT&CK, Purdue, Anomaly)|✅|
|11|Advanced Features (Baseline, Timeline, Plugins)|✅|
|12|Protocol Expansion (EtherNet/IP, S7, BACnet, IEC104, PROFINET DCP)|✅|
|13|LLDP, SNMP, Redundancy deep parse|✅|
|14|(see CHANGELOG.md)|✅|
|15A|Identity Group Engine|✅|
|15B|Zone/Conduit Recommender (IEC 62443)|✅|
|15C|Least-Privilege Matrix|✅|
|15D|Enforcement Export|✅|
|15E|Policy Simulation|✅|

**All phases complete. No active phase. Before adding any new phase, define its scope in `docs/PROJECT-REQUIREMENTS.md` first, then implement it.**

---

## Architecture

```text
┌────────────────────────────────────────────────────────────────┐
│  SvelteKit Frontend (Tauri Webview)                            │
│  ├── CaptureView       import + live + sessions + ingest       │
│  ├── LogicalView       fcose + Wireshark + Show Physical       │
│  ├── PhysicalView      switch/port Cytoscape                   │
│  ├── MeshView          all-to-all mesh                         │
│  ├── InventoryView     table + edit + detail + bulk            │
│  ├── ProtocolStats     traffic + FC distribution               │
│  ├── SignatureEditor   CodeMirror 6 YAML editor                │
│  ├── ExportView        CSV/JSON/PDF/SBOM/STIX                  │
│  ├── PurdueOverlay     Purdue level assignment + violations     │
│  ├── SegmentationView  5 tabs: Groups/Zones/Matrix/Enforcement/Sim │
│  └── SettingsView      DB path, OUI, GeoIP, plugins, theme     │
├────────────────────────────────────────────────────────────────┤
│  Tauri IPC: 95 Commands + Events (capture_stats / error)       │
├────────────────────────────────────────────────────────────────┤
│  Rust Backend                                                  │
│  ├── gm-capture        pcap, live, L2-L4 parsing               │
│  ├── gm-parsers        protocol ID + 10 deep parsers           │
│  ├── gm-signatures     YAML engine, 31 signatures              │
│  ├── gm-topology       petgraph network graph                  │
│  ├── gm-db             SQLite, OUI, GeoIP                      │
│  ├── gm-physical       Cisco IOS config/CAM/CDP/ARP            │
│  ├── gm-ingest         Zeek, Suricata, Nmap, Masscan           │
│  ├── gm-report         PDF, CSV, JSON, SBOM, STIX 2.1          │
│  ├── gm-analysis       ATT&CK, Purdue, anomaly scoring         │
│  ├── gm-segmentation   IEC 62443 zones, enforcement, sim       │
│  └── commands/         Tauri command handlers                  │
└────────────────────────────────────────────────────────────────┘
```

**Data flow:** capture/ingest → parsers → signatures → topology → analysis → segmentation → db → report → frontend

### Crate Responsibilities

|Crate|Does|Does NOT|
|-----|----|--------|
|`gm-capture`|PCAPs, live capture, L2-L4 framing|ICS protocols, UI, Tauri|
|`gm-parsers`|Protocol ID + 10 deep parsers|Network state, Tauri|
|`gm-signatures`|YAML fingerprints, confidence scoring|Raw packet parsing|
|`gm-topology`|Directed graph (petgraph)|Protocol knowledge|
|`gm-db`|SQLite CRUD, OUI, GeoIP|Parsing, network|
|`gm-physical`|Cisco config/CAM/CDP/ARP, physical topology|Logical topology|
|`gm-ingest`|Zeek, Suricata, Nmap, Masscan import|Direct capture|
|`gm-report`|PDF, CSV, JSON, SBOM, STIX 2.1|UI, capture|
|`gm-analysis`|ATT&CK, Purdue, anomaly scoring|Persistence, parsing|
|`gm-segmentation`|IEC 62443 zones/conduits, enforcement, sim|gm-analysis dependency|

`gm-segmentation` consumes outputs of `purdue.rs`, `risk.rs`, `comm_patterns.rs`, `allowlist.rs`, `compliance.rs`. It has **no crate dependency on `gm-analysis`**.

---

## File Map

### Config

```text
src-tauri/Cargo.toml                  # Workspace: 11 crates, all deps
src-tauri/tauri.conf.json             # "kusanaginokajiki" everywhere
src-tauri/capabilities/default.json   # dialog:allow-open/save, shell:allow-open
package.json                          # --legacy-peer-deps on Fedora
```

### Rust Commands (`src-tauri/src/commands/`)

```text
mod.rs          AppState: SignatureEngine, live_capture, processing_thread,
                oui_lookup, geoip_lookup, db, current_session, physical_topology
system.rs       list_interfaces, get_app_info, settings, plugins
capture.rs      import_pcap + start/stop/pause/resume/status
processor.rs    PacketProcessor: parse → deep → sigs → topology → OUI/GeoIP
data.rs         get_topology, assets, connections, connection_packets, deep_parse_info,
                fc_stats, timeline
signatures.rs   get/reload/test signatures
session.rs      save/load/list/delete, update/bulk assets, export/import .kkj
baseline.rs     compare_sessions (drift detection)
physical.rs     Cisco config/CAM/CDP/ARP import, get/clear physical topology
ingest.rs       import_zeek_logs, import_suricata_eve, import_nmap_xml,
                import_masscan_json + merge
wireshark.rs    detect/open Wireshark, get_connection_frames, export/save_frames_csv
export.rs       export_assets_csv, export_connections_csv, export_topology_json,
                export_assets_json, generate_pdf_report, export_sbom_json,
                export_sbom_csv, export_stix_bundle
analysis.rs     ATT&CK findings, Purdue assignment, anomaly scoring
segmentation.rs run_segmentation, export_enforcement_config
```

### Frontend (`src/lib/`)

```text
types/index.ts          All types (Capture*, Session*, Asset, Ingest*, Wireshark*,
                        Frame*, ReportConfig, SbomEntry, ExportFormat, ViewTab,
                        SegmentationInput, AssetProfile, ObservedConnection)
stores/index.ts         All stores: capture*, sessions, currentSession,
                        physicalHighlightIp + topology tab helpers
utils/tauri.ts          All Tauri command wrappers (keep in sync with main.rs)
utils/graph.ts          BFS, filtering, scaling, grouping, colors
components/             (see docs/ARCHITECTURE.md for full component list)
layouts/purdueLayout.ts Purdue level layout positions
```

### Data & Signatures

```text
src-tauri/signatures/           31 YAML signature files (runtime-loaded)
src-tauri/data/oui.tsv          IEEE OUI database (~30k entries)
src-tauri/data/dbip-country-lite.mmdb   DB-IP Lite GeoIP
```

### Paths (Runtime)

```text
~/.kusanaginokajiki/data.db       SQLite database
~/.kusanaginokajiki/settings.json Settings
~/.kusanaginokajiki/plugins/      Plugin directory
```

---

## Rules

1. **Read before writing.** If a file exists, read it first. Extend or modify — never duplicate.
2. **Preserve all existing functionality.** All phases (0–15E) are complete and must keep working.
3. **Follow established patterns.** Error handling, Tauri commands, PacketProcessor pipeline, store management.
4. **Register new commands** in `main.rs` AND typed wrappers in `tauri.ts`.
5. **Use `kusanaginokajiki` everywhere.** No "grassmarlin" references anywhere.
6. **No `unwrap()` in library crates.** Use `?`, `.map_err()`, `.unwrap_or_default()`.
7. **Cross-platform casts.** pcap `tv_sec as i64`, `tv_usec as u32` (i32 on Windows).
8. **Data bundled via `include_str!`.** No network calls for core features.
9. **NEVER generate network traffic.** Live capture = promiscuous receive only.

---

## Common Pitfalls

### Rust / Backend

- Tauri commands: return `Result<T, String>`, convert with `.map_err(|e| e.to_string())`
- etherparse 0.16: use `tcp.payload()` / `udp.payload()`, not `SlicedPacket.payload`
- pcap `PacketHeader`: dereference `*raw_packet.header`, cast `tv_sec as i64`, `tv_usec as u32`
- `IcsProtocol`: use `from_name()` for deserialization, `to_name()` for snake_case (Debug gives wrong names)
- Clippy: range patterns, `.contains()`, `.is_multiple_of()`, `.get()` returns ref, `.replace(['\n', '\r'], "")` not chained, `strip_prefix()` not manual, no needless `Ok()?`
- `gm-db`: rusqlite bundled, WAL mode, FK enforcement, `dirs` v6, `zip` v2
- `gm-ingest`: quick-xml 0.37 uses `@attr` for Nmap XML; Masscan input needs `clean_masscan_json()`
- `gm-physical`: `normalize_mac()` for Cisco dot format, `shorten_interface_name()` for port aliases
- `gm-report`: genpdf for PDF, manual CSV string builder (no csv crate), serde_json for JSON/SBOM/STIX
- `gm-segmentation`: `SegmentationInput` / `AssetProfile` / `ObservedConnection` are decoupled from Tauri state
- Enforcement configs: Cisco ACL names sanitized to uppercase; Suricata SIDs start at 9000001
- Zone score: 1.0 = perfect segmentation, 0.0 = flat network; derived from cross-Purdue violation ratio
- SCALANCE detection: `classify_scalance_model()` in `infrastructure.rs` classifies by model prefix (X→Switch, W→AP, M→Router, S→Firewall). `AssetSnapshot` has `hostname` + `product_family` for LLDP/SNMP enrichment
- Findings table exists in DB schema since Phase 6 — analysis commands populate it, don't recreate it
- `AssetInfo`: `oui_vendor`, `country`, `is_public_ip` use `#[serde(default)]`
- Active scan data (Nmap/Masscan) gets `[active-scan]` tag on connections

### Frontend / Build

- Static adapter only — no `+page.server.ts`
- **Build order:** `npm run build` before `cargo check`
- Dynamic import Cytoscape.js; Svelte 5 has no event modifiers
- Tauri plugins: dialog (open+save), shell (open); **NO plugin-fs**
- Plugin config in `tauri.conf.json`: use `null` not `{}` — empty objects cause runtime panic
- cytoscape-fcose: requires `.d.ts` + guard flag; compound nodes need `parent` + `'compound'` class
- Context menus: fixed-position `div` + `window` listener for dismiss
- CodeMirror 6: `codemirror`, `@codemirror/lang-yaml`, `theme-one-dark`, `state`, `view`
- `src-tauri/icons/icon.ico` required on Windows

---

## Protocol Support

### Port Mapping

```text
502       Modbus TCP
20000     DNP3
44818     EtherNet/IP (explicit messaging)
2222      EtherNet/IP I/O (implicit)
47808     BACnet/IP
102       S7comm (ISO-TSAP)
4840      OPC UA
34962-34964 PROFINET
2404      IEC 104
1883/8883 MQTT
5094      HART-IP
1089-1091 FF HSE
18245-18246 GE SRTP
5007      Wonderware SuiteLink
```

### Deep-Parsed Protocols (10 complete)

|Protocol|Module|Info Struct|
|--------|------|-----------|
|Modbus TCP|`modbus.rs`|`ModbusInfo`|
|DNP3|`dnp3.rs`|`Dnp3Info`|
|EtherNet/IP + CIP|`enip.rs`|`EnipInfo`|
|S7comm|`s7comm.rs`|`S7Info`|
|BACnet/IP|`bacnet.rs`|`BacnetInfo`|
|IEC 104|`iec104.rs`|`Iec104Info`|
|PROFINET DCP|`profinet_dcp.rs`|`ProfinetDcpInfo`|
|LLDP|`lldp.rs`|`LldpInfo`|
|SNMP|`snmp.rs`|`SnmpInfo`|
|Redundancy (MRP/RSTP/HSR/PRP/DLR)|`redundancy.rs`|`RedundancyInfo`|

Wire formats, struct definitions, and test payloads: see `docs/PROTOCOL-DEEP-PARSE.md`.

Confidence levels: 1=port, 2=pattern, 3=OUI, 4=payload, 5=deep parse.

---

## ATT&CK for ICS Detections

|Behavior|Technique|Severity|
|--------|---------|--------|
|Unknown device polling PLCs|T0846 Remote System Discovery|High|
|Modbus broadcast writes (FC 5/6/15/16, Unit 255 or dst 0.0.0.0)|T0855 Unauthorized Command Message|Critical|
|Modbus FC 8 from non-engineer|T0814 Denial of Service|High|
|DNP3 unsolicited (FC 130) to unknown master|T0856 Modify Alarm Settings|Medium|
|Cross-Purdue communication (L1↔L4+)|T0886 Remote Services|Medium|
|CIP write commands|T0855|Critical|
|CIP firmware upload|T0836 Firmware Update|High|
|S7 download block|T0843 Program Download|Critical|
|S7 upload block|T0845 Program Upload|High|
|S7 block delete|T0809 Data Destruction|High|
|S7 PLC stop / BACnet reinitialize / IEC104 reset|T0816 Device Restart/Shutdown|Critical|
|BACnet alarm suppress|T0856|Medium|
|BACnet output write|T0855|Critical|
|BACnet comm control|T0811 Communication Manipulation|High|
|IEC104 command (ASDU TypeID 45-64)|T0855|Critical|
|IEC104 interrogation flood|T0814|High|

Full detection specifications: see `docs/PRODUCT-REQUIREMENTS.md`.

---

## ICS/SCADA Domain Knowledge

### Purdue Model Assignment Heuristics

- **L0** — sensors/actuators (field device OUI, no IP services)
- **L1** — PLCs/RTUs (port 502/44818/102 responder, Modbus/EIP/S7 server)
- **L2** — HMIs (multi-OT client, RDP/VNC, Windows OUI)
- **L3** — Historians/SCADA (OPC UA server, high fan-out, database ports)
- **L3.5** — DMZ
- **L4–5** — Enterprise IT (HTTP/HTTPS only, no OT protocols)

Cross-level communication is a security finding. L1↔L4 = T0886.

### Phase 15 — Microsegmentation (Complete)

- **15A**: Identity Group Engine — clusters by Purdue level, role, vendor, communication community
- **15B**: Zone/Conduit Recommender — IEC 62443 boundaries, flat network detection
- **15C**: Least-Privilege Matrix — per-zone-pair allow rules with risk classification
- **15D**: Enforcement Export — Cisco IOS ACL, Cisco ASA, generic firewall, Suricata rules, JSON policy
- **15E**: Policy Simulation — replay traffic vs policy, risk reduction scoring, false positive detection

`SegmentationView.svelte` has 5 tabs: Policy Groups, Zones, Matrix, Enforcement, Simulation.

---

## Tech Stack

### Rust Backend

Tauri 2.0, pcap, etherparse, petgraph, serde/serde_json/serde_yaml, tokio, thiserror, chrono, uuid, rusqlite (bundled), maxminddb, dirs (v6), zip (v2), regex, quick-xml, genpdf, clap

### Frontend

SvelteKit (Svelte 5), TypeScript (strict), Cytoscape.js + fcose, Tailwind CSS 4, @tauri-apps/api + plugins (dialog, shell), CodeMirror 6

---

## Build & Test

```bash
# Fedora / WSL2
sudo dnf install libpcap-devel webkit2gtk4.1-devel libsoup3-devel javascriptcoregtk4.1-devel
npm install                        # add --legacy-peer-deps on Fedora if needed
npm run build                      # MUST run before cargo check
npm run tauri dev

# Full test suite (431 tests expected)
cd src-tauri && cargo test --all
cargo clippy --all -- -D warnings
cd .. && npm run check

# Production build
npm run tauri build

# CLI
kusanaginokajiki --open capture.pcap
kusanaginokajiki --import-pcap data.pcap
kusanaginokajiki --open session.kkj
```

**Ubuntu:** `libpcap-dev libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf`  
**macOS:** `brew install libpcap`  
**Windows:** VS C++ Build Tools + Npcap (WinPcap compat mode) + Npcap SDK (`LIB` → `Lib\x64`). Requires `src-tauri/icons/icon.ico`.

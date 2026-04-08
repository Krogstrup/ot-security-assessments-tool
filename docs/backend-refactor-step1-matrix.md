<!-- markdownlint-disable MD013 -->

# Backend Refactor Step 1: Responsibility + Duplication Matrix

Last reviewed: 2026-04-08
Scope: `src-tauri/src/*` + workspace crates under `src-tauri/crates/*`
Goal: establish backend-only baseline of oversized modules, duplicated responsibilities, and hard-coded policy anchors before extraction work.

## 1) Oversized module hotspots

| Module | LOC | Layer today | Mixed responsibilities observed | Split target (next steps) |
|---|---:|---|---|---|
| `src-tauri/src/commands/session.rs` | 643 | interface+application+infrastructure | session orchestration + DB mapping + ZIP import/export + state mutation | `application/use_cases/session/*` + `infrastructure/session_archive/*` + mapper module |
| `src-tauri/src/commands/ingest.rs` | 596 | interface+application+domain-policy | source-specific imports + merge policy + topology rebuild + alert correlation | `application/use_cases/ingest/*` + shared ingest merge policy service |
| `src-tauri/src/commands/export.rs` | 475 | interface+application+infrastructure | export orchestration + mapping + file writes + manual base64 decoder | `application/use_cases/export/*` + `infrastructure/export_fs/*` + shared mappers |
| `src-tauri/src/commands/data.rs` | 451 | interface+application-query | topology capping + paging/sorting + stats + deep-parse aggregation | `application/queries/data/*` + shared stats/paging utilities |
| `src-tauri/src/commands/capture.rs` | 393 | interface+application | import orchestration + processing pipeline finalization + state commit + progress events | `application/use_cases/capture/import_pcap.rs` + shared pipeline commit service |
| `src-tauri/src/commands/analysis_builders.rs` | 390 | application+domain-policy | analysis input mapping + context derivation + OT heuristics/ports | `application/mappers/analysis/*` + constants from single policy source |
| `src-tauri/src/commands/segmentation.rs` | 350 | interface+application+domain-policy | segmentation input assembly + OT classification + allowlist derivation + format parsing | `application/use_cases/segmentation/*` + mapper modules |
| `src-tauri/src/bin/kusanaginokajiki_web/web_support.rs` | 365 | interface+infrastructure | API error envelope + import/export path policy + extension allowlists | `interface/http/support/*` + optional shared path policy service |
| `src-tauri/crates/gm-segmentation/src/zones.rs` | 1146 | domain (overgrown) | zone modeling + subzone split + conduit build + scoring + recommendation cross-check | split into `zones/model`, `zones/conduits`, `zones/recommendations`, `zones/scoring` |
| `src-tauri/crates/gm-parsers/src/enip.rs` | 860 | domain parser (overgrown) | ENIP header parsing + CIP parsing + identity parsing + mapping tables | split into `enip/header`, `enip/cip`, `enip/identity`, `enip/mapping` |
| `src-tauri/crates/gm-analysis/src/compliance.rs` | 767 | domain policy (overgrown) | mapping load + framework routing + many check evaluators + reporting assembly | split by check families + framework resolver module |
| `src-tauri/crates/gm-segmentation/src/simulation.rs` | 751 | domain simulation | simulation orchestration + rule matching + risk/deployment scoring | split into engine, scoring, reason-builder |

## 2) Duplicated responsibility matrix

| Responsibility | Current duplicate owners | Drift risk | Single source of truth target |
|---|---|---|---|
| Capture pipeline state-commit logic (topology/assets/stats update) | `commands/capture.rs::compute_and_apply_import_state` and `bin/.../web_runtime.rs::flush_batch_headless` | High | `application/services/capture_pipeline_commit.rs` consumed by both Tauri+HTTP adapters |
| HTTP adapter passthrough to command adapters (dual interface layering) | `bin/.../web_handlers_*.rs` directly calling `commands::*` | Medium | move shared use-cases under `application/*`; keep both transports as thin adapters |
| Asset/connection snapshot mapping across analysis/export/segmentation | `commands/analysis_builders.rs`, `commands/export.rs`, `commands/segmentation.rs` | High | shared mapper modules per DTO family (`application/mappers/*`) |
| `/24` subnet helper logic | `commands/segmentation.rs::compute_subnet_24` and `gm-segmentation/src/zones.rs::compute_subnet_24` | Medium | one subnet helper in `gm-segmentation` (or `gm-constants`) reused by adapter |
| Compliance framework allowlist validation | `commands/analysis.rs` and `gm-analysis/src/compliance.rs` | Medium | `gm-analysis::supported_frameworks()` as sole validator |
| Ingest source wrappers with near-identical flow | `commands/ingest.rs` import functions (`zeek/suricata/nmap/wazuh/masscan/sinema/tia`) | Medium | single generic `run_ingest(source, parser_fn)` helper |

## 3) Hard-coded policy anchors to centralize

| Hard-coded logic | Current locations | Consolidation target |
|---|---|---|
| OT server port lists and protocol mapping | `commands/analysis_builders.rs`, `commands/ingest.rs`, `commands/mod.rs`, `gm-analysis/src/purdue.rs`, `gm-analysis/src/helpers.rs` | use `gm-constants` (`OT_SERVER_PORTS`, `is_ot_server_port`, protocol mapping helpers) as canonical source |
| Device type heuristics | `commands/mod.rs::infer_device_type`, `gm-ingest/src/sinema.rs::infer_device_type_from_model` | central policy helper in `gm-analysis` or `gm-constants` with protocol/model hooks |
| Enforcement format alias parsing | `commands/segmentation.rs::parse_enforcement_format` | parser/helper in `gm-segmentation` crate; adapter only validates transport input |
| Import-kind extension allowlists | `bin/.../web_support.rs::ImportKind::extensions` | optional shared policy module (`gm-constants` or `application/import_policy`) |

## 4) Layer boundary gaps to address in extraction

- Interface layer currently contains application orchestration in `commands/*`.
- HTTP layer currently calls command adapters directly instead of shared application use-cases.
- Infrastructure concerns (zip/file/db mapping) are mixed into command modules (`session`, `export`, `physical`).
- Domain heuristics are duplicated across command layer and crates instead of staying in one policy owner.

## 5) First extraction slice selected for Step 2

Selected slice: **capture pipeline unification**.

Why this first:

- Highest duplication with active runtime risk (`capture.rs` vs `web_runtime.rs`).
- Crosses both transports (WebUI+API and compatibility command surface) with clear single-source target.
- Enables immediate adapter thinning without contract changes.

Step-2 target files/modules:

- new: `src-tauri/src/application/services/capture_pipeline_commit.rs`
- refactor callers: `src-tauri/src/commands/capture.rs`, `src-tauri/src/bin/kusanaginokajiki_web/web_runtime.rs`
- keep external API contracts unchanged in step 2.

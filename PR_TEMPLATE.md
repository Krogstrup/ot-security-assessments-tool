# Golden Architecture PR Review

## Summary

Describe the architectural finish achieved by this PR.

- What boundary was completed?
- Which command hotspots were thinned?
- Which application modules were fully detached from runtime/container state?
- Which contract/validation paths were standardized?

---

## Scope

- [ ] No intentional behavior change
- [ ] Ready for `main`
- [ ] Not an intermediate modularization step
- [ ] Documentation updated to match final code shape

---

## Target architecture

```text
api_* / web binary  -> transport only
commands/*          -> thin adapters only
application/*       -> use-cases, mappers, services, ports
gm-* crates         -> domain / infra logic
```

Hard rule:

```text
application/* must not depend on concrete AppState or crate::commands::*
```

---

## 1. Boundary correctness

### 1.1 No command/runtime coupling inside application

- [ ] `src-tauri/src/application/**` does not import `crate::commands::*`
- [ ] `src-tauri/src/application/**` does not import `commands::support::*`
- [ ] `src-tauri/src/application/**` does not import `AppState`
- [ ] `src-tauri/src/application/**` does not reference command-layer state structs (`CaptureState`, `InventoryState`, `AnalysisState`, `SessionState`)

Verification:

```bash
rg -n 'crate::commands::|use crate::commands|commands::support|AppState|CaptureState|InventoryState|AnalysisState|SessionState' src-tauri/src/application
```

Expected:

```text
0 matches
```

Result:
- [ ] Pass
- [ ] Fail

Notes:

---

### 1.2 No lock choreography in application

- [ ] `src-tauri/src/application/**` does not call `.lock()`, `.read()`, `.write()`
- [ ] application code does not encode lock order
- [ ] application code does not directly mutate runtime state containers

Verification:

```bash
rg -n '\.lock\(|\.read\(|\.write\(' src-tauri/src/application
rg -n 'lock order|capture -> inventory -> analysis|capture.?→.?inventory' src-tauri/src/application
```

Expected:

```text
0 matches
```

Result:
- [ ] Pass
- [ ] Fail

Notes:

---

### 1.3 No upward dependency from domain crates

- [ ] `src-tauri/crates/gm-*` does not import `crate::application::*`
- [ ] `src-tauri/crates/gm-*` does not import `crate::commands::*`
- [ ] domain crates do not depend on transport/runtime frameworks unless explicitly infra-only

Verification:

```bash
rg -n 'crate::application::|use crate::application|crate::commands::|use crate::commands' src-tauri/crates
rg -n 'axum|http::|tower|tauri' src-tauri/crates/gm-*
```

Expected:

```text
0 matches
```

Result:
- [ ] Pass
- [ ] Fail

Notes:

---

## 2. Commands layer acceptance

### 2.1 Commands are thin adapters only

- [ ] commands only read/write state, acquire snapshots/ports, call application, map errors/results
- [ ] no substantive business orchestration remains in commands
- [ ] no duplicate snapshot-building logic remains in commands when application mappers/use-cases exist

Manual review targets:
- [ ] `src-tauri/src/commands/analysis.rs`
- [ ] `src-tauri/src/commands/capture.rs`
- [ ] `src-tauri/src/commands/physical.rs`
- [ ] `src-tauri/src/commands/projects.rs`
- [ ] `src-tauri/src/commands/baseline.rs`
- [ ] `src-tauri/src/commands/wireshark.rs`

Verification:

```bash
wc -l src-tauri/src/commands/*.rs
rg -n 'for |while |match ' src-tauri/src/commands
rg -n 'gm_[a-z_]+::' src-tauri/src/commands
```

Expected:
- hotspot files materially smaller/simpler
- minimal direct `gm-*` orchestration
- loops/branching mostly limited to request/result plumbing

Result:
- [ ] Pass
- [ ] Fail

Notes:

---

### 2.2 Commands do not carry business policy

- [ ] no merge/confidence policy in commands
- [ ] no analysis orchestration in commands
- [ ] no segmentation policy assembly in commands
- [ ] no session workflow policy in commands
- [ ] no export workflow policy in commands

Verification:

```bash
rg -n 'confidence|merge|orchestr|build_.*input|generate_|run_.*analysis|run_.*segmentation|save_session|load_session' src-tauri/src/commands
```

Expected:
Only adapter-level wrappers remain.

Result:
- [ ] Pass
- [ ] Fail

Notes:

---

## 3. Application layer acceptance

### 3.1 Use-case contracts are clean

- [ ] use-case entry points do not take `&AppState`
- [ ] use-case entry points do not take command-layer state structs
- [ ] use-case entry points accept request DTOs, snapshots, repositories, ports, or focused services
- [ ] use-case entry points return explicit result DTOs or domain results

Verification:

```bash
rg -n 'fn .*AppState|fn .*(CaptureState|InventoryState|AnalysisState|SessionState)' src-tauri/src/application
```

Expected:

```text
0 matches
```

Result:
- [ ] Pass
- [ ] Fail

Notes:

---

### 3.2 Extracted modules are fully complete, not half-extracted

- [ ] session use-cases detached from commands/runtime container
- [ ] export use-cases detached from commands/runtime container
- [ ] segmentation use-cases detached from commands/runtime container
- [ ] ingest use-cases detached from commands/runtime container
- [ ] data query modules detached from commands/runtime container where intended
- [ ] capture services detached from commands/runtime container where intended

Verification:

```bash
rg -n 'crate::commands::|AppState|CaptureState|InventoryState|AnalysisState|SessionState' src-tauri/src/application/use_cases src-tauri/src/application/queries src-tauri/src/application/services
```

Expected:

```text
0 matches
```

Result:
- [ ] Pass
- [ ] Fail

Notes:

---

### 3.3 Mappers are only mappers

- [ ] `application/mappers/*` does mapping only
- [ ] no DB writes, file writes, or orchestration in mappers
- [ ] no lock/state access in mappers

Verification:

```bash
rg -n 'Database|write_|read_|mutex_|lock\(|run_|save_|load_|export_' src-tauri/src/application/mappers
```

Expected:
Only mapping/conversion behavior.

Result:
- [ ] Pass
- [ ] Fail

Notes:

---

## 4. Web / transport layer acceptance

### 4.1 Web module structure matches intended design

- [ ] `web_support.rs` remains a thin shim/re-export surface
- [ ] `http_types.rs` remains the focused home for HTTP envelope types
- [ ] `import_support.rs` remains the focused home for import/export path helpers
- [ ] `web_api_paths.rs` remains the route-path authority
- [ ] route composition remains explicit and readable

Verification:

```bash
wc -l src-tauri/src/bin/kusanaginokajiki_web/web_support.rs
rg -n 'pub struct ApiError|ImportPcapFilesResponse' src-tauri/src/bin/kusanaginokajiki_web
rg -n 'resolve_import_input_path|resolve_export_output_path|list_import_files_for_kind' src-tauri/src/bin/kusanaginokajiki_web
```

Result:
- [ ] Pass
- [ ] Fail

Notes:

---

### 4.2 HTTP handlers are transport-only

- [ ] handlers only parse/normalize request data, call commands, map responses/errors
- [ ] handlers do not orchestrate domain workflows

Verification:

```bash
rg -n 'gm_[a-z_]+::|TopologyBuilder|Database|SegmentationInput|AnalysisInput' src-tauri/src/bin/kusanaginokajiki_web
```

Expected:
No direct domain orchestration in handlers.

Result:
- [ ] Pass
- [ ] Fail

Notes:

---

## 5. Contract and validation acceptance

### 5.1 Backend error contract is structured

- [ ] core endpoints return a machine-readable error envelope
- [ ] frontend does not depend on ad hoc string errors
- [ ] structured `code/message` path is standard for high-traffic flows

Verification:

```bash
rg -n 'Result<.*String>|Err\(.*to_string\(' src-tauri/src/commands src-tauri/src/bin/kusanaginokajiki_web src-tauri/src/application
rg -n 'code.*message|ApiError' src-tauri/src/bin/kusanaginokajiki_web src-tauri/src/commands
```

Result:
- [ ] Pass
- [ ] Fail

Notes:

---

### 5.2 Frontend response validation is broad enough for production

- [ ] analysis endpoints validated
- [ ] projects endpoints validated
- [ ] session endpoints validated
- [ ] data endpoints validated
- [ ] capture endpoints validated
- [ ] export endpoints validated
- [ ] ingest endpoints validated
- [ ] physical/signatures/system/wireshark validated where material

Verification:

```bash
rg -n 'httpValidated|zod|schema' src/lib/api
rg -n 'httpJson\(' src/lib/api
```

Result:
- [ ] Pass
- [ ] Fail

Notes:

---

### 5.3 Generated bindings and schemas are current

- [ ] Rust model changes reflected in generated TS bindings
- [ ] frontend schema/type usage matches backend contract changes
- [ ] no stale generated contract files remain

Verification:

```bash
git diff -- src-tauri/bindings/gen/types
rg -n 'bindings/gen/types' src
```

Result:
- [ ] Pass
- [ ] Fail

Notes:

---

## 6. Domain crate coherence

- [ ] constants remain clearly owned
- [ ] DTO/model contracts remain clearly owned
- [ ] parser models remain clearly owned
- [ ] analysis logic remains in `gm-analysis`
- [ ] segmentation logic remains in `gm-segmentation`
- [ ] no “god shared crate” introduced without strong rationale

Result:
- [ ] Pass
- [ ] Fail

Notes:

---

## 7. Build, tests, and guardrails

### 7.1 Build/test commands

- [ ] `cargo check` passes
- [ ] `cargo test` passes
- [ ] `cargo build --bin kusanaginokajiki_web` passes
- [ ] frontend check/build passes

Verification:

```bash
cargo check --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml --bin kusanaginokajiki_web
npm run check
```

Results:

```text
cargo check:
cargo test:
cargo build:
npm run check:
```

---

### 7.2 Architecture guardrails pass

- [ ] no `AppState` in application
- [ ] no `crate::commands::*` in application
- [ ] no upward deps from crates into application/commands

Verification:

```bash
rg -n 'AppState' src-tauri/src/application
rg -n 'crate::commands::|use crate::commands' src-tauri/src/application
rg -n 'crate::application::|use crate::application|crate::commands::|use crate::commands' src-tauri/crates
```

Expected:

```text
0 matches
```

Result:
- [ ] Pass
- [ ] Fail

Notes:

---

## 8. Documentation alignment

- [ ] `AGENTS.md` matches final rules
- [ ] architecture review docs match actual code shape
- [ ] file map updated
- [ ] no stale docs describing pre-refactor ownership

Verification:

```bash
rg -n 'AppState|commands|application|web_support|http_types|import_support' AGENTS.md ARCHITECTURE_REVIEW.md SYSTEM_ARCHITECTURE_REVIEW.md FRONTEND_ARCHITECTURE_REVIEW.md docs/src-tauri-map.md
```

Result:
- [ ] Pass
- [ ] Fail

Notes:

---

## Final decision

Approve for `main` only if all are true:

- [ ] Boundary correctness achieved
- [ ] Commands are thin adapters
- [ ] Application detached from concrete runtime/container state
- [ ] Domain crates remain inward and clean
- [ ] Contracts and validation are production-safe
- [ ] Build/tests/checks pass
- [ ] Documentation matches final reality

Reject if any are true:

- [ ] `AppState` appears in `src-tauri/src/application/**`
- [ ] `crate::commands::*` appears in `src-tauri/src/application/**`
- [ ] lock helpers or lock calls appear in `src-tauri/src/application/**`
- [ ] command hotspots still contain orchestration
- [ ] extracted use-cases are only relocated command logic
- [ ] web cleanup regressed into larger mixed files
- [ ] validation/error handling remains partial on critical paths

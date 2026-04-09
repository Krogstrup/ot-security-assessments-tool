# Kusanagi Kajiki 草薙カジキ

![KusanaginoKajikiLogo](KusanaginoKajikiLogo.jpg)

**Passive ICS/SCADA network discovery and topology visualization for OT security assessments**

![Rust](https://img.shields.io/badge/rust-1.77+-orange?logo=rust)
![Axum](https://img.shields.io/badge/axum-0.8-5A3EBA)
![Svelte](https://img.shields.io/badge/svelte-5-FF3E00?logo=svelte)
![License](https://img.shields.io/badge/license-Apache%202.0-blue)

---

## 🚀 What is Kusanagi Kajiki?

Kusanagi Kajiki is a **passive-first OT security assessment platform** for Industrial Control Systems (ICS) and SCADA environments.

It analyzes network traffic (PCAP or live capture) to:

* discover devices
* identify protocols
* map topology
* detect security risks

⚠️ **No active scanning. No packets sent. No risk to PLCs or processes.**

---

## 🧠 Why it exists

In OT environments:

* active scanning can crash PLCs
* misconfigured probes can disrupt production
* visibility is often limited or outdated

Kusanagi Kajiki provides **full network visibility without touching the network**.

---

## ✨ Core Capabilities

### 🔍 Passive Discovery

* Multi-PCAP ingestion with origin tracking
* Live capture with real-time updates
* 19+ OT/ICS protocols detected
* Connection tracking (packets, bytes, timing)

---

### 🗺️ Topology & Visualization

* Logical topology (subnets + Purdue layers)
* Physical topology (switch/port + inference)
* Timeline replay of network evolution
* Mesh and filtered views

---

### 🧬 Deep Protocol Analysis

Supports deep parsing for major ICS protocols:

* Modbus, DNP3, EtherNet/IP (CIP)
* S7comm, BACnet, IEC-104
* PROFINET, LLDP, SNMP
* Redundancy protocols (MRP, RSTP, HSR, PRP, DLR)

Extracts:

* roles (master/slave, client/server)
* function codes and services
* vendor/device identity
* communication patterns

---

### 🧾 Device Identification

* YAML-based signatures (human-readable)
* MAC OUI lookup (~30k vendors)
* Protocol-specific vendor IDs
* Confidence scoring (1–5 scale)

---

### 🛡️ Security Analysis

* MITRE ATT&CK for ICS (40+ detections)
* Purdue model classification + violations
* ICS malware behavioral detection
* CVE matching (OT-focused)
* Communication anomaly detection
* Cleartext protocol auditing
* Internet exposure detection

---

### 🔌 Integrations

* Zeek
* Suricata
* Nmap / Masscan
* Wazuh
* Siemens SINEMA / TIA Portal
* Wireshark (external inspection)

---

### 📊 Reporting & Export

* PDF assessment reports
* CSV / JSON exports
* STIX 2.1 bundles
* SBOM (CISA BOD 23-01 aligned)
* PCAP filtering/export

---

## ⚡ Quick Start

```bash
git clone https://github.com/TheSecurityLead/KusanagiNoKajiki.git
cd KusanagiNoKajiki
npm install
npm run web
```

Then open:

```
http://localhost:4173
```

Import a PCAP and start exploring.

---

## 🧱 Architecture (Simplified)

```text
Frontend (SvelteKit)
    ↓
HTTP API (Axum)
    ↓
Application (use-cases)
    ↓
Domain (gm-* crates)
```

### Key principles

* **Passive-first processing pipeline**
* **Strict layer separation**
* **Domain logic isolated in `gm-*` crates**
* **Commands act as thin adapters (being reduced over time)**

---

## 🔄 Data Flow

```text
PCAP / Live Capture
    → Parsing (L2–L4)
    → Protocol Detection + Deep Parse
    → Signature Matching
    → Topology Graph
    → Enrichment (OUI / GeoIP)
    → Security Analysis (ATT&CK / CVE / Anomalies)
    → Storage (SQLite)
    → Visualization + Reporting
```

---

## 🛠️ Development

### Run

```bash
npm run web          # full stack (recommended)
npm run web:dev      # frontend only
npm run web:start    # backend only
```

---

### Build

```bash
npm run build
```

---

### Checks

```bash
cargo check
cargo test
npm run check
```

---

## 🌐 Headless Mode (Server Deployment)

Run backend + UI as a web server:

```bash
npm run web:start -- --port 4173
```

* Access via browser
* Server-side file imports
* No desktop environment required

---

## 🧪 Testing

* Rust: `cargo test`
* Frontend: `npm run check`

Test datasets:

* ICS PCAP collections (4SICS, Wireshark samples, etc.)

---

## 🆚 Compared to GRASSMARLIN

Kusanagi Kajiki is a **full rewrite** of [GRASSMARLIN](https://github.com/nsacyber/GRASSMARLIN?utm_source=chatgpt.com) with:

* modern Rust backend
* real protocol parsing (not heuristics only)
* built-in security analysis (ATT&CK, CVE, anomalies)
* reporting + compliance mapping
* web-based UI (no Java Swing)

---

## 📦 Requirements

* Node.js (LTS)
* Rust (stable)
* libpcap (Linux/macOS) or Npcap (Windows)

---

## 🤝 Contributing

Contributions are welcome:

* protocol parsers
* signatures
* test PCAPs
* bug fixes

---

## 📄 License

Apache 2.0 — see `LICENSE`

---

## 🙏 Acknowledgments

* NSA Cybersecurity (GRASSMARLIN concept)
* MITRE ATT&CK for ICS
* CISA ICS tooling and guidance
* IEEE OUI database

---

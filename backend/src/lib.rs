//! # Kusanagi Kajiki (草薙カジキ)
//!
//! Modern ICS/SCADA passive network discovery tool.
//! Successor to the NSA's GRASSMARLIN, rebuilt with Rust and SvelteKit.
//!
//! ## Crate Architecture
//!
//! - `gm-capture` — Packet capture engine (PCAP import + live capture)
//! - `gm-parsers` — ICS protocol identification and deep parsing
//! - `gm-topology` — Network topology graph engine
//! - `gm-db` — SQLite asset persistence

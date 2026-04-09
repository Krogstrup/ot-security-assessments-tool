//! Export use-cases: CSV, JSON, PDF, SBOM, STIX, filtered PCAP, allowlist,
//! topology image, and firewall rule generation.

pub mod allowlist;
pub mod base64;
pub mod file_exports;
pub mod report_builders;

// ─── Public re-exports ────────────────────────────────────────────────────────
pub use allowlist::{
    export_allowlist_csv, export_firewall_rules, generate_communication_allowlist,
};
pub use file_exports::{
    export_assets_csv, export_assets_json, export_connections_csv, export_filtered_pcap,
    export_sbom, export_stix_bundle, export_topology_json, generate_pdf_report,
    save_topology_image, ReportConfigInput,
};

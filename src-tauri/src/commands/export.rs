//! Export & reporting adapter layer.
//!
//! Public types and function signatures are preserved here so that web
//! handlers resolving `commands::export::*` compile unchanged.

pub use crate::application::use_cases::export::{
    export_allowlist_csv, export_assets_csv, export_assets_json, export_connections_csv,
    export_filtered_pcap, export_firewall_rules, export_sbom, export_stix_bundle,
    export_topology_json, generate_communication_allowlist, generate_pdf_report,
    save_topology_image, ReportConfigInput,
};

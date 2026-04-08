//! Vendor-aware context helpers for enforcement config generation.
//!
//! Provides port context remarks and zone→vendor mappings derived from
//! PolicyGroup names that encode a vendor suffix (`L{N}-{role}-{vendor}`).

use std::collections::HashMap;

use crate::{PolicyGroup, Zone};

// ── Vendor-aware context ─────────────────────────────────────────────────────

/// Return vendor-specific port context for known OT vendor + port combinations.
///
/// Used to enrich enforcement config remarks with protocol-level context that
/// helps network engineers understand what each firewall rule actually permits.
pub(super) fn vendor_port_context(vendor: &str, port: u16) -> Option<&'static str> {
    let v = vendor.to_lowercase();
    if v.contains("siemens") {
        match port {
            102 => return Some("S7comm PLC communication"),
            443 => return Some("SCALANCE web management"),
            161 => return Some("SCALANCE SNMP monitoring"),
            34962 => return Some("PROFINET IO"),
            _ => {}
        }
    }
    if v.contains("rockwell") {
        match port {
            44818 => return Some("EtherNet/IP explicit messaging"),
            2222 => return Some("EtherNet/IP I/O"),
            _ => {}
        }
    }
    if v.contains("schneider") && port == 502 {
        return Some("Modbus TCP");
    }
    if v.contains("abb") && port == 502 {
        return Some("Modbus TCP");
    }
    if v.contains("honeywell") && port == 502 {
        return Some("Modbus TCP");
    }
    None
}

/// Extract the vendor suffix from a PolicyGroup auto-generated name.
///
/// Group names follow the pattern `L{N}-{role}` (no vendor) or
/// `L{N}-{role}-{vendor}` (with vendor split). Returns `Some(vendor)` when
/// the name has the three-segment form starting with `L`.
pub(super) fn extract_vendor_from_group_name(name: &str) -> Option<&str> {
    let parts: Vec<&str> = name.splitn(3, '-').collect();
    if parts.len() == 3 && parts[0].starts_with('L') {
        Some(parts[2])
    } else {
        None
    }
}

/// Build a mapping from zone ID → list of vendor names extracted from the
/// PolicyGroups assigned to each zone.
pub(super) fn build_zone_vendors(
    zones: &[Zone],
    groups: &[PolicyGroup],
) -> HashMap<String, Vec<String>> {
    let group_by_id: HashMap<&str, &PolicyGroup> =
        groups.iter().map(|g| (g.id.as_str(), g)).collect();

    let mut zone_vendors: HashMap<String, Vec<String>> = HashMap::new();
    for zone in zones {
        let mut vendors: Vec<String> = Vec::new();
        for gid in &zone.policy_group_ids {
            if let Some(group) = group_by_id.get(gid.as_str()) {
                if let Some(vendor) = extract_vendor_from_group_name(&group.name) {
                    if !vendors.iter().any(|v| v == vendor) {
                        vendors.push(vendor.to_string());
                    }
                }
            }
        }
        zone_vendors.insert(zone.id.clone(), vendors);
    }
    zone_vendors
}

/// Build a vendor context remark for a rule between two zones on a given port.
///
/// Checks both source and destination zone vendors against the port and returns
/// a combined remark string, or `None` if no vendor context applies.
pub(super) fn vendor_remark_for_rule(
    zone_vendors: &HashMap<String, Vec<String>>,
    src_zone_id: &str,
    dst_zone_id: &str,
    port: Option<u16>,
) -> Option<String> {
    let port = port?;
    let empty = Vec::new();
    let src_vendors = zone_vendors.get(src_zone_id).unwrap_or(&empty);
    let dst_vendors = zone_vendors.get(dst_zone_id).unwrap_or(&empty);

    let mut remarks: Vec<String> = Vec::new();
    for vendor in src_vendors.iter().chain(dst_vendors.iter()) {
        if let Some(context) = vendor_port_context(vendor, port) {
            let remark = format!("{vendor} — {context}");
            if !remarks.contains(&remark) {
                remarks.push(remark);
            }
        }
    }

    if remarks.is_empty() {
        None
    } else {
        Some(remarks.join("; "))
    }
}

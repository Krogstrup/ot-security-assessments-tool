//! Shared graph / similarity utilities for identity-group clustering.

use std::collections::{HashMap, HashSet};

use crate::{AssetProfile, Criticality, ObservedConnection, SecurityLevel};

// ── Neighbor sets ─────────────────────────────────────────────────────────────

/// Build per-IP neighbor sets from observed connections (bidirectional).
pub(super) fn build_neighbor_sets(
    connections: &[ObservedConnection],
) -> HashMap<String, HashSet<String>> {
    let mut neighbors: HashMap<String, HashSet<String>> = HashMap::new();
    for conn in connections {
        neighbors
            .entry(conn.src_ip.clone())
            .or_default()
            .insert(conn.dst_ip.clone());
        neighbors
            .entry(conn.dst_ip.clone())
            .or_default()
            .insert(conn.src_ip.clone());
    }
    neighbors
}

// ── Jaccard similarity ────────────────────────────────────────────────────────

/// Jaccard similarity between two neighbor sets.
///
/// Returns 1.0 if both sets are empty (identical empty neighborhoods),
/// and 0.0 if the union is non-empty but the intersection is empty.
pub(super) fn jaccard(a: &HashSet<String>, b: &HashSet<String>) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }
    let intersection = a.intersection(b).count();
    let union_size = a.union(b).count();
    if union_size == 0 {
        1.0
    } else {
        intersection as f64 / union_size as f64
    }
}

/// Average pairwise Jaccard similarity between two groups of IPs.
///
/// Returns 0.0 if either group is empty.
pub(super) fn avg_group_jaccard(
    group_a: &[String],
    group_b: &[String],
    neighbor_sets: &HashMap<String, HashSet<String>>,
) -> f64 {
    let empty: HashSet<String> = HashSet::new();
    let mut total = 0.0f64;
    let mut count = 0usize;

    for ip_a in group_a {
        for ip_b in group_b {
            let na = neighbor_sets.get(ip_a).unwrap_or(&empty);
            let nb = neighbor_sets.get(ip_b).unwrap_or(&empty);
            total += jaccard(na, nb);
            count += 1;
        }
    }

    if count == 0 {
        0.0
    } else {
        total / count as f64
    }
}

// ── Criticality helpers ───────────────────────────────────────────────────────

/// Return the maximum criticality level across a set of member IPs.
pub(super) fn max_criticality_for_ips(ips: &[String], assets: &[AssetProfile]) -> Criticality {
    ips.iter()
        .filter_map(|ip| assets.iter().find(|a| &a.ip == ip))
        .map(|a| parse_criticality(a.criticality.as_deref()))
        .max()
        .unwrap_or(Criticality::Unknown)
}

/// Parse a criticality string from `risk.rs` output into a [`Criticality`] enum.
fn parse_criticality(s: Option<&str>) -> Criticality {
    match s {
        Some("critical") => Criticality::Critical,
        Some("high") => Criticality::High,
        Some("medium") => Criticality::Medium,
        Some("low") => Criticality::Low,
        _ => Criticality::Unknown,
    }
}

// ── Security Level ────────────────────────────────────────────────────────────

/// Map Purdue level to IEC 62443 Security Level.
///
/// - L0/L1 → SL3 (basic control, direct process impact)
/// - L2/L3/L3.5 → SL2 (supervisory access control)
/// - L4+ / unassigned → SL1 (IT network baseline)
pub fn security_level_from_purdue(level: Option<u8>) -> SecurityLevel {
    match level {
        Some(0) | Some(1) => SecurityLevel::Sl3,
        Some(2) | Some(3) => SecurityLevel::Sl2,
        _ => SecurityLevel::Sl1,
    }
}

//! Step 3: Optional vendor-based split within a role sub-group.

use std::collections::{HashMap, HashSet};

use crate::SegmentationInput;

use super::graph;

/// Optionally split a role sub-group by vendor.
///
/// Splits only if the vendor subgroups communicate with disjoint peer sets
/// (average pairwise Jaccard similarity < [`thresholds::VENDOR_SPLIT_JACCARD_THRESHOLD`]).
/// If any vendor pair shares enough neighbors the whole group is kept together.
///
/// Returns a list of `(vendor_label, ips)` pairs. `vendor_label` is empty when
/// no split is performed (group name will be `"L{N}-{role}"`).
pub(super) fn maybe_split_by_vendor(
    ips: &[String],
    input: &SegmentationInput,
    neighbor_sets: &HashMap<String, HashSet<String>>,
) -> Vec<(String, Vec<String>)> {
    if ips.len() < 2 {
        return vec![(String::new(), ips.to_vec())];
    }

    // Group IPs by vendor name.
    let mut by_vendor: HashMap<String, Vec<String>> = HashMap::new();
    for ip in ips {
        let vendor = input
            .assets
            .iter()
            .find(|a| &a.ip == ip)
            .and_then(|a| a.vendor.as_deref())
            .unwrap_or("unknown")
            .to_string();
        by_vendor.entry(vendor).or_default().push(ip.clone());
    }

    // Single vendor bucket — no split possible.
    if by_vendor.len() <= 1 {
        return vec![(String::new(), ips.to_vec())];
    }

    // Check if any vendor pair is "close enough" to keep together.
    // Use a block so the borrows on by_vendor drop before we move it.
    let should_split = {
        let vendors: Vec<(&String, &Vec<String>)> = by_vendor.iter().collect();
        let mut split = true;
        'outer: for i in 0..vendors.len() {
            for j in (i + 1)..vendors.len() {
                if graph::avg_group_jaccard(vendors[i].1, vendors[j].1, neighbor_sets)
                    >= crate::thresholds::VENDOR_SPLIT_JACCARD_THRESHOLD
                {
                    split = false;
                    break 'outer;
                }
            }
        }
        split
    };

    if should_split {
        by_vendor.into_iter().collect()
    } else {
        vec![(String::new(), ips.to_vec())]
    }
}

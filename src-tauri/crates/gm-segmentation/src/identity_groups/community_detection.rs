//! Step 4: Greedy community detection for assets without a Purdue level.

use std::collections::{HashMap, HashSet};

use crate::{AssetProfile, DeviceCategory, PolicyGroup, SecurityLevel, SegmentationInput};

use super::graph;

/// Greedy community detection for assets without a Purdue level assignment.
///
/// Iterates over unassigned assets. For each asset, computes Jaccard similarity
/// of its neighbor set against each existing community. If the best match
/// exceeds [`thresholds::COMMUNITY_MERGE_JACCARD_THRESHOLD`], the asset joins
/// that community; otherwise it starts a new one.
pub(super) fn detect_communities(
    unassigned: &[&AssetProfile],
    input: &SegmentationInput,
    neighbor_sets: &HashMap<String, HashSet<String>>,
) -> Vec<PolicyGroup> {
    if unassigned.is_empty() {
        return Vec::new();
    }

    let mut communities: Vec<Vec<String>> = Vec::new();

    for asset in unassigned {
        let ip = &asset.ip;
        let single = std::slice::from_ref(ip);

        let mut best_match: Option<usize> = None;
        // threshold — must exceed (not equal) COMMUNITY_MERGE_JACCARD_THRESHOLD
        let mut best_sim = crate::thresholds::COMMUNITY_MERGE_JACCARD_THRESHOLD;

        for (idx, members) in communities.iter().enumerate() {
            let sim = graph::avg_group_jaccard(single, members, neighbor_sets);
            if sim > best_sim {
                best_sim = sim;
                best_match = Some(idx);
            }
        }

        if let Some(idx) = best_match {
            communities[idx].push(ip.clone());
        } else {
            communities.push(vec![ip.clone()]);
        }
    }

    communities
        .into_iter()
        .enumerate()
        .map(|(idx, ips)| {
            let has_ot_neighbor = ips.iter().any(|ip| {
                if let Some(neighbors) = neighbor_sets.get(ip) {
                    neighbors
                        .iter()
                        .any(|nb| input.assets.iter().any(|a| &a.ip == nb && a.is_ot))
                } else {
                    false
                }
            });

            let category = if has_ot_neighbor {
                DeviceCategory::NetworkInfra
            } else {
                DeviceCategory::Unknown
            };

            let criticality = graph::max_criticality_for_ips(&ips, &input.assets);

            PolicyGroup::new(
                format!("Unclassified-{}", idx + 1),
                ips,
                None,
                category,
                SecurityLevel::Sl1,
                criticality,
            )
        })
        .collect()
}

//! Segmentation engine tuning constants.
//!
//! All magic numbers used across the gm-segmentation crate live here so they
//! have a single source of truth and can be tuned without hunting through logic.

// ── Phase 15A: Identity Groups ────────────────────────────────────────────────

/// Jaccard similarity threshold for the **vendor split** decision (Step 3).
///
/// If the average pairwise Jaccard similarity between any two vendor sub-groups
/// meets or exceeds this value they share enough peer overlap to stay in one
/// PolicyGroup. If every pair falls below it the group is split by vendor.
pub const VENDOR_SPLIT_JACCARD_THRESHOLD: f64 = 0.3;

/// Jaccard similarity threshold for **community merge** (Step 4).
///
/// An unassigned asset joins an existing community only when its neighbor-set
/// Jaccard similarity against that community *exceeds* (strictly greater than)
/// this value. If no community qualifies a new community is started.
pub const COMMUNITY_MERGE_JACCARD_THRESHOLD: f64 = 0.6;

// ── Phase 15B: Zones ──────────────────────────────────────────────────────────

/// Percentage of assets on the same /24 subnet above which the network is
/// considered **flat** (integer, compared with `>`).
///
/// Example: 80 means >80 % of assets on one /24 triggers a flat-network flag.
pub const FLAT_NETWORK_SUBNET_PERCENT: usize = 80;

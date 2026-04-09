//! Detection threshold constants for analysis modules.
//!
//! All magic numbers that control detection sensitivity live here.
//! Changing a threshold in one place propagates to every detection that uses it.

// ── T0806 Brute Force I/O ────────────────────────────────────────────────────

/// Minimum write-class command count from a single source to a single OT target
/// before flagging as T0806 (Brute Force I/O).
pub(crate) const WRITE_RATE_THRESHOLD: u64 = 500;

// ── T0802 Automated Collection ───────────────────────────────────────────────

/// Minimum number of distinct OT targets polled by a single source
/// before flagging as T0802 (Automated Collection).
pub(crate) const AUTOMATED_COLLECTION_TARGET_THRESHOLD: usize = 10;

// ── T0861 Point and Tag Identification ───────────────────────────────────────

/// Minimum distinct Modbus unit IDs read by a single master
/// before flagging as T0861 (Point and Tag Identification).
pub(crate) const UNIT_ID_SCAN_THRESHOLD: usize = 5;

// ── T0840 Network Connection Enumeration ─────────────────────────────────────

/// Minimum distinct OT ports or hosts contacted by a single source
/// before flagging as T0840 (Network Connection Enumeration).
pub(crate) const PORT_SWEEP_THRESHOLD: usize = 10;

// ── T0864 Transient Cyber Asset ───────────────────────────────────────────────

/// Maximum presence duration in seconds for a device to be considered transient
/// for T0864 detection. Devices visible for less than this time that communicate
/// with OT assets are flagged. (5 minutes)
pub(crate) const TRANSIENT_ASSET_SECS: f64 = 300.0;

// ── Polling anomaly detection (anomaly.rs) ───────────────────────────────────

/// Coefficient of variation (CV = range/avg) above which polling deviation is
/// flagged as an `AnomalyScore`.
pub(crate) const POLLING_CV_THRESHOLD: f64 = 0.5;

/// CV above this level upgrades the anomaly to High severity and 0.9 confidence.
pub(crate) const POLLING_CV_HIGH_THRESHOLD: f64 = 2.0;

/// CV above this level generates a `Finding` in addition to an `AnomalyScore`.
pub(crate) const POLLING_CV_FINDING_THRESHOLD: f64 = 1.0;

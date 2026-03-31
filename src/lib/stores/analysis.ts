import type { AnalysisSummary, BaselineDiff, Finding, PurdueAssignment } from '$lib/types/analysis';
import type { SegmentationReport } from '$lib/types/segmentation';
import { writable, derived } from 'svelte/store';
import type { AnomalyScore } from '$lib/types/analysis';

/** Security findings from last analysis run */
export const findings = writable<Finding[]>([]);

/** Purdue level assignments from last analysis run */
export const purdueAssignments = writable<PurdueAssignment[]>([]);

/** Anomaly scores from last analysis run */
export const anomalies = writable<AnomalyScore[]>([]);

/** Analysis summary from last analysis run */
export const analysisSummary = writable<AnalysisSummary | null>(null);

/** Last computed baseline diff (null if not run) */
export const baselineDiff = writable<BaselineDiff | null>(null);

/** Last computed segmentation report (null if not run) */
export const segmentationReport = writable<SegmentationReport | null>(null);

/** Set of IPs that are "new" in the baseline diff (for LogicalView highlighting) */
export const driftNewIps = derived(baselineDiff, ($diff) => {
	if (!$diff) return new Set<string>();
	return new Set($diff.new_assets.map((a) => a.ip_address));
});

/** Set of IPs that are "missing" in the baseline diff */
export const driftMissingIps = derived(baselineDiff, ($diff) => {
	if (!$diff) return new Set<string>();
	return new Set($diff.missing_assets.map((a) => a.ip_address));
});

/** Set of IPs that are "changed" in the baseline diff */
export const driftChangedIps = derived(baselineDiff, ($diff) => {
	if (!$diff) return new Set<string>();
	return new Set($diff.changed_assets.map((a) => a.ip_address));
});

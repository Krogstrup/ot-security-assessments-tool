import type { GroupingMode } from '$lib/types/topology';
import { DEVICE_COLORS, DEVICE_LABELS } from '$lib/utils/graph';

export const FCOSE_NODE_LIMIT = 2000;

export const groupingOptions: { mode: GroupingMode; label: string }[] = [
	{ mode: 'subnet', label: 'Subnet (/24)' },
	{ mode: 'protocol', label: 'Protocol' },
	{ mode: 'device_role', label: 'Device Role' },
	{ mode: 'vendor', label: 'Vendor' },
	{ mode: 'none', label: 'None (Flat)' }
];

export function getLogicalLegendEntries() {
	return Object.entries(DEVICE_COLORS).map(([type, color]) => ({
		label: DEVICE_LABELS[type] ?? type,
		color
	}));
}

import type { FindingSeverity } from '$lib/types/analysis';

export const severityOrder: FindingSeverity[] = ['critical', 'high', 'medium', 'low', 'info'];

export const severityColors: Record<FindingSeverity, string> = {
	critical: 'var(--gm-severity-critical)',
	high: 'var(--gm-severity-high)',
	medium: 'var(--gm-severity-medium)',
	low: 'var(--gm-severity-low)',
	info: 'var(--gm-severity-info)'
};

export const purdueColors: Record<number, string> = {
	0: 'var(--gm-purdue-l0)',
	1: 'var(--gm-purdue-l1)',
	2: 'var(--gm-purdue-l2)',
	3: 'var(--gm-purdue-l3)',
	4: 'var(--gm-purdue-l4)',
	5: 'var(--gm-purdue-l5)'
};

export function getSeverityIcon(severity: FindingSeverity): string {
	switch (severity) {
		case 'critical':
			return '!!';
		case 'high':
			return '!';
		case 'medium':
			return '~';
		case 'low':
			return '-';
		case 'info':
			return 'i';
	}
}

export function getTypeLabel(type: string): string {
	switch (type) {
		case 'attack_technique':
			return 'ATT&CK';
		case 'purdue_violation':
			return 'Purdue';
		case 'anomaly':
			return 'Anomaly';
		default:
			return type;
	}
}

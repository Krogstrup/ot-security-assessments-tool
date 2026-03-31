import type { SimulationResult } from '$lib/types/segmentation';

export const SEGMENTATION_LOADING_STAGES = [
	'Clustering assets into policy groups...',
	'Generating IEC 62443 zone boundaries...',
	'Computing least-privilege matrix...',
	'Generating enforcement configs...',
	'Running policy simulation...'
] as const;

export function slLabel(sl: string): string {
	const map: Record<string, string> = {
		sl1: 'SL1',
		sl2: 'SL2',
		sl3: 'SL3',
		sl4: 'SL4'
	};
	return map[sl] ?? sl.toUpperCase();
}

export function slClass(sl: string): string {
	const map: Record<string, string> = {
		sl1: 'sl1',
		sl2: 'sl2',
		sl3: 'sl3',
		sl4: 'sl4'
	};
	return map[sl] ?? '';
}

export function riskClass(risk: string): string {
	if (risk === 'high') return 'high';
	if (risk === 'medium') return 'medium';
	return 'low';
}

export function scorePercent(value: number): string {
	return `${(value * 100).toFixed(1)}%`;
}

export function blockedPercent(result: SimulationResult): string {
	return `${result.blocked_percent.toFixed(1)}%`;
}

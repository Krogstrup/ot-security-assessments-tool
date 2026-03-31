import type { ConnectionStats, PatternAnomaly } from '$lib/types/analysis';

export type PatternsSortColumn = keyof ConnectionStats;

export function rowKey(stat: ConnectionStats): string {
	return `${stat.src_ip}|${stat.dst_ip}|${stat.port}|${stat.protocol}`;
}

export function buildAnomalyMap(anomalies: PatternAnomaly[]): Map<string, PatternAnomaly[]> {
	return anomalies.reduce((map, anomaly) => {
		const key = `${anomaly.src_ip}|${anomaly.dst_ip}|${anomaly.port}`;
		const existing = map.get(key);
		if (existing) {
			existing.push(anomaly);
		} else {
			map.set(key, [anomaly]);
		}
		return map;
	}, new Map<string, PatternAnomaly[]>());
}

export function anomalyCountForRow(
	stat: ConnectionStats,
	anomalyMap: Map<string, PatternAnomaly[]>
): number {
	return (anomalyMap.get(`${stat.src_ip}|${stat.dst_ip}|${stat.port}`) ?? []).length;
}

export function anomaliesForRow(
	stat: ConnectionStats,
	anomalyMap: Map<string, PatternAnomaly[]>
): PatternAnomaly[] {
	return anomalyMap.get(`${stat.src_ip}|${stat.dst_ip}|${stat.port}`) ?? [];
}

export function listProtocols(stats: ConnectionStats[]): string[] {
	return [...new Set(stats.map((stat) => stat.protocol))].sort();
}

export function sortStats(
	stats: ConnectionStats[],
	sortCol: PatternsSortColumn,
	sortAsc: boolean
): ConnectionStats[] {
	return stats.slice().sort((left, right) => {
		const leftValue = left[sortCol];
		const rightValue = right[sortCol];
		if (typeof leftValue === 'number' && typeof rightValue === 'number') {
			return sortAsc ? leftValue - rightValue : rightValue - leftValue;
		}
		const leftText = String(leftValue);
		const rightText = String(rightValue);
		return sortAsc ? leftText.localeCompare(rightText) : rightText.localeCompare(leftText);
	});
}

export function fmtBytes(bytes: number): string {
	if (bytes < 1024) return `${bytes} B`;
	if (bytes < 1048576) return `${(bytes / 1024).toFixed(1)} KB`;
	return `${(bytes / 1048576).toFixed(1)} MB`;
}

export function fmtDuration(seconds: number): string {
	if (seconds < 1) return `${(seconds * 1000).toFixed(0)} ms`;
	if (seconds < 60) return `${seconds.toFixed(1)} s`;
	return `${(seconds / 60).toFixed(1)} min`;
}

export function severityColor(severity: string): string {
	switch (severity) {
		case 'critical':
			return '#ef4444';
		case 'high':
			return '#f97316';
		case 'medium':
			return '#f59e0b';
		default:
			return '#64748b';
	}
}

export function anomalyLabel(type: string): string {
	switch (type) {
		case 'one_off_connection':
			return 'One-off';
		case 'high_frequency':
			return 'High freq';
		case 'irregular_polling':
			return 'Irregular';
		case 'burst_traffic':
			return 'Burst';
		default:
			return type;
	}
}

export function buildHistogram(
	stat: ConnectionStats
): { bins: number[]; edges: number[]; maxCount: number } {
	if (stat.packet_count < 2 || stat.max_interval_ms <= 0) {
		return { bins: [], edges: [], maxCount: 0 };
	}

	const numBins = 10;
	const min = stat.min_interval_ms;
	const max = stat.max_interval_ms;
	const step = (max - min) / numBins || 1;
	const bins = new Array<number>(numBins).fill(0);
	const mean = stat.avg_interval_ms;
	const std = stat.std_interval_ms || 0.001;
	const sampleCount = Math.min(stat.packet_count - 1, 200);

	for (let index = 0; index < sampleCount; index++) {
		const u1 = Math.random();
		const u2 = Math.random();
		const z = Math.sqrt(-2 * Math.log(u1)) * Math.cos(2 * Math.PI * u2);
		const value = mean + std * z;
		const binIndex = Math.floor((value - min) / step);
		if (binIndex >= 0 && binIndex < numBins) bins[binIndex]++;
	}

	const maxCount = Math.max(...bins, 1);
	const edges = Array.from({ length: numBins + 1 }, (_, index) => min + index * step);
	return { bins, edges, maxCount };
}

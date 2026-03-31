import type { ConnectionStats } from '$lib/types/analysis';

export const PATTERNS_TABLE_COLUMNS: Array<[keyof ConnectionStats, string]> = [
	['src_ip', 'Source IP'],
	['dst_ip', 'Destination IP'],
	['protocol', 'Protocol'],
	['port', 'Port'],
	['packet_count', 'Packets'],
	['byte_count', 'Bytes'],
	['duration_secs', 'Duration'],
	['avg_interval_ms', 'Avg Interval'],
	['std_interval_ms', 'Std Dev'],
	['is_periodic', 'Periodic'],
	['packets_per_sec', 'Pkt/s']
];

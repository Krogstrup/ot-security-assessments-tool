/**
 * Network connections, patterns, and statistics.
 */

import type { ConnectionStats, PatternAnomaly } from '$lib/types/analysis';
import type { PacketSummary } from '$lib/types/connections';
import type { RedundancyInfo } from '$lib/types/deep-parse';
import type { DataCounts } from '$lib/types/pagination';
import type { ProtocolStats } from '$lib/types/protocols';
import { invokeCompat, httpJson } from './core';
import type { ConnectionPage } from '$lib/types';

export async function getConnections(page = 0, pageSize = 500, sortBy?: string): Promise<ConnectionPage> {
	const params = new URLSearchParams({
		page: String(page),
		pageSize: String(pageSize)
	});
	if (sortBy) params.set('sortBy', sortBy);
	return httpJson<ConnectionPage>(`/api/data/connections?${params.toString()}`);
}

export async function getDataCounts(): Promise<DataCounts> {
	return httpJson<DataCounts>('/api/data/counts');
}

export async function getConnectionPackets(connectionId: string): Promise<PacketSummary[]> {
	return httpJson<PacketSummary[]>(
		`/api/data/connection-packets/${encodeURIComponent(connectionId)}`
	);
}

export async function getProtocolStats(): Promise<ProtocolStats[]> {
	return httpJson<ProtocolStats[]>('/api/data/protocol-stats');
}

export async function getConnectionStats(): Promise<ConnectionStats[]> {
	return invokeCompat<ConnectionStats[]>('get_connection_stats');
}

export async function getPatternAnomalies(): Promise<PatternAnomaly[]> {
	return invokeCompat<PatternAnomaly[]>('get_pattern_anomalies');
}

export async function getRedundancyProtocols(): Promise<RedundancyInfo[]> {
	return invokeCompat<RedundancyInfo[]>('get_redundancy_protocols');
}

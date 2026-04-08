/**
 * Network connections, patterns, and statistics.
 */

import type { ConnectionStats, PatternAnomaly } from '$lib/types/analysis';
import type { PacketSummary } from '$lib/types/connections';
import type { RedundancyInfo } from '$lib/types/deep-parse';
import type { DataCounts } from '$lib/types/pagination';
import type { ProtocolStats } from '$lib/types/protocols';
import { httpJson } from './core';
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
	return httpJson<ConnectionStats[]>('/api/v1/patterns/connection-stats');
}

export async function getPatternAnomalies(): Promise<PatternAnomaly[]> {
	return httpJson<PatternAnomaly[]>('/api/v1/patterns/anomalies');
}

export async function getRedundancyProtocols(): Promise<RedundancyInfo[]> {
	return httpJson<RedundancyInfo[]>('/api/v1/patterns/redundancy-protocols');
}

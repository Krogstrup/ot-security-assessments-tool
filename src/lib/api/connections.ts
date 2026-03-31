/**
 * Network connections, patterns, and statistics.
 */

import { invokeCompat, httpJson, isTauriRuntime } from './core';
import type { ConnectionPage, DataCounts, PacketSummary, ProtocolStats, ConnectionStats, PatternAnomaly, RedundancyInfo } from '/types';

export async function getConnections(page = 0, pageSize = 500, sortBy?: string): Promise<ConnectionPage> {
	if (!isTauriRuntime()) {
		const params = new URLSearchParams({
			page: String(page),
			pageSize: String(pageSize)
		});
		if (sortBy) params.set('sortBy', sortBy);
		return httpJson<ConnectionPage>(`/api/data/connections?${params.toString()}`);
	}
	return invokeCompat<ConnectionPage>('get_connections', {
		page,
		pageSize,
		sortBy: sortBy ?? null
	});
}

export async function getDataCounts(): Promise<DataCounts> {
	if (!isTauriRuntime()) {
		return httpJson<DataCounts>('/api/data/counts');
	}
	return invokeCompat<DataCounts>('get_data_counts');
}

export async function getConnectionPackets(connectionId: string): Promise<PacketSummary[]> {
	if (!isTauriRuntime()) {
		return httpJson<PacketSummary[]>(
			`/api/data/connection-packets/${encodeURIComponent(connectionId)}`
		);
	}
	return invokeCompat<PacketSummary[]>('get_connection_packets', { connectionId });
}

export async function getProtocolStats(): Promise<ProtocolStats[]> {
	if (!isTauriRuntime()) {
		return httpJson<ProtocolStats[]>('/api/data/protocol-stats');
	}
	return invokeCompat<ProtocolStats[]>('get_protocol_stats');
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

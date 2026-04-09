/**
 * Network connections, patterns, and statistics.
 */

import type { ConnectionStats, PatternAnomaly } from '$lib/types/analysis';
import type { PacketSummary } from '$lib/types/connections';
import type { RedundancyInfo } from '$lib/types/deep-parse';
import type { DataCounts } from '$lib/types/pagination';
import type { ProtocolStats } from '$lib/types/protocols';
import { z } from 'zod';
import {
	connectionPageSchema,
	dataCountsSchema,
	protocolStatSchema
} from '$lib/schemas';
import { httpValidated } from './core';
import type { ConnectionPage } from '$lib/types';
import {
	DEFAULT_CONNECTION_PAGE_SIZE,
	type ConnectionSortBy,
	type ProtocolStatsSortBy
} from './contracts';

export async function getConnections(
	page = 0,
	pageSize = DEFAULT_CONNECTION_PAGE_SIZE,
	sortBy?: ConnectionSortBy
): Promise<ConnectionPage> {
	const params = new URLSearchParams({
		page: String(page),
		pageSize: String(pageSize)
	});
	if (sortBy) params.set('sortBy', sortBy);
	return httpValidated(connectionPageSchema, `/api/data/connections?${params.toString()}`) as Promise<ConnectionPage>;
}

export async function getDataCounts(): Promise<DataCounts> {
	return httpValidated(dataCountsSchema, '/api/data/counts');
}

export async function getConnectionPackets(connectionId: string): Promise<PacketSummary[]> {
	return httpValidated(
		z.unknown(),
		`/api/data/connection-packets/${encodeURIComponent(connectionId)}`
	) as Promise<PacketSummary[]>;
}

export async function getProtocolStats(sortBy?: ProtocolStatsSortBy): Promise<ProtocolStats[]> {
	const params = new URLSearchParams();
	if (sortBy) params.set('sortBy', sortBy);
	const query = params.toString();
	return httpValidated(
		z.array(protocolStatSchema),
		query ? `/api/data/protocol-stats?${query}` : '/api/data/protocol-stats'
	) as Promise<ProtocolStats[]>;
}

export async function getConnectionStats(): Promise<ConnectionStats[]> {
	return httpValidated(z.unknown(), '/api/v1/patterns/connection-stats') as Promise<ConnectionStats[]>;
}

export async function getPatternAnomalies(): Promise<PatternAnomaly[]> {
	return httpValidated(z.unknown(), '/api/v1/patterns/anomalies') as Promise<PatternAnomaly[]>;
}

export async function getRedundancyProtocols(): Promise<RedundancyInfo[]> {
	return httpValidated(z.unknown(), '/api/v1/patterns/redundancy-protocols') as Promise<RedundancyInfo[]>;
}

/**
 * Asset discovery, queries, and updates.
 */

import type { Asset } from '$lib/types/assets';
import type { DeepParseInfo, FunctionCodeStat } from '$lib/types/deep-parse';
import type { AssetUpdate } from '$lib/types/operations';
import type { TopologyGraph } from '$lib/types/topology';
import { httpJson } from './core';
import type { AssetPage } from '$lib/types';
import { DEFAULT_ASSET_PAGE_SIZE, type AssetSortBy } from './contracts';

export async function getAssets(
	page = 0,
	pageSize = DEFAULT_ASSET_PAGE_SIZE,
	sortBy?: AssetSortBy
): Promise<AssetPage> {
	const params = new URLSearchParams({
		page: String(page),
		pageSize: String(pageSize)
	});
	if (sortBy) params.set('sortBy', sortBy);
	return httpJson<AssetPage>(`/api/data/assets?${params.toString()}`);
}

export async function updateAsset(assetId: string, updates: AssetUpdate): Promise<Asset> {
	return httpJson<Asset>(`/api/v1/assets/${encodeURIComponent(assetId)}`, {
		method: 'PUT',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ updates })
	});
}

export async function bulkUpdateAssets(assetIds: string[], updates: AssetUpdate): Promise<number> {
	return httpJson<number>('/api/v1/assets/bulk-update', {
		method: 'PUT',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ assetIds, updates })
	});
}

export async function getDeepParseInfo(ipAddress: string): Promise<DeepParseInfo | null> {
	return httpJson<DeepParseInfo | null>(`/api/v1/data/deep-parse/${encodeURIComponent(ipAddress)}`);
}

export async function getFunctionCodeStats(): Promise<Record<string, FunctionCodeStat[]>> {
	return httpJson<Record<string, FunctionCodeStat[]>>('/api/v1/data/function-code-stats');
}

export async function getTopology(): Promise<TopologyGraph> {
	return httpJson<TopologyGraph>('/api/data/topology');
}

/**
 * Asset discovery, queries, and updates.
 */

import { invokeCompat, httpJson, isTauriRuntime } from './core';
import type { AssetPage, AssetUpdate, Asset, DeepParseInfo, FunctionCodeStat, TopologyGraph } from '/types';

export async function getAssets(page = 0, pageSize = 200, sortBy?: string): Promise<AssetPage> {
	if (!isTauriRuntime()) {
		const params = new URLSearchParams({
			page: String(page),
			pageSize: String(pageSize)
		});
		if (sortBy) params.set('sortBy', sortBy);
		return httpJson<AssetPage>(`/api/data/assets?${params.toString()}`);
	}
	return invokeCompat<AssetPage>('get_assets', {
		page,
		pageSize,
		sortBy: sortBy ?? null
	});
}

export async function updateAsset(assetId: string, updates: AssetUpdate): Promise<Asset> {
	return invokeCompat<Asset>('update_asset', { assetId, updates });
}

export async function bulkUpdateAssets(assetIds: string[], updates: AssetUpdate): Promise<number> {
	return invokeCompat<number>('bulk_update_assets', { assetIds, updates });
}

export async function getDeepParseInfo(ipAddress: string): Promise<DeepParseInfo | null> {
	return invokeCompat<DeepParseInfo | null>('get_deep_parse_info', { ipAddress });
}

export async function getFunctionCodeStats(): Promise<Record<string, FunctionCodeStat[]>> {
	return invokeCompat<Record<string, FunctionCodeStat[]>>('get_function_code_stats');
}

export async function getTopology(): Promise<TopologyGraph> {
	if (!isTauriRuntime()) {
		return httpJson<TopologyGraph>('/api/data/topology');
	}
	return invokeCompat<TopologyGraph>('get_topology');
}

/**
 * Asset discovery, queries, and updates.
 */

import type { Asset } from '$lib/types/assets';
import type { DeepParseInfo, FunctionCodeStat } from '$lib/types/deep-parse';
import type { AssetUpdate } from '$lib/types/operations';
import type { TopologyGraph } from '$lib/types/topology';
import { z } from 'zod';
import { assetPageSchema, assetSchema } from '$lib/schemas';
import { httpJson, httpValidated } from './core';
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
	return httpValidated(assetPageSchema, `/api/data/assets?${params.toString()}`) as Promise<AssetPage>;
}

export async function updateAsset(assetId: string, updates: AssetUpdate): Promise<Asset> {
	return httpValidated(assetSchema, `/api/v1/assets/${encodeURIComponent(assetId)}`, {
		method: 'PUT',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ updates })
	}) as Promise<Asset>;
}

export async function bulkUpdateAssets(assetIds: string[], updates: AssetUpdate): Promise<number> {
	return httpValidated(z.number(), '/api/v1/assets/bulk-update', {
		method: 'PUT',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ assetIds, updates })
	});
}

export async function getDeepParseInfo(ipAddress: string): Promise<DeepParseInfo | null> {
	return httpValidated(
		z.unknown(),
		`/api/v1/data/deep-parse/${encodeURIComponent(ipAddress)}`
	) as Promise<DeepParseInfo | null>;
}

export async function getFunctionCodeStats(): Promise<Record<string, FunctionCodeStat[]>> {
	return httpValidated(z.unknown(), '/api/v1/data/function-code-stats') as Promise<
		Record<string, FunctionCodeStat[]>
	>;
}

export async function getTopology(): Promise<TopologyGraph> {
	return httpValidated(z.unknown(), '/api/data/topology') as Promise<TopologyGraph>;
}

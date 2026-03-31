import { getAssets } from '$lib/api';
import { assetCount, assets } from '$lib/stores/core';
import type { Asset } from '$lib/types/assets';
import type { AssetUpdate } from '$lib/types/operations';

export interface AssetEditFields {
	deviceType: string;
	hostname: string;
	notes: string;
	purdueLevel: number | null;
	tags: string;
}

export function createAssetEditFields(asset: Asset): AssetEditFields {
	return {
		deviceType: asset.device_type,
		hostname: asset.hostname ?? '',
		notes: asset.notes,
		purdueLevel: asset.purdue_level ?? null,
		tags: asset.tags.join(', ')
	};
}

export function parseTagInput(input: string): string[] {
	return input
		.split(',')
		.map((tag) => tag.trim())
		.filter(Boolean);
}

export function buildAssetEditUpdate(asset: Asset, fields: AssetEditFields): AssetUpdate | null {
	const updates: AssetUpdate = {};

	if (fields.deviceType !== asset.device_type) updates.device_type = fields.deviceType;
	if (fields.hostname !== (asset.hostname ?? '')) updates.hostname = fields.hostname;
	if (fields.notes !== asset.notes) updates.notes = fields.notes;

	const nextPurdue = fields.purdueLevel ?? 255;
	const prevPurdue = asset.purdue_level ?? 255;
	if (nextPurdue !== prevPurdue) updates.purdue_level = fields.purdueLevel ?? 255;

	const nextTags = parseTagInput(fields.tags);
	if (JSON.stringify(nextTags) !== JSON.stringify(asset.tags)) updates.tags = nextTags;

	return Object.keys(updates).length > 0 ? updates : null;
}

export function buildBulkAssetUpdate(fields: {
	deviceType: string;
	purdueLevel: string;
	tag: string;
	notes: string;
}): AssetUpdate | null {
	const updates: AssetUpdate = {};
	if (fields.deviceType) updates.device_type = fields.deviceType;
	if (fields.purdueLevel) updates.purdue_level = parseInt(fields.purdueLevel, 10);
	if (fields.tag.trim()) updates.tags = [fields.tag.trim()];
	if (fields.notes.trim()) updates.notes = fields.notes.trim();
	return Object.keys(updates).length > 0 ? updates : null;
}

export async function refreshAssetsStore(pageSize = 200) {
	const page = await getAssets(0, pageSize);
	assets.set(page.assets);
	assetCount.set(page.total);
}

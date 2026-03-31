import type { Asset } from '$lib/types/assets';

export type AssetColKey =
	| 'ip'
	| 'mac'
	| 'type'
	| 'confidence'
	| 'vendor'
	| 'oui'
	| 'product'
	| 'protocols'
	| 'country'
	| 'packets'
	| 'purdue'
	| 'first_seen'
	| 'last_seen';

export const INVENTORY_PAGE_SIZE = 50;

export const INVENTORY_COLUMNS: Array<{ key: AssetColKey; label: string }> = [
	{ key: 'ip', label: 'IP Address' },
	{ key: 'mac', label: 'MAC Address' },
	{ key: 'type', label: 'Type' },
	{ key: 'confidence', label: 'Confidence' },
	{ key: 'vendor', label: 'Vendor' },
	{ key: 'oui', label: 'OUI' },
	{ key: 'product', label: 'Product' },
	{ key: 'protocols', label: 'Protocols' },
	{ key: 'country', label: 'Country' },
	{ key: 'packets', label: 'Packets' },
	{ key: 'purdue', label: 'Purdue Level' },
	{ key: 'first_seen', label: 'First Seen' },
	{ key: 'last_seen', label: 'Last Seen' }
];

export const DEFAULT_VISIBLE_COLUMNS: AssetColKey[] = ['ip', 'mac', 'type', 'confidence', 'vendor', 'oui'];

export function compareIps(a: string, b: string): number {
	const pa = a.split('.').map(Number);
	const pb = b.split('.').map(Number);
	for (let i = 0; i < 4; i++) {
		if ((pa[i] || 0) !== (pb[i] || 0)) {
			return (pa[i] || 0) - (pb[i] || 0);
		}
	}
	return 0;
}

export function countryFlagEmoji(code: string): string {
	const base = 0x1f1e6;
	const a = code.charCodeAt(0) - 65;
	const b = code.charCodeAt(1) - 65;
	return String.fromCodePoint(base + a) + String.fromCodePoint(base + b);
}

export function sortAssetsByColumn(
	list: Asset[],
	sortColumn: AssetColKey | '',
	sortDirection: 'asc' | 'desc'
): Asset[] {
	if (!sortColumn) return [...list];

	const sorted = [...list];
	const dir = sortDirection === 'asc' ? 1 : -1;
	sorted.sort((a, b) => {
		switch (sortColumn) {
			case 'ip':
				return dir * compareIps(a.ip_address, b.ip_address);
			case 'mac':
				return dir * (a.mac_address ?? '').localeCompare(b.mac_address ?? '');
			case 'type':
				return dir * a.device_type.localeCompare(b.device_type);
			case 'confidence':
				return dir * (a.confidence - b.confidence);
			case 'vendor':
				return dir * (a.vendor ?? '').localeCompare(b.vendor ?? '');
			case 'oui':
				return dir * (a.oui_vendor ?? '').localeCompare(b.oui_vendor ?? '');
			case 'product':
				return dir * (a.product_family ?? '').localeCompare(b.product_family ?? '');
			case 'protocols':
				return dir * a.protocols.join(',').localeCompare(b.protocols.join(','));
			case 'country':
				return dir * (a.country ?? '').localeCompare(b.country ?? '');
			case 'packets':
				return dir * (a.packet_count - b.packet_count);
			case 'purdue':
				return dir * ((a.purdue_level ?? -1) - (b.purdue_level ?? -1));
			case 'first_seen':
				return dir * a.first_seen.localeCompare(b.first_seen);
			case 'last_seen':
				return dir * a.last_seen.localeCompare(b.last_seen);
			default:
				return 0;
		}
	});
	return sorted;
}

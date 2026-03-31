import type { Connection, ConnectionTreeNode } from '$lib/types/connections';
import type { IcsProtocol, ProtocolStats } from '$lib/types/protocols';
import type { SignatureSummary } from '$lib/types/signatures';
import type { TopologyGraph } from '$lib/types/topology';
import { writable, derived } from 'svelte/store';
import type {
	Asset } from '$lib/types';

/** Discovered assets (devices on the network) */
export const assets = writable<Asset[]>([]);

/** Observed connections between assets */
export const connections = writable<Connection[]>([]);

/** Network topology graph for visualization */
export const topology = writable<TopologyGraph>({ nodes: [], edges: [] });

/** Protocol statistics */
export const protocolStats = writable<ProtocolStats[]>([]);

/** Loaded signature information */
export const signatureSummary = writable<SignatureSummary>({ total_count: 0, signatures: [] });

/** Currently selected asset ID (for detail panel) */
export const selectedAssetId = writable<string | null>(null);

/** Search/filter text for asset inventory */
export const assetFilter = writable<string>('');

/** Protocol filter (null = show all) */
export const protocolFilter = writable<IcsProtocol | null>(null);

/** Total count of discovered assets — fed by getDataCounts() or manual set */
export const assetCount = writable<number>(0);

/** Total count of connections — fed by getDataCounts() or manual set */
export const connectionCount = writable<number>(0);

/** Currently selected asset object */
export const selectedAsset = derived([assets, selectedAssetId], ([$assets, $selectedAssetId]) => {
	if (!$selectedAssetId) return null;
	return $assets.find((a) => a.id === $selectedAssetId) ?? null;
});

/** Filtered assets based on search text and protocol filter */
export const filteredAssets = derived([assets, assetFilter, protocolFilter], ([$assets, $filter, $protocol]) => {
	let result = $assets;

	if ($filter) {
		const lower = $filter.toLowerCase();
		result = result.filter(
			(a) =>
				a.ip_address.includes(lower) ||
				a.mac_address?.toLowerCase().includes(lower) ||
				a.hostname?.toLowerCase().includes(lower) ||
				a.vendor?.toLowerCase().includes(lower) ||
				a.device_type.includes(lower) ||
				a.notes.toLowerCase().includes(lower)
		);
	}

	if ($protocol) {
		result = result.filter((a) => a.protocols.includes($protocol));
	}

	return result;
});

/** OT-specific assets only (excludes IT devices) */
export const otAssets = derived(assets, ($assets) =>
	$assets.filter((a) => a.device_type !== 'it_device' && a.device_type !== 'unknown')
);

/** Connection tree: groups connections by source IP, with asset metadata */
export const connectionTree = derived([assets, connections], ([$assets, $connections]): ConnectionTreeNode[] => {
	if ($connections.length === 0) return [];

	// Build a map of IP → asset info for quick lookup
	const assetMap = new Map($assets.map((a) => [a.ip_address, a]));

	// Group connections by source IP
	const grouped = new Map<string, Connection[]>();
	for (const conn of $connections) {
		const existing = grouped.get(conn.src_ip);
		if (existing) {
			existing.push(conn);
		} else {
			grouped.set(conn.src_ip, [conn]);
		}
	}

	// Build tree nodes
	const nodes: ConnectionTreeNode[] = [];
	for (const [ip, conns] of grouped) {
		const asset = assetMap.get(ip);
		nodes.push({
			ip,
			device_type: (asset?.device_type as ConnectionTreeNode['device_type']) ?? 'unknown',
			mac_address: asset?.mac_address ?? null,
			packet_count: conns.reduce((sum, c) => sum + c.packet_count, 0),
			connections: conns.sort((a, b) => b.packet_count - a.packet_count)
		});
	}

	// Sort: OT devices first, then by packet count
	nodes.sort((a, b) => {
		const aOt = a.device_type !== 'it_device' && a.device_type !== 'unknown';
		const bOt = b.device_type !== 'it_device' && b.device_type !== 'unknown';
		if (aOt !== bOt) return bOt ? 1 : -1;
		return b.packet_count - a.packet_count;
	});

	return nodes;
});

import { getAssets, getConnections, getDataCounts, getProtocolStats, getTopology } from '$lib/api';
import { assetCount, assets, connectionCount, connections, protocolStats, topology } from '$lib/stores/core';

export const DEFAULT_ASSET_PAGE_SIZE = 200;
export const DEFAULT_CONNECTION_PAGE_SIZE = 500;

export async function refreshCoreStores(
	assetPageSize = DEFAULT_ASSET_PAGE_SIZE,
	connectionPageSize = DEFAULT_CONNECTION_PAGE_SIZE
) {
	const [assetPage, connectionPage, nextTopology, nextProtocolStats, counts] = await Promise.all([
		getAssets(0, assetPageSize),
		getConnections(0, connectionPageSize),
		getTopology(),
		getProtocolStats(),
		getDataCounts()
	]);

	assets.set(assetPage.assets);
	connections.set(connectionPage.connections);
	topology.set(nextTopology);
	protocolStats.set(nextProtocolStats);
	assetCount.set(counts.asset_count);
	connectionCount.set(counts.connection_count);
}

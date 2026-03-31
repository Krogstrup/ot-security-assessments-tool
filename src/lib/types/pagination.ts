import type { Asset } from './assets';
import type { Connection } from './connections';

/** Paginated page of assets from get_assets. */
export interface AssetPage {
	assets: Asset[];
	total: number;
	page: number;
	page_size: number;
	has_more: boolean;
}

/** Paginated page of connections from get_connections. */
export interface ConnectionPage {
	connections: Connection[];
	total: number;
	page: number;
	page_size: number;
	has_more: boolean;
}

/** Lightweight counts for summary cards and sidebars. */
export interface DataCounts {
	asset_count: number;
	connection_count: number;
}

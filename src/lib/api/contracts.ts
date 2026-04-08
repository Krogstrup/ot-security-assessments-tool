/**
 * Shared API query contracts for pagination/sort parameters.
 *
 * Keep these values in sync with Rust enums in `commands/data.rs`.
 */

export const ASSET_SORTS = ['ip', 'packets', 'protocol', 'connections'] as const;
export type AssetSortBy = (typeof ASSET_SORTS)[number];

export const CONNECTION_SORTS = ['packets', 'bytes'] as const;
export type ConnectionSortBy = (typeof CONNECTION_SORTS)[number];

export const PROTOCOL_STATS_SORTS = ['packets', 'bytes', 'connections', 'devices'] as const;
export type ProtocolStatsSortBy = (typeof PROTOCOL_STATS_SORTS)[number];

export const DEFAULT_ASSET_PAGE_SIZE = 200;
export const DEFAULT_CONNECTION_PAGE_SIZE = 500;

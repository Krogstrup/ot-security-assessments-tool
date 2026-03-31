<script lang="ts">
	import type { Asset } from '$lib/types/assets';
	import { INVENTORY_COLUMNS, INVENTORY_PAGE_SIZE } from './assetTableConfig';
	import type { AssetColKey } from './assetTableConfig';
	import AssetTableRow from './AssetTableRow.svelte';
	import AssetTablePagination from './AssetTablePagination.svelte';

	interface Props {
		assets: Asset[];
		filteredCount: number;
		selectedAssetId: string | null;
		selectedIds: Set<string>;
		sortColumn: AssetColKey | '';
		sortDirection: 'asc' | 'desc';
		visibleColumns: Set<AssetColKey>;
		showColumnPicker: boolean;
		pageSize?: number;
		onSelectAsset: (id: string | null) => void;
		onToggleSelect: (id: string) => void;
		onSelectAll: () => void;
		onSort: (key: AssetColKey) => void;
		onToggleColumnVisibility: (key: AssetColKey) => void;
		onToggleColumnPicker: () => void;
		onPageChange: (page: number) => void;
	}

	let {
		assets,
		filteredCount,
		selectedAssetId,
		selectedIds,
		sortColumn,
		sortDirection,
		visibleColumns,
		showColumnPicker,
		pageSize = INVENTORY_PAGE_SIZE,
		onSelectAsset,
		onToggleSelect,
		onSelectAll,
		onSort,
		onToggleColumnVisibility,
		onToggleColumnPicker,
		onPageChange
	}: Props = $props();

	let currentPage = $state(0);
	let totalPages = $derived(Math.max(1, Math.ceil(filteredCount / pageSize)));

	function handlePageChange(newPage: number) {
		currentPage = newPage;
		onPageChange(newPage);
	}
</script>

<div class="table-container">
	{#if filteredCount === 0}
		<div class="empty-state">
			<p>No assets discovered yet. Import a PCAP file to get started.</p>
		</div>
	{:else}
		<table class="asset-table">
			<thead>
				<tr>
					<th class="th-check">
						<input
							type="checkbox"
							checked={selectedIds.size === filteredCount && filteredCount > 0}
							onchange={onSelectAll}
						/>
					</th>
					{#each INVENTORY_COLUMNS as col}
						{#if visibleColumns.has(col.key)}
							<th
								class="sortable-th"
								class:sort-active={sortColumn === col.key}
								onclick={() => onSort(col.key)}
							>
								{col.label}
								{#if sortColumn === col.key}
									<span class="sort-arrow">{sortDirection === 'asc' ? '▲' : '▼'}</span>
								{/if}
							</th>
						{/if}
					{/each}
				</tr>
			</thead>
			<tbody>
				{#each assets as asset}
					<AssetTableRow
						{asset}
						{visibleColumns}
						selected={selectedAssetId === asset.id}
						checked={selectedIds.has(asset.id)}
						onSelect={() => onSelectAsset(selectedAssetId === asset.id ? null : asset.id)}
						onToggleSelect={() => onToggleSelect(asset.id)}
					/>
				{/each}
			</tbody>
		</table>
		<AssetTablePagination
			{totalPages}
			{currentPage}
			{pageSize}
			{filteredCount}
			onPageChange={handlePageChange}
		/>
	{/if}
</div>

<style>
	.table-container {
		flex: 1;
		overflow: auto;
		background: var(--gm-bg-tertiary);
		border-right: 1px solid var(--gm-border);
	}

	.empty-state {
		padding: 2rem;
		text-align: center;
		color: var(--gm-text-secondary);
	}

	.asset-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 0.8125rem;
	}

	.asset-table thead {
		position: sticky;
		top: 0;
		background: var(--gm-bg-secondary);
		z-index: 10;
	}

	.asset-table th {
		padding: 0.75rem;
		text-align: left;
		font-weight: 600;
		color: var(--gm-text-secondary);
		border-bottom: 1px solid var(--gm-border);
		white-space: nowrap;
	}

	.sortable-th {
		cursor: pointer;
		user-select: none;
		transition: background 0.15s;
	}

	.sortable-th:hover {
		background: rgba(255, 255, 255, 0.05);
	}

	.sortable-th.sort-active {
		color: var(--gm-text-primary);
		background: rgba(99, 102, 241, 0.1);
	}

	.sort-arrow {
		margin-left: 0.35rem;
		font-size: 0.7rem;
		opacity: 0.6;
	}

	.th-check {
		width: 40px;
		text-align: center;
		padding: 0.5rem;
	}

</style>

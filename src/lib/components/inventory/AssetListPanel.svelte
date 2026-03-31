<script lang="ts">
	import type { Asset } from '$lib/types';
	import { DEVICE_TYPE_LABELS, DEVICE_TYPE_COLORS, CONFIDENCE_LABELS, CONFIDENCE_COLORS } from '$lib/constants';

	type ColKey = 'ip' | 'mac' | 'type' | 'confidence' | 'vendor' | 'oui' | 'product' | 'protocols' | 'country' | 'packets' | 'purdue' | 'first_seen' | 'last_seen';

	interface Props {
		assets: Asset[];
		filteredCount: number;
		selectedAssetId: string | null;
		selectedIds: Set<string>;
		sortColumn: ColKey | '';
		sortDirection: 'asc' | 'desc';
		visibleColumns: Set<ColKey>;
		showColumnPicker: boolean;
		pageSize?: number;
		onSelectAsset: (id: string | null) => void;
		onToggleSelect: (id: string) => void;
		onSelectAll: () => void;
		onSort: (key: ColKey) => void;
		onToggleColumnVisibility: (key: ColKey) => void;
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
		pageSize = 50,
		onSelectAsset,
		onToggleSelect,
		onSelectAll,
		onSort,
		onToggleColumnVisibility,
		onToggleColumnPicker,
		onPageChange
	}: Props = $props();

	const allColumns: { key: ColKey; label: string }[] = [
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

	function countryFlag(code: string): string {
		const base = 0x1f1e6;
		const a = code.charCodeAt(0) - 65;
		const b = code.charCodeAt(1) - 65;
		return String.fromCodePoint(base + a) + String.fromCodePoint(base + b);
	}

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
					{#each allColumns as col}
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
					<tr
						class="asset-row"
						class:selected={selectedAssetId === asset.id}
						onclick={() => onSelectAsset(selectedAssetId === asset.id ? null : asset.id)}
					>
						<td class="cell-check">
							<input
								type="checkbox"
								checked={selectedIds.has(asset.id)}
								onchange={() => onToggleSelect(asset.id)}
								onclick={(e) => e.stopPropagation()}
							/>
						</td>
						{#if visibleColumns.has('ip')}
							<td class="cell-ip">
								{asset.ip_address}
								{#if asset.is_public_ip}
									<span class="public-badge" title="Public IP">PUB</span>
								{/if}
							</td>
						{/if}
						{#if visibleColumns.has('mac')}
							<td class="cell-mac">{asset.mac_address ?? '—'}</td>
						{/if}
						{#if visibleColumns.has('type')}
							<td>
								<span
									class="device-badge"
									style="color: {DEVICE_TYPE_COLORS[asset.device_type]}; background: {DEVICE_TYPE_COLORS[asset.device_type]}18"
								>
									{DEVICE_TYPE_LABELS[asset.device_type]}
								</span>
							</td>
						{/if}
						{#if visibleColumns.has('confidence')}
							<td>
								{#if asset.confidence > 0}
									<span
										class="confidence-badge"
										style="color: {CONFIDENCE_COLORS[asset.confidence] ?? '#64748b'}; background: {(CONFIDENCE_COLORS[asset.confidence] ?? '#64748b')}18"
										title="{CONFIDENCE_LABELS[asset.confidence] ?? '?'} ({asset.confidence}/5)"
									>
										{asset.confidence}/5
									</span>
								{:else}
									<span class="confidence-none">—</span>
								{/if}
							</td>
						{/if}
						{#if visibleColumns.has('vendor')}
							<td class="cell-vendor">{asset.vendor ?? '—'}</td>
						{/if}
						{#if visibleColumns.has('oui')}
							<td class="cell-vendor cell-oui">{asset.oui_vendor ?? '—'}</td>
						{/if}
						{#if visibleColumns.has('product')}
							<td class="cell-vendor">{asset.product_family ?? '—'}</td>
						{/if}
						{#if visibleColumns.has('protocols')}
							<td class="cell-protocols">
								{#each asset.protocols as proto}
									<span class="proto-tag">{proto}</span>
								{/each}
							</td>
						{/if}
						{#if visibleColumns.has('country')}
							<td class="cell-country">
								{#if asset.country}
									<span title={asset.country}>{countryFlag(asset.country)}</span>
								{:else}
									—
								{/if}
							</td>
						{/if}
						{#if visibleColumns.has('packets')}
							<td class="cell-numeric">{asset.packet_count.toLocaleString()}</td>
						{/if}
						{#if visibleColumns.has('purdue')}
							<td class="cell-numeric">
								{asset.purdue_level !== null && asset.purdue_level !== undefined ? `L${asset.purdue_level}` : '—'}
							</td>
						{/if}
						{#if visibleColumns.has('first_seen')}
							<td class="cell-date">{asset.first_seen.slice(0, 19).replace('T', ' ')}</td>
						{/if}
						{#if visibleColumns.has('last_seen')}
							<td class="cell-date">{asset.last_seen.slice(0, 19).replace('T', ' ')}</td>
						{/if}
					</tr>
				{/each}
			</tbody>
		</table>
		{#if totalPages > 1}
			<div class="inv-pagination">
				<button class="inv-page-btn" disabled={currentPage === 0} onclick={() => handlePageChange(0)}>«</button>
				<button class="inv-page-btn" disabled={currentPage === 0} onclick={() => handlePageChange(currentPage - 1)}>‹</button>
				<span class="inv-page-info">
					{currentPage + 1} / {totalPages}
					&nbsp;·&nbsp;
					{currentPage * pageSize + 1}–{Math.min((currentPage + 1) * pageSize, filteredCount)} of {filteredCount}
				</span>
				<button class="inv-page-btn" disabled={currentPage >= totalPages - 1} onclick={() => handlePageChange(currentPage + 1)}>›</button>
				<button class="inv-page-btn" disabled={currentPage >= totalPages - 1} onclick={() => handlePageChange(totalPages - 1)}>»</button>
			</div>
		{/if}
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

	.asset-table tbody tr {
		border-bottom: 1px solid var(--gm-border);
		cursor: pointer;
		transition: background 0.15s;
	}

	.asset-table tbody tr:hover {
		background: rgba(255, 255, 255, 0.03);
	}

	.asset-table tbody tr.selected {
		background: rgba(99, 102, 241, 0.15);
	}

	.asset-table td {
		padding: 0.5rem 0.75rem;
		color: var(--gm-text-primary);
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.cell-check {
		text-align: center;
		padding: 0.5rem;
		width: 40px;
	}

	.cell-check input {
		cursor: pointer;
	}

	.cell-ip {
		font-family: 'JetBrains Mono', monospace;
		color: var(--gm-text-accent);
		font-weight: 500;
	}

	.public-badge {
		margin-left: 0.5rem;
		padding: 0.1rem 0.3rem;
		background: rgba(239, 68, 68, 0.2);
		color: #ef4444;
		border-radius: 2px;
		font-size: 0.7rem;
		font-weight: 600;
	}

	.device-badge {
		padding: 0.25rem 0.5rem;
		border-radius: 3px;
		font-weight: 500;
		font-size: 0.75rem;
	}

	.confidence-badge {
		padding: 0.25rem 0.5rem;
		border-radius: 3px;
		font-weight: 600;
		font-size: 0.75rem;
	}

	.confidence-none {
		color: var(--gm-text-secondary);
	}

	.cell-vendor {
		max-width: 150px;
	}

	.cell-oui {
		font-size: 0.75rem;
		color: var(--gm-text-secondary);
	}

	.cell-protocols {
		display: flex;
		flex-wrap: wrap;
		gap: 0.35rem;
	}

	.proto-tag {
		padding: 0.2rem 0.4rem;
		background: rgba(99, 102, 241, 0.2);
		color: #6366f1;
		border-radius: 2px;
		font-weight: 500;
		font-size: 0.7rem;
		white-space: nowrap;
	}

	.cell-numeric {
		text-align: right;
		font-family: 'JetBrains Mono', monospace;
		color: var(--gm-text-secondary);
	}

	.cell-date {
		font-family: 'JetBrains Mono', monospace;
		font-size: 0.75rem;
		color: var(--gm-text-secondary);
	}

	.inv-pagination {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.75rem;
		background: var(--gm-bg-secondary);
		border-top: 1px solid var(--gm-border);
	}

	.inv-page-btn {
		padding: 0.35rem 0.5rem;
		background: transparent;
		border: 1px solid var(--gm-border);
		color: var(--gm-text-primary);
		border-radius: 3px;
		cursor: pointer;
		font-size: 0.75rem;
		transition: all 0.15s;
	}

	.inv-page-btn:hover:not(:disabled) {
		background: rgba(99, 102, 241, 0.2);
		border-color: #6366f1;
	}

	.inv-page-btn:disabled {
		opacity: 0.3;
		cursor: not-allowed;
	}

	.inv-page-info {
		color: var(--gm-text-secondary);
		font-size: 0.8rem;
		min-width: 200px;
		text-align: center;
	}
</style>

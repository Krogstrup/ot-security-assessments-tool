<script lang="ts">
	import type { Asset } from '$lib/types/assets';
	import type { AssetColKey } from './assetTableConfig';
	import { countryFlagEmoji } from './assetTableConfig';
	import {
		DEVICE_TYPE_LABELS,
		DEVICE_TYPE_COLORS,
		CONFIDENCE_LABELS,
		CONFIDENCE_COLORS
	} from '$lib/constants';

	interface Props {
		asset: Asset;
		visibleColumns: Set<AssetColKey>;
		selected: boolean;
		checked: boolean;
		onSelect: () => void;
		onToggleSelect: () => void;
	}

	let { asset, visibleColumns, selected, checked, onSelect, onToggleSelect }: Props = $props();
</script>

<tr class="asset-row" class:selected={selected} onclick={onSelect}>
	<td class="cell-check">
		<input type="checkbox" {checked} onchange={onToggleSelect} onclick={(e) => e.stopPropagation()} />
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
				<span title={asset.country}>{countryFlagEmoji(asset.country)}</span>
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

<style>
	.asset-row {
		border-bottom: 1px solid var(--gm-border);
		cursor: pointer;
		transition: background 0.15s;
	}

	.asset-row:hover {
		background: rgba(255, 255, 255, 0.03);
	}

	.asset-row.selected {
		background: rgba(99, 102, 241, 0.15);
	}

	td {
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
</style>

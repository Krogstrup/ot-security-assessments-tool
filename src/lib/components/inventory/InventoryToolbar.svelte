<script lang="ts">
	import type { IcsProtocol } from '$lib/types/protocols';

	interface Props {
		protocols: IcsProtocol[];
		filteredCount: number;
		onFilterInput: (event: Event) => void;
		onProtocolChange: (protocol: IcsProtocol | null) => void;
	}

	let { protocols, filteredCount, onFilterInput, onProtocolChange }: Props = $props();
</script>

<div class="inventory-toolbar">
	<h2 class="view-title">Asset Inventory</h2>
	<div class="toolbar-controls">
		<input
			type="text"
			class="search-input"
			placeholder="Filter by IP, MAC, vendor, hostname..."
			oninput={onFilterInput}
		/>
		<select
			class="protocol-select"
			onchange={(e) => {
				const val = (e.target as HTMLSelectElement).value;
				onProtocolChange(val === 'all' ? null : (val as IcsProtocol));
			}}
		>
			<option value="all">All Protocols</option>
			{#each protocols as proto}
				<option value={proto}>{proto.toUpperCase()}</option>
			{/each}
		</select>
		<span class="result-count">{filteredCount} assets</span>
	</div>
</div>

<style>
	.inventory-toolbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 10px 16px;
		border-bottom: 1px solid var(--gm-border);
		background: var(--gm-bg-secondary);
		gap: 16px;
		flex-shrink: 0;
	}

	.view-title {
		font-size: 13px;
		font-weight: 600;
		letter-spacing: 1px;
		text-transform: uppercase;
		color: var(--gm-text-primary);
		margin: 0;
		white-space: nowrap;
	}

	.toolbar-controls {
		display: flex;
		align-items: center;
		gap: 10px;
		flex: 1;
		justify-content: flex-end;
	}

	.search-input {
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		padding: 6px 12px;
		color: var(--gm-text-primary);
		font-family: inherit;
		font-size: 11px;
		width: 280px;
		outline: none;
	}

	.search-input:focus {
		border-color: var(--gm-border-active);
	}

	.protocol-select {
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		padding: 6px 8px;
		color: var(--gm-text-secondary);
		font-family: inherit;
		font-size: 11px;
		outline: none;
	}

	.result-count {
		font-size: 10px;
		color: var(--gm-text-muted);
		white-space: nowrap;
	}
</style>

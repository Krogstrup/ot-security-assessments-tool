<script lang="ts">
	let {
		availableProtocols,
		filterProtocol,
		filterMinPackets,
		onProtocolChange,
		onMinPacketsChange,
		onFit,
		onRelayout
	}: {
		availableProtocols: string[];
		filterProtocol: string;
		filterMinPackets: number;
		onProtocolChange: (value: string) => void;
		onMinPacketsChange: (value: number) => void;
		onFit: () => void;
		onRelayout: () => void;
	} = $props();
</script>

<div class="mesh-toolbar">
	<div class="toolbar-section">
		<h2 class="view-title">Mesh View</h2>
		<span class="toolbar-sep"></span>
		<label class="filter-label">
			Protocol:
			<select
				class="filter-select"
				value={filterProtocol}
				onchange={(event) => onProtocolChange((event.target as HTMLSelectElement).value)}
			>
				<option value="all">All</option>
				{#each availableProtocols as protocol}
					<option value={protocol}>{protocol}</option>
				{/each}
			</select>
		</label>
		<label class="filter-label">
			Min packets:
			<input
				type="number"
				class="filter-input"
				value={filterMinPackets}
				min="0"
				step="10"
				oninput={(event) => onMinPacketsChange(parseInt((event.target as HTMLInputElement).value, 10) || 0)}
			/>
		</label>
	</div>
	<div class="toolbar-section">
		<button class="tool-btn" onclick={onFit}>Fit</button>
		<button class="tool-btn" onclick={onRelayout}>Relayout</button>
	</div>
</div>

<style>
	.mesh-toolbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 8px 16px;
		border-bottom: 1px solid var(--gm-border);
		background: var(--gm-bg-secondary);
	}

	.toolbar-section {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.toolbar-sep {
		width: 1px;
		height: 18px;
		background: var(--gm-border);
		margin: 0 4px;
	}

	.view-title {
		font-size: 13px;
		font-weight: 600;
		letter-spacing: 1px;
		text-transform: uppercase;
		color: var(--gm-text-primary);
		margin: 0;
	}

	.filter-label {
		font-size: 11px;
		color: var(--gm-text-secondary);
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.filter-select,
	.filter-input {
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		color: var(--gm-text-primary);
		font-family: inherit;
		font-size: 11px;
		padding: 3px 8px;
	}

	.filter-input {
		width: 70px;
	}

	.tool-btn {
		padding: 5px 12px;
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		color: var(--gm-text-secondary);
		font-family: inherit;
		font-size: 11px;
		cursor: pointer;
		transition: all 0.15s;
	}

	.tool-btn:hover {
		background: var(--gm-bg-hover);
		color: var(--gm-text-primary);
		border-color: var(--gm-border-active);
	}
</style>

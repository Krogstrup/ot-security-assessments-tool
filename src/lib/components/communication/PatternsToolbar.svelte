<script lang="ts">
	interface Props {
		filteredCount: number;
		totalCount: number;
		protocols: string[];
		protocolFilter: string;
		anomaliesOnly: boolean;
		onProtocolFilterChange: (protocol: string) => void;
		onAnomaliesOnlyChange: (enabled: boolean) => void;
		onRefresh: () => Promise<void>;
	}

	let {
		filteredCount,
		totalCount,
		protocols,
		protocolFilter,
		anomaliesOnly,
		onProtocolFilterChange,
		onAnomaliesOnlyChange,
		onRefresh
	}: Props = $props();
</script>

<div class="cp-toolbar">
	<div class="toolbar-section">
		<h2 class="view-title">Communication Patterns</h2>
		<span class="toolbar-sep"></span>
		<span class="row-count">{filteredCount.toLocaleString()} / {totalCount.toLocaleString()} connections</span>
	</div>
	<div class="toolbar-section">
		<label class="filter-label">
			Protocol:
			<select
				class="filter-select"
				value={protocolFilter}
				onchange={(e) => onProtocolFilterChange((e.target as HTMLSelectElement).value)}
			>
				<option value="">All</option>
				{#each protocols as p}
					<option value={p}>{p}</option>
				{/each}
			</select>
		</label>
		<label class="toggle-label">
			<input
				type="checkbox"
				class="toggle-cb"
				checked={anomaliesOnly}
				onchange={(e) => onAnomaliesOnlyChange((e.target as HTMLInputElement).checked)}
			/>
			Anomalies only
		</label>
		<button class="tool-btn" onclick={onRefresh}>Refresh</button>
	</div>
</div>

<style>
	.cp-toolbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 8px 16px;
		border-bottom: 1px solid var(--gm-border);
		background: var(--gm-bg-secondary);
		flex-shrink: 0;
	}

	.toolbar-section {
		display: flex;
		align-items: center;
		gap: 10px;
	}

	.toolbar-sep {
		width: 1px;
		height: 18px;
		background: var(--gm-border);
	}

	.view-title {
		font-size: 13px;
		font-weight: 600;
		letter-spacing: 1px;
		text-transform: uppercase;
		color: var(--gm-text-primary);
		margin: 0;
	}

	.row-count {
		font-size: 11px;
		color: var(--gm-text-muted);
	}

	.filter-label,
	.toggle-label {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 11px;
		color: var(--gm-text-secondary);
	}

	.filter-select {
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		color: var(--gm-text-primary);
		font-family: inherit;
		font-size: 11px;
		padding: 3px 8px;
		cursor: pointer;
	}

	.toggle-cb {
		cursor: pointer;
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
	}
</style>

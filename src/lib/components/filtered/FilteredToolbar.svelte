<script lang="ts">
	let {
		hiddenCount,
		onShowAll,
		onFit,
		onRelayout
	}: {
		hiddenCount: number;
		onShowAll: () => void;
		onFit: () => void;
		onRelayout: () => void;
	} = $props();
</script>

<div class="filtered-toolbar">
	<div class="toolbar-section">
		<h2 class="view-title">Filtered View</h2>
		<span class="toolbar-sep"></span>
		<span class="filter-info">{hiddenCount} node{hiddenCount !== 1 ? 's' : ''} hidden</span>
		{#if hiddenCount > 0}
			<button class="tool-btn" onclick={onShowAll}>Show All</button>
		{/if}
	</div>
	<div class="toolbar-section">
		<button class="tool-btn" onclick={onFit}>Fit</button>
		<button class="tool-btn" onclick={onRelayout}>Relayout</button>
	</div>
</div>

<style>
	.filtered-toolbar {
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

	.filter-info {
		font-size: 11px;
		color: var(--gm-text-muted);
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

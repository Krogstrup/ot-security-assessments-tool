<script lang="ts">
	import type { GroupingMode } from '$lib/types/topology';

	interface Props {
		layout: 'fcose' | 'purdue';
		groupingMode: GroupingMode;
		groupingOptions: { mode: GroupingMode; label: string }[];
		onLayoutChange: (layout: 'fcose' | 'purdue') => void;
		onGroupingChange: (mode: GroupingMode) => void;
		onFit: () => void;
		onCenter: () => void;
		onRelayout: () => void;
		onExportPng: () => void;
	}

	let {
		layout,
		groupingMode,
		groupingOptions,
		onLayoutChange,
		onGroupingChange,
		onFit,
		onCenter,
		onRelayout,
		onExportPng
	}: Props = $props();
</script>

<div class="topology-toolbar">
	<div class="toolbar-section">
		<h2 class="view-title">Logical View</h2>
		<span class="toolbar-sep"></span>
		<label class="group-label">
			Layout:
			<select
				class="group-select"
				value={layout}
				onchange={(e) => onLayoutChange((e.target as HTMLSelectElement).value as 'fcose' | 'purdue')}
			>
				<option value="fcose">Force-Directed</option>
				<option value="purdue">Purdue Layers</option>
			</select>
		</label>
		{#if layout === 'fcose'}
			<span class="toolbar-sep"></span>
			<label class="group-label">
				Group:
				<select
					class="group-select"
					value={groupingMode}
					onchange={(e) => onGroupingChange((e.target as HTMLSelectElement).value as GroupingMode)}
				>
					{#each groupingOptions as opt}
						<option value={opt.mode}>{opt.label}</option>
					{/each}
				</select>
			</label>
		{/if}
	</div>
	<div class="toolbar-section">
		<button class="tool-btn" onclick={onFit}>Fit</button>
		<button class="tool-btn" onclick={onCenter}>Center</button>
		<button class="tool-btn" onclick={onRelayout}>Relayout</button>
		<button class="tool-btn" onclick={onExportPng}>Export PNG</button>
	</div>
</div>

<style>
	.topology-toolbar {
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

	.group-label {
		font-size: 11px;
		color: var(--gm-text-secondary);
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.group-select {
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		color: var(--gm-text-primary);
		font-family: inherit;
		font-size: 11px;
		padding: 3px 8px;
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
		border-color: var(--gm-border-active);
	}
</style>

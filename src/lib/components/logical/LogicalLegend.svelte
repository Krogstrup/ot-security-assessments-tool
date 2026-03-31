<script lang="ts">
	interface LegendEntry {
		label: string;
		color: string;
	}

	interface Props {
		entries: LegendEntry[];
	}

	let { entries }: Props = $props();
</script>

<div class="topology-legend">
	<span class="legend-title">DEVICES</span>
	{#each entries as entry}
		<span class="legend-item">
			<span class="legend-dot" style="background: {entry.color}"></span>
			{entry.label}
		</span>
	{/each}
</div>

<style>
	.topology-legend {
		display: flex;
		align-items: center;
		gap: 14px;
		padding: 8px 16px;
		border-top: 1px solid var(--gm-border);
		background: var(--gm-bg-secondary);
		font-size: 9px;
		letter-spacing: 0.5px;
		flex-wrap: wrap;
	}

	.legend-title {
		color: var(--gm-text-muted);
		font-weight: 600;
		letter-spacing: 1.5px;
	}

	.legend-item {
		display: flex;
		align-items: center;
		gap: 4px;
		color: var(--gm-text-secondary);
	}

	.legend-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
	}
</style>

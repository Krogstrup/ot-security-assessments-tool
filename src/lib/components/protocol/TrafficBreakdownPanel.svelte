<script lang="ts">
	import type { ProtocolStats } from '$lib/types/protocols';

	interface Props {
		stats: ProtocolStats[];
		getColor: (protocol: string) => string;
	}

	let { stats, getColor }: Props = $props();

	let maxPackets = $derived(Math.max(1, ...stats.map((s) => s.packet_count)));
</script>

<div class="stats-panel">
	<h3 class="panel-title">Traffic by Protocol</h3>
	<div class="bar-chart">
		{#each stats as stat}
			{@const pct = (stat.packet_count / maxPackets) * 100}
			<div class="bar-row">
				<div class="bar-label">{stat.protocol}</div>
				<div class="bar-track">
					<div
						class="bar-fill"
						style="width: {pct}%; background: {getColor(stat.protocol)}"
					></div>
				</div>
				<div class="bar-value">{stat.packet_count.toLocaleString()}</div>
			</div>
		{/each}
	</div>
</div>

<style>
	.stats-panel {
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 6px;
		padding: 14px;
	}

	.panel-title {
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		color: var(--gm-text-muted);
		margin: 0 0 12px;
	}

	.bar-chart {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.bar-row {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.bar-label {
		width: 120px;
		font-size: 10px;
		color: var(--gm-text-secondary);
		text-align: right;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.bar-track {
		flex: 1;
		height: 16px;
		background: rgba(255, 255, 255, 0.03);
		border-radius: 3px;
		overflow: hidden;
	}

	.bar-fill {
		height: 100%;
		border-radius: 3px;
		min-width: 2px;
		transition: width 0.3s ease;
	}

	.bar-value {
		width: 70px;
		font-size: 10px;
		color: var(--gm-text-muted);
		font-variant-numeric: tabular-nums;
		text-align: right;
	}
</style>

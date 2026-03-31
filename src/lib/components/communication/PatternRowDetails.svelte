<script lang="ts">
	import type { ConnectionStats, PatternAnomaly } from '$lib/types/analysis';

	interface Props {
		row: ConnectionStats;
		rowAnomalies: PatternAnomaly[];
		buildHistogram: (row: ConnectionStats) => { bins: number[]; edges: number[]; maxCount: number };
		severityColor: (severity: string) => string;
		anomalyLabel: (type: string) => string;
	}

	let { row, rowAnomalies, buildHistogram, severityColor, anomalyLabel }: Props = $props();
</script>

<div class="detail-panel">
	{#if row.packet_count >= 2}
		{@const hist = buildHistogram(row)}
		<div class="histogram-section">
			<h4 class="detail-heading">Interval Distribution (approximated from stats)</h4>
			{#if hist.bins.length > 0}
				<svg class="histogram-svg" viewBox="0 0 200 60" preserveAspectRatio="none">
					{#each hist.bins as count, i}
						{@const barH = (count / hist.maxCount) * 50}
						<rect
							x={i * 20 + 1}
							y={58 - barH}
							width="18"
							height={barH}
							fill={barH > 0 ? '#10b981' : 'transparent'}
							opacity="0.8"
						/>
					{/each}
				</svg>
				<div class="hist-labels">
					<span>{row.min_interval_ms.toFixed(0)} ms</span>
					<span>{row.max_interval_ms.toFixed(0)} ms</span>
				</div>
			{/if}
			<div class="stat-grid">
				<div class="stat-item">
					<span class="stat-label">Min</span>
					<span class="stat-value">{row.min_interval_ms.toFixed(2)} ms</span>
				</div>
				<div class="stat-item">
					<span class="stat-label">Avg</span>
					<span class="stat-value">{row.avg_interval_ms.toFixed(2)} ms</span>
				</div>
				<div class="stat-item">
					<span class="stat-label">Std Dev</span>
					<span class="stat-value">{row.std_interval_ms.toFixed(2)} ms</span>
				</div>
				<div class="stat-item">
					<span class="stat-label">Max</span>
					<span class="stat-value">{row.max_interval_ms.toFixed(2)} ms</span>
				</div>
				<div class="stat-item">
					<span class="stat-label">CV</span>
					<span class="stat-value">
						{row.avg_interval_ms > 0 ? (row.std_interval_ms / row.avg_interval_ms).toFixed(2) : '—'}
					</span>
				</div>
				<div class="stat-item">
					<span class="stat-label">Periodic</span>
					<span class="stat-value" class:green={row.is_periodic}>
						{row.is_periodic ? 'Yes' : 'No'}
					</span>
				</div>
			</div>
		</div>
	{/if}

	{#if rowAnomalies.length > 0}
		<div class="anomaly-section">
			<h4 class="detail-heading">Anomalies</h4>
			{#each rowAnomalies as a}
				<div class="anomaly-item" style="border-left-color: {severityColor(a.severity)}">
					<span class="anomaly-type" style="color: {severityColor(a.severity)}">
						{anomalyLabel(a.anomaly_type)}
					</span>
					<span class="anomaly-desc">{a.description}</span>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.detail-panel {
		padding: 12px 16px;
		background: var(--gm-bg-panel);
		display: flex;
		gap: 24px;
		flex-wrap: wrap;
	}

	.detail-heading {
		font-size: 10px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.8px;
		color: var(--gm-text-muted);
		margin: 0 0 8px 0;
	}

	.histogram-section,
	.anomaly-section {
		flex: 1;
		min-width: 200px;
	}

	.histogram-svg {
		width: 100%;
		height: 60px;
		display: block;
		border-bottom: 1px solid var(--gm-border);
		margin-bottom: 4px;
	}

	.hist-labels {
		display: flex;
		justify-content: space-between;
		font-size: 9px;
		color: var(--gm-text-muted);
		margin-bottom: 10px;
	}

	.stat-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 8px;
	}

	.stat-item {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.stat-label {
		font-size: 9px;
		text-transform: uppercase;
		letter-spacing: 0.8px;
		color: var(--gm-text-muted);
	}

	.stat-value {
		font-size: 11px;
		font-family: 'JetBrains Mono', monospace;
		color: var(--gm-text-primary);
	}

	.stat-value.green {
		color: #10b981;
	}

	.anomaly-item {
		padding: 6px 8px;
		border-left: 3px solid;
		background: rgba(51, 65, 85, 0.3);
		margin-bottom: 4px;
		border-radius: 0 4px 4px 0;
		display: flex;
		gap: 10px;
		align-items: baseline;
	}

	.anomaly-type {
		font-size: 10px;
		font-weight: 700;
		white-space: nowrap;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.anomaly-desc {
		font-size: 11px;
		color: var(--gm-text-secondary);
		line-height: 1.4;
	}
</style>

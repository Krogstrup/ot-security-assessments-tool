<script lang="ts">
	import type { PatternAnomaly } from '$lib/types/analysis';

	interface Props {
		anomalies: PatternAnomaly[];
		severityColor: (severity: string) => string;
	}

	let { anomalies, severityColor }: Props = $props();
	let severities = $derived([...new Set(anomalies.map((a) => a.severity))].sort());
</script>

{#if anomalies.length > 0}
	<div class="anomaly-bar">
		<span class="anomaly-bar-label">&#9888; {anomalies.length} pattern anomal{anomalies.length === 1 ? 'y' : 'ies'} detected</span>
		{#each severities as sev}
			<span class="anomaly-badge" style="border-color: {severityColor(sev)}; color: {severityColor(sev)}">
				{anomalies.filter((a) => a.severity === sev).length} {sev}
			</span>
		{/each}
	</div>
{/if}

<style>
	.anomaly-bar {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 6px 16px;
		background: rgba(245, 158, 11, 0.08);
		border-bottom: 1px solid rgba(245, 158, 11, 0.2);
		flex-shrink: 0;
	}

	.anomaly-bar-label {
		font-size: 11px;
		color: #f59e0b;
		font-weight: 600;
	}

	.anomaly-badge {
		font-size: 10px;
		padding: 1px 7px;
		border: 1px solid;
		border-radius: 10px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}
</style>

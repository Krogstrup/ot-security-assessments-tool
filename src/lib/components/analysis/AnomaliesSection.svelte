<script lang="ts">
	import type { AnomalyScore, FindingSeverity } from '$lib/types/analysis';

	interface Props {
		anomalies: AnomalyScore[];
		severityColors: Record<FindingSeverity, string>;
		onNavigateAsset: (ip: string) => void;
	}

	let { anomalies, severityColors, onNavigateAsset }: Props = $props();
</script>

{#if anomalies.length === 0}
	<div class="empty-section">
		<p>No anomalies detected. Run analysis to check for behavioral deviations.</p>
	</div>
{:else}
	<div class="anomaly-list">
		{#each anomalies as anomaly}
			<div class="anomaly-card">
				<div class="anomaly-header">
					<span class="severity-badge" style="background: {severityColors[anomaly.severity]}">
						{anomaly.severity.toUpperCase()}
					</span>
					<span class="anomaly-type">{anomaly.anomaly_type.replace(/_/g, ' ')}</span>
					<span class="anomaly-confidence">
						{Math.round(anomaly.confidence * 100)}% confidence
					</span>
				</div>
				<div class="anomaly-evidence">{anomaly.evidence}</div>
				<button class="asset-link" onclick={() => onNavigateAsset(anomaly.affected_asset)}>
					{anomaly.affected_asset}
				</button>
			</div>
		{/each}
	</div>
{/if}

<style>
	.empty-section {
		padding: 40px 20px;
		text-align: center;
		color: var(--gm-text-muted);
		font-size: 12px;
	}

	.severity-badge {
		display: inline-flex;
		align-items: center;
		gap: 3px;
		padding: 2px 8px;
		border-radius: 4px;
		font-size: 9px;
		font-weight: 700;
		color: #0a0e17;
		letter-spacing: 0.5px;
	}

	.anomaly-list {
		display: flex;
		flex-direction: column;
		gap: 10px;
	}

	.anomaly-card {
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 8px;
		padding: 14px;
	}

	.anomaly-header {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-bottom: 8px;
	}

	.anomaly-type {
		font-size: 11px;
		color: var(--gm-text-secondary);
		text-transform: capitalize;
	}

	.anomaly-confidence {
		font-size: 10px;
		color: var(--gm-text-muted);
		margin-left: auto;
	}

	.anomaly-evidence {
		font-size: 11px;
		color: var(--gm-text-muted);
		margin-bottom: 8px;
		line-height: 1.4;
	}

	.asset-link {
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.3);
		border-radius: 4px;
		color: #10b981;
		padding: 2px 8px;
		font-family: inherit;
		font-size: 10px;
		cursor: pointer;
		transition: all 0.15s;
	}

	.asset-link:hover {
		background: rgba(16, 185, 129, 0.2);
	}
</style>

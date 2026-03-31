<script lang="ts">
	import type { AnalysisSummary, FindingSeverity } from '$lib/types/analysis';

	interface Props {
		analysisSummary: AnalysisSummary | null;
		severityOrder: FindingSeverity[];
		severityColors: Record<FindingSeverity, string>;
	}

	let { analysisSummary, severityOrder, severityColors }: Props = $props();
</script>

{#if analysisSummary}
	<div class="summary-grid">
		<div class="summary-card">
			<div class="card-label">Total Findings</div>
			<div class="card-value">{analysisSummary.total_findings}</div>
		</div>
		<div class="summary-card severity-critical">
			<div class="card-label">Critical</div>
			<div class="card-value">{analysisSummary.critical_count}</div>
		</div>
		<div class="summary-card severity-high">
			<div class="card-label">High</div>
			<div class="card-value">{analysisSummary.high_count}</div>
		</div>
		<div class="summary-card severity-medium">
			<div class="card-label">Medium</div>
			<div class="card-value">{analysisSummary.medium_count}</div>
		</div>
		<div class="summary-card">
			<div class="card-label">Assets Analyzed</div>
			<div class="card-value">{analysisSummary.assets_analyzed}</div>
		</div>
		<div class="summary-card">
			<div class="card-label">Connections</div>
			<div class="card-value">{analysisSummary.connections_analyzed}</div>
		</div>
		<div class="summary-card">
			<div class="card-label">Purdue Violations</div>
			<div class="card-value">{analysisSummary.purdue_violations}</div>
		</div>
		<div class="summary-card">
			<div class="card-label">Unencrypted OT</div>
			<div class="card-value">{analysisSummary.unencrypted_ot_percent}%</div>
		</div>
	</div>

	{#if analysisSummary.total_findings > 0}
		<div class="severity-bar-section">
			<h3 class="subsection-title">Severity Distribution</h3>
			<div class="severity-bar">
				{#each severityOrder as sev}
					{@const count = analysisSummary[`${sev}_count` as keyof typeof analysisSummary] as number}
					{#if count > 0}
						<div
							class="severity-segment"
							style="width: {(count / analysisSummary.total_findings) * 100}%; background: {severityColors[sev]}"
							title="{sev}: {count}"
						></div>
					{/if}
				{/each}
			</div>
			<div class="severity-legend">
				{#each severityOrder as sev}
					{@const count = analysisSummary[`${sev}_count` as keyof typeof analysisSummary] as number}
					{#if count > 0}
						<span class="legend-item">
							<span class="legend-dot" style="background: {severityColors[sev]}"></span>
							{sev} ({count})
						</span>
					{/if}
				{/each}
			</div>
		</div>
	{/if}
{:else}
	<div class="empty-section">
		<p>Click "Run Analysis" to analyze the current network data for security findings.</p>
	</div>
{/if}

<style>
	.empty-section {
		padding: 40px 20px;
		text-align: center;
		color: var(--gm-text-muted);
		font-size: 12px;
	}

	.summary-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 12px;
		margin-bottom: 24px;
	}

	.summary-card {
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 8px;
		padding: 14px;
		text-align: center;
	}

	.summary-card.severity-critical {
		border-color: var(--gm-severity-critical);
	}

	.summary-card.severity-high {
		border-color: var(--gm-severity-high);
	}

	.summary-card.severity-medium {
		border-color: var(--gm-severity-medium);
	}

	.card-label {
		font-size: 10px;
		color: var(--gm-text-muted);
		text-transform: uppercase;
		letter-spacing: 0.5px;
		margin-bottom: 6px;
	}

	.card-value {
		font-size: 22px;
		font-weight: 700;
		color: var(--gm-text-primary);
	}

	.severity-bar-section {
		margin-bottom: 24px;
	}

	.subsection-title {
		font-size: 12px;
		font-weight: 600;
		color: var(--gm-text-secondary);
		margin: 0 0 10px;
	}

	.severity-bar {
		display: flex;
		height: 12px;
		border-radius: 6px;
		overflow: hidden;
		background: var(--gm-bg-panel);
	}

	.severity-segment {
		transition: width 0.3s ease;
	}

	.severity-legend {
		display: flex;
		gap: 14px;
		margin-top: 8px;
	}

	.legend-item {
		display: flex;
		align-items: center;
		gap: 4px;
		font-size: 10px;
		color: var(--gm-text-secondary);
		text-transform: capitalize;
	}

	.legend-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
	}
</style>

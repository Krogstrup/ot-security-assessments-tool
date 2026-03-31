<script lang="ts">
	import type { Finding, FindingSeverity } from '$lib/types/analysis';

	interface Props {
		findings: Finding[];
		severityColors: Record<FindingSeverity, string>;
		getSeverityIcon: (severity: FindingSeverity) => string;
		getTypeLabel: (type: string) => string;
		onNavigateAsset: (ip: string) => void;
	}

	let { findings, severityColors, getSeverityIcon, getTypeLabel, onNavigateAsset }: Props = $props();
</script>

{#if findings.length === 0}
	<div class="empty-section">
		<p>No findings yet. Run analysis to detect security issues.</p>
	</div>
{:else}
	<div class="findings-list">
		{#each findings as finding}
			<div class="finding-card severity-border-{finding.severity}">
				<div class="finding-header">
					<span class="severity-badge" style="background: {severityColors[finding.severity]}">
						{getSeverityIcon(finding.severity)} {finding.severity.toUpperCase()}
					</span>
					<span class="finding-type">{getTypeLabel(finding.finding_type)}</span>
					{#if finding.technique_id}
						<span class="technique-id">{finding.technique_id}</span>
					{/if}
				</div>
				<h3 class="finding-title">{finding.title}</h3>
				<p class="finding-desc">{finding.description}</p>
				<div class="finding-evidence">
					<span class="evidence-label">Evidence:</span>
					{finding.evidence}
				</div>
				<div class="finding-assets">
					<span class="assets-label">Affected:</span>
					{#each finding.affected_assets as ip}
						<button class="asset-link" onclick={() => onNavigateAsset(ip)}>
							{ip}
						</button>
					{/each}
				</div>
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

	.findings-list {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.finding-card {
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 8px;
		padding: 14px;
	}

	.finding-card.severity-border-critical { border-left: 3px solid var(--gm-severity-critical); }
	.finding-card.severity-border-high { border-left: 3px solid var(--gm-severity-high); }
	.finding-card.severity-border-medium { border-left: 3px solid var(--gm-severity-medium); }
	.finding-card.severity-border-low { border-left: 3px solid var(--gm-severity-low); }
	.finding-card.severity-border-info { border-left: 3px solid var(--gm-severity-info); }

	.finding-header {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-bottom: 8px;
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

	.finding-type {
		font-size: 10px;
		color: var(--gm-text-muted);
		background: var(--gm-bg-hover);
		padding: 2px 6px;
		border-radius: 3px;
	}

	.technique-id {
		font-size: 10px;
		color: #10b981;
		font-weight: 600;
	}

	.finding-title {
		font-size: 13px;
		font-weight: 600;
		color: var(--gm-text-primary);
		margin: 0 0 6px;
	}

	.finding-desc {
		font-size: 11px;
		color: var(--gm-text-secondary);
		margin: 0 0 8px;
		line-height: 1.5;
	}

	.finding-evidence {
		font-size: 10px;
		color: var(--gm-text-muted);
		background: var(--gm-bg-secondary);
		padding: 8px;
		border-radius: 4px;
		margin-bottom: 8px;
		font-style: italic;
	}

	.evidence-label,
	.assets-label {
		font-weight: 600;
		color: var(--gm-text-secondary);
		font-style: normal;
	}

	.finding-assets {
		display: flex;
		align-items: center;
		gap: 6px;
		flex-wrap: wrap;
		font-size: 10px;
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

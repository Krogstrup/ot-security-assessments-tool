<script lang="ts">
	import type { Finding } from '$lib/types/analysis';

	interface Props {
		findings: Finding[];
		loading: boolean;
		error: string;
	}

	let { findings, loading, error }: Props = $props();

	function severityRank(severity: Finding['severity']): number {
		switch (severity) {
			case 'critical':
				return 5;
			case 'high':
				return 4;
			case 'medium':
				return 3;
			case 'low':
				return 2;
			default:
				return 1;
		}
	}

	let sortedFindings = $derived(
		[...findings].sort((a, b) => severityRank(b.severity) - severityRank(a.severity)).slice(0, 10)
	);
</script>

<section class="capture-section">
	<h3 class="section-title">Live Alerts</h3>
	<p class="section-desc">
		Latest ATT&CK and analysis findings from imported data and live capture.
	</p>

	{#if loading}
		<div class="status status-loading">Refreshing alerts…</div>
	{:else if error}
		<div class="status status-error">{error}</div>
	{:else if sortedFindings.length === 0}
		<div class="status">No alerts yet.</div>
	{:else}
		<div class="alerts-list">
			{#each sortedFindings as finding}
				<div class="alert-item">
					<span class="severity sev-{finding.severity}">{finding.severity.toUpperCase()}</span>
					<div class="alert-copy">
						<div class="alert-title">{finding.title}</div>
						<div class="alert-meta">
							{finding.finding_type} · {finding.affected_assets.length} affected assets
						</div>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</section>

<style>
	.capture-section {
		padding: 1.5rem;
		background: var(--gm-bg-secondary);
		border: 1px solid var(--gm-border);
		border-radius: 6px;
		margin-bottom: 1rem;
	}

	.section-title {
		font-size: 1.125rem;
		font-weight: 600;
		margin-bottom: 0.5rem;
		color: var(--gm-text-primary);
	}

	.section-desc {
		font-size: 0.8125rem;
		color: var(--gm-text-secondary);
		margin-bottom: 1rem;
	}

	.status {
		font-size: 0.8125rem;
		color: var(--gm-text-secondary);
	}

	.status-error {
		color: #ef4444;
	}

	.alerts-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.alert-item {
		display: flex;
		gap: 0.75rem;
		align-items: flex-start;
		padding: 0.6rem 0.75rem;
		background: var(--gm-bg-tertiary);
		border: 1px solid var(--gm-border);
		border-radius: 4px;
	}

	.severity {
		font-size: 0.7rem;
		font-weight: 700;
		padding: 0.2rem 0.45rem;
		border-radius: 3px;
		min-width: 56px;
		text-align: center;
	}

	.sev-critical {
		background: rgba(239, 68, 68, 0.2);
		color: #ef4444;
	}

	.sev-high {
		background: rgba(249, 115, 22, 0.2);
		color: #f97316;
	}

	.sev-medium {
		background: rgba(245, 158, 11, 0.2);
		color: #f59e0b;
	}

	.sev-low {
		background: rgba(34, 197, 94, 0.2);
		color: #22c55e;
	}

	.sev-info {
		background: rgba(59, 130, 246, 0.2);
		color: #3b82f6;
	}

	.alert-copy {
		min-width: 0;
	}

	.alert-title {
		font-size: 0.8125rem;
		font-weight: 600;
		color: var(--gm-text-primary);
	}

	.alert-meta {
		font-size: 0.75rem;
		color: var(--gm-text-secondary);
	}
</style>


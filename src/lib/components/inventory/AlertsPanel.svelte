<script lang="ts">
	import type { CorrelatedAlert } from '$lib/types/analysis';

	interface Props {
		alerts: CorrelatedAlert[];
	}

	let { alerts }: Props = $props();

	function getSeverityLabel(severity: number): string {
		return severity === 1 ? 'HIGH' : severity === 2 ? 'MED' : 'LOW';
	}

	function getSeverityClass(severity: number): string {
		return severity === 1 ? 'high' : severity === 2 ? 'medium' : 'low';
	}
</script>

{#if alerts.length > 0}
	<div class="detail-section alert-section">
		<h4 class="section-title alert-title">&#128680; External Alerts ({alerts.length})</h4>
		{#each alerts as alert}
			<div class="asset-alert-row">
				<span class="alert-sev-badge sev-{getSeverityClass(alert.severity)}">
					{getSeverityLabel(alert.severity)}
				</span>
				<span class="alert-source-tag">{alert.source}</span>
				<span class="alert-sig-text">{alert.signature}</span>
			</div>
		{/each}
	</div>
{/if}

<style>
	.detail-section {
		margin-top: 1.5rem;
		padding: 1rem;
		border-radius: 6px;
		background: #0f172a;
		border: 1px solid #1e293b;
	}

	.alert-section {
		background: rgba(249, 115, 22, 0.05);
		border: 1px solid rgba(249, 115, 22, 0.2);
	}

	.section-title {
		font-size: 0.875rem;
		font-weight: 600;
		margin-bottom: 1rem;
		color: #e2e8f0;
	}

	.alert-title {
		color: #f97316;
	}

	.asset-alert-row {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.5rem;
		margin-bottom: 0.5rem;
		background: rgba(0, 0, 0, 0.2);
		border-radius: 3px;
		font-size: 0.8125rem;
	}

	.alert-sev-badge {
		padding: 0.25rem 0.5rem;
		border-radius: 3px;
		font-weight: 600;
		font-size: 0.7rem;
		min-width: 40px;
		text-align: center;
	}

	.alert-sev-badge.sev-high {
		background: rgba(239, 68, 68, 0.2);
		color: #ef4444;
	}

	.alert-sev-badge.sev-medium {
		background: rgba(249, 115, 22, 0.2);
		color: #f97316;
	}

	.alert-sev-badge.sev-low {
		background: rgba(34, 197, 94, 0.2);
		color: #22c55e;
	}

	.alert-source-tag {
		background: #1e293b;
		padding: 0.25rem 0.5rem;
		border-radius: 3px;
		color: #94a3b8;
		font-weight: 500;
		min-width: 70px;
	}

	.alert-sig-text {
		color: #cbd5e1;
		flex: 1;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}
</style>

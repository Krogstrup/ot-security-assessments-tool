<script lang="ts">
	import type { SegmentationReport } from '$lib/types/segmentation';

	interface Props {
		report: SegmentationReport;
		slLabel: (sl: string) => string;
		slClass: (sl: string) => string;
	}

	let { report, slLabel, slClass }: Props = $props();
</script>

<div class="tab-content">
	<div class="summary-row">
		<span class="metric">{report.policy_groups.length.toLocaleString()} <small>groups</small></span>
		<span class="metric">{new Set(report.policy_groups.flatMap((g) => g.member_ips)).size.toLocaleString()} <small>classified assets</small></span>
	</div>
	<div class="card-grid">
		{#each report.policy_groups as group}
			<div class="group-card">
				<div class="group-header">
					<span class="group-name">{group.name}</span>
					<span class="sl-badge {slClass(group.security_level)}">{slLabel(group.security_level)}</span>
				</div>
				<div class="group-meta">
					<span>Purdue L{group.purdue_level ?? '?'}</span>
					<span>·</span>
					<span>{group.device_category.replace(/_/g, ' ')}</span>
					<span>·</span>
					<span class="criticality {group.criticality}">{group.criticality}</span>
				</div>
				<div class="member-ips">
					{#each group.member_ips as ip}
						<span class="ip-pill">{ip}</span>
					{/each}
				</div>
			</div>
		{/each}
	</div>
</div>

<style>
	.tab-content {
		flex: 1;
		overflow-y: auto;
		min-height: 0;
		animation: fadeIn 0.15s ease;
	}

	@keyframes fadeIn {
		from { opacity: 0; }
		to { opacity: 1; }
	}

	.summary-row {
		display: flex;
		gap: 2rem;
		margin-bottom: 1.5rem;
	}

	.metric {
		font-size: 1.4rem;
		font-weight: 600;
	}

	.metric small {
		font-size: 0.75rem;
		font-weight: 400;
		color: var(--text-muted, #888);
		margin-left: 0.25rem;
	}

	.card-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 1rem;
	}

	.group-card {
		background: var(--surface-2, #1e1e1e);
		border: 1px solid var(--border, #333);
		border-radius: 6px;
		padding: 1rem;
	}

	.group-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.5rem;
	}

	.group-name {
		font-weight: 600;
		font-size: 0.95rem;
	}

	.group-meta {
		font-size: 0.8rem;
		color: var(--text-muted, #888);
		display: flex;
		gap: 0.4rem;
		margin-bottom: 0.75rem;
	}

	.member-ips {
		display: flex;
		flex-wrap: wrap;
		gap: 0.3rem;
		max-height: 200px;
		overflow-y: auto;
	}

	.ip-pill {
		background: var(--surface-3, #2a2a2a);
		padding: 0.1rem 0.5rem;
		border-radius: 3px;
		font-size: 0.78rem;
		font-family: monospace;
	}

	.sl-badge {
		font-size: 0.72rem;
		font-weight: 700;
		padding: 0.15rem 0.4rem;
		border-radius: 3px;
		letter-spacing: 0.03em;
	}

	.sl-badge.sl3, .sl-badge.sl4 {
		background: var(--purdue-l1, #22c55e22);
		color: var(--purdue-l1-text, #22c55e);
		border: 1px solid var(--purdue-l1, #22c55e44);
	}

	.sl-badge.sl2 {
		background: #ca8a0422;
		color: #ca8a04;
		border: 1px solid #ca8a0444;
	}

	.sl-badge.sl1 {
		background: #64748b22;
		color: #94a3b8;
		border: 1px solid #64748b44;
	}

	.criticality.critical { color: #ef4444; }
	.criticality.high { color: #f97316; }
	.criticality.medium { color: #eab308; }
	.criticality.low { color: #22c55e; }
</style>

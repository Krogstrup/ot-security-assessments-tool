<script lang="ts">
	import type { SegmentationReport } from '$lib/types/segmentation';

	interface Props {
		report: SegmentationReport;
		zoneNameById: (id: string) => string;
		riskClass: (risk: string) => string;
	}

	let { report, zoneNameById, riskClass }: Props = $props();
</script>

<div class="tab-content">
	<div class="summary-row">
		<span class="metric">{report.communication_matrix.zone_pairs.length.toLocaleString()} <small>zone pairs</small></span>
		<span class="metric">{report.communication_matrix.coverage_percent.toFixed(1)}% <small>coverage</small></span>
		<span class="metric">{report.communication_matrix.default_action} <small>default</small></span>
	</div>
	{#each report.communication_matrix.zone_pairs as pair}
		<div class="zone-pair">
			<div class="zone-pair-header">
				<span class="zone-name">{zoneNameById(pair.src_zone_id)}</span>
				<span class="arrow">→</span>
				<span class="zone-name">{zoneNameById(pair.dst_zone_id)}</span>
				<span class="rule-count">{pair.rules.length} rule{pair.rules.length !== 1 ? 's' : ''}</span>
			</div>
			<div class="rule-table-container">
				<table class="rule-table">
					<thead>
						<tr><th>Protocol</th><th>Port</th><th>Risk</th><th>Justification</th><th>Packets</th></tr>
					</thead>
					<tbody>
						{#each pair.rules as rule}
							<tr>
								<td>{rule.protocol}</td>
								<td>{rule.dst_port ?? 'any'}</td>
								<td><span class="risk-badge {riskClass(rule.risk)}">{rule.risk}</span></td>
								<td class="justification">{rule.justification}</td>
								<td>{rule.packet_count.toLocaleString()}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>
	{/each}
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

	.zone-pair {
		margin-bottom: 1.5rem;
		border: 1px solid var(--border, #333);
		border-radius: 6px;
		overflow: hidden;
	}

	.zone-pair-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.6rem 1rem;
		background: var(--surface-2, #1e1e1e);
		font-size: 0.9rem;
	}

	.zone-name {
		font-weight: 600;
	}

	.arrow {
		color: var(--text-muted, #888);
	}

	.rule-count {
		margin-left: auto;
		color: var(--text-muted, #888);
		font-size: 0.8rem;
	}

	.rule-table-container {
		max-height: 400px;
		overflow-y: auto;
	}

	.rule-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 0.83rem;
	}

	.rule-table th {
		text-align: left;
		padding: 0.4rem 0.75rem;
		border-bottom: 1px solid var(--border, #333);
		color: var(--text-muted, #888);
		font-weight: 500;
		position: sticky;
		top: 0;
		background: var(--gm-bg-secondary, #1a2332);
		z-index: 1;
	}

	.rule-table td {
		padding: 0.35rem 0.75rem;
		border-bottom: 1px solid var(--border-subtle, #222);
	}

	.justification {
		color: var(--text-muted, #888);
		font-size: 0.8rem;
	}

	.risk-badge {
		font-size: 0.72rem;
		font-weight: 700;
		padding: 0.1rem 0.4rem;
		border-radius: 3px;
	}

	.risk-badge.high { background: #ef444422; color: #ef4444; border: 1px solid #ef444444; }
	.risk-badge.medium { background: #f9731622; color: #f97316; border: 1px solid #f9731644; }
	.risk-badge.low { background: #22c55e22; color: #22c55e; border: 1px solid #22c55e44; }
</style>

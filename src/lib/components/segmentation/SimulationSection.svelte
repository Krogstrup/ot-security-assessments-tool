<script lang="ts">
	import type { SegmentationReport, SimulationResult } from '$lib/types/segmentation';
	import SimulationBlockTable from './SimulationBlockTable.svelte';

	interface Props {
		report: SegmentationReport;
		visibleBlockCount: number;
		visibleFpCount: number;
		zoneNameById: (id: string) => string;
		blockedPercent: (sim: SimulationResult) => string;
		scorePercent: (value: number) => string;
		onShowMoreBlocks: () => void;
		onShowMoreFalsePositives: () => void;
	}

	let {
		report,
		visibleBlockCount,
		visibleFpCount,
		zoneNameById,
		blockedPercent,
		scorePercent,
		onShowMoreBlocks,
		onShowMoreFalsePositives
	}: Props = $props();

	let sim = $derived(report.simulation);
</script>

<div class="tab-content">
	<div class="sim-metrics">
		<div class="sim-metric">
			<div class="metric-value green">{sim.allowed.toLocaleString()}</div>
			<div class="metric-label">Allowed</div>
		</div>
		<div class="sim-metric">
			<div class="metric-value {sim.blocked > 0 ? 'red' : 'green'}">{sim.blocked.toLocaleString()}</div>
			<div class="metric-label">Blocked ({blockedPercent(sim)})</div>
		</div>
		<div class="sim-metric">
			<div class="metric-value">{scorePercent(sim.risk_reduction_score)}</div>
			<div class="metric-label">Risk Reduction</div>
		</div>
		<div class="sim-metric">
			<div class="metric-value">{scorePercent(sim.deployment_score)}</div>
			<div class="metric-label">Deployment Score</div>
		</div>
	</div>

	{#if sim.zone_block_summaries.length > 0}
		<h3>Blocked by Zone Pair</h3>
		<table class="data-table">
			<thead>
				<tr><th>From Zone</th><th>To Zone</th><th>Blocked Flows</th></tr>
			</thead>
			<tbody>
				{#each sim.zone_block_summaries as zbs}
					<tr>
						<td>{zoneNameById(zbs.src_zone_id)}</td>
						<td>{zoneNameById(zbs.dst_zone_id)}</td>
						<td class="count">{zbs.blocked_count.toLocaleString()}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	{/if}

	{#if sim.critical_blocks.length > 0}
		<SimulationBlockTable
			title="Critical Blocks"
			items={sim.critical_blocks}
			visibleCount={visibleBlockCount}
			onShowMore={onShowMoreBlocks}
			showReason={true}
		/>
	{/if}

	{#if sim.false_positive_candidates.length > 0}
		<SimulationBlockTable
			title="False Positive Candidates"
			items={sim.false_positive_candidates}
			visibleCount={visibleFpCount}
			onShowMore={onShowMoreFalsePositives}
			countBadgeClass="fp"
			hint="These periodic, read-only, allowlisted connections are likely safe — review before enforcement."
		/>
	{/if}

	{#if sim.allowed === 0 && sim.blocked === 0}
		<p class="hint">No traffic data to simulate. Import a PCAP or run live capture first.</p>
	{/if}
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

	.sim-metrics {
		display: flex;
		gap: 2rem;
		margin-bottom: 2rem;
	}

	.sim-metric {
		text-align: center;
	}

	.metric-value {
		font-size: 2rem;
		font-weight: 700;
		font-variant-numeric: tabular-nums;
	}

	.metric-value.green { color: #22c55e; }
	.metric-value.red { color: #ef4444; }

	.metric-label {
		font-size: 0.8rem;
		color: var(--text-muted, #888);
	}

	.data-table {
		width: 100%;
		border-collapse: collapse;
		margin-bottom: 1.5rem;
		font-size: 0.85rem;
	}

	.data-table th {
		text-align: left;
		padding: 0.5rem 0.75rem;
		border-bottom: 1px solid var(--border, #333);
		color: var(--text-muted, #888);
		font-weight: 500;
		position: sticky;
		top: 0;
		background: var(--gm-bg-secondary, #1a2332);
		z-index: 1;
	}

	.data-table td {
		padding: 0.4rem 0.75rem;
		border-bottom: 1px solid var(--border-subtle, #222);
	}

	.data-table .count {
		text-align: right;
		font-variant-numeric: tabular-nums;
	}

	.hint {
		font-size: 0.82rem;
		color: var(--text-muted, #888);
	}

	h3 {
		margin: 1.5rem 0 0.75rem;
		font-size: 1rem;
		font-weight: 600;
	}
</style>

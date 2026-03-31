<script lang="ts">
	import type { SegmentationReport } from '$lib/types/segmentation';

	interface Props {
		report: SegmentationReport;
		slLabel: (sl: string) => string;
		slClass: (sl: string) => string;
		zoneNameById: (id: string) => string;
	}

	let { report, slLabel, slClass, zoneNameById }: Props = $props();
</script>

<div class="tab-content">
	<div class="summary-row">
		<span class="metric">{report.zone_model.zones.length.toLocaleString()} <small>zones</small></span>
		<span class="metric">{report.zone_model.conduits.length.toLocaleString()} <small>conduits</small></span>
		<span class="metric">{(report.zone_model.zone_score * 100).toFixed(0)}% <small>zone score</small></span>
	</div>
	{#if report.zone_model.recommendations.length > 0}
		<div class="recommendations">
			<h3>Recommendations</h3>
			<ul>
				{#each report.zone_model.recommendations as rec}
					<li>{rec}</li>
				{/each}
			</ul>
		</div>
	{/if}
	<h3>Zones</h3>
	<table class="data-table">
		<thead>
			<tr>
				<th>Name</th>
				<th>Security Level</th>
				<th>Purdue Levels</th>
				<th>Assets</th>
			</tr>
		</thead>
		<tbody>
			{#each report.zone_model.zones as zone}
				<tr>
					<td>{zone.name}</td>
					<td><span class="sl-badge {slClass(zone.security_level)}">{slLabel(zone.security_level)}</span></td>
					<td>{zone.purdue_levels.map((l) => 'L' + l).join(', ') || '—'}</td>
					<td>{zone.asset_count.toLocaleString()}</td>
				</tr>
			{/each}
		</tbody>
	</table>
	{#if report.zone_model.conduits.length > 0}
		<h3>Conduits</h3>
		<table class="data-table">
			<thead>
				<tr>
					<th>From Zone</th>
					<th>To Zone</th>
					<th>Direction</th>
					<th>Rules</th>
					<th>Cross-Purdue</th>
				</tr>
			</thead>
			<tbody>
				{#each report.zone_model.conduits as conduit}
					<tr class={conduit.cross_purdue_risk ? 'row-warning' : ''}>
						<td>{zoneNameById(conduit.src_zone_id)}</td>
						<td>{zoneNameById(conduit.dst_zone_id)}</td>
						<td>{conduit.direction}</td>
						<td>{conduit.rules.length.toLocaleString()}</td>
						<td>{conduit.cross_purdue_risk ? '⚠ Yes' : '—'}</td>
					</tr>
				{/each}
			</tbody>
		</table>
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

	.row-warning td {
		color: #f97316;
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

	.recommendations {
		background: #ca8a0411;
		border: 1px solid #ca8a0444;
		border-radius: 4px;
		padding: 0.75rem 1rem;
		margin-bottom: 1.5rem;
	}

	.recommendations h3 {
		margin: 0 0 0.5rem;
		font-size: 0.9rem;
		color: #ca8a04;
	}

	.recommendations ul {
		margin: 0;
		padding-left: 1.2rem;
		font-size: 0.85rem;
	}

	.recommendations li {
		margin-bottom: 0.25rem;
	}

	h3 {
		margin: 1.5rem 0 0.75rem;
		font-size: 1rem;
		font-weight: 600;
	}
</style>

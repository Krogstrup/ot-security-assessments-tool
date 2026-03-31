<script lang="ts">
	import type { ConnectionStats, PatternAnomaly } from '$lib/types/analysis';
	import PatternRowDetails from './PatternRowDetails.svelte';

	interface Props {
		row: ConnectionStats;
		rowKey: string;
		expanded: boolean;
		anomalyCount: number;
		rowAnomalies: PatternAnomaly[];
		onToggleExpand: (key: string) => void;
		fmtBytes: (bytes: number) => string;
		fmtDuration: (seconds: number) => string;
		buildHistogram: (row: ConnectionStats) => { bins: number[]; edges: number[]; maxCount: number };
		severityColor: (severity: string) => string;
		anomalyLabel: (type: string) => string;
	}

	let {
		row,
		rowKey,
		expanded,
		anomalyCount,
		rowAnomalies,
		onToggleExpand,
		fmtBytes,
		fmtDuration,
		buildHistogram,
		severityColor,
		anomalyLabel
	}: Props = $props();
</script>

<tr
	class="data-row"
	class:anomalous={anomalyCount > 0}
	class:expanded={expanded}
	onclick={() => onToggleExpand(rowKey)}
>
	<td class="mono">{row.src_ip}</td>
	<td class="mono">{row.dst_ip}</td>
	<td><span class="proto-badge">{row.protocol}</span></td>
	<td class="num">{row.port}</td>
	<td class="num">{row.packet_count.toLocaleString()}</td>
	<td class="num">{fmtBytes(row.byte_count)}</td>
	<td class="num">{fmtDuration(row.duration_secs)}</td>
	<td class="num">{row.avg_interval_ms.toFixed(1)} ms</td>
	<td class="num">{row.std_interval_ms.toFixed(1)} ms</td>
	<td class="center">{row.is_periodic ? '✓' : '✗'}</td>
	<td class="num">{row.packets_per_sec.toFixed(1)}</td>
	<td class="center">
		{#if anomalyCount > 0}
			<span class="anomaly-count">{anomalyCount}</span>
		{/if}
	</td>
</tr>

{#if expanded}
	<tr class="detail-row">
		<td colspan="12">
			<PatternRowDetails
				{row}
				{rowAnomalies}
				{buildHistogram}
				{severityColor}
				{anomalyLabel}
			/>
		</td>
	</tr>
{/if}

<style>
	td {
		padding: 6px 10px;
		border-bottom: 1px solid rgba(51, 65, 85, 0.4);
		color: var(--gm-text-secondary);
		white-space: nowrap;
	}

	.data-row {
		cursor: pointer;
		transition: background 0.1s;
	}

	.data-row:hover {
		background: var(--gm-bg-hover);
	}

	.data-row.anomalous td {
		background: rgba(245, 158, 11, 0.05);
	}

	.data-row.anomalous:hover td {
		background: rgba(245, 158, 11, 0.1);
	}

	.data-row.expanded td {
		border-bottom: none;
	}

	.mono {
		font-family: 'JetBrains Mono', monospace;
		font-size: 10px;
	}

	.num {
		text-align: right;
	}

	.center {
		text-align: center;
	}

	.proto-badge {
		font-size: 9px;
		font-weight: 600;
		padding: 2px 6px;
		border-radius: 3px;
		background: rgba(16, 185, 129, 0.15);
		color: #10b981;
		letter-spacing: 0.5px;
	}

	.anomaly-count {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 18px;
		height: 18px;
		border-radius: 50%;
		background: rgba(245, 158, 11, 0.2);
		color: #f59e0b;
		font-size: 10px;
		font-weight: 700;
	}

	.detail-row td {
		padding: 0;
		border-bottom: 1px solid var(--gm-border);
	}
</style>

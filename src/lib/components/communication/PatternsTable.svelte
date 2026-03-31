<script lang="ts">
	import type { ConnectionStats, PatternAnomaly } from '$lib/types/analysis';
	import { PATTERNS_TABLE_COLUMNS } from './patternsTableColumns';
	import PatternsDataRow from './PatternsDataRow.svelte';
	import PatternsPaginationBar from './PatternsPaginationBar.svelte';

	interface Props {
		rows: ConnectionStats[];
		filteredCount: number;
		pageSize: number;
		currentPage: number;
		totalPages: number;
		sortCol: keyof ConnectionStats;
		sortAsc: boolean;
		expandedKey: string | null;
		onSort: (col: keyof ConnectionStats) => void;
		onToggleExpand: (key: string) => void;
		onPageChange: (page: number) => void;
		rowKey: (row: ConnectionStats) => string;
		anomalyCount: (row: ConnectionStats) => number;
		getRowAnomalies: (row: ConnectionStats) => PatternAnomaly[];
		fmtBytes: (bytes: number) => string;
		fmtDuration: (seconds: number) => string;
		buildHistogram: (row: ConnectionStats) => { bins: number[]; edges: number[]; maxCount: number };
		severityColor: (severity: string) => string;
		anomalyLabel: (type: string) => string;
	}

	let {
		rows,
		filteredCount,
		pageSize,
		currentPage,
		totalPages,
		sortCol,
		sortAsc,
		expandedKey,
		onSort,
		onToggleExpand,
		onPageChange,
		rowKey,
		anomalyCount,
		getRowAnomalies,
		fmtBytes,
		fmtDuration,
		buildHistogram,
		severityColor,
		anomalyLabel
	}: Props = $props();
</script>

<div class="table-wrapper">
	<table class="cp-table">
		<thead>
			<tr>
				{#each PATTERNS_TABLE_COLUMNS as [col, label]}
					<th class="sortable" class:active={sortCol === col} onclick={() => onSort(col)}>
						{label}
						{#if sortCol === col}
							<span class="sort-arrow">{sortAsc ? '↑' : '↓'}</span>
						{/if}
					</th>
				{/each}
				<th>Anomalies</th>
			</tr>
		</thead>
		<tbody>
			{#each rows as row (rowKey(row))}
				{@const key = rowKey(row)}
				<PatternsDataRow
					{row}
					rowKey={key}
					expanded={expandedKey === key}
					anomalyCount={anomalyCount(row)}
					rowAnomalies={getRowAnomalies(row)}
					{onToggleExpand}
					{fmtBytes}
					{fmtDuration}
					{buildHistogram}
					{severityColor}
					{anomalyLabel}
				/>
			{/each}
		</tbody>
	</table>
</div>

<PatternsPaginationBar
	{totalPages}
	{currentPage}
	{pageSize}
	{filteredCount}
	{onPageChange}
/>

<style>
	.table-wrapper {
		flex: 1;
		overflow: auto;
	}

	.cp-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 11px;
	}

	.cp-table thead {
		position: sticky;
		top: 0;
		background: var(--gm-bg-secondary);
		z-index: 1;
	}

	.cp-table th {
		padding: 8px 10px;
		text-align: left;
		font-size: 10px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.8px;
		color: var(--gm-text-muted);
		border-bottom: 1px solid var(--gm-border);
		white-space: nowrap;
	}

	.cp-table th.sortable {
		cursor: pointer;
		user-select: none;
	}

	.cp-table th.sortable:hover {
		color: var(--gm-text-secondary);
	}

	.cp-table th.active {
		color: #10b981;
	}

	.sort-arrow {
		margin-left: 4px;
		font-size: 10px;
	}

</style>

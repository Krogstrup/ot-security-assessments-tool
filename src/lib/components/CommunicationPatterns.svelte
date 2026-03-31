<script lang="ts">
	import { onMount } from 'svelte';
	import { getConnectionStats, getPatternAnomalies } from '$lib/api';
	import type { ConnectionStats, PatternAnomaly } from '$lib/types/analysis';
	import PatternsToolbar from './communication/PatternsToolbar.svelte';
	import AnomalySummaryBar from './communication/AnomalySummaryBar.svelte';
	import PatternsTable from './communication/PatternsTable.svelte';
	import {
		anomalyCountForRow,
		anomalyLabel,
		anomaliesForRow,
		buildAnomalyMap,
		buildHistogram,
		fmtBytes,
		fmtDuration,
		listProtocols,
		rowKey,
		severityColor,
		sortStats,
		type PatternsSortColumn
	} from './communication/patternsUtils';

	// ── State ─────────────────────────────────────────────────────

	let stats = $state<ConnectionStats[]>([]);
	let anomalies = $state<PatternAnomaly[]>([]);
	let loading = $state(false);
	let error = $state<string | null>(null);

	// Filters
	let anomaliesOnly = $state(false);
	let protocolFilter = $state('');

	// Sorting
	let sortCol = $state<PatternsSortColumn>('packet_count');
	let sortAsc = $state(false);

	// Expanded row for histogram
	let expandedKey = $state<string | null>(null);

	// Pagination
	const PAGE_SIZE = 50;
	let currentPage = $state(0);

	// ── Data Loading ──────────────────────────────────────────────

	async function load() {
		loading = true;
		error = null;
		currentPage = 0;
		try {
			const [s, a] = await Promise.all([getConnectionStats(), getPatternAnomalies()]);
			stats = s;
			anomalies = a;
		} catch (e) {
			error = String(e);
		} finally {
			loading = false;
		}
	}

	onMount(load);

	// ── Derived / Helpers ─────────────────────────────────────────

	let anomalyMap = $derived(buildAnomalyMap(anomalies));
	let protocols = $derived(listProtocols(stats));

	/** Filtered + sorted rows (all pages) */
	let filteredRows = $derived(
		sortStats(
			stats
			.filter((s) => {
				if (protocolFilter && s.protocol !== protocolFilter) return false;
				if (anomaliesOnly && anomalyCountForRow(s, anomalyMap) === 0) return false;
				return true;
			}),
			sortCol,
			sortAsc
		)
	);

	/** Current page of rows — only these are rendered in the DOM */
	let rows = $derived(
		filteredRows.slice(currentPage * PAGE_SIZE, (currentPage + 1) * PAGE_SIZE)
	);

	let totalPages = $derived(Math.max(1, Math.ceil(filteredRows.length / PAGE_SIZE)));

	function setSort(col: PatternsSortColumn) {
		if (sortCol === col) {
			sortAsc = !sortAsc;
		} else {
			sortCol = col;
			sortAsc = false;
		}
		currentPage = 0;
	}

	function toggleExpand(key: string) {
		expandedKey = expandedKey === key ? null : key;
	}
</script>

<div class="cp-container">
	<PatternsToolbar
		filteredCount={filteredRows.length}
		totalCount={stats.length}
		{protocols}
		{protocolFilter}
		{anomaliesOnly}
		onProtocolFilterChange={(protocol) => (protocolFilter = protocol)}
		onAnomaliesOnlyChange={(enabled) => (anomaliesOnly = enabled)}
		onRefresh={load}
	/>

	<AnomalySummaryBar {anomalies} {severityColor} />

	<!-- Main content -->
	{#if loading}
		<div class="empty-state">
			<div class="spinner"></div>
			<p>Loading pattern data…</p>
		</div>
	{:else if error}
		<div class="empty-state error-state">
			<p>&#9888; {error}</p>
			<button class="tool-btn" onclick={load}>Retry</button>
		</div>
	{:else if stats.length === 0}
		<div class="empty-state">
			<div class="empty-icon">&#8771;</div>
			<h3>No Pattern Data</h3>
			<p>Import a PCAP file or run a live capture to see communication patterns.</p>
		</div>
	{:else}
		<PatternsTable
			{rows}
			filteredCount={filteredRows.length}
			pageSize={PAGE_SIZE}
			{currentPage}
			{totalPages}
			{sortCol}
			{sortAsc}
			{expandedKey}
			onSort={setSort}
			onToggleExpand={toggleExpand}
			onPageChange={(page) => (currentPage = page)}
			{rowKey}
			anomalyCount={(row) => anomalyCountForRow(row, anomalyMap)}
			getRowAnomalies={(row) => anomaliesForRow(row, anomalyMap)}
			{fmtBytes}
			{fmtDuration}
			{buildHistogram}
			{severityColor}
			{anomalyLabel}
		/>
	{/if}
</div>

<style>
	.cp-container {
		display: flex;
		flex-direction: column;
		height: 100%;
		overflow: hidden;
		font-size: 12px;
	}

	/* ── Empty / Error States ────────────────────── */

	.empty-state {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 12px;
		color: var(--gm-text-muted);
		text-align: center;
	}

	.empty-icon {
		font-size: 48px;
		opacity: 0.3;
	}

	.empty-state h3 {
		font-size: 14px;
		font-weight: 600;
		color: var(--gm-text-secondary);
		margin: 0;
	}

	.empty-state p {
		font-size: 12px;
		margin: 0;
	}

	.tool-btn {
		padding: 5px 12px;
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		color: var(--gm-text-secondary);
		font-family: inherit;
		font-size: 11px;
		cursor: pointer;
		transition: all 0.15s;
	}

	.tool-btn:hover {
		background: var(--gm-bg-hover);
		color: var(--gm-text-primary);
	}

	.error-state {
		color: #ef4444;
	}

	.spinner {
		width: 28px;
		height: 28px;
		border: 3px solid var(--gm-border);
		border-top-color: #10b981;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}
</style>

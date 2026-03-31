<script lang="ts">
	import { tick } from 'svelte';
	import { segmentationReport } from '$lib/stores/analysis';
	import type { EnforcementFormat } from '$lib/types/segmentation';
	import { runSegmentation, exportEnforcementConfig } from '$lib/api';
	import SegmentationHeader from './segmentation/SegmentationHeader.svelte';
	import SegmentationTabs from './segmentation/SegmentationTabs.svelte';
	import type { SubTab } from './segmentation/SegmentationTabs.svelte';
	import PolicyGroupsSection from './segmentation/PolicyGroupsSection.svelte';
	import ZonesConduitsSection from './segmentation/ZonesConduitsSection.svelte';
	import MatrixSection from './segmentation/MatrixSection.svelte';
	import EnforcementSection from './segmentation/EnforcementSection.svelte';
	import SimulationSection from './segmentation/SimulationSection.svelte';
	import {
		blockedPercent,
		riskClass,
		scorePercent,
		SEGMENTATION_LOADING_STAGES,
		slClass,
		slLabel
	} from './segmentation/segmentationViewUtils';

	let activeTab = $state<SubTab>('groups');
	let isRunning = $state(false);
	let loadingStage = $state('');
	let error = $state<string | null>(null);

	// Enforcement export
	let selectedFormat = $state<EnforcementFormat>('cisco_ios_acl');
	let exportedContent = $state<string>('');
	let isExporting = $state(false);

	// Simulation pagination
	let visibleBlockCount = $state(50);
	let visibleFpCount = $state(50);

	async function handleRunSegmentation() {
		isRunning = true;
		error = null;
		visibleBlockCount = 50;
		visibleFpCount = 50;

		let stageIndex = 0;
		const stageInterval = setInterval(() => {
			if (stageIndex < SEGMENTATION_LOADING_STAGES.length) {
				loadingStage = SEGMENTATION_LOADING_STAGES[stageIndex];
				stageIndex++;
			}
		}, 800);

		try {
			loadingStage = SEGMENTATION_LOADING_STAGES[0];
			await tick();
			const result = await runSegmentation();
			segmentationReport.set(result);
		} catch (e) {
			error = String(e);
		} finally {
			clearInterval(stageInterval);
			isRunning = false;
			loadingStage = '';
		}
	}

	async function handleExport() {
		isExporting = true;
		exportedContent = '';
		try {
			exportedContent = await exportEnforcementConfig(selectedFormat);
		} catch (e) {
			error = String(e);
		} finally {
			isExporting = false;
		}
	}

	function copyToClipboard(text: string) {
		navigator.clipboard.writeText(text).catch(() => {});
	}

	function zoneNameById(id: string): string {
		const report = $segmentationReport;
		if (!report) return id;
		return report.zone_model.zones.find((zone) => zone.id === id)?.name ?? id;
	}
</script>

<div class="segmentation-view">
	<SegmentationHeader
		{isRunning}
		hasReport={$segmentationReport !== null}
		onRunSegmentation={handleRunSegmentation}
	/>

	{#if error}
		<div class="error-banner">{error}</div>
	{/if}

	{#if isRunning}
		<div class="loading-overlay">
			<div class="loading-spinner"></div>
			<p class="loading-stage">{loadingStage}</p>
		</div>
	{/if}

	{#if $segmentationReport}
		<SegmentationTabs
			{activeTab}
			onTabChange={(tab) => (activeTab = tab)}
		/>

		{#if activeTab === 'groups'}
			<PolicyGroupsSection
				report={$segmentationReport}
				{slLabel}
				{slClass}
			/>

		{:else if activeTab === 'zones'}
			<ZonesConduitsSection
				report={$segmentationReport}
				{slLabel}
				{slClass}
				{zoneNameById}
			/>

		{:else if activeTab === 'matrix'}
			<MatrixSection
				report={$segmentationReport}
				{zoneNameById}
				{riskClass}
			/>

		{:else if activeTab === 'enforcement'}
			<EnforcementSection
				report={$segmentationReport}
				{selectedFormat}
				{exportedContent}
				{isExporting}
				onFormatChange={(format) => (selectedFormat = format)}
				onExport={handleExport}
				onCopy={copyToClipboard}
			/>

		{:else if activeTab === 'simulation'}
			<SimulationSection
				report={$segmentationReport}
				{visibleBlockCount}
				{visibleFpCount}
				{zoneNameById}
				{blockedPercent}
				{scorePercent}
				onShowMoreBlocks={() => (visibleBlockCount += 50)}
				onShowMoreFalsePositives={() => (visibleFpCount += 50)}
			/>
		{/if}

	{:else if !isRunning}
		<div class="empty-state">
			<p>No segmentation analysis has been run yet.</p>
			<p>Click <strong>Run Segmentation Analysis</strong> to generate IEC 62443 zone recommendations,
			a least-privilege communication matrix, enforcement configurations, and a policy simulation.</p>
			<p class="hint">Requires at least one PCAP import or live capture session with discovered assets.</p>
		</div>
	{/if}
</div>

<style>
	.segmentation-view {
		padding: 1.5rem;
		max-width: 1200px;
		height: 100%;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.error-banner {
		background: var(--severity-critical-bg, #3b1a1a);
		border: 1px solid var(--severity-critical, #ef4444);
		border-radius: 4px;
		padding: 0.75rem 1rem;
		margin-bottom: 1rem;
		color: var(--severity-critical, #ef4444);
		flex-shrink: 0;
	}

	/* ── Loading Indicator ───────────────────────────── */

	.loading-overlay {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		padding: 3rem 2rem;
		flex: 1;
	}

	.loading-spinner {
		width: 28px;
		height: 28px;
		border: 3px solid var(--gm-border, #333);
		border-top-color: #10b981;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.loading-stage {
		font-size: 0.9rem;
		color: var(--text-muted, #888);
		margin: 0;
		animation: pulse 1.5s ease-in-out infinite;
	}

	@keyframes pulse {
		0%, 100% { opacity: 0.6; }
		50% { opacity: 1; }
	}

	/* Empty state */
	.empty-state {
		text-align: center;
		padding: 3rem 2rem;
		color: var(--text-muted, #888);
		max-width: 500px;
		margin: 0 auto;
	}

	.empty-state p { margin-bottom: 0.75rem; }
	.hint { font-size: 0.82rem; color: var(--text-muted, #888); }
</style>

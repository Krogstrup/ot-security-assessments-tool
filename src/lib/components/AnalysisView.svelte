<script lang="ts">
	import { onMount } from 'svelte';
	import { runAnalysis, getFindings, getPurdueAssignments, getAnomalies, getAssets, getSwitchSecurityFindings, getCorrelatedAlerts, clearAlerts, getMalwareFindings, getComplianceReport } from '$lib/api';
	import { anomalies, analysisSummary, findings, purdueAssignments } from '$lib/stores/analysis';
	import { assetCount, assets, selectedAssetId } from '$lib/stores/core';
	import { activeTab, type ViewTab } from '$lib/stores/navigation';
	import type {
		AnalysisResult,
		ComplianceMapping,
		CorrelatedAlert,
		MalwareFinding
	} from '$lib/types/analysis';
	import type { SwitchSecurityFinding } from '$lib/types/deep-parse';
	import { PURDUE_LABELS as purdueLabels } from '$lib/constants';
	import AnalysisHeader from './analysis/AnalysisHeader.svelte';
	import AnalysisContent from './analysis/AnalysisContent.svelte';
	import AnalysisSectionTabs from './analysis/AnalysisSectionTabs.svelte';
	import type { AnalysisSection } from './analysis/AnalysisSectionTabs.svelte';
	import { loadAnalysisSection } from './analysis/analysisSectionLoader';
	import {
		getSeverityIcon,
		getTypeLabel,
		purdueColors,
		severityColors,
		severityOrder
	} from './analysis/analysisViewConfig';

	let activeSection = $state<AnalysisSection>('summary');
	let switchFindings = $state<SwitchSecurityFinding[]>([]);
	let loadingSwitchFindings = $state(false);
	let correlatedAlerts = $state<CorrelatedAlert[]>([]);
	let loadingAlerts = $state(false);

	async function loadCorrelatedAlerts() {
		loadingAlerts = true;
		try {
			correlatedAlerts = await getCorrelatedAlerts();
		} catch {
			correlatedAlerts = [];
		} finally {
			loadingAlerts = false;
		}
	}

	async function handleClearAlerts() {
		await clearAlerts();
		correlatedAlerts = [];
	}

	async function loadSwitchFindings() {
		loadingSwitchFindings = true;
		try {
			switchFindings = await getSwitchSecurityFindings();
		} catch {
			switchFindings = [];
		} finally {
			loadingSwitchFindings = false;
		}
	}
	let isRunning = $state(false);
	let lastRunTime = $state<string | null>(null);
	let error = $state<string | null>(null);

	// ─── ICS Malware Signatures ──────────────────────────
	let malwareFindings = $state<MalwareFinding[]>([]);
	let loadingMalware = $state(false);

	async function loadMalwareFindings() {
		loadingMalware = true;
		try {
			malwareFindings = await getMalwareFindings();
		} catch (e) {
			error = `Malware detection failed: ${e}`;
		} finally {
			loadingMalware = false;
		}
	}

	// ─── Compliance Framework Mapping ───────────────────
	let complianceFramework = $state<'iec62443' | 'nist80082' | 'nerccip'>('iec62443');
	let complianceMappings = $state<ComplianceMapping[]>([]);
	let loadingCompliance = $state(false);

	async function loadComplianceReport() {
		loadingCompliance = true;
		try {
			complianceMappings = await getComplianceReport(complianceFramework);
		} catch (e) {
			error = `Compliance mapping failed: ${e}`;
		} finally {
			loadingCompliance = false;
		}
	}


	onMount(async () => {
		// Load previous results if available
		try {
			const [f, p, a] = await Promise.all([
				getFindings(),
				getPurdueAssignments(),
				getAnomalies()
			]);
			findings.set(f);
			purdueAssignments.set(p);
			anomalies.set(a);
		} catch {
			// No previous results
		}
	});

	async function handleRunAnalysis() {
		isRunning = true;
		error = null;
		try {
			const result: AnalysisResult = await runAnalysis();
			findings.set(result.findings);
			purdueAssignments.set(result.purdue_assignments);
			anomalies.set(result.anomalies);
			analysisSummary.set(result.summary);
			lastRunTime = new Date().toLocaleTimeString();

			// Refresh assets (Purdue levels may have been auto-assigned)
			const assetPage = await getAssets(0, 200);
			assets.set(assetPage.assets);
			assetCount.set(assetPage.total);
		} catch (e) {
			error = String(e);
		} finally {
			isRunning = false;
		}
	}

	function navigateToAsset(ip: string) {
		selectedAssetId.set(ip);
		activeTab.set('inventory' as ViewTab);
	}

	function setActiveSection(section: AnalysisSection) {
		activeSection = section;
		loadAnalysisSection(section, {
			loadSwitchFindings,
			loadCorrelatedAlerts,
			loadMalwareFindings,
			loadComplianceReport
		});
	}

</script>

<div class="analysis-container">
	<AnalysisHeader
		{lastRunTime}
		{isRunning}
		hasAssets={$assets.length > 0}
		onRunAnalysis={handleRunAnalysis}
	/>

	{#if error}
		<div class="error-banner">{error}</div>
	{/if}

	{#if $assets.length === 0}
		<div class="empty-state">
			<div class="empty-icon">⚑</div>
			<p>Import PCAPs or start a live capture first, then run security analysis.</p>
		</div>
	{:else}
		<AnalysisSectionTabs
			{activeSection}
			findingsCount={$findings.length}
			anomaliesCount={$anomalies.length}
			switchFindingsCount={switchFindings.length}
			correlatedAlertsCount={correlatedAlerts.length}
			malwareFindingsCount={malwareFindings.length}
			onSectionChange={setActiveSection}
		/>

		<AnalysisContent
			{activeSection}
			analysisSummary={$analysisSummary}
			findings={$findings}
			purdueAssignments={$purdueAssignments}
			anomalies={$anomalies}
			{switchFindings}
			{loadingSwitchFindings}
			correlatedAlerts={correlatedAlerts}
			{loadingAlerts}
			malwareFindings={malwareFindings}
			{loadingMalware}
			{complianceFramework}
			complianceMappings={complianceMappings}
			{loadingCompliance}
			{severityOrder}
			{severityColors}
			{purdueColors}
			purdueLabels={purdueLabels}
			{getSeverityIcon}
			{getTypeLabel}
			onNavigateAsset={navigateToAsset}
			onRefreshSwitchFindings={loadSwitchFindings}
			onRefreshAlerts={loadCorrelatedAlerts}
			onClearAlerts={handleClearAlerts}
			onRefreshMalware={loadMalwareFindings}
			onRefreshCompliance={loadComplianceReport}
			onComplianceFrameworkChange={(framework) => {
				complianceFramework = framework;
				void loadComplianceReport();
			}}
		/>
	{/if}
</div>

<style>
	.analysis-container {
		height: 100%;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	/* ── Error / Empty State ─────────────────────────── */

	.error-banner {
		margin: 12px 20px;
		padding: 10px 14px;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 6px;
		color: #ef4444;
		font-size: 11px;
	}

	.empty-state {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		color: var(--gm-text-muted);
		gap: 12px;
	}

	.empty-icon {
		font-size: 36px;
		opacity: 0.4;
	}

</style>

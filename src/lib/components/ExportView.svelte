<script lang="ts">
	import { assetCount, connectionCount } from '$lib/stores/core';
	import type { ReportConfig } from '$lib/types/operations';
	import { exportSbom } from '$lib/api';
	import { extractTopologyImageData } from './export/exportUtils';
	import {
		exportTopologyImageFlow,
		generatePdfFlow,
		runPathExportFlow,
		SIMPLE_EXPORTS
	} from './export/exportFlows';
	import ExportHeader from './export/ExportHeader.svelte';
	import ExportStatusBanner from './export/ExportStatusBanner.svelte';
	import DataExportsSection from './export/DataExportsSection.svelte';
	import PdfReportSection from './export/PdfReportSection.svelte';
	import RemediationContainer from './export/RemediationContainer.svelte';
	import AllowlistContainer from './export/AllowlistContainer.svelte';
	import TopologyImageSection from './export/TopologyImageSection.svelte';
	import SbomSection from './export/SbomSection.svelte';
	import StixSection from './export/StixSection.svelte';

	let assessorName = $state('');
	let clientName = $state('');
	let assessmentDate = $state(new Date().toISOString().slice(0, 10));
	let reportTitle = $state('');
	let includeExecSummary = $state(true);
	let includeAssetInventory = $state(true);
	let includeProtocolAnalysis = $state(true);
	let includeFindings = $state(true);
	let includeRecommendations = $state(true);

	let sbomFormat = $state<'csv' | 'json'>('json');

	let statusMessage = $state('');
	let statusType = $state<'success' | 'error' | 'info'>('info');
	let busyAction = $state<string | null>(null);

	let hasData = $derived($assetCount > 0 || $connectionCount > 0);

	function showStatus(msg: string, type: 'success' | 'error' | 'info' = 'info') {
		statusMessage = msg;
		statusType = type;
		if (type !== 'error') {
			setTimeout(() => {
				statusMessage = '';
			}, 6000);
		}
	}

	const flowHooks = {
		setBusyAction: (value: string | null) => {
			busyAction = value;
		},
		showStatus
	};

	function setBusyAction(value: string | null) {
		busyAction = value;
	}

	async function runSimpleExport(key: keyof typeof SIMPLE_EXPORTS) {
		await runPathExportFlow(SIMPLE_EXPORTS[key], flowHooks);
	}

	async function handleGeneratePdf() {
		if (!assessorName.trim() || !clientName.trim()) {
			showStatus('Assessor Name and Client Name are required', 'error');
			return;
		}
		const config: ReportConfig = {
			assessor_name: assessorName.trim(),
			client_name: clientName.trim(),
			assessment_date: assessmentDate || undefined,
			title: reportTitle.trim() || undefined,
			include_executive_summary: includeExecSummary,
			include_asset_inventory: includeAssetInventory,
			include_protocol_analysis: includeProtocolAnalysis,
			include_findings: includeFindings,
			include_recommendations: includeRecommendations
		};
		const defaultName = `${clientName.trim().replace(/\s+/g, '_')}_ICS_Assessment_${assessmentDate}.pdf`;
		await generatePdfFlow(config, defaultName, flowHooks);
	}

	async function handleExportSbom() {
		const ext = sbomFormat === 'csv' ? 'csv' : 'json';
		await runPathExportFlow({
			action: 'sbom',
			dialogTitle: 'Export SBOM',
			defaultName: `sbom.${ext}`,
			filterName: `${ext.toUpperCase()} Files`,
			extensions: [ext],
			errorPrefix: 'SBOM export failed',
			exporter: (path) => exportSbom(sbomFormat, path)
		}, flowHooks);
	}


	async function handleExportTopologyImage(format: 'png' | 'svg') {
		const imageData = extractTopologyImageData(format);
		await exportTopologyImageFlow(format, imageData, flowHooks);
	}

</script>

<div class="export-container">
	<ExportHeader />

	<div class="export-content">
		<ExportStatusBanner message={statusMessage} type={statusType} onDismiss={() => (statusMessage = '')} />

		{#if !hasData}
			<div class="empty-state">
				<p>No data to export. Import PCAPs or start a live capture first.</p>
			</div>
		{/if}

		<DataExportsSection
			assetCount={$assetCount}
			connectionCount={$connectionCount}
			{hasData}
			{busyAction}
			onExportAssetsCsv={() => runSimpleExport('assets_csv')}
			onExportConnectionsCsv={() => runSimpleExport('conn_csv')}
			onExportTopologyJson={() => runSimpleExport('topo_json')}
			onExportAssetsJson={() => runSimpleExport('assets_json')}
		/>

		<TopologyImageSection
			{busyAction}
			onExportPng={() => handleExportTopologyImage('png')}
			onExportSvg={() => handleExportTopologyImage('svg')}
		/>

		<PdfReportSection
			{hasData}
			{busyAction}
			bind:assessorName
			bind:clientName
			bind:assessmentDate
			bind:reportTitle
			bind:includeExecSummary
			bind:includeAssetInventory
			bind:includeProtocolAnalysis
			bind:includeFindings
			bind:includeRecommendations
			onGeneratePdf={handleGeneratePdf}
		/>

		<SbomSection
			{hasData}
			{busyAction}
			{sbomFormat}
			onFormatChange={(format) => (sbomFormat = format)}
			onExportSbom={handleExportSbom}
		/>

		<StixSection {hasData} {busyAction} onExportStix={() => runSimpleExport('stix')} />

		<RemediationContainer
			{busyAction}
			onBusyActionChange={setBusyAction}
			onShowStatus={showStatus}
		/>

		<AllowlistContainer
			{hasData}
			{busyAction}
			onShowStatus={showStatus}
			onExportAllowlistCsv={() => runSimpleExport('allowlist_csv')}
			onExportFirewallRules={() => runSimpleExport('fw_rules')}
		/>
	</div>
</div>

<style>
	.export-container {
		display: flex;
		flex-direction: column;
		height: 100%;
	}

	.export-content {
		flex: 1;
		overflow-y: auto;
		padding: 20px 24px;
		display: flex;
		flex-direction: column;
		gap: 20px;
		max-width: 800px;
	}

	.empty-state {
		background: var(--gm-bg-secondary);
		border: 1px solid var(--gm-border);
		border-radius: 8px;
		padding: 24px;
		text-align: center;
		color: var(--gm-text-muted);
		font-size: 12px;
	}
</style>

<script lang="ts">
	import { onMount } from 'svelte';
	import { openPathDialog } from '$lib/utils/dialog';
	import { activeTab } from '$lib/stores/navigation';
	import { selectedAssetId } from '$lib/stores/core';
	import { physicalHighlightIp, physicalTopology } from '$lib/stores/physical';
	import type { RedundancyInfo } from '$lib/types/deep-parse';
	import type { InferredTopology, PhysicalPort, PhysicalSwitch } from '$lib/types/operations';
	import { type PhysicalImportType } from './physical/physicalImport';
	import {
		handleClearFlow,
		handleImportFlow,
		handleRunInferenceFlow,
		initializePhysicalView,
		loadRedundancyFlow,
		type PhysicalViewHooks
	} from './physical/physicalViewHandlers';

	import PhysicalImportPanel from './physical/PhysicalImportPanel.svelte';
	import PhysicalGraph from './physical/PhysicalGraph.svelte';
	import PhysicalDetailPanel from './physical/PhysicalDetailPanel.svelte';
	import InferredTopologyPanel from './physical/InferredTopologyPanel.svelte';
	import RedundancyPanel from './physical/RedundancyPanel.svelte';
	import PhysicalToolbar from './physical/PhysicalToolbar.svelte';
	import PhysicalLegend from './physical/PhysicalLegend.svelte';
	import DeviceLocationsPanel from './physical/DeviceLocationsPanel.svelte';

	let graphRef = $state<{ fit: () => void; runLayout: () => void } | null>(null);
	let activePhysicalTab = $state<'imported' | 'inferred' | 'redundancy'>('imported');
	let inferredTopology = $state<InferredTopology | null>(null);
	let inferring = $state(false);
	let redundancyProtocols = $state<RedundancyInfo[]>([]);
	let loadingRedundancy = $state(false);
	let importType = $state<PhysicalImportType>('config');
	let switchHostname = $state('');
	let importError = $state('');
	let importSuccess = $state('');
	let importing = $state(false);
	let selectedSwitch = $state<PhysicalSwitch | null>(null);
	let selectedPort = $state<PhysicalPort | null>(null);

	let currentTopo = $derived($physicalTopology);
	const switchOptions = $derived(currentTopo.switches.map((s) => s.hostname));
	const deviceLocations = $derived(Object.values(currentTopo.device_locations ?? {}));
	const redundancyManagerCount = $derived(redundancyProtocols.filter((item) => item.is_manager).length);
	const redundancyTcCount = $derived(redundancyProtocols.filter((item) => item.topology_change).length);

	const hooks: PhysicalViewHooks = {
		getImportType: () => importType,
		getSwitchHostname: () => switchHostname,
		setImporting: (v) => (importing = v),
		setImportError: (v) => (importError = v),
		setImportSuccess: (v) => (importSuccess = v),
		setInferring: (v) => (inferring = v),
		setInferredTopology: (v) => (inferredTopology = v),
		setRedundancyProtocols: (v) => (redundancyProtocols = v),
		setLoadingRedundancy: (v) => (loadingRedundancy = v),
		setSelectedSwitch: (v) => (selectedSwitch = v),
		setSelectedPort: (v) => (selectedPort = v)
	};

	onMount(async () => {
		await initializePhysicalView(hooks);
	});

	const handleImport = () =>
		handleImportFlow(
			() => openPathDialog({ multiple: false, filters: [{ name: 'Text Files', extensions: ['txt', 'cfg', 'conf', 'log'] }] }),
			hooks
		);
	const handleClear = () => handleClearFlow(hooks);
	const handleRunInference = () => handleRunInferenceFlow(hooks);
	const loadRedundancy = () => loadRedundancyFlow(hooks);

	function showInLogical(ip: string) {
		selectedAssetId.set(ip);
		activeTab.set('topology');
	}
</script>

<div class="physical-container">
	<PhysicalToolbar
		activeTab={activePhysicalTab}
		switchCount={currentTopo.switches.length}
		linkCount={currentTopo.links.length}
		mappedDeviceCount={deviceLocations.length}
		inferredSubnetCount={inferredTopology?.subnets.length ?? 0}
		inferredGatewayCount={inferredTopology?.gateways.length ?? 0}
		inferredSwitchCandidateCount={inferredTopology?.switch_candidates.length ?? 0}
		redundancyDeviceCount={redundancyProtocols.length}
		{redundancyManagerCount}
		{redundancyTcCount}
		{inferring}
		onTabChange={(tab) => {
			activePhysicalTab = tab;
			if (tab === 'redundancy') loadRedundancy();
		}}
		onFit={() => graphRef?.fit()}
		onRelayout={() => graphRef?.runLayout()}
		onClear={handleClear}
		onRunInference={handleRunInference}
	/>

	{#if activePhysicalTab === 'imported'}
		<div class="physical-body">
			<PhysicalImportPanel
				{importType}
				{switchHostname}
				{switchOptions}
				{importError}
				{importSuccess}
				{importing}
				onTypeChange={(type) => (importType = type)}
				onSwitchChange={(hostname) => (switchHostname = hostname)}
				onImport={handleImport}
				onClear={handleClear}
			/>

			<div class="graph-container">
				<PhysicalGraph
					bind:this={graphRef}
					topology={currentTopo}
					highlightIp={$physicalHighlightIp}
					onSelectSwitch={(hostname) => {
						const sw = currentTopo.switches.find(s => s.hostname === hostname);
						selectedSwitch = sw ?? null;
						selectedPort = null;
					}}
					onSelectPort={(switchHostname, portName) => {
						const sw = currentTopo.switches.find(s => s.hostname === switchHostname);
						if (sw) {
							selectedSwitch = sw;
							selectedPort = sw.ports.find(p => p.name === portName) ?? null;
						}
					}}
					onDeselect={() => {
						selectedSwitch = null;
						selectedPort = null;
					}}
				/>

				<PhysicalDetailPanel
					{selectedSwitch}
					{selectedPort}
					onClose={() => {
						selectedSwitch = null;
						selectedPort = null;
					}}
					onShowInLogical={showInLogical}
					onSelectPort={(port) => (selectedPort = port)}
				/>
			</div>

			{#if deviceLocations.length > 0}
				<DeviceLocationsPanel locations={deviceLocations} onShowInLogical={showInLogical} />
			{/if}
		</div>

		<PhysicalLegend />
	{:else if activePhysicalTab === 'inferred'}
		<InferredTopologyPanel {inferredTopology} {inferring} onRunInference={handleRunInference} />
	{:else}
		<RedundancyPanel {redundancyProtocols} loading={loadingRedundancy} />
	{/if}
</div>

<style>
	.physical-container {
		display: flex;
		flex-direction: column;
		height: 100%;
		background: var(--gm-bg-primary);
	}

	.physical-body {
		flex: 1;
		display: flex;
		overflow: hidden;
		gap: 0;
	}

	.graph-container {
		flex: 1;
		position: relative;
		display: flex;
		overflow: hidden;
	}
</style>

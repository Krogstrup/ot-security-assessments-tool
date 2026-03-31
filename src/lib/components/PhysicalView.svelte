<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { openPathDialog } from '$lib/utils/dialog';
	import {
		physicalTopology,
		physicalHighlightIp,
		selectedAssetId,
		activeTab,
		assets
	} from '$lib/stores';
	import type { PhysicalTopology, PhysicalSwitch, PhysicalPort, InferredTopology, RedundancyInfo } from '$lib/types';
	import {
		importCiscoConfig,
		importMacTableAuto,
		importNeighborTable,
		importArpTable,
		getPhysicalTopology,
		clearPhysicalTopology,
		importNetworkConfig,
		runTopologyInference,
		getInferredTopology,
		getRedundancyProtocols
	} from '$lib/api';

	import PhysicalImportPanel from './physical/PhysicalImportPanel.svelte';
	import PhysicalGraph from './physical/PhysicalGraph.svelte';
	import PhysicalDetailPanel from './physical/PhysicalDetailPanel.svelte';
	import InferredTopologyPanel from './physical/InferredTopologyPanel.svelte';
	import RedundancyPanel from './physical/RedundancyPanel.svelte';

	// ── Graph Ref ──────────────────────────────────────────────────
	let graphRef = $state<{ fit: () => void; runLayout: () => void } | null>(null);

	// ── Tab State ──────────────────────────────────────────────────
	let activePhysicalTab = $state<'imported' | 'inferred' | 'redundancy'>('imported');
	let inferredTopology = $state<InferredTopology | null>(null);
	let inferring = $state(false);

	// ── Redundancy State ───────────────────────────────────────────
	let redundancyProtocols = $state<RedundancyInfo[]>([]);
	let loadingRedundancy = $state(false);

	// ── Import State ───────────────────────────────────────────────
	let importType = $state<'config' | 'mac' | 'cdp' | 'arp'>('config');
	let switchHostname = $state('');
	let importError = $state('');
	let importSuccess = $state('');
	let importing = $state(false);

	// ── Detail Panel ───────────────────────────────────────────────
	let selectedSwitch = $state<PhysicalSwitch | null>(null);
	let selectedPort = $state<PhysicalPort | null>(null);

	// ── Store Subscriptions ────────────────────────────────────────
	let currentTopo = $state<PhysicalTopology>({ switches: [], links: [], device_locations: {} });

	const unsubTopo = physicalTopology.subscribe((t) => {
		currentTopo = t;
	});

	onMount(async () => {
		// Load existing physical topology
		try {
			const topo = await getPhysicalTopology();
			physicalTopology.set(topo);
		} catch {
			// Expected in browser dev mode
		}
		// Load previously computed inferred topology if available
		try {
			const inferred = await getInferredTopology();
			if (inferred) {
				inferredTopology = inferred;
			}
		} catch {
			// Expected in browser dev mode
		}
	});

	onDestroy(() => {
		unsubTopo();
	});

	// ────────────────────────────────────────────────────────────────
	// IMPORT HANDLERS
	// ────────────────────────────────────────────────────────────────

	async function handleImport() {
		importError = '';
		importSuccess = '';

		const result = await openPathDialog({
			multiple: false,
			filters: [{ name: 'Text Files', extensions: ['txt', 'cfg', 'conf', 'log'] }]
		});

		if (!result) return;
		const filePath = result as string;

		importing = true;
		try {
			let topo: PhysicalTopology;

			switch (importType) {
				case 'config':
					topo = await importNetworkConfig(filePath);
					importSuccess = `Imported config from ${filePath.split(/[/\\]/).pop()}`;
					break;
				case 'mac':
					if (!switchHostname.trim()) {
						importError = 'Select a switch hostname for MAC table import';
						importing = false;
						return;
					}
					topo = await importMacTableAuto(filePath, switchHostname.trim());
					importSuccess = `Imported MAC table for ${switchHostname}`;
					break;
				case 'cdp':
					if (!switchHostname.trim()) {
						importError = 'Select a switch hostname for CDP/LLDP import';
						importing = false;
						return;
					}
					topo = await importNeighborTable(filePath, switchHostname.trim());
					importSuccess = `Imported neighbors for ${switchHostname}`;
					break;
				case 'arp':
					topo = await importArpTable(filePath);
					importSuccess = `Imported ARP table from ${filePath.split(/[/\\]/).pop()}`;
					break;
			}

			physicalTopology.set(topo!);
		} catch (err) {
			importError = `Import failed: ${err}`;
		} finally {
			importing = false;
		}
	}

	async function handleClear() {
		try {
			await clearPhysicalTopology();
			physicalTopology.set({ switches: [], links: [], device_locations: {} });
			selectedSwitch = null;
			selectedPort = null;
			importSuccess = 'Physical topology cleared';
		} catch (err) {
			importError = `Clear failed: ${err}`;
		}
	}

	async function handleRunInference() {
		inferring = true;
		try {
			inferredTopology = await runTopologyInference();
		} catch (err) {
			importError = `Inference failed: ${err}`;
		} finally {
			inferring = false;
		}
	}

	async function loadRedundancy() {
		loadingRedundancy = true;
		try {
			redundancyProtocols = await getRedundancyProtocols();
		} catch {
			redundancyProtocols = [];
		} finally {
			loadingRedundancy = false;
		}
	}

	// ────────────────────────────────────────────────────────────────
	// NAVIGATION
	// ────────────────────────────────────────────────────────────────

	function showInLogical(ip: string) {
		selectedAssetId.set(ip);
		activeTab.set('topology');
	}

	// ────────────────────────────────────────────────────────────────
	// DERIVED STATE
	// ────────────────────────────────────────────────────────────────

	const switchOptions = $derived(currentTopo.switches.map((s) => s.hostname));
</script>

<div class="physical-container">
	<!-- Toolbar -->
	<div class="physical-toolbar">
		<div class="toolbar-section">
			<h2 class="view-title">Physical View</h2>
			<span class="toolbar-sep"></span>
			<div class="tab-switcher">
				<button
					class="tab-btn"
					class:active={activePhysicalTab === 'imported'}
					onclick={() => (activePhysicalTab = 'imported')}
				>Imported</button>
				<button
					class="tab-btn"
					class:active={activePhysicalTab === 'inferred'}
					onclick={() => (activePhysicalTab = 'inferred')}
				>Inferred</button>
				<button
					class="tab-btn"
					class:active={activePhysicalTab === 'redundancy'}
					onclick={() => { activePhysicalTab = 'redundancy'; loadRedundancy(); }}
				>Ring Redundancy</button>
			</div>
			<span class="toolbar-sep"></span>
			{#if activePhysicalTab === 'imported'}
				<span class="switch-count">{currentTopo.switches.length} switches</span>
				<span class="link-count">{currentTopo.links.length} links</span>
				<span class="device-count">{Object.keys(currentTopo.device_locations).length} mapped devices</span>
			{:else if activePhysicalTab === 'inferred'}
				<span class="switch-count">{inferredTopology?.subnets.length ?? 0} subnets</span>
				<span class="link-count">{inferredTopology?.gateways.length ?? 0} gateways</span>
				<span class="device-count">{inferredTopology?.switch_candidates.length ?? 0} switch candidates</span>
			{:else}
				<span class="switch-count">{redundancyProtocols.length} devices</span>
				<span class="link-count">{redundancyProtocols.filter(r => r.is_manager).length} managers</span>
				<span class="device-count">{redundancyProtocols.filter(r => r.topology_change).length} TC events</span>
			{/if}
		</div>
		<div class="toolbar-section">
			{#if activePhysicalTab === 'imported'}
				<button class="tool-btn" onclick={() => graphRef?.fit()}>Fit</button>
				<button class="tool-btn" onclick={() => graphRef?.runLayout()}>Relayout</button>
				<button class="tool-btn danger" onclick={handleClear}>Clear</button>
			{:else}
				<button class="tool-btn" onclick={handleRunInference} disabled={inferring}>
					{inferring ? 'Running...' : 'Run Inference'}
				</button>
			{/if}
		</div>
	</div>

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

			{#if currentTopo.device_locations && Object.keys(currentTopo.device_locations).length > 0}
				<div class="locations-panel">
					<h4>Device Locations</h4>
					<div class="locations-list">
						{#each Object.values(currentTopo.device_locations) as loc}
							<button class="location-item" onclick={() => showInLogical(loc.ip_address)}>
								<span class="loc-ip">{loc.ip_address}</span>
								<span class="loc-detail">
									{loc.switch_hostname} / {loc.port_name}
									{#if loc.vlan}
										<span class="loc-vlan">V{loc.vlan}</span>
									{/if}
								</span>
							</button>
						{/each}
					</div>
				</div>
			{/if}
		</div>

		<!-- Legend -->
		<div class="physical-legend">
			<span class="legend-title">PORTS</span>
			<span class="legend-item">
				<span class="legend-dot" style="background: #475569"></span> Empty
			</span>
			<span class="legend-item">
				<span class="legend-dot" style="background: #3b82f6"></span> Has Devices
			</span>
			<span class="legend-item">
				<span class="legend-dot" style="background: #f59e0b"></span> CDP Link
			</span>
			<span class="legend-item">
				<span class="legend-dot" style="background: #8b5cf6; border-radius: 2px"></span> Trunk
			</span>
			<span class="legend-item">
				<span class="legend-dot" style="background: #374151; opacity: 0.5"></span> Shutdown
			</span>
			<span class="legend-item">
				<span class="legend-dot" style="background: #ef4444"></span> Highlighted
			</span>
		</div>
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

	.physical-toolbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem 1rem;
		background: var(--gm-bg-secondary);
		border-bottom: 1px solid var(--gm-border);
		flex-shrink: 0;
		gap: 1rem;
		flex-wrap: wrap;
	}

	.toolbar-section {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		flex-wrap: wrap;
	}

	.view-title {
		margin: 0;
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--gm-text-primary);
		white-space: nowrap;
	}

	.toolbar-sep {
		width: 1px;
		height: 1.5rem;
		background: var(--gm-border);
	}

	.tab-switcher {
		display: flex;
		gap: 0.35rem;
	}

	.tab-btn {
		padding: 0.35rem 0.75rem;
		border: 1px solid var(--gm-border);
		border-radius: 3px;
		background: transparent;
		color: var(--gm-text-secondary);
		cursor: pointer;
		font-size: 0.8125rem;
		font-weight: 500;
		transition: all 0.2s;
	}

	.tab-btn:hover {
		background: rgba(255, 255, 255, 0.05);
	}

	.tab-btn.active {
		background: #6366f1;
		color: white;
		border-color: #6366f1;
	}

	.switch-count,
	.link-count,
	.device-count {
		font-size: 0.75rem;
		color: var(--gm-text-secondary);
		font-weight: 500;
		white-space: nowrap;
	}

	.tool-btn {
		padding: 0.35rem 0.75rem;
		border: 1px solid var(--gm-border);
		border-radius: 3px;
		background: var(--gm-bg-tertiary);
		color: var(--gm-text-primary);
		cursor: pointer;
		font-size: 0.8125rem;
		font-weight: 500;
		transition: all 0.2s;
	}

	.tool-btn:hover:not(:disabled) {
		background: rgba(255, 255, 255, 0.05);
	}

	.tool-btn.danger {
		color: #ef4444;
	}

	.tool-btn.danger:hover:not(:disabled) {
		background: rgba(239, 68, 68, 0.1);
	}

	.tool-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
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

	.locations-panel {
		position: absolute;
		bottom: 0;
		left: 280px;
		right: 0;
		max-height: 200px;
		background: var(--gm-bg-secondary);
		border-top: 1px solid var(--gm-border);
		padding: 0.75rem;
		overflow-y: auto;
	}

	.locations-panel h4 {
		margin: 0 0 0.5rem 0;
		font-size: 0.875rem;
		color: var(--gm-text-primary);
		font-weight: 600;
	}

	.locations-list {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.location-item {
		padding: 0.5rem 0.75rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.3);
		border-radius: 3px;
		color: #6366f1;
		cursor: pointer;
		font-size: 0.8125rem;
		transition: all 0.2s;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
		white-space: nowrap;
	}

	.location-item:hover {
		background: rgba(99, 102, 241, 0.2);
	}

	.loc-ip {
		font-family: 'JetBrains Mono', monospace;
		font-weight: 600;
	}

	.loc-detail {
		font-size: 0.75rem;
		opacity: 0.8;
	}

	.loc-vlan {
		font-size: 0.7rem;
		background: rgba(0, 0, 0, 0.2);
		padding: 0.1rem 0.3rem;
		border-radius: 2px;
		margin-left: 0.25rem;
	}

	.physical-legend {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.5rem 1rem;
		background: var(--gm-bg-secondary);
		border-top: 1px solid var(--gm-border);
		flex-wrap: wrap;
		flex-shrink: 0;
		font-size: 0.75rem;
	}

	.legend-title {
		font-weight: 600;
		color: var(--gm-text-secondary);
		margin-right: 0.5rem;
	}

	.legend-item {
		display: flex;
		align-items: center;
		gap: 0.35rem;
		color: var(--gm-text-secondary);
	}

	.legend-dot {
		width: 12px;
		height: 12px;
		border-radius: 2px;
	}
</style>

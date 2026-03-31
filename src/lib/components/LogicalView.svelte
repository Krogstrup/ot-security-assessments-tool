<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { openWiresharkForNode } from '$lib/api';
	import { driftChangedIps, driftNewIps } from '$lib/stores/analysis';
	import { assets, selectedAssetId, topology } from '$lib/stores/core';
	import { activeTab } from '$lib/stores/navigation';
	import { physicalHighlightIp } from '$lib/stores/physical';
	import { addFilteredView, addWatchTab, groupingMode } from '$lib/stores/topology-tabs';
	import type { GroupingMode } from '$lib/types/topology';
	import LargeNetworkBanner from './logical/LargeNetworkBanner.svelte';
	import LogicalContextMenu from './logical/LogicalContextMenu.svelte';
	import LogicalEmptyState from './logical/LogicalEmptyState.svelte';
	import LogicalLegend from './logical/LogicalLegend.svelte';
	import LogicalToolbar from './logical/LogicalToolbar.svelte';
	import PurdueOverlay from './PurdueOverlay.svelte';
	import TimelineScrubber from './TimelineScrubber.svelte';
	import {
		closedContextMenu,
		hideContextMenu as hiddenContextMenu,
		openCanvasContextMenu,
		openNodeContextMenu,
		type LogicalContextMenuState
	} from './logical/contextMenuState';
	import { getLogicalLegendEntries, groupingOptions } from './logical/logicalConfig';
	import {
		exportLogicalPng,
		isWiresharkAvailable,
		type LogicalLayoutMode
	} from './logical/logicalLayout';
	import {
		initLogicalGraph,
		runLogicalGraphLayoutOnly,
		updateLogicalGraph
	} from './logical/logicalGraphRuntime';

	let wiresharkAvailable = $state(false);
	let layout = $state<LogicalLayoutMode>('fcose');
	let largeNetworkWarning = $state(false);
	let forceLayout = $state(false);

	let graphContainer: HTMLDivElement;
	let cy: any = null;

	let ctxMenu = $state<LogicalContextMenuState>(closedContextMenu());
	let groupSubmenu = $state(false);

	function closeContextMenu() {
		ctxMenu = hiddenContextMenu();
		groupSubmenu = false;
	}

	function runLayout() {
		if (!cy) return;
		largeNetworkWarning = runLogicalGraphLayoutOnly(cy, layout, forceLayout);
	}

	function updateGraph() {
		if (!cy) return;
		largeNetworkWarning = updateLogicalGraph(cy, {
			graph: $topology,
			mode: $groupingMode,
			assets: $assets,
			newIps: $driftNewIps,
			changedIps: $driftChangedIps,
			layout,
			forceLayout
		});
	}

	async function initCytoscape() {
		cy = await initLogicalGraph({
			container: graphContainer,
			onNodeTap: (nodeId) => selectedAssetId.set(nodeId),
			onBackgroundTap: () => {
				selectedAssetId.set(null);
				closeContextMenu();
			},
			onNodeContext: (position, bounds, nodeId) => {
				ctxMenu = openNodeContextMenu(position, bounds, nodeId);
				groupSubmenu = false;
			},
			onCanvasContext: (position, bounds) => {
				ctxMenu = openCanvasContextMenu(position, bounds);
				groupSubmenu = false;
			}
		});
	}

	function handleWindowClick() {
		if (ctxMenu.show) closeContextMenu();
	}

	function handleGroupBy(mode: GroupingMode) {
		groupingMode.set(mode);
		closeContextMenu();
	}

	function handleWatch() {
		if (ctxMenu.nodeId) addWatchTab(ctxMenu.nodeId, 2);
		closeContextMenu();
	}

	function handleCreateFilteredView() {
		addFilteredView([]);
		closeContextMenu();
	}

	function handleShowInPhysical() {
		if (ctxMenu.nodeId) {
			physicalHighlightIp.set(ctxMenu.nodeId);
			activeTab.set('physical');
		}
		closeContextMenu();
	}

	async function handleOpenInWireshark() {
		if (!ctxMenu.nodeId) return;
		try {
			await openWiresharkForNode(ctxMenu.nodeId);
		} catch (err) {
			console.error('Failed to open Wireshark:', err);
		} finally {
			groupSubmenu = false;
			closeContextMenu();
		}
	}

	$effect(() => {
		updateGraph();
	});

	onMount(async () => {
		await initCytoscape();
		updateGraph();
		wiresharkAvailable = await isWiresharkAvailable();
		window.addEventListener('click', handleWindowClick);
	});

	onDestroy(() => {
		window.removeEventListener('click', handleWindowClick);
		cy?.destroy();
	});

	const legendEntries = $derived(getLogicalLegendEntries());
</script>

<div class="topology-container">
	<LogicalToolbar
		{layout}
		groupingMode={$groupingMode}
		{groupingOptions}
		onLayoutChange={(nextLayout) => {
			layout = nextLayout;
			runLayout();
		}}
		onGroupingChange={(nextMode) => groupingMode.set(nextMode)}
		onFit={() => cy?.fit(undefined, 40)}
		onCenter={() => cy?.center()}
		onRelayout={runLayout}
		onExportPng={() => exportLogicalPng(cy)}
	/>

	<div class="graph-area">
		{#if layout === 'purdue' && $topology.nodes.length > 0}
			<PurdueOverlay />
		{/if}
		<div class="cy-canvas" bind:this={graphContainer}></div>
		{#if $topology.nodes.length === 0}
			<LogicalEmptyState />
		{/if}
		{#if largeNetworkWarning}
			<LargeNetworkBanner
				nodeCount={$topology.nodes.length}
				onForceLayout={() => {
					forceLayout = true;
					largeNetworkWarning = false;
					runLayout();
				}}
				onUsePurdue={() => {
					layout = 'purdue';
					largeNetworkWarning = false;
					runLayout();
				}}
				onDismiss={() => {
					largeNetworkWarning = false;
				}}
			/>
		{/if}
		<TimelineScrubber />
	</div>

	<LogicalContextMenu
		show={ctxMenu.show}
		x={ctxMenu.x}
		y={ctxMenu.y}
		hasNode={Boolean(ctxMenu.nodeId)}
		{wiresharkAvailable}
		{groupSubmenu}
		{groupingOptions}
		selectedGroupingMode={$groupingMode}
		onWatch={handleWatch}
		onShowInPhysical={handleShowInPhysical}
		onOpenInWireshark={handleOpenInWireshark}
		onCreateFilteredView={handleCreateFilteredView}
		onToggleGroupSubmenu={() => (groupSubmenu = !groupSubmenu)}
		onGroupBy={handleGroupBy}
		onClose={closeContextMenu}
	/>

	<LogicalLegend entries={legendEntries} />
</div>

<style>
	.topology-container {
		display: flex;
		flex-direction: column;
		height: 100%;
		position: relative;
	}

	.graph-area {
		flex: 1;
		position: relative;
		background: var(--gm-bg-primary);
		background-image: radial-gradient(circle, var(--gm-bg-dot) 1px, transparent 1px);
		background-size: 24px 24px;
		overflow: hidden;
	}

	.cy-canvas {
		position: absolute;
		inset: 0;
	}
</style>

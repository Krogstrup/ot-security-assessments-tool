<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { selectedAssetId, topology } from '$lib/stores/core';
	import { groupingMode, toggleNodeInFilter, topologyTabs } from '$lib/stores/topology-tabs';
	import type { FilteredViewConfig, GroupingMode, TopologyGraph } from '$lib/types/topology';
	import FilteredToolbar from './filtered/FilteredToolbar.svelte';
	import {
		createFilteredElements,
		filteredCytoscapeStyle,
		runFilteredLayout
	} from './filtered/filteredGraph';

	let { tabId }: { tabId: string } = $props();

	let graphContainer: HTMLDivElement;
	let cy: any = null;
	let fcoseRegistered = false;

	let config = $derived($topologyTabs.find((tab) => tab.id === tabId) as FilteredViewConfig | undefined);
	let hiddenSet = $derived(new Set(config?.hiddenNodeIds ?? []));

	async function initCytoscape() {
		const cytoscape = (await import('cytoscape')).default;
		if (!fcoseRegistered) {
			const fcose = (await import('cytoscape-fcose')).default;
			cytoscape.use(fcose);
			fcoseRegistered = true;
		}

		cy = cytoscape({
			container: graphContainer,
			style: filteredCytoscapeStyle,
			layout: { name: 'grid' },
			minZoom: 0.1,
			maxZoom: 5,
			wheelSensitivity: 0.3
		});

		cy.on('tap', 'node.device', (event: any) => {
			selectedAssetId.set(event.target.id());
		});

		cy.on('tap', (event: any) => {
			if (event.target === cy) selectedAssetId.set(null);
		});

		cy.on('cxttap', 'node.device', (event: any) => {
			toggleNodeInFilter(tabId, event.target.id());
		});
	}

	function updateGraph(graph: TopologyGraph, hidden: Set<string>, mode: GroupingMode) {
		if (!cy) return;

		cy.elements().remove();
		const elements = createFilteredElements(graph, hidden, mode);
		if (elements.length === 0) return;

		cy.add(elements);
		runFilteredLayout(cy);
	}

	function showAllNodes() {
		if (!config) return;
		for (const nodeId of [...config.hiddenNodeIds]) {
			toggleNodeInFilter(tabId, nodeId);
		}
	}

	$effect(() => {
		const graph = $topology;
		const hidden = hiddenSet;
		const mode = $groupingMode;
		updateGraph(graph, hidden, mode);
	});

	onMount(() => {
		initCytoscape();
	});

	onDestroy(() => {
		cy?.destroy();
	});
</script>

<div class="filtered-container">
	<FilteredToolbar
		hiddenCount={hiddenSet.size}
		onShowAll={showAllNodes}
		onFit={() => cy?.fit(undefined, 40)}
		onRelayout={() => {
			if (cy) runFilteredLayout(cy);
		}}
	/>

	<div class="graph-area" bind:this={graphContainer}>
		{#if $topology.nodes.length === 0}
			<div class="empty-state">
				<p>No topology data. Import a PCAP first.</p>
			</div>
		{:else if hiddenSet.size === $topology.nodes.length}
			<div class="empty-state">
				<p>All nodes are hidden. Right-click was used to hide nodes. Click "Show All" to restore.</p>
			</div>
		{/if}
	</div>

	<div class="filter-hint">Right-click a node to hide it from this view. Main Logical View is never modified.</div>
</div>

<style>
	.filtered-container {
		display: flex;
		flex-direction: column;
		height: 100%;
	}

	.graph-area {
		flex: 1;
		position: relative;
		background: var(--gm-bg-primary);
		background-image: radial-gradient(circle, var(--gm-bg-dot) 1px, transparent 1px);
		background-size: 24px 24px;
	}

	.empty-state {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		text-align: center;
		color: var(--gm-text-muted);
		font-size: 12px;
	}

	.filter-hint {
		padding: 6px 16px;
		font-size: 9px;
		color: var(--gm-text-muted);
		border-top: 1px solid var(--gm-border);
		background: var(--gm-bg-secondary);
	}
</style>

<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { selectedAssetId, topology } from '$lib/stores/core';
	import { topologyTabs, updateWatchDepth } from '$lib/stores/topology-tabs';
	import type { TopologyGraph, WatchViewConfig } from '$lib/types/topology';
	import WatchToolbar from './watch/WatchToolbar.svelte';
	import { createWatchElements, runWatchLayout, watchCytoscapeStyle } from './watch/watchGraph';

	let { tabId }: { tabId: string } = $props();

	let graphContainer: HTMLDivElement;
	let cy: any = null;
	let fcoseRegistered = false;

	let config = $derived($topologyTabs.find((tab) => tab.id === tabId) as WatchViewConfig | undefined);
	let targetNodeId = $derived(config?.targetNodeId ?? '');
	let depth = $derived(config?.depth ?? 2);

	async function initCytoscape() {
		const cytoscape = (await import('cytoscape')).default;
		if (!fcoseRegistered) {
			const fcose = (await import('cytoscape-fcose')).default;
			cytoscape.use(fcose);
			fcoseRegistered = true;
		}

		cy = cytoscape({
			container: graphContainer,
			style: watchCytoscapeStyle,
			layout: { name: 'grid' },
			minZoom: 0.1,
			maxZoom: 5,
			wheelSensitivity: 3
		});

		cy.on('tap', 'node', (event: any) => {
			selectedAssetId.set(event.target.id());
		});

		cy.on('tap', (event: any) => {
			if (event.target === cy) selectedAssetId.set(null);
		});
	}

	function updateGraph(graph: TopologyGraph, nodeId: string, hops: number) {
		if (!cy) return;

		cy.elements().remove();
		const elements = createWatchElements(graph, nodeId, hops);
		if (elements.length === 0) return;

		cy.add(elements);
		runWatchLayout(cy, nodeId);
	}

	function handleDepthChange(nextDepth: number) {
		updateWatchDepth(tabId, nextDepth);
	}

	$effect(() => {
		const graph = $topology;
		const nodeId = targetNodeId;
		const hops = depth;
		updateGraph(graph, nodeId, hops);
	});

	onMount(() => {
		initCytoscape();
	});

	onDestroy(() => {
		cy?.destroy();
	});
</script>

<div class="watch-container">
	<WatchToolbar
		{targetNodeId}
		{depth}
		onDepthChange={handleDepthChange}
		onFit={() => cy?.fit(undefined, 40)}
		onRelayout={() => {
			if (cy && targetNodeId) runWatchLayout(cy, targetNodeId);
		}}
	/>

	<div class="graph-area" bind:this={graphContainer}>
		{#if !targetNodeId}
			<div class="empty-state">
				<p>No target node configured.</p>
			</div>
		{/if}
	</div>

	<div class="watch-info">
		Showing {targetNodeId} and all devices within {depth} hop{depth !== 1 ? 's' : ''}. Green-bordered node is the watch target.
	</div>
</div>

<style>
	.watch-container {
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

	.watch-info {
		padding: 6px 16px;
		font-size: 9px;
		color: var(--gm-text-muted);
		border-top: 1px solid var(--gm-border);
		background: var(--gm-bg-secondary);
	}
</style>

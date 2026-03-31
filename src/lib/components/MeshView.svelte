<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { selectedAssetId, topology } from '$lib/stores/core';
	import type { TopologyGraph } from '$lib/types/topology';
	import MeshToolbar from './mesh/MeshToolbar.svelte';
	import {
		createMeshElements,
		getAvailableProtocols,
		meshCytoscapeStyle,
		runMeshLayout
	} from './mesh/meshGraph';

	let graphContainer: HTMLDivElement;
	let cy: any = null;

	let filterProtocol = $state<string>('all');
	let filterMinPackets = $state(0);

	let availableProtocols = $derived(getAvailableProtocols($topology));

	async function initCytoscape() {
		const cytoscape = (await import('cytoscape')).default;

		cy = cytoscape({
			container: graphContainer,
			style: meshCytoscapeStyle,
			layout: { name: 'grid' },
			minZoom: 0.1,
			maxZoom: 5,
			wheelSensitivity: 0.3
		});

		cy.on('tap', 'node', (event: any) => {
			selectedAssetId.set(event.target.id());
		});

		cy.on('tap', (event: any) => {
			if (event.target === cy) selectedAssetId.set(null);
		});
	}

	function updateMesh(graph: TopologyGraph, protocol: string, minPackets: number) {
		if (!cy) return;

		cy.elements().remove();
		const elements = createMeshElements(graph, {
			filterProtocol: protocol,
			filterMinPackets: minPackets
		});
		if (elements.length === 0) return;

		cy.add(elements);
		runMeshLayout(cy);
	}

	$effect(() => {
		const graph = $topology;
		updateMesh(graph, filterProtocol, filterMinPackets);
	});

	onMount(async () => {
		await initCytoscape();
		if ($topology.nodes.length > 0) {
			updateMesh($topology, filterProtocol, filterMinPackets);
		}
	});

	onDestroy(() => {
		cy?.destroy();
	});
</script>

<div class="mesh-container">
	<MeshToolbar
		{availableProtocols}
		{filterProtocol}
		{filterMinPackets}
		onProtocolChange={(value) => {
			filterProtocol = value;
		}}
		onMinPacketsChange={(value) => {
			filterMinPackets = value;
		}}
		onFit={() => cy?.fit(undefined, 40)}
		onRelayout={() => {
			if (cy) runMeshLayout(cy);
		}}
	/>

	<div class="graph-area" bind:this={graphContainer}>
		{#if $topology.nodes.length === 0}
			<div class="empty-state">
				<div class="empty-icon">&#x25CE;</div>
				<h3>No Data</h3>
				<p>Import a PCAP to see the all-to-all mesh view.</p>
			</div>
		{/if}
	</div>
</div>

<style>
	.mesh-container {
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
	}

	.empty-state {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		text-align: center;
		color: var(--gm-text-muted);
		z-index: 1;
	}

	.empty-icon {
		font-size: 48px;
		margin-bottom: 12px;
		opacity: 0.3;
	}

	.empty-state h3 {
		font-size: 14px;
		font-weight: 600;
		color: var(--gm-text-secondary);
		margin: 0 0 8px 0;
	}

	.empty-state p {
		font-size: 12px;
		margin: 4px 0;
		line-height: 1.5;
	}
</style>

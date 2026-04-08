<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import type { PhysicalTopology } from '$lib/types';
	import { buildPhysicalElements } from './buildPhysicalElements';
	import { physicalGraphStyles } from './physicalGraphStyles';
	import PhysicalGraphEmptyState from './PhysicalGraphEmptyState.svelte';

	interface Props {
		topology: PhysicalTopology;
		highlightIp: string | null;
		onSelectSwitch: (hostname: string) => void;
		onSelectPort: (switchHostname: string, portName: string) => void;
		onDeselect: () => void;
	}

	let {
		topology,
		highlightIp,
		onSelectSwitch,
		onSelectPort,
		onDeselect
	}: Props = $props();

	let graphContainer: HTMLDivElement;
	let cy: any = null;

	export function runLayout() {
		if (!cy) return;
		cy.layout({
			name: 'cose',
			animate: true,
			animationDuration: 400,
			nodeOverlap: 20,
			idealEdgeLength: 100,
			componentSpacing: 100,
			nodeRepulsion: 400000,
			edgeElasticity: 100,
			nestingFactor: 5,
			gravity: 80,
			numIter: 1000,
			coolingFactor: 0.95,
			minTemp: 1.0
		}).run();
	}

	export function fit() {
		if (!cy) return;
		cy.fit(undefined, 40);
	}

	function updateGraph() {
		if (!cy) return;
		cy.elements().remove();
		if (topology.switches.length === 0) return;

		const elements = buildPhysicalElements(topology, highlightIp);
		cy.add(elements);
		runLayout();
	}

	onMount(async () => {
		const cytoscape = (await import('cytoscape')).default;

		cy = cytoscape({
			container: graphContainer,
			style: physicalGraphStyles,
			layout: { name: 'preset' },
			minZoom: 0.1,
			maxZoom: 5,
			wheelSensitivity: 3
		});

		cy.on('tap', 'node.switch', (event: any) => {
			onSelectSwitch(event.target.data('hostname'));
		});

		cy.on('tap', 'node.port', (event: any) => {
			onSelectPort(event.target.data('switchHostname'), event.target.data('portName'));
		});

		cy.on('tap', (event: any) => {
			if (event.target === cy) {
				onDeselect();
			}
		});

		updateGraph();
	});

	onDestroy(() => {
		cy?.destroy();
	});

	$effect(() => {
		updateGraph();
	});
</script>

<div class="graph-wrapper">
	<div class="graph-area" bind:this={graphContainer}>
		{#if topology.switches.length === 0}
			<PhysicalGraphEmptyState />
		{/if}
	</div>
</div>

<style>
	.graph-wrapper {
		flex: 1;
		position: relative;
		background: var(--gm-bg-secondary);
		border-right: 1px solid var(--gm-border);
	}

	.graph-area {
		width: 100%;
		height: 100%;
		background: var(--gm-bg-tertiary);
	}
</style>

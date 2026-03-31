<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import type { PhysicalTopology, PhysicalSwitch, PhysicalPort } from '$lib/types';

	interface Props {
		topology: PhysicalTopology;
		highlightIp: string | null;
		onSelectSwitch: (hostname: string) => void;
		onSelectPort: (switchHostname: string, portName: string) => void;
		onDeselect: () => void;
		onFit: () => void;
		onLayout: () => void;
	}

	let {
		topology,
		highlightIp,
		onSelectSwitch,
		onSelectPort,
		onDeselect,
		onFit,
		onLayout
	}: Props = $props();

	let graphContainer: HTMLDivElement;
	let cy: any = null;

	function shortenName(name: string): string {
		const prefixes: [string, string][] = [
			['TenGigabitEthernet', 'Te'],
			['GigabitEthernet', 'Gi'],
			['FastEthernet', 'Fa'],
			['Ethernet', 'Et'],
			['Loopback', 'Lo'],
			['Tunnel', 'Tu'],
			['Port-channel', 'Po'],
			['Vlan', 'Vl']
		];
		for (const [long, short] of prefixes) {
			if (name.startsWith(long)) {
				return short + name.slice(long.length);
			}
		}
		return name;
	}

	function buildElements(topo: PhysicalTopology) {
		const elements: any[] = [];

		for (const sw of topo.switches) {
			const switchId = `sw-${sw.hostname}`;
			let switchLabel = sw.hostname;
			if (sw.management_ip) {
				switchLabel += `\n${sw.management_ip}`;
			}

			elements.push({
				group: 'nodes',
				data: {
					id: switchId,
					label: switchLabel,
					hostname: sw.hostname
				},
				classes: 'switch'
			});

			const visiblePorts = sw.ports.filter((p) => {
				const name = p.name.toLowerCase();
				return (
					name.startsWith('gigabitethernet') ||
					name.startsWith('fastethernet') ||
					name.startsWith('tengigabitethernet') ||
					name.startsWith('port-channel') ||
					(name.startsWith('vlan') && p.ip_address)
				);
			});

			for (const port of visiblePorts) {
				const portId = `${switchId}-${port.short_name}`;
				const hasDevice = port.mac_addresses.length > 0 || port.ip_addresses.length > 0;
				const hasCdp = port.cdp_neighbor !== null;
				const isTrunk = port.mode === 'trunk';
				const isHighlighted = highlightIp !== null && port.ip_addresses.includes(highlightIp);

				let classes = 'port';
				if (hasDevice) classes += ' has-device';
				if (hasCdp) classes += ' has-cdp';
				if (port.shutdown) classes += ' shutdown';
				if (isTrunk) classes += ' trunk';
				if (isHighlighted) classes += ' highlighted';

				let label = port.short_name;
				if (port.description) {
					label += `\n${port.description}`;
				}
				if (port.ip_addresses.length > 0) {
					label += `\n${port.ip_addresses[0]}`;
				}

				let color = '#475569';
				if (isHighlighted) color = '#ef4444';
				else if (hasCdp) color = '#f59e0b';
				else if (hasDevice) color = '#3b82f6';
				else if (isTrunk) color = '#8b5cf6';
				else if (port.shutdown) color = '#374151';

				elements.push({
					group: 'nodes',
					data: {
						id: portId,
						label,
						parent: switchId,
						portName: port.name,
						switchHostname: sw.hostname,
						color,
						vlans: port.vlans.join(', '),
						macCount: port.mac_addresses.length,
						ipCount: port.ip_addresses.length
					},
					classes
				});
			}
		}

		const addedLinks = new Set<string>();
		for (const link of topology.links) {
			const key = [link.src_switch, link.dst_switch].sort().join('---');
			if (addedLinks.has(key)) continue;
			addedLinks.add(key);

			const srcPortId = `sw-${link.src_switch}-${shortenName(link.src_port)}`;
			const dstPortId = `sw-${link.dst_switch}-${shortenName(link.dst_port)}`;

			const srcExists = elements.some((e) => e.data?.id === srcPortId);
			const dstExists = elements.some((e) => e.data?.id === dstPortId);

			if (srcExists && dstExists) {
				elements.push({
					group: 'edges',
					data: {
						id: `link-${link.src_switch}-${link.dst_switch}`,
						source: srcPortId,
						target: dstPortId,
						label: 'CDP'
					},
					classes: 'cdp-link'
				});
			}
		}

		return elements;
	}

	function updateGraph() {
		if (!cy) return;
		cy.elements().remove();
		if (topology.switches.length === 0) return;

		const elements = buildElements(topology);
		cy.add(elements);
		onLayout();
	}

	onMount(async () => {
		const cytoscape = (await import('cytoscape')).default;

		cy = cytoscape({
			container: graphContainer,
			style: [
				{
					selector: 'node.switch',
					style: {
						'background-color': 'rgba(16, 185, 129, 0.08)',
						'background-opacity': 0.6,
						'border-color': '#10b981',
						'border-width': 2,
						'border-style': 'solid' as any,
						label: 'data(label)',
						color: '#e2e8f0',
						'font-size': '11px',
						'font-family': 'JetBrains Mono, monospace',
						'font-weight': '600' as any,
						'text-valign': 'top',
						'text-halign': 'center',
						'text-margin-y': -6,
						padding: '20px',
						shape: 'roundrectangle'
					}
				},
				{
					selector: 'node.port',
					style: {
						'background-color': '#1e293b',
						'border-color': 'data(color)',
						'border-width': 1.5,
						label: 'data(label)',
						color: '#94a3b8',
						'font-size': '8px',
						'font-family': 'JetBrains Mono, monospace',
						'text-valign': 'bottom',
						'text-margin-y': 4,
						width: 20,
						height: 20,
						'text-wrap': 'wrap' as any,
						'text-max-width': '80px'
					}
				},
				{
					selector: 'node.port.has-device',
					style: {
						'background-color': '#0f2942',
						'border-color': '#3b82f6',
						'border-width': 2,
						width: 24,
						height: 24
					}
				},
				{
					selector: 'node.port.has-cdp',
					style: {
						'border-color': '#f59e0b',
						'border-width': 2
					}
				},
				{
					selector: 'node.port.shutdown',
					style: {
						'background-color': '#1a1a2e',
						'border-color': '#374151',
						'border-style': 'dashed' as any,
						opacity: 0.5
					}
				},
				{
					selector: 'node.port.trunk',
					style: {
						'border-color': '#8b5cf6',
						shape: 'diamond'
					}
				},
				{
					selector: 'node.port.highlighted',
					style: {
						'background-color': '#1e3a5f',
						'border-color': '#ef4444',
						'border-width': 3,
						width: 28,
						height: 28
					}
				},
				{
					selector: 'node:selected',
					style: {
						'border-color': '#3b82f6',
						'border-width': 3
					}
				},
				{
					selector: 'edge.cdp-link',
					style: {
						width: 2.5,
						'line-color': '#f59e0b',
						'line-style': 'solid' as any,
						'target-arrow-shape': 'none',
						'curve-style': 'bezier',
						opacity: 0.8,
						label: 'data(label)',
						color: '#f59e0b',
						'font-size': '8px',
						'text-rotation': 'autorotate' as any,
						'text-background-color': '#0a0e17',
						'text-background-opacity': 0.9,
						'text-background-padding': '2px' as any
					}
				}
			],
			layout: { name: 'preset' },
			minZoom: 0.1,
			maxZoom: 5,
			wheelSensitivity: 0.3
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
			<div class="empty-state">
				<div class="empty-icon">&#x2B22;</div>
				<h3>No Physical Topology</h3>
				<p>Import a Cisco IOS running-config to build the physical topology.</p>
				<p class="hint">Use the import panel on the left to get started.</p>
			</div>
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

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100%;
		color: var(--gm-text-secondary);
		text-align: center;
		padding: 2rem;
	}

	.empty-icon {
		font-size: 3rem;
		opacity: 0.3;
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		font-size: 1.25rem;
		margin: 0 0 0.5rem 0;
		color: var(--gm-text-primary);
	}

	.empty-state p {
		margin: 0.25rem 0;
		font-size: 0.8125rem;
	}

	.empty-state p.hint {
		font-style: italic;
		font-size: 0.75rem;
		margin-top: 0.75rem;
	}
</style>

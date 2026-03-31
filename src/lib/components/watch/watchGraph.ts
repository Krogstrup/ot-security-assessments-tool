import type { TopologyGraph } from '$lib/types/topology';
import { DEVICE_COLORS, PROTOCOL_COLORS, edgeWidth, getNeighborSubgraph, isOtProtocol } from '$lib/utils/graph';

type CytoscapeElement = {
	group: 'nodes' | 'edges';
	data: Record<string, unknown>;
	classes?: string;
};

export const watchCytoscapeStyle: any[] = [
	{
		selector: 'node',
		style: {
			'background-color': '#1e293b',
			'border-color': 'data(color)',
			'border-width': 2,
			label: 'data(label)',
			color: '#e2e8f0',
			'font-size': '10px',
			'font-family': 'JetBrains Mono, monospace',
			'text-valign': 'bottom',
			'text-margin-y': 6,
			width: 32,
			height: 32
		}
	},
	{
		selector: 'node.target',
		style: {
			'border-color': '#10b981',
			'border-width': 4,
			'background-color': '#0f2d1f',
			width: 42,
			height: 42,
			'font-size': '11px',
			'font-weight': 700
		}
	},
	{
		selector: 'node.ot',
		style: {
			'background-color': '#0f1d2e',
			'border-width': 2.5
		}
	},
	{
		selector: 'node:selected',
		style: {
			'border-color': '#3b82f6',
			'border-width': 3,
			'background-color': '#1e3a5f'
		}
	},
	{
		selector: 'edge',
		style: {
			width: 'data(weight)',
			'line-color': 'data(color)',
			'target-arrow-color': 'data(color)',
			'target-arrow-shape': 'triangle',
			'arrow-scale': 0.8,
			'curve-style': 'bezier',
			opacity: 0.7
		}
	},
	{
		selector: 'edge.bidirectional',
		style: {
			'source-arrow-color': 'data(color)',
			'source-arrow-shape': 'triangle'
		}
	},
	{
		selector: 'edge:selected',
		style: {
			'line-color': '#3b82f6',
			'target-arrow-color': '#3b82f6',
			'source-arrow-color': '#3b82f6',
			opacity: 1
		}
	}
];

export function createWatchElements(graph: TopologyGraph, nodeId: string, hops: number): CytoscapeElement[] {
	if (!nodeId) return [];
	const subgraph = getNeighborSubgraph(graph, nodeId, hops);
	if (subgraph.nodes.length === 0) return [];

	const elements: CytoscapeElement[] = [];

	for (const node of subgraph.nodes) {
		const hasOt = node.protocols.some((protocol) => isOtProtocol(protocol));
		const color = DEVICE_COLORS[node.device_type] ?? DEVICE_COLORS.unknown;
		const isTarget = node.id === nodeId;
		const classes = ['device', hasOt ? 'ot' : '', isTarget ? 'target' : ''].filter(Boolean).join(' ');

		elements.push({
			group: 'nodes',
			data: { id: node.id, label: node.ip_address, color },
			classes
		});
	}

	for (const edge of subgraph.edges) {
		const color = PROTOCOL_COLORS[edge.protocol as string] ?? PROTOCOL_COLORS.unknown;
		elements.push({
			group: 'edges',
			data: {
				id: edge.id,
				source: edge.source,
				target: edge.target,
				color,
				weight: edgeWidth(edge.packet_count)
			},
			classes: edge.bidirectional ? 'bidirectional' : ''
		});
	}

	return elements;
}

export function runWatchLayout(cy: any, nodeId: string) {
	cy.layout({
		name: 'concentric',
		animate: true,
		animationDuration: 500,
		concentric: (node: any) => (node.id() === nodeId ? 10 : 1),
		levelWidth: () => 1,
		padding: 40,
		minNodeSpacing: 60
	}).run();
}

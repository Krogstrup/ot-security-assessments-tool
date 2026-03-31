import type { TopologyGraph } from '$lib/types/topology';
import { DEVICE_COLORS, PROTOCOL_COLORS, edgeWidth, isOtProtocol } from '$lib/utils/graph';

type CytoscapeElement = {
	group: 'nodes' | 'edges';
	data: Record<string, unknown>;
	classes?: string;
};

export const meshCytoscapeStyle: any[] = [
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
			width: 28,
			height: 28
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
			'arrow-scale': 0.7,
			'curve-style': 'bezier',
			opacity: 0.6
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

export interface MeshFilterState {
	filterProtocol: string;
	filterMinPackets: number;
}

export function getAvailableProtocols(graph: TopologyGraph): string[] {
	const protocols = new Set<string>();
	for (const edge of graph.edges) {
		protocols.add(edge.protocol as string);
	}
	return [...protocols].sort();
}

export function createMeshElements(graph: TopologyGraph, filters: MeshFilterState): CytoscapeElement[] {
	let filteredEdges = graph.edges;
	if (filters.filterProtocol !== 'all') {
		filteredEdges = filteredEdges.filter((edge) => (edge.protocol as string) === filters.filterProtocol);
	}
	if (filters.filterMinPackets > 0) {
		filteredEdges = filteredEdges.filter((edge) => edge.packet_count >= filters.filterMinPackets);
	}

	const connectedNodes = new Set<string>();
	for (const edge of filteredEdges) {
		connectedNodes.add(edge.source);
		connectedNodes.add(edge.target);
	}
	const filteredNodes = graph.nodes.filter((node) => connectedNodes.has(node.id));

	const elements: CytoscapeElement[] = [];

	for (const node of filteredNodes) {
		const hasOt = node.protocols.some((protocol) => isOtProtocol(protocol));
		const color = DEVICE_COLORS[node.device_type] ?? DEVICE_COLORS.unknown;
		elements.push({
			group: 'nodes',
			data: {
				id: node.id,
				label: node.ip_address,
				color
			},
			classes: hasOt ? 'ot' : ''
		});
	}

	for (const edge of filteredEdges) {
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

export function runMeshLayout(cy: any) {
	cy.layout({
		name: 'circle',
		animate: true,
		animationDuration: 500,
		padding: 40
	}).run();
}

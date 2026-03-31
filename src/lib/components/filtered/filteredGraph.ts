import type { GroupingMode, TopologyGraph } from '$lib/types/topology';
import {
	DEVICE_COLORS,
	PROTOCOL_COLORS,
	edgeWidth,
	filterGraph,
	getGroupId,
	getGroupLabel,
	isOtProtocol
} from '$lib/utils/graph';

type CytoscapeElement = {
	group: 'nodes' | 'edges';
	data: Record<string, unknown>;
	classes?: string;
};

export const filteredCytoscapeStyle: any[] = [
	{
		selector: 'node.compound',
		style: {
			'background-color': 'rgba(30, 41, 59, 0.4)',
			'background-opacity': 0.4,
			'border-color': '#334155',
			'border-width': 1,
			'border-style': 'dashed',
			label: 'data(label)',
			color: '#64748b',
			'font-size': '9px',
			'font-family': 'JetBrains Mono, monospace',
			'text-valign': 'top',
			'text-halign': 'center',
			'text-margin-y': -4,
			padding: '16px',
			shape: 'roundrectangle'
		}
	},
	{
		selector: 'node.device',
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
		selector: 'node.device.ot',
		style: {
			'background-color': '#0f1d2e',
			'border-width': 2.5
		}
	},
	{
		selector: 'node.device:selected',
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

export function createFilteredElements(
	graph: TopologyGraph,
	hidden: Set<string>,
	mode: GroupingMode
): CytoscapeElement[] {
	const filtered = filterGraph(graph, hidden);
	if (filtered.nodes.length === 0) return [];

	const elements: CytoscapeElement[] = [];

	if (mode !== 'none') {
		const groups = new Set<string>();
		for (const node of filtered.nodes) {
			const groupId = getGroupId(node, mode);
			if (groupId) groups.add(groupId);
		}

		for (const groupId of groups) {
			elements.push({
				group: 'nodes',
				data: { id: groupId, label: getGroupLabel(groupId, mode) },
				classes: 'compound'
			});
		}
	}

	for (const node of filtered.nodes) {
		const hasOt = node.protocols.some((protocol) => isOtProtocol(protocol));
		const color = DEVICE_COLORS[node.device_type] ?? DEVICE_COLORS.unknown;
		const parentId = mode !== 'none' ? getGroupId(node, mode) : undefined;
		elements.push({
			group: 'nodes',
			data: {
				id: node.id,
				label: node.ip_address,
				color,
				...(parentId ? { parent: parentId } : {})
			},
			classes: hasOt ? 'device ot' : 'device'
		});
	}

	for (const edge of filtered.edges) {
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

export function runFilteredLayout(cy: any) {
	cy.layout({
		name: 'fcose',
		animate: true,
		animationDuration: 600,
		quality: 'default',
		nodeRepulsion: () => 8000,
		idealEdgeLength: () => 140,
		nestingFactor: 0.1,
		gravity: 0.25,
		padding: 40,
		fit: true,
		randomize: false
	}).run();
}

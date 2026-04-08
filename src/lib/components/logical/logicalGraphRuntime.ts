import { PurdueLayout } from '$lib/layouts/purdueLayout';
import { FCOSE_NODE_LIMIT } from './logicalConfig';
import { runLogicalLayout, type LogicalLayoutMode } from './logicalLayout';
import { buildTopologyElements } from './topologyElements';
import { logicalCytoscapeStyle } from './cytoscapeStyles';
import type { Asset } from '$lib/types/assets';
import type { GroupingMode, TopologyGraph } from '$lib/types/topology';

let fcoseRegistered = false;
let purdueRegistered = false;

export interface InitLogicalGraphOptions {
	container: HTMLDivElement;
	onNodeTap: (nodeId: string) => void;
	onBackgroundTap: () => void;
	onNodeContext: (position: { x: number; y: number }, bounds: DOMRect, nodeId: string) => void;
	onCanvasContext: (position: { x: number; y: number }, bounds: DOMRect) => void;
}

export async function initLogicalGraph(options: InitLogicalGraphOptions): Promise<any> {
	const cytoscape = (await import('cytoscape')).default;
	if (!fcoseRegistered) {
		const fcose = (await import('cytoscape-fcose')).default;
		cytoscape.use(fcose);
		fcoseRegistered = true;
	}
	if (!purdueRegistered) {
		cytoscape('layout', 'purdue', PurdueLayout);
		purdueRegistered = true;
	}

	const cy = cytoscape({
		container: options.container,
		style: logicalCytoscapeStyle,
		layout: { name: 'grid' },
		minZoom: 0.1,
		maxZoom: 5,
		wheelSensitivity: 3
	});

	cy.on('tap', 'node.device', (event: any) => {
		options.onNodeTap(event.target.id());
	});

	cy.on('tap', (event: any) => {
		if (event.target === cy) {
			options.onBackgroundTap();
		}
	});

	cy.on('cxttap', 'node.device', (event: any) => {
		const position = event.renderedPosition || event.position;
		options.onNodeContext(position, options.container.getBoundingClientRect(), event.target.id());
	});

	cy.on('cxttap', (event: any) => {
		if (event.target !== cy) return;
		options.onCanvasContext(event.renderedPosition, options.container.getBoundingClientRect());
	});

	return cy;
}

export interface UpdateLogicalGraphOptions {
	graph: TopologyGraph;
	mode: GroupingMode;
	assets: Asset[];
	newIps: Set<string>;
	changedIps: Set<string>;
	layout: LogicalLayoutMode;
	forceLayout: boolean;
}

export function runLogicalGraphLayoutOnly(
	cy: any,
	layout: LogicalLayoutMode,
	forceLayout: boolean
): boolean {
	return runLogicalLayout(cy, layout, forceLayout, FCOSE_NODE_LIMIT).largeNetworkWarning;
}

export function updateLogicalGraph(cy: any, options: UpdateLogicalGraphOptions): boolean {
	cy.elements().remove();
	if (options.graph.nodes.length === 0) return false;

	const assetMap = new Map(options.assets.map((asset) => [asset.ip_address, asset]));
	const elements = buildTopologyElements({
		graph: options.graph,
		mode: options.mode,
		assetMap,
		newIps: options.newIps,
		changedIps: options.changedIps
	});

	cy.add(elements);
	return runLogicalGraphLayoutOnly(cy, options.layout, options.forceLayout);
}

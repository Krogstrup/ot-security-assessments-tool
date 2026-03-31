import { detectWireshark } from '$lib/api';

export type LogicalLayoutMode = 'fcose' | 'purdue';

export interface LogicalLayoutResult {
	largeNetworkWarning: boolean;
}

export async function isWiresharkAvailable() {
	try {
		const info = await detectWireshark();
		return info.found;
	} catch {
		return false;
	}
}

export function runLogicalLayout(
	cy: any,
	layout: LogicalLayoutMode,
	forceLayout: boolean,
	nodeLimit: number
): LogicalLayoutResult {
	if (!cy || cy.nodes('.device').length === 0) {
		return { largeNetworkWarning: false };
	}

	const nodeCount = cy.nodes('.device').length;
	if (layout === 'purdue') {
		cy.layout({ name: 'purdue' }).run();
		return { largeNetworkWarning: false };
	}

	if (nodeCount > nodeLimit && !forceLayout) {
		return { largeNetworkWarning: true };
	}

	cy.layout({
		name: 'fcose',
		animate: true,
		animationDuration: 600,
		quality: 'default',
		nodeRepulsion: () => 8000,
		idealEdgeLength: () => 140,
		edgeElasticity: () => 0.45,
		nestingFactor: 0.1,
		gravity: 0.25,
		gravityRange: 3.8,
		tile: true,
		tilingPaddingVertical: 20,
		tilingPaddingHorizontal: 20,
		padding: 40,
		fit: true,
		randomize: false
	}).run();

	return { largeNetworkWarning: false };
}

export function exportLogicalPng(cy: any) {
	if (!cy) return;
	const png = cy.png({ scale: 2, full: true, bg: '#0f172a' });
	const anchor = document.createElement('a');
	anchor.href = png;
	anchor.download = 'topology.png';
	anchor.click();
}

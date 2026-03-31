import type { Asset } from '$lib/types/assets';
import type { GroupingMode, TopologyGraph, TopologyNode } from '$lib/types/topology';
import {
	DEVICE_COLORS,
	PROTOCOL_COLORS,
	edgeWidth,
	getGroupId,
	getGroupLabel,
	isOtProtocol
} from '$lib/utils/graph';

interface BuildTopologyElementsInput {
	graph: TopologyGraph;
	mode: GroupingMode;
	assetMap: Map<string, Asset>;
	newIps: Set<string>;
	changedIps: Set<string>;
}

function nodeLabel(node: TopologyNode, assetMap: Map<string, Asset>): string {
	const asset = assetMap.get(node.ip_address);
	const vendor = asset?.vendor ?? node.vendor;
	if (!vendor) return node.ip_address;
	const shortVendor = vendor.length > 20 ? `${vendor.substring(0, 18)}...` : vendor;
	return `${node.ip_address}\n${shortVendor}`;
}

export function buildTopologyElements({
	graph,
	mode,
	assetMap,
	newIps,
	changedIps
}: BuildTopologyElementsInput): any[] {
	const elements: any[] = [];

	const purdueMap = new Map<string, number | null>();
	for (const node of graph.nodes) {
		const asset = assetMap.get(node.ip_address);
		purdueMap.set(node.id, asset?.purdue_level ?? null);
	}

	if (mode !== 'none') {
		const groups = new Set<string>();
		for (const node of graph.nodes) {
			const gid = getGroupId(node, mode);
			if (gid) groups.add(gid);
		}
		for (const gid of groups) {
			elements.push({
				group: 'nodes',
				data: { id: gid, label: getGroupLabel(gid, mode) },
				classes: 'compound'
			});
		}
	}

	for (const node of graph.nodes) {
		const hasOt = node.protocols.some((p) => isOtProtocol(p));
		const color = DEVICE_COLORS[node.device_type] ?? DEVICE_COLORS.unknown;
		const parentId = mode !== 'none' ? getGroupId(node, mode) : undefined;
		const asset = assetMap.get(node.ip_address);
		const confidence = asset?.confidence ?? 0;

		let driftClass = '';
		if (newIps.has(node.ip_address)) driftClass = ' drift-new';
		else if (changedIps.has(node.ip_address)) driftClass = ' drift-changed';

		elements.push({
			group: 'nodes',
			data: {
				id: node.id,
				label: nodeLabel(node, assetMap),
				deviceType: node.device_type,
				vendor: asset?.vendor ?? node.vendor,
				productFamily: asset?.product_family,
				confidence,
				subnet: node.subnet,
				protocols: node.protocols.join(', '),
				packetCount: node.packet_count,
				color,
				purdueLevel: asset?.purdue_level ?? null,
				...(parentId ? { parent: parentId } : {})
			},
			classes: (hasOt ? 'device ot' : 'device') + driftClass
		});
	}

	for (const edge of graph.edges) {
		const color = PROTOCOL_COLORS[edge.protocol as string] ?? PROTOCOL_COLORS.unknown;
		const weight = edgeWidth(edge.packet_count);

		const srcLevel = purdueMap.get(edge.source);
		const dstLevel = purdueMap.get(edge.target);
		const isCrossZone =
			srcLevel !== null &&
			srcLevel !== undefined &&
			dstLevel !== null &&
			dstLevel !== undefined &&
			Math.abs(srcLevel - dstLevel) >= 2;

		const edgeClasses = [edge.bidirectional ? 'bidirectional' : '', isCrossZone ? 'cross-zone' : '']
			.filter(Boolean)
			.join(' ');

		elements.push({
			group: 'edges',
			data: {
				id: edge.id,
				source: edge.source,
				target: edge.target,
				protocol: edge.protocol,
				packetCount: edge.packet_count,
				byteCount: edge.byte_count,
				color,
				weight
			},
			classes: edgeClasses
		});
	}

	return elements;
}

import type { DeviceType } from './assets';
import type { IcsProtocol } from './protocols';

export interface TopologyGraph {
	nodes: TopologyNode[];
	edges: TopologyEdge[];
}

export interface TopologyNode {
	id: string;
	ip_address: string;
	mac_address: string | null;
	device_type: DeviceType;
	vendor: string | null;
	protocols: IcsProtocol[];
	subnet: string;
	packet_count: number;
}

export interface TopologyEdge {
	id: string;
	source: string;
	target: string;
	protocol: IcsProtocol;
	packet_count: number;
	byte_count: number;
	bidirectional: boolean;
}

/** How to group/cluster nodes in topology views. */
export type GroupingMode = 'subnet' | 'protocol' | 'device_role' | 'vendor' | 'none';

/** Shared shape for topology sub-tabs. */
export interface TopologyTab {
	id: string;
	type: 'logical' | 'mesh' | 'filtered' | 'watch';
	label: string;
	closeable: boolean;
}

/** Configuration for a filtered topology tab. */
export interface FilteredViewConfig extends TopologyTab {
	type: 'filtered';
	hiddenNodeIds: string[];
}

/** Configuration for a watch topology tab. */
export interface WatchViewConfig extends TopologyTab {
	type: 'watch';
	targetNodeId: string;
	depth: number;
}

import type { GroupingMode, TopologyTab, WatchViewConfig } from '$lib/types/topology';
import { writable } from 'svelte/store';
import type { FilteredViewConfig } from '$lib/types';

/** How nodes are grouped/clustered in the logical view */
export const groupingMode = writable<GroupingMode>('subnet');

/** All open topology sub-tabs */
export const topologyTabs = writable<TopologyTab[]>([
	{ id: 'logical', type: 'logical', label: 'Logical View', closeable: false },
	{ id: 'mesh', type: 'mesh', label: 'Mesh View', closeable: false }
]);

/** Which topology sub-tab is currently active */
export const activeTopologyTabId = writable<string>('logical');

/** Counter for generating unique tab IDs */
let nextTabId = 1;

/** Add a filtered view tab */
export function addFilteredView(hiddenNodeIds: string[] = []): string {
	const id = `filtered-${nextTabId++}`;
	const tab: FilteredViewConfig = {
		id,
		type: 'filtered',
		label: `Filtered ${nextTabId - 1}`,
		closeable: true,
		hiddenNodeIds
	};
	topologyTabs.update((tabs) => [...tabs, tab]);
	activeTopologyTabId.set(id);
	return id;
}

/** Add a watch tab for a specific node */
export function addWatchTab(targetNodeId: string, depth = 2): string {
	const id = `watch-${nextTabId++}`;
	const tab: WatchViewConfig = {
		id,
		type: 'watch',
		label: `Watch ${targetNodeId}`,
		closeable: true,
		targetNodeId,
		depth
	};
	topologyTabs.update((tabs) => [...tabs, tab]);
	activeTopologyTabId.set(id);
	return id;
}

/** Close a topology sub-tab */
export function closeTopologyTab(tabId: string) {
	topologyTabs.update((tabs) => tabs.filter((t) => t.id !== tabId));
	// If the closed tab was active, switch to logical view
	activeTopologyTabId.update((current) => (current === tabId ? 'logical' : current));
}

/** Update a watch tab's depth */
export function updateWatchDepth(tabId: string, depth: number) {
	topologyTabs.update((tabs) =>
		tabs.map((t) => (t.id === tabId && t.type === 'watch' ? ({ ...t, depth } as WatchViewConfig) : t))
	);
}

/** Toggle node visibility in a filtered view */
export function toggleNodeInFilter(tabId: string, nodeId: string) {
	topologyTabs.update((tabs) =>
		tabs.map((t) => {
			if (t.id !== tabId || t.type !== 'filtered') return t;
			const ft = t as FilteredViewConfig;
			const hidden = ft.hiddenNodeIds.includes(nodeId)
				? ft.hiddenNodeIds.filter((id) => id !== nodeId)
				: [...ft.hiddenNodeIds, nodeId];
			return { ...ft, hiddenNodeIds: hidden };
		})
	);
}

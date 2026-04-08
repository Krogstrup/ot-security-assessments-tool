<script lang="ts">
	import type { Connection, ConnectionTreeNode, PacketSummary } from '$lib/types/connections';
	import ConnectionEntry from './ConnectionEntry.svelte';
	import { DEVICE_TYPE_LABELS as deviceTypeLabels, DEVICE_TYPE_COLORS as deviceTypeColors } from '$lib/constants';

	interface Props {
		node: ConnectionTreeNode;
		isExpanded: boolean;
		isSelected: boolean;
		expandedConns: Set<string>;
		loadingConns: Set<string>;
		packetCache: Map<string, PacketSummary[]>;
		formatBytes: (bytes: number) => string;
		formatTime: (iso: string) => string;
		onToggleNode: (ip: string) => void;
		onSelectNode: (ip: string) => void;
		onToggleConnection: (conn: Connection) => void;
		onShowConnContextMenu: (event: MouseEvent, connId: string) => void;
	}

	let {
		node,
		isExpanded,
		isSelected,
		expandedConns,
		loadingConns,
		packetCache,
		formatBytes,
		formatTime,
		onToggleNode,
		onSelectNode,
		onToggleConnection,
		onShowConnContextMenu
	}: Props = $props();
</script>

<div class="tree-node">
	<button
		class="node-header"
		class:expanded={isExpanded}
		class:selected={isSelected}
		onclick={() => {
			onToggleNode(node.ip);
			onSelectNode(node.ip);
		}}
	>
		<span class="expand-icon">{isExpanded ? '\u25BC' : '\u25B6'}</span>
		<span class="device-dot" style="background: {deviceTypeColors[node.device_type]}"></span>
		<span class="node-ip">{node.ip}</span>
		<span class="node-meta">
			<span class="node-type" style="color: {deviceTypeColors[node.device_type]}">
				{deviceTypeLabels[node.device_type]}
			</span>
			<span class="node-pkt">{node.connections.length} conn</span>
		</span>
	</button>

	{#if isExpanded}
		<div class="node-connections">
			{#each node.connections as conn}
				<ConnectionEntry
					{conn}
					isExpanded={expandedConns.has(conn.id)}
					isLoading={loadingConns.has(conn.id)}
					packets={packetCache.get(conn.id)}
					{formatBytes}
					{formatTime}
					onToggle={() => onToggleConnection(conn)}
					onContextMenu={(event) => onShowConnContextMenu(event, conn.id)}
				/>
			{/each}
		</div>
	{/if}
</div>

<style>
	.tree-node {
		border-bottom: 1px solid rgba(45, 58, 79, 0.3);
	}

	.node-header {
		display: flex;
		align-items: center;
		gap: 6px;
		width: 100%;
		padding: 7px 10px;
		background: transparent;
		border: none;
		color: var(--gm-text-primary);
		font-family: inherit;
		font-size: 13px;
		cursor: pointer;
		text-align: left;
		transition: background 0.1s;
	}

	.node-header:hover {
		background: var(--gm-bg-hover);
	}

	.node-header.selected {
		background: rgba(59, 130, 246, 0.1);
	}

	.expand-icon {
		font-size: 18px;
		width: 12px;
		color: var(--gm-text-muted);
		flex-shrink: 0;
	}

	.device-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		flex-shrink: 0;
	}

	.node-ip {
		font-weight: 600;
		flex-shrink: 0;
	}

	.node-meta {
		display: flex;
		gap: 8px;
		margin-left: auto;
		flex-shrink: 0;
	}

	.node-type {
		font-size: 9px;
		font-weight: 600;
	}

	.node-pkt {
		font-size: 9px;
		color: var(--gm-text-muted);
		font-variant-numeric: tabular-nums;
	}

	.node-connections {
		padding-left: 16px;
		background: rgba(0, 0, 0, 0.15);
	}
</style>

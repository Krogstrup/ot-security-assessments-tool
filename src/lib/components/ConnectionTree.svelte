<script lang="ts">
	import type { PacketSummary } from '$lib/types/connections';
	import type { FrameRow } from '$lib/types/operations';
	import { connectionTree, selectedAssetId } from '$lib/stores';
	import { getConnectionPackets, openInWireshark, detectWireshark, getConnectionFrames } from '$lib/api';
	import { savePathDialog } from '$lib/utils/dialog';
	import type { Connection } from '$lib/types';
	import ConnectionContextMenu from '$lib/components/connection/ConnectionContextMenu.svelte';
	import ConnectionFramesDialog from '$lib/components/connection/ConnectionFramesDialog.svelte';
	import ConnectionTreeNode from '$lib/components/connection/ConnectionTreeNode.svelte';
	import { onMount } from 'svelte';

	let expandedNodes = $state<Set<string>>(new Set());
	let expandedConns = $state<Set<string>>(new Set());
	let packetCache = $state<Map<string, PacketSummary[]>>(new Map());
	let loadingConns = $state<Set<string>>(new Set());

	function toggleNode(ip: string) {
		const next = new Set(expandedNodes);
		if (next.has(ip)) next.delete(ip);
		else next.add(ip);
		expandedNodes = next;
	}

	async function toggleConnection(conn: Connection) {
		const next = new Set(expandedConns);
		if (next.has(conn.id)) {
			next.delete(conn.id);
			expandedConns = next;
			return;
		}
		next.add(conn.id);
		expandedConns = next;

		if (!packetCache.has(conn.id)) {
			const loading = new Set(loadingConns);
			loading.add(conn.id);
			loadingConns = loading;
			try {
				const packets = await getConnectionPackets(conn.id);
				const cache = new Map(packetCache);
				cache.set(conn.id, packets);
				packetCache = cache;
			} catch (err) {
				console.error('Failed to fetch packets for', conn.id, err);
			} finally {
				const done = new Set(loadingConns);
				done.delete(conn.id);
				loadingConns = done;
			}
		}
	}

	function selectNode(ip: string) {
		selectedAssetId.set(ip);
	}

	function formatBytes(bytes: number): string {
		if (bytes < 1024) return `${bytes} B`;
		if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
		return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
	}

	function formatTime(iso: string): string {
		try {
			const d = new Date(iso);
			return d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' });
		} catch {
			return iso;
		}
	}

	let wiresharkAvailable = $state(false);
	let connCtxMenu = $state<{ x: number; y: number; connId: string; show: boolean }>({
		x: 0,
		y: 0,
		connId: '',
		show: false
	});
	let viewFrames = $state<{ connId: string; frames: FrameRow[]; show: boolean }>({
		connId: '',
		frames: [],
		show: false
	});

	onMount(() => {
		checkWireshark();
		const handler = () => {
			if (connCtxMenu.show) connCtxMenu = { ...connCtxMenu, show: false };
		};
		window.addEventListener('click', handler);
		return () => window.removeEventListener('click', handler);
	});

	async function checkWireshark() {
		try {
			const info = await detectWireshark();
			wiresharkAvailable = info.found;
		} catch {
			wiresharkAvailable = false;
		}
	}

	function showConnContextMenu(e: MouseEvent, connId: string) {
		e.preventDefault();
		e.stopPropagation();
		connCtxMenu = { x: e.clientX, y: e.clientY, connId, show: true };
	}

	async function handleOpenInWireshark() {
		if (!connCtxMenu.connId) return;
		try {
			await openInWireshark(connCtxMenu.connId);
		} catch (err) {
			console.error('Wireshark launch failed:', err);
		}
		connCtxMenu = { ...connCtxMenu, show: false };
	}

	async function handleViewFrames(connId?: string) {
		const id = connId || connCtxMenu.connId;
		if (!id) return;
		try {
			const frames = await getConnectionFrames(id);
			viewFrames = { connId: id, frames, show: true };
		} catch (err) {
			console.error('Failed to get frames:', err);
		}
		connCtxMenu = { ...connCtxMenu, show: false };
	}

	async function handleExportCsv() {
		if (!viewFrames.connId) return;
		try {
			const path = await savePathDialog({
				title: 'Export Frames as CSV',
				defaultPath: `frames_${viewFrames.connId.slice(0, 8)}.csv`,
				filters: [{ name: 'CSV Files', extensions: ['csv'] }]
			});
			if (path) {
				const { saveFramesCsv } = await import('$lib/api');
				await saveFramesCsv(viewFrames.connId, path);
			}
		} catch (err) {
			console.error('CSV export failed:', err);
		}
	}

	function closeViewFrames() {
		viewFrames = { ...viewFrames, show: false };
	}
</script>

<div class="tree-container">
	<div class="tree-header">
		<span class="tree-title">CONNECTION TREE</span>
		<span class="tree-count">{$connectionTree.length} nodes</span>
	</div>

	<div class="tree-content">
		{#if $connectionTree.length === 0}
			<div class="tree-empty">No connections. Import a PCAP file to populate the tree.</div>
		{:else}
			{#each $connectionTree as node}
				<ConnectionTreeNode
					{node}
					isExpanded={expandedNodes.has(node.ip)}
					isSelected={$selectedAssetId === node.ip}
					{expandedConns}
					{loadingConns}
					{packetCache}
					{formatBytes}
					{formatTime}
					onToggleNode={toggleNode}
					onSelectNode={selectNode}
					onToggleConnection={toggleConnection}
					onShowConnContextMenu={showConnContextMenu}
				/>
			{/each}
		{/if}
	</div>
</div>

<ConnectionContextMenu
	show={connCtxMenu.show}
	x={connCtxMenu.x}
	y={connCtxMenu.y}
	wiresharkAvailable={wiresharkAvailable}
	onViewFrames={() => handleViewFrames()}
	onOpenInWireshark={handleOpenInWireshark}
	onClose={() => (connCtxMenu = { ...connCtxMenu, show: false })}
/>

<ConnectionFramesDialog
	show={viewFrames.show}
	frames={viewFrames.frames}
	{formatTime}
	onClose={closeViewFrames}
	onExportCsv={handleExportCsv}
/>

<style>
	.tree-container {
		display: flex;
		flex-direction: column;
		height: 100%;
		background: var(--gm-bg-secondary);
		border-right: 1px solid var(--gm-border);
		width: 100%;
	}

	.tree-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 10px 12px;
		border-bottom: 1px solid var(--gm-border);
	}

	.tree-title {
		font-size: 10px;
		font-weight: 600;
		letter-spacing: 1.5px;
		color: var(--gm-text-muted);
	}

	.tree-count {
		font-size: 10px;
		color: var(--gm-text-muted);
	}

	.tree-content {
		flex: 1;
		overflow-y: auto;
		padding: 4px 0;
	}

	.tree-empty {
		padding: 20px 12px;
		font-size: 11px;
		color: var(--gm-text-muted);
		text-align: center;
		line-height: 1.5;
	}
</style>

<script lang="ts">
	import type { Connection, PacketSummary } from '$lib/types/connections';
	import ConnectionPacketList from './ConnectionPacketList.svelte';

	interface Props {
		conn: Connection;
		isExpanded: boolean;
		isLoading: boolean;
		packets: PacketSummary[] | undefined;
		formatBytes: (bytes: number) => string;
		formatTime: (iso: string) => string;
		onToggle: () => void;
		onContextMenu: (event: MouseEvent) => void;
	}

	let { conn, isExpanded, isLoading, packets, formatBytes, formatTime, onToggle, onContextMenu }: Props = $props();
</script>

<div class="conn-entry">
	<button
		class="conn-header"
		class:expanded={isExpanded}
		onclick={onToggle}
		oncontextmenu={onContextMenu}
	>
		<span class="expand-icon small">{isExpanded ? '\u25BC' : '\u25B6'}</span>
		<span class="conn-arrow">
			:{conn.src_port} → {conn.dst_ip}:{conn.dst_port}
		</span>
		<span class="conn-proto">{conn.protocol}</span>
		<span class="conn-count">{conn.packet_count.toLocaleString()}</span>
	</button>

	{#if isExpanded}
		<div class="conn-details">
			<div class="conn-meta-row">
				<span class="meta-label">Transport</span>
				<span class="meta-value">{conn.transport.toUpperCase()}</span>
			</div>
			<div class="conn-meta-row">
				<span class="meta-label">Bytes</span>
				<span class="meta-value">{formatBytes(conn.byte_count)}</span>
			</div>
			{#if conn.origin_files.length > 0}
				<div class="conn-meta-row">
					<span class="meta-label">Files</span>
					<span class="meta-value">{conn.origin_files.join(', ')}</span>
				</div>
			{/if}

			{#if isLoading}
				<div class="packet-loading">Loading packets...</div>
			{:else if packets}
				<ConnectionPacketList {packets} {formatTime} />
			{/if}
		</div>
	{/if}
</div>

<style>
	.conn-entry {
		border-top: 1px solid rgba(45, 58, 79, 0.2);
	}

	.conn-header {
		display: flex;
		align-items: center;
		gap: 6px;
		width: 100%;
		padding: 5px 8px;
		background: transparent;
		border: none;
		color: var(--gm-text-secondary);
		font-family: inherit;
		font-size: 13px;
		cursor: pointer;
		text-align: left;
		transition: background 0.1s;
	}

	.conn-header:hover {
		background: var(--gm-bg-hover);
	}

	.expand-icon.small {
		font-size: 7px;
		width: 10px;
		color: var(--gm-text-muted);
		flex-shrink: 0;
	}

	.conn-arrow {
		flex: 1;
		min-width: 0;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.conn-proto {
		font-size: 9px;
		font-weight: 600;
		color: var(--gm-modbus);
		flex-shrink: 0;
	}

	.conn-count {
		font-size: 9px;
		color: var(--gm-text-muted);
		font-variant-numeric: tabular-nums;
		flex-shrink: 0;
	}

	.conn-details {
		padding: 4px 8px 8px 28px;
		background: rgba(0, 0, 0, 0.1);
		font-size: 13px;
	}

	.conn-meta-row {
		display: flex;
		justify-content: space-between;
		padding: 2px 0;
	}

	.meta-label {
		color: var(--gm-text-muted);
	}

	.meta-value {
		color: var(--gm-text-secondary);
		font-weight: 500;
		text-align: right;
		max-width: 180px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.packet-loading {
		font-size: 9px;
		color: var(--gm-text-muted);
		padding: 6px 0;
	}
</style>

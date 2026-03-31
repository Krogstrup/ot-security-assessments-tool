<script lang="ts">
	import type { PacketSummary } from '$lib/types/connections';

	interface Props {
		packets: PacketSummary[];
		formatTime: (iso: string) => string;
	}

	let { packets, formatTime }: Props = $props();
</script>

<div class="packet-list">
	<div class="packet-header-row">
		<span>Time</span>
		<span>Size</span>
		<span>Protocol</span>
		<span>Source</span>
	</div>
	{#each packets.slice(0, 50) as pkt}
		<div class="packet-row">
			<span class="pkt-time">{formatTime(pkt.timestamp)}</span>
			<span class="pkt-size">{pkt.length}</span>
			<span class="pkt-proto">{pkt.protocol}</span>
			<span class="pkt-file">{pkt.origin_file}</span>
		</div>
	{/each}
	{#if packets.length > 50}
		<div class="packet-more">
			...and {packets.length - 50} more packets
		</div>
	{/if}
</div>

<style>
	.packet-list {
		margin-top: 6px;
		border-top: 1px solid rgba(45, 58, 79, 0.3);
		padding-top: 4px;
	}

	.packet-header-row {
		display: grid;
		grid-template-columns: 1fr 50px 70px 1fr;
		gap: 4px;
		font-size: 8px;
		color: var(--gm-text-muted);
		text-transform: uppercase;
		letter-spacing: 0.5px;
		padding: 2px 0;
		font-weight: 600;
	}

	.packet-row {
		display: grid;
		grid-template-columns: 1fr 50px 70px 1fr;
		gap: 4px;
		font-size: 9px;
		color: var(--gm-text-secondary);
		padding: 1px 0;
	}

	.pkt-time {
		color: var(--gm-text-muted);
	}

	.pkt-size {
		text-align: right;
		font-variant-numeric: tabular-nums;
	}

	.pkt-proto {
		font-weight: 500;
	}

	.pkt-file {
		color: var(--gm-text-muted);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.packet-more {
		font-size: 9px;
		color: var(--gm-text-muted);
		padding: 4px 0;
		font-style: italic;
	}
</style>

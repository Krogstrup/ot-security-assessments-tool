<script lang="ts">
	import type { PhysicalPort } from '$lib/types/operations';

	interface Props {
		port: PhysicalPort;
		onShowInLogical: (ip: string) => void;
	}

	let { port, onShowInLogical }: Props = $props();
</script>

<div class="detail-section">
	<h4>{port.short_name}</h4>
	{#if port.description}
		<div class="detail-row">
			<span class="detail-label">Description</span>
			<span class="detail-value">{port.description}</span>
		</div>
	{/if}
	<div class="detail-row">
		<span class="detail-label">Mode</span>
		<span class="detail-value badge" class:badge-purple={port.mode === 'trunk'}>
			{port.mode}
		</span>
	</div>
	<div class="detail-row">
		<span class="detail-label">VLANs</span>
		<span class="detail-value">{port.vlans.join(', ') || 'none'}</span>
	</div>
	<div class="detail-row">
		<span class="detail-label">Status</span>
		<span class="detail-value" class:text-red={port.shutdown}>{port.shutdown ? 'shutdown' : 'up'}</span>
	</div>
	{#if port.speed}
		<div class="detail-row">
			<span class="detail-label">Speed</span>
			<span class="detail-value">{port.speed}</span>
		</div>
	{/if}
	{#if port.ip_address}
		<div class="detail-row">
			<span class="detail-label">IP</span>
			<span class="detail-value">{port.ip_address}/{port.subnet_mask}</span>
		</div>
	{/if}
</div>

{#if port.ip_addresses.length > 0}
	<div class="detail-section">
		<h4>Connected Devices</h4>
		{#each port.ip_addresses as ip}
			<button class="device-item" onclick={() => onShowInLogical(ip)}>
				{ip}
				<span class="show-logical">Show in Logical</span>
			</button>
		{/each}
	</div>
{/if}

{#if port.mac_addresses.length > 0}
	<div class="detail-section">
		<h4>MAC Addresses ({port.mac_addresses.length})</h4>
		{#each port.mac_addresses as mac}
			<div class="mac-item">{mac}</div>
		{/each}
	</div>
{/if}

{#if port.cdp_neighbor}
	<div class="detail-section">
		<h4>CDP Neighbor</h4>
		<div class="detail-row">
			<span class="detail-label">Device</span>
			<span class="detail-value">{port.cdp_neighbor.device_id}</span>
		</div>
		<div class="detail-row">
			<span class="detail-label">Port</span>
			<span class="detail-value">{port.cdp_neighbor.remote_port}</span>
		</div>
		{#if port.cdp_neighbor.platform}
			<div class="detail-row">
				<span class="detail-label">Platform</span>
				<span class="detail-value">{port.cdp_neighbor.platform}</span>
			</div>
		{/if}
		{#if port.cdp_neighbor.ip_address}
			<div class="detail-row">
				<span class="detail-label">IP</span>
				<span class="detail-value">{port.cdp_neighbor.ip_address}</span>
			</div>
		{/if}
	</div>
{/if}

<style>
	.detail-section {
		padding: 0.75rem;
		border-bottom: 1px solid var(--gm-border);
		flex-shrink: 0;
	}

	.detail-section h4 {
		margin: 0 0 0.5rem 0;
		font-size: 0.75rem;
		color: var(--gm-text-primary);
		font-weight: 600;
		text-transform: uppercase;
	}

	.detail-row {
		display: flex;
		justify-content: space-between;
		margin-bottom: 0.35rem;
		font-size: 0.75rem;
	}

	.detail-label {
		color: var(--gm-text-secondary);
		font-weight: 600;
	}

	.detail-value {
		color: var(--gm-text-primary);
		font-family: 'JetBrains Mono', monospace;
		text-align: right;
		max-width: 50%;
		word-break: break-all;
	}

	.detail-value.badge {
		padding: 0.15rem 0.3rem;
		border-radius: 2px;
		background: rgba(0, 0, 0, 0.3);
		font-size: 0.7rem;
		font-weight: 600;
		font-family: inherit;
	}

	.detail-value.badge.badge-purple {
		background: rgba(139, 92, 246, 0.2);
		color: #8b5cf6;
	}

	.detail-value.text-red {
		color: #ef4444;
	}

	.device-item {
		width: 100%;
		padding: 0.5rem;
		margin: 0.25rem 0;
		background: rgba(59, 130, 246, 0.1);
		border: 1px solid rgba(59, 130, 246, 0.3);
		border-radius: 3px;
		color: #3b82f6;
		cursor: pointer;
		font-size: 0.7rem;
		font-family: 'JetBrains Mono', monospace;
		text-align: left;
		transition: all 0.2s;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.device-item:hover {
		background: rgba(59, 130, 246, 0.2);
		border-color: #3b82f6;
	}

	.show-logical {
		font-size: 0.65rem;
		opacity: 0.7;
	}

	.mac-item {
		padding: 0.35rem;
		background: rgba(0, 0, 0, 0.2);
		border-radius: 2px;
		font-size: 0.7rem;
		color: var(--gm-text-secondary);
		font-family: 'JetBrains Mono', monospace;
		margin-bottom: 0.25rem;
	}
</style>

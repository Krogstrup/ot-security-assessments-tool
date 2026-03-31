<script lang="ts">
	import type { PhysicalSwitch, PhysicalPort } from '$lib/types/operations';

	interface Props {
		switchInfo: PhysicalSwitch;
		onSelectPort: (port: PhysicalPort) => void;
	}

	let { switchInfo, onSelectPort }: Props = $props();

	let visiblePorts = $derived(
		switchInfo.ports.filter((port) => !port.name.startsWith('Vlan') && !port.name.startsWith('Loopback'))
	);
</script>

<div class="detail-section">
	{#if switchInfo.management_ip}
		<div class="detail-row">
			<span class="detail-label">Mgmt IP</span>
			<span class="detail-value">{switchInfo.management_ip}</span>
		</div>
	{/if}
	{#if switchInfo.ios_version}
		<div class="detail-row">
			<span class="detail-label">IOS Version</span>
			<span class="detail-value">{switchInfo.ios_version}</span>
		</div>
	{/if}
	<div class="detail-row">
		<span class="detail-label">Ports</span>
		<span class="detail-value">{switchInfo.ports.length}</span>
	</div>
	<div class="detail-row">
		<span class="detail-label">VLANs</span>
		<span class="detail-value">{Object.keys(switchInfo.vlans).length}</span>
	</div>
</div>

{#if Object.keys(switchInfo.vlans).length > 0}
	<div class="detail-section">
		<h4>VLANs</h4>
		{#each Object.entries(switchInfo.vlans) as [id, name]}
			<div class="detail-row">
				<span class="detail-label">VLAN {id}</span>
				<span class="detail-value">{name}</span>
			</div>
		{/each}
	</div>
{/if}

<div class="detail-section">
	<h4>Ports</h4>
	<div class="port-grid">
		{#each visiblePorts as port}
			<button
				class="port-chip"
				class:has-device={port.mac_addresses.length > 0 || port.ip_addresses.length > 0}
				class:is-shutdown={port.shutdown}
				class:is-trunk={port.mode === 'trunk'}
				onclick={() => onSelectPort(port)}
			>
				{port.short_name}
			</button>
		{/each}
	</div>
</div>

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

	.port-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 0.35rem;
	}

	.port-chip {
		padding: 0.4rem;
		border: 1px solid var(--gm-border);
		border-radius: 3px;
		background: rgba(0, 0, 0, 0.2);
		color: var(--gm-text-secondary);
		cursor: pointer;
		font-size: 0.7rem;
		font-weight: 600;
		font-family: 'JetBrains Mono', monospace;
		transition: all 0.2s;
	}

	.port-chip:hover {
		border-color: #3b82f6;
		color: #3b82f6;
	}

	.port-chip.has-device {
		background: rgba(59, 130, 246, 0.15);
		border-color: #3b82f6;
		color: #3b82f6;
	}

	.port-chip.is-trunk {
		background: rgba(139, 92, 246, 0.15);
		border-color: #8b5cf6;
		color: #8b5cf6;
	}

	.port-chip.is-shutdown {
		opacity: 0.5;
		border-style: dashed;
	}
</style>

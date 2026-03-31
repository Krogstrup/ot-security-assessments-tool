<script lang="ts">
	import type { PhysicalSwitch, PhysicalPort } from '$lib/types';

	interface Props {
		selectedSwitch: PhysicalSwitch | null;
		selectedPort: PhysicalPort | null;
		onClose: () => void;
		onShowInLogical: (ip: string) => void;
		onSelectPort: (port: PhysicalPort) => void;
	}

	let { selectedSwitch, selectedPort, onClose, onShowInLogical, onSelectPort }: Props = $props();
</script>

{#if selectedSwitch}
	<div class="detail-panel">
		<div class="detail-header">
			<h3>{selectedSwitch.hostname}</h3>
			<button class="detail-close" onclick={onClose}>&times;</button>
		</div>

		{#if selectedPort}
			<!-- Port detail -->
			<div class="detail-section">
				<h4>{selectedPort.short_name}</h4>
				{#if selectedPort.description}
					<div class="detail-row">
						<span class="detail-label">Description</span>
						<span class="detail-value">{selectedPort.description}</span>
					</div>
				{/if}
				<div class="detail-row">
					<span class="detail-label">Mode</span>
					<span class="detail-value badge" class:badge-purple={selectedPort.mode === 'trunk'}>
						{selectedPort.mode}
					</span>
				</div>
				<div class="detail-row">
					<span class="detail-label">VLANs</span>
					<span class="detail-value">{selectedPort.vlans.join(', ') || 'none'}</span>
				</div>
				<div class="detail-row">
					<span class="detail-label">Status</span>
					<span class="detail-value" class:text-red={selectedPort.shutdown}>
						{selectedPort.shutdown ? 'shutdown' : 'up'}
					</span>
				</div>
				{#if selectedPort.speed}
					<div class="detail-row">
						<span class="detail-label">Speed</span>
						<span class="detail-value">{selectedPort.speed}</span>
					</div>
				{/if}
				{#if selectedPort.ip_address}
					<div class="detail-row">
						<span class="detail-label">IP</span>
						<span class="detail-value">{selectedPort.ip_address}/{selectedPort.subnet_mask}</span>
					</div>
				{/if}
			</div>

			<!-- Connected devices on this port -->
			{#if selectedPort.ip_addresses.length > 0}
				<div class="detail-section">
					<h4>Connected Devices</h4>
					{#each selectedPort.ip_addresses as ip}
						<button class="device-item" onclick={() => onShowInLogical(ip)}>
							{ip}
							<span class="show-logical">Show in Logical</span>
						</button>
					{/each}
				</div>
			{/if}

			{#if selectedPort.mac_addresses.length > 0}
				<div class="detail-section">
					<h4>MAC Addresses ({selectedPort.mac_addresses.length})</h4>
					{#each selectedPort.mac_addresses as mac}
						<div class="mac-item">{mac}</div>
					{/each}
				</div>
			{/if}

			{#if selectedPort.cdp_neighbor}
				<div class="detail-section">
					<h4>CDP Neighbor</h4>
					<div class="detail-row">
						<span class="detail-label">Device</span>
						<span class="detail-value">{selectedPort.cdp_neighbor.device_id}</span>
					</div>
					<div class="detail-row">
						<span class="detail-label">Port</span>
						<span class="detail-value">{selectedPort.cdp_neighbor.remote_port}</span>
					</div>
					{#if selectedPort.cdp_neighbor.platform}
						<div class="detail-row">
							<span class="detail-label">Platform</span>
							<span class="detail-value">{selectedPort.cdp_neighbor.platform}</span>
						</div>
					{/if}
					{#if selectedPort.cdp_neighbor.ip_address}
						<div class="detail-row">
							<span class="detail-label">IP</span>
							<span class="detail-value">{selectedPort.cdp_neighbor.ip_address}</span>
						</div>
					{/if}
				</div>
			{/if}
		{:else}
			<!-- Switch overview -->
			<div class="detail-section">
				{#if selectedSwitch.management_ip}
					<div class="detail-row">
						<span class="detail-label">Mgmt IP</span>
						<span class="detail-value">{selectedSwitch.management_ip}</span>
					</div>
				{/if}
				{#if selectedSwitch.ios_version}
					<div class="detail-row">
						<span class="detail-label">IOS Version</span>
						<span class="detail-value">{selectedSwitch.ios_version}</span>
					</div>
				{/if}
				<div class="detail-row">
					<span class="detail-label">Ports</span>
					<span class="detail-value">{selectedSwitch.ports.length}</span>
				</div>
				<div class="detail-row">
					<span class="detail-label">VLANs</span>
					<span class="detail-value">{Object.keys(selectedSwitch.vlans).length}</span>
				</div>
			</div>

			<!-- VLAN list -->
			{#if Object.keys(selectedSwitch.vlans).length > 0}
				<div class="detail-section">
					<h4>VLANs</h4>
					{#each Object.entries(selectedSwitch.vlans) as [id, name]}
						<div class="detail-row">
							<span class="detail-label">VLAN {id}</span>
							<span class="detail-value">{name}</span>
						</div>
					{/each}
				</div>
			{/if}

			<!-- Port summary -->
			<div class="detail-section">
				<h4>Ports</h4>
				<div class="port-grid">
					{#each selectedSwitch.ports.filter((p) => !p.name.startsWith('Vlan') && !p.name.startsWith('Loopback')) as port}
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
		{/if}
	</div>
{/if}

<style>
	.detail-panel {
		position: absolute;
		right: 0;
		top: 0;
		bottom: 0;
		width: 280px;
		background: var(--gm-bg-secondary);
		border-left: 1px solid var(--gm-border);
		display: flex;
		flex-direction: column;
		overflow: hidden;
		z-index: 100;
		box-shadow: -2px 0 8px rgba(0, 0, 0, 0.3);
	}

	.detail-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem;
		border-bottom: 1px solid var(--gm-border);
		flex-shrink: 0;
	}

	.detail-header h3 {
		margin: 0;
		font-size: 0.95rem;
		color: var(--gm-text-primary);
		font-family: 'JetBrains Mono', monospace;
	}

	.detail-close {
		background: none;
		border: none;
		color: var(--gm-text-secondary);
		cursor: pointer;
		font-size: 1.5rem;
		padding: 0;
		width: 2rem;
		height: 2rem;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.detail-close:hover {
		color: var(--gm-text-primary);
	}

	:global(.detail-panel) {
		overflow-y: auto;
	}

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

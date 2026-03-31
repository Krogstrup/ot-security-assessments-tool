<script lang="ts">
	import type { DeviceLocation } from '$lib/types/operations';

	interface Props {
		locations: DeviceLocation[];
		onShowInLogical: (ip: string) => void;
	}

	let { locations, onShowInLogical }: Props = $props();
</script>

<div class="locations-panel">
	<h4>Device Locations</h4>
	<div class="locations-list">
		{#each locations as loc}
			<button class="location-item" onclick={() => onShowInLogical(loc.ip_address)}>
				<span class="loc-ip">{loc.ip_address}</span>
				<span class="loc-detail">
					{loc.switch_hostname} / {loc.port_name}
					{#if loc.vlan}
						<span class="loc-vlan">V{loc.vlan}</span>
					{/if}
				</span>
			</button>
		{/each}
	</div>
</div>

<style>
	.locations-panel {
		position: absolute;
		bottom: 0;
		left: 280px;
		right: 0;
		max-height: 200px;
		background: var(--gm-bg-secondary);
		border-top: 1px solid var(--gm-border);
		padding: 0.75rem;
		overflow-y: auto;
	}

	.locations-panel h4 {
		margin: 0 0 0.5rem 0;
		font-size: 0.875rem;
		color: var(--gm-text-primary);
		font-weight: 600;
	}

	.locations-list {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.location-item {
		padding: 0.5rem 0.75rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.3);
		border-radius: 3px;
		color: #6366f1;
		cursor: pointer;
		font-size: 0.8125rem;
		transition: all 0.2s;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
		white-space: nowrap;
	}

	.location-item:hover {
		background: rgba(99, 102, 241, 0.2);
	}

	.loc-ip {
		font-family: 'JetBrains Mono', monospace;
		font-weight: 600;
	}

	.loc-detail {
		font-size: 0.75rem;
		opacity: 0.8;
	}

	.loc-vlan {
		font-size: 0.7rem;
		background: rgba(0, 0, 0, 0.2);
		padding: 0.1rem 0.3rem;
		border-radius: 2px;
		margin-left: 0.25rem;
	}
</style>

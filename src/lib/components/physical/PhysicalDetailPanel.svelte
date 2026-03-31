<script lang="ts">
	import type { PhysicalSwitch, PhysicalPort } from '$lib/types/operations';
	import PhysicalPortDetail from './PhysicalPortDetail.svelte';
	import PhysicalSwitchOverview from './PhysicalSwitchOverview.svelte';

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
			<PhysicalPortDetail port={selectedPort} {onShowInLogical} />
		{:else}
			<PhysicalSwitchOverview switchInfo={selectedSwitch} {onSelectPort} />
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
</style>

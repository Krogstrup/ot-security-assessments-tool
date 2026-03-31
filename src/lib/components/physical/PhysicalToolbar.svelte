<script lang="ts">
	interface Props {
		activeTab: 'imported' | 'inferred' | 'redundancy';
		switchCount: number;
		linkCount: number;
		mappedDeviceCount: number;
		inferredSubnetCount: number;
		inferredGatewayCount: number;
		inferredSwitchCandidateCount: number;
		redundancyDeviceCount: number;
		redundancyManagerCount: number;
		redundancyTcCount: number;
		inferring: boolean;
		onTabChange: (tab: 'imported' | 'inferred' | 'redundancy') => void;
		onFit: () => void;
		onRelayout: () => void;
		onClear: () => Promise<void>;
		onRunInference: () => Promise<void>;
	}

	let {
		activeTab,
		switchCount,
		linkCount,
		mappedDeviceCount,
		inferredSubnetCount,
		inferredGatewayCount,
		inferredSwitchCandidateCount,
		redundancyDeviceCount,
		redundancyManagerCount,
		redundancyTcCount,
		inferring,
		onTabChange,
		onFit,
		onRelayout,
		onClear,
		onRunInference
	}: Props = $props();
</script>

<div class="physical-toolbar">
	<div class="toolbar-section">
		<h2 class="view-title">Physical View</h2>
		<span class="toolbar-sep"></span>
		<div class="tab-switcher">
			<button class="tab-btn" class:active={activeTab === 'imported'} onclick={() => onTabChange('imported')}
				>Imported</button>
			<button class="tab-btn" class:active={activeTab === 'inferred'} onclick={() => onTabChange('inferred')}
				>Inferred</button>
			<button
				class="tab-btn"
				class:active={activeTab === 'redundancy'}
				onclick={() => onTabChange('redundancy')}
			>Ring Redundancy</button>
		</div>
		<span class="toolbar-sep"></span>
		{#if activeTab === 'imported'}
			<span class="switch-count">{switchCount} switches</span>
			<span class="link-count">{linkCount} links</span>
			<span class="device-count">{mappedDeviceCount} mapped devices</span>
		{:else if activeTab === 'inferred'}
			<span class="switch-count">{inferredSubnetCount} subnets</span>
			<span class="link-count">{inferredGatewayCount} gateways</span>
			<span class="device-count">{inferredSwitchCandidateCount} switch candidates</span>
		{:else}
			<span class="switch-count">{redundancyDeviceCount} devices</span>
			<span class="link-count">{redundancyManagerCount} managers</span>
			<span class="device-count">{redundancyTcCount} TC events</span>
		{/if}
	</div>
	<div class="toolbar-section">
		{#if activeTab === 'imported'}
			<button class="tool-btn" onclick={onFit}>Fit</button>
			<button class="tool-btn" onclick={onRelayout}>Relayout</button>
			<button class="tool-btn danger" onclick={onClear}>Clear</button>
		{:else}
			<button class="tool-btn" onclick={onRunInference} disabled={inferring}>
				{inferring ? 'Running...' : 'Run Inference'}
			</button>
		{/if}
	</div>
</div>

<style>
	.physical-toolbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem 1rem;
		background: var(--gm-bg-secondary);
		border-bottom: 1px solid var(--gm-border);
		flex-shrink: 0;
		gap: 1rem;
		flex-wrap: wrap;
	}

	.toolbar-section {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		flex-wrap: wrap;
	}

	.view-title {
		margin: 0;
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--gm-text-primary);
		white-space: nowrap;
	}

	.toolbar-sep {
		width: 1px;
		height: 1.5rem;
		background: var(--gm-border);
	}

	.tab-switcher {
		display: flex;
		gap: 0.35rem;
	}

	.tab-btn {
		padding: 0.35rem 0.75rem;
		border: 1px solid var(--gm-border);
		border-radius: 3px;
		background: transparent;
		color: var(--gm-text-secondary);
		cursor: pointer;
		font-size: 0.8125rem;
		font-weight: 500;
		transition: all 0.2s;
	}

	.tab-btn:hover {
		background: rgba(255, 255, 255, 0.05);
	}

	.tab-btn.active {
		background: #6366f1;
		color: white;
		border-color: #6366f1;
	}

	.switch-count,
	.link-count,
	.device-count {
		font-size: 0.75rem;
		color: var(--gm-text-secondary);
		font-weight: 500;
		white-space: nowrap;
	}

	.tool-btn {
		padding: 0.35rem 0.75rem;
		border: 1px solid var(--gm-border);
		border-radius: 3px;
		background: var(--gm-bg-tertiary);
		color: var(--gm-text-primary);
		cursor: pointer;
		font-size: 0.8125rem;
		font-weight: 500;
		transition: all 0.2s;
	}

	.tool-btn:hover:not(:disabled) {
		background: rgba(255, 255, 255, 0.05);
	}

	.tool-btn.danger {
		color: #ef4444;
	}

	.tool-btn.danger:hover:not(:disabled) {
		background: rgba(239, 68, 68, 0.1);
	}

	.tool-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>

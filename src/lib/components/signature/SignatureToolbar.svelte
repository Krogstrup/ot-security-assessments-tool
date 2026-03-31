<script lang="ts">
	interface Props {
		totalCount: number;
		reloading: boolean;
		testing: boolean;
		onReload: () => void;
		onTest: () => void;
	}

	let { totalCount, reloading, testing, onReload, onTest }: Props = $props();
</script>

<div class="sig-toolbar">
	<h2 class="view-title">Signature Editor</h2>
	<div class="toolbar-controls">
		<span class="sig-count">{totalCount} signatures loaded</span>
		<button class="tool-btn" onclick={onReload} disabled={reloading}>
			{reloading ? 'Reloading...' : 'Reload'}
		</button>
		<button class="tool-btn test-btn" onclick={onTest} disabled={testing}>
			{testing ? 'Testing...' : 'Test Signature'}
		</button>
	</div>
</div>

<style>
	.sig-toolbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 10px 16px;
		border-bottom: 1px solid var(--gm-border);
		background: var(--gm-bg-secondary);
		gap: 16px;
	}

	.view-title {
		font-size: 13px;
		font-weight: 600;
		letter-spacing: 1px;
		text-transform: uppercase;
		color: var(--gm-text-primary);
		margin: 0;
		white-space: nowrap;
	}

	.toolbar-controls {
		display: flex;
		align-items: center;
		gap: 10px;
	}

	.sig-count {
		font-size: 10px;
		color: var(--gm-text-muted);
	}

	.tool-btn {
		padding: 5px 12px;
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		color: var(--gm-text-secondary);
		font-family: inherit;
		font-size: 11px;
		cursor: pointer;
		transition: all 0.15s;
	}

	.tool-btn:hover:not(:disabled) {
		background: var(--gm-bg-hover);
		color: var(--gm-text-primary);
		border-color: var(--gm-border-active);
	}

	.tool-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.test-btn {
		background: rgba(16, 185, 129, 0.1);
		border-color: rgba(16, 185, 129, 0.3);
		color: #10b981;
	}

	.test-btn:hover:not(:disabled) {
		background: rgba(16, 185, 129, 0.2);
		border-color: rgba(16, 185, 129, 0.5);
	}
</style>

<script lang="ts">
	interface Props {
		nodeCount: number;
		onForceLayout: () => void;
		onUsePurdue: () => void;
		onDismiss: () => void;
	}

	let { nodeCount, onForceLayout, onUsePurdue, onDismiss }: Props = $props();
</script>

<div class="large-network-banner">
	<span class="large-net-icon">&#9888;</span>
	<span class="large-net-msg">
		Large network ({nodeCount.toLocaleString()} devices).
		Force-directed layout may freeze the UI.
	</span>
	<button class="large-net-btn" onclick={onForceLayout}>Run Layout Anyway</button>
	<button class="large-net-btn secondary" onclick={onUsePurdue}>Use Purdue Layout</button>
	<button class="large-net-dismiss" onclick={onDismiss}>&times;</button>
</div>

<style>
	.large-network-banner {
		position: absolute;
		top: 16px;
		left: 50%;
		transform: translateX(-50%);
		z-index: 20;
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 10px 16px;
		background: var(--gm-bg-panel);
		border: 1px solid #f59e0b;
		border-radius: 6px;
		color: #f59e0b;
		font-size: 12px;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
		max-width: 600px;
		flex-wrap: wrap;
	}

	.large-net-icon {
		font-size: 16px;
		flex-shrink: 0;
	}

	.large-net-msg {
		flex: 1;
		min-width: 200px;
		color: var(--gm-text-primary);
	}

	.large-net-btn {
		padding: 4px 12px;
		background: #f59e0b;
		border: none;
		border-radius: 4px;
		color: #0f172a;
		font-family: inherit;
		font-size: 11px;
		font-weight: 600;
		cursor: pointer;
		white-space: nowrap;
		flex-shrink: 0;
	}

	.large-net-btn:hover {
		background: #fbbf24;
	}

	.large-net-btn.secondary {
		background: transparent;
		border: 1px solid #f59e0b;
		color: #f59e0b;
	}

	.large-net-btn.secondary:hover {
		background: rgba(245, 158, 11, 0.1);
	}

	.large-net-dismiss {
		padding: 2px 6px;
		background: transparent;
		border: none;
		color: #64748b;
		font-size: 16px;
		cursor: pointer;
		flex-shrink: 0;
		line-height: 1;
	}

	.large-net-dismiss:hover {
		color: #94a3b8;
	}
</style>

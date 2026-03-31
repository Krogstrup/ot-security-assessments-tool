<script lang="ts">
	let {
		targetNodeId,
		depth,
		onDepthChange,
		onFit,
		onRelayout
	}: {
		targetNodeId: string;
		depth: number;
		onDepthChange: (value: number) => void;
		onFit: () => void;
		onRelayout: () => void;
	} = $props();
</script>

<div class="watch-toolbar">
	<div class="toolbar-section">
		<h2 class="view-title">Watch: {targetNodeId}</h2>
		<span class="toolbar-sep"></span>
		<label class="depth-label">
			Depth:
			<input
				type="range"
				class="depth-slider"
				min="1"
				max="5"
				value={depth}
				oninput={(event) => onDepthChange(parseInt((event.target as HTMLInputElement).value, 10))}
			/>
			<span class="depth-value">{depth} hop{depth !== 1 ? 's' : ''}</span>
		</label>
	</div>
	<div class="toolbar-section">
		<button class="tool-btn" onclick={onFit}>Fit</button>
		<button class="tool-btn" onclick={onRelayout}>Relayout</button>
	</div>
</div>

<style>
	.watch-toolbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 8px 16px;
		border-bottom: 1px solid var(--gm-border);
		background: var(--gm-bg-secondary);
	}

	.toolbar-section {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.toolbar-sep {
		width: 1px;
		height: 18px;
		background: var(--gm-border);
		margin: 0 4px;
	}

	.view-title {
		font-size: 13px;
		font-weight: 600;
		letter-spacing: 1px;
		text-transform: uppercase;
		color: var(--gm-text-primary);
		margin: 0;
	}

	.depth-label {
		font-size: 11px;
		color: var(--gm-text-secondary);
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.depth-slider {
		width: 80px;
		accent-color: #10b981;
	}

	.depth-value {
		font-weight: 600;
		color: #10b981;
		min-width: 50px;
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

	.tool-btn:hover {
		background: var(--gm-bg-hover);
		color: var(--gm-text-primary);
		border-color: var(--gm-border-active);
	}
</style>

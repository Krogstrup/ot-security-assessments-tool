<script lang="ts">
	interface Props {
		expanded: boolean;
		enabled: boolean;
		playing: boolean;
		speed: 1 | 2 | 5;
		hasData: boolean;
		onToggleExpanded: () => void;
		onToggleEnabled: () => void;
		onTogglePlayback: () => void;
		onCycleSpeed: () => void;
	}

	let {
		expanded,
		enabled,
		playing,
		speed,
		hasData,
		onToggleExpanded,
		onToggleEnabled,
		onTogglePlayback,
		onCycleSpeed
	}: Props = $props();
</script>

<button
	class="expand-btn"
	onclick={onToggleExpanded}
	title={expanded ? 'Collapse timeline' : 'Expand timeline'}
>
	{#if expanded}&#9660;{:else}&#9650;{/if}
</button>

<button
	class="toggle-btn"
	class:active={enabled}
	onclick={onToggleEnabled}
	disabled={!hasData}
	title={enabled ? 'Disable timeline filter' : 'Enable timeline filter'}
>
	{#if enabled}ON{:else}OFF{/if}
</button>

{#if hasData && enabled}
	<button
		class="play-btn"
		onclick={onTogglePlayback}
		title={playing ? 'Pause playback' : 'Start playback'}
	>
		{#if playing}&#10074;&#10074;{:else}&#9654;{/if}
	</button>
	<button class="speed-btn" onclick={onCycleSpeed} title="Playback speed">{speed}x</button>
{:else if !hasData}
	<span class="no-data">No timeline data</span>
{:else}
	<span class="no-data">Timeline disabled</span>
{/if}

<style>
	.expand-btn {
		background: none;
		border: 1px solid var(--gm-border);
		border-radius: 3px;
		color: var(--gm-text-muted);
		font-size: 8px;
		width: 22px;
		height: 22px;
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
		flex-shrink: 0;
		transition: all 0.15s;
		font-family: inherit;
		padding: 0;
	}

	.expand-btn:hover {
		color: var(--gm-text-primary);
		border-color: var(--gm-active);
	}

	.toggle-btn {
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		color: var(--gm-text-muted);
		font-family: inherit;
		font-size: 9px;
		font-weight: 700;
		padding: 3px 8px;
		cursor: pointer;
		flex-shrink: 0;
		transition: all 0.15s;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.toggle-btn:hover:not(:disabled) {
		border-color: var(--gm-active);
		color: var(--gm-text-primary);
	}

	.toggle-btn.active {
		background: rgba(16, 185, 129, 0.15);
		border-color: var(--gm-active);
		color: var(--gm-active);
	}

	.toggle-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.play-btn {
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		color: var(--gm-active);
		font-size: 10px;
		width: 28px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
		flex-shrink: 0;
		transition: all 0.15s;
		font-family: inherit;
		padding: 0;
	}

	.play-btn:hover {
		border-color: var(--gm-active);
		background: rgba(16, 185, 129, 0.1);
	}

	.speed-btn {
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		color: var(--gm-text-secondary);
		font-family: inherit;
		font-size: 10px;
		font-weight: 600;
		padding: 3px 8px;
		cursor: pointer;
		flex-shrink: 0;
		transition: all 0.15s;
	}

	.speed-btn:hover {
		border-color: var(--gm-active);
		color: var(--gm-active);
	}

	.no-data {
		font-size: 10px;
		color: var(--gm-text-muted);
		font-style: italic;
	}
</style>

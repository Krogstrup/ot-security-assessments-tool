<script lang="ts">
	interface Props {
		hasData: boolean;
		enabled: boolean;
		earliestLabel: string;
		latestLabel: string;
		currentTimestamp: string;
		progressPercent: number;
		position: number;
		onSliderInput: (event: Event) => void;
	}

	let {
		hasData,
		enabled,
		earliestLabel,
		latestLabel,
		currentTimestamp,
		progressPercent,
		position,
		onSliderInput
	}: Props = $props();
</script>

{#if hasData && enabled}
	<div class="slider-container">
		<span class="range-label">{earliestLabel}</span>
		<div class="slider-track-wrapper">
			<input
				type="range"
				class="timeline-slider"
				min="0"
				max="1"
				step="0.001"
				value={position}
				oninput={onSliderInput}
			/>
			<div class="slider-progress" style="width: {progressPercent}%"></div>
		</div>
		<span class="range-label">{latestLabel}</span>
	</div>

	<span class="current-time" title={currentTimestamp}>{currentTimestamp}</span>
{/if}

<style>
	.slider-container {
		flex: 1;
		display: flex;
		align-items: center;
		gap: 6px;
		min-width: 0;
	}

	.range-label {
		font-size: 9px;
		color: var(--gm-text-muted);
		font-family: inherit;
		flex-shrink: 0;
		white-space: nowrap;
	}

	.slider-track-wrapper {
		flex: 1;
		position: relative;
		height: 18px;
		display: flex;
		align-items: center;
	}

	.timeline-slider {
		width: 100%;
		height: 4px;
		-webkit-appearance: none;
		appearance: none;
		background: var(--gm-bg-panel);
		border-radius: 2px;
		outline: none;
		cursor: pointer;
		position: relative;
		z-index: 2;
	}

	.timeline-slider::-webkit-slider-thumb {
		-webkit-appearance: none;
		appearance: none;
		width: 12px;
		height: 12px;
		border-radius: 50%;
		background: var(--gm-active);
		border: 2px solid var(--gm-bg-secondary);
		cursor: pointer;
		transition: transform 0.1s;
	}

	.timeline-slider::-webkit-slider-thumb:hover {
		transform: scale(1.3);
	}

	.timeline-slider::-moz-range-thumb {
		width: 12px;
		height: 12px;
		border-radius: 50%;
		background: var(--gm-active);
		border: 2px solid var(--gm-bg-secondary);
		cursor: pointer;
	}

	.slider-progress {
		position: absolute;
		left: 0;
		top: 50%;
		transform: translateY(-50%);
		height: 4px;
		background: var(--gm-active);
		border-radius: 2px;
		pointer-events: none;
		z-index: 1;
		opacity: 0.5;
	}

	.current-time {
		font-size: 10px;
		color: var(--gm-text-primary);
		font-family: inherit;
		background: var(--gm-bg-panel);
		padding: 3px 8px;
		border-radius: 3px;
		border: 1px solid var(--gm-border);
		white-space: nowrap;
		flex-shrink: 0;
		max-width: 200px;
		overflow: hidden;
		text-overflow: ellipsis;
	}
</style>

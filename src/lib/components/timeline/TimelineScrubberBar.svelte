<script lang="ts">
	import TimelineBarButtons from './TimelineBarButtons.svelte';
	import TimelineBarSlider from './TimelineBarSlider.svelte';

	interface Props {
		expanded: boolean;
		enabled: boolean;
		playing: boolean;
		speed: 1 | 2 | 5;
		hasData: boolean;
		earliestLabel: string;
		latestLabel: string;
		currentTimestamp: string;
		progressPercent: number;
		position: number;
		onToggleExpanded: () => void;
		onToggleEnabled: () => void;
		onTogglePlayback: () => void;
		onCycleSpeed: () => void;
		onSliderInput: (event: Event) => void;
	}

	let {
		expanded,
		enabled,
		playing,
		speed,
		hasData,
		earliestLabel,
		latestLabel,
		currentTimestamp,
		progressPercent,
		position,
		onToggleExpanded,
		onToggleEnabled,
		onTogglePlayback,
		onCycleSpeed,
		onSliderInput
	}: Props = $props();
</script>

<div class="scrubber-bar">
	<TimelineBarButtons
		{expanded}
		{enabled}
		{playing}
		{speed}
		{hasData}
		{onToggleExpanded}
		{onToggleEnabled}
		{onTogglePlayback}
		{onCycleSpeed}
	/>

	<TimelineBarSlider
		{hasData}
		{enabled}
		{earliestLabel}
		{latestLabel}
		{currentTimestamp}
		{progressPercent}
		{position}
		{onSliderInput}
	/>
</div>

<style>
	.scrubber-bar {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 6px 12px;
		height: 40px;
		box-sizing: border-box;
	}
</style>

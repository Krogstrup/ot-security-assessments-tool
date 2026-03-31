<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import type { TimelineRange } from '$lib/types/analysis';
	import TimelineScrubberBar from './timeline/TimelineScrubberBar.svelte';
	import TimelineScrubberDetail from './timeline/TimelineScrubberDetail.svelte';
	import { formatTimelineCompact, formatTimelineTimestamp } from './timeline/timelineFormatting';
	import { type TimelineSpeed } from './timeline/timelinePlayback';
	import { createTimelineController } from './timeline/timelineController';

	let position = $state(1.0);
	let enabled = $state(false);
	let playing = $state(false);
	let speed = $state<TimelineSpeed>(1);
	let expanded = $state(false);
	let range = $state<TimelineRange | null>(null);
	let error = $state<string | null>(null);

	let earliestDate = $derived(range?.earliest ? new Date(range.earliest) : null);
	let latestDate = $derived(range?.latest ? new Date(range.latest) : null);
	let totalSpanMs = $derived(earliestDate && latestDate ? latestDate.getTime() - earliestDate.getTime() : 0);
	let currentDate = $derived(earliestDate && totalSpanMs > 0 ? new Date(earliestDate.getTime() + totalSpanMs * position) : latestDate);
	let currentTimestamp = $derived(currentDate ? formatTimelineTimestamp(currentDate) : '--');
	let hasData = $derived(range !== null && range.earliest !== null && range.latest !== null && range.connection_count > 0);
	let progressPercent = $derived(Math.round(position * 100));
	let rangeStartLabel = $derived(range?.earliest ? formatTimelineTimestamp(new Date(range.earliest)) : '--');
	let rangeEndLabel = $derived(range?.latest ? formatTimelineTimestamp(new Date(range.latest)) : '--');

	const ctrl = createTimelineController({
		getPosition: () => position,
		setPosition: (v) => (position = v),
		getEnabled: () => enabled,
		setEnabled: (v) => (enabled = v),
		getPlaying: () => playing,
		setPlaying: (v) => (playing = v),
		getSpeed: () => speed,
		setSpeed: (v) => (speed = v),
		setRange: (v) => (range = v),
		setError: (v) => (error = v),
		isDataAvailable: () => hasData
	});

	onMount(async () => {
		await ctrl.fetchRange();
	});

	onDestroy(() => {
		ctrl.stopPlayback();
	});
</script>

<div class="timeline-scrubber" class:expanded class:enabled>
	<TimelineScrubberBar
		{expanded}
		{enabled}
		{playing}
		{speed}
		{hasData}
		earliestLabel={formatTimelineCompact(earliestDate)}
		latestLabel={formatTimelineCompact(latestDate)}
		{currentTimestamp}
		{progressPercent}
		{position}
		onToggleExpanded={() => (expanded = !expanded)}
		onToggleEnabled={ctrl.toggleEnabled}
		onTogglePlayback={ctrl.togglePlayback}
		onCycleSpeed={ctrl.cycleSpeed}
		onSliderInput={ctrl.handleSliderInput}
	/>

	{#if expanded && hasData}
		<TimelineScrubberDetail
			startLabel={rangeStartLabel}
			endLabel={rangeEndLabel}
			connectionCount={range?.connection_count ?? 0}
			{progressPercent}
			{enabled}
			onResetStart={ctrl.resetToStart}
			onResetEnd={ctrl.resetToEnd}
			onRefresh={ctrl.fetchRange}
		/>
	{/if}
</div>

<style>
	.timeline-scrubber {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		z-index: 20;
		background: var(--gm-bg-secondary);
		border-top: 1px solid var(--gm-border);
		transition: all 0.2s ease;
		font-size: 11px;
	}

	.timeline-scrubber:not(.expanded) {
		max-height: 40px;
	}
</style>

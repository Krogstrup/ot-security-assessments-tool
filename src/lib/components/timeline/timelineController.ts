import { timelineEnabled, timelinePlaying, timelinePosition, timelineRange } from '$lib/stores/timeline';
import { getTimelineRange } from '$lib/api';
import { nextTimelinePosition, nextTimelineSpeed, startTimelineInterval, type TimelineSpeed } from './timelinePlayback';
import type { TimelineRange } from '$lib/types/analysis';

export interface TimelineControllerRefs {
	getPosition(): number;
	setPosition(v: number): void;
	getEnabled(): boolean;
	setEnabled(v: boolean): void;
	getPlaying(): boolean;
	setPlaying(v: boolean): void;
	getSpeed(): TimelineSpeed;
	setSpeed(v: TimelineSpeed): void;
	setRange(v: TimelineRange | null): void;
	setError(v: string | null): void;
	isDataAvailable(): boolean;
}

export function createTimelineController(refs: TimelineControllerRefs) {
	let playbackInterval: ReturnType<typeof setInterval> | null = null;

	function stopPlayback() {
		refs.setPlaying(false);
		timelinePlaying.set(false);
		if (playbackInterval !== null) {
			clearInterval(playbackInterval);
			playbackInterval = null;
		}
	}

	function startPlayback() {
		if (!refs.isDataAvailable() || !refs.getEnabled()) return;
		refs.setPlaying(true);
		timelinePlaying.set(true);

		playbackInterval = startTimelineInterval(() => {
			const newPos = nextTimelinePosition(refs.getPosition(), refs.getSpeed());
			refs.setPosition(newPos);
			timelinePosition.set(newPos);
			if (newPos >= 1.0) stopPlayback();
		});
	}

	function togglePlayback() {
		if (refs.getPlaying()) {
			stopPlayback();
		} else {
			if (refs.getPosition() >= 1.0) {
				refs.setPosition(0.0);
				timelinePosition.set(0.0);
			}
			startPlayback();
		}
	}

	function toggleEnabled() {
		const next = !refs.getEnabled();
		refs.setEnabled(next);
		timelineEnabled.set(next);
		if (!next) {
			stopPlayback();
			refs.setPosition(1.0);
			timelinePosition.set(1.0);
		}
	}

	function cycleSpeed() {
		const next = nextTimelineSpeed(refs.getSpeed());
		refs.setSpeed(next);
		if (refs.getPlaying()) {
			stopPlayback();
			startPlayback();
		}
	}

	function handleSliderInput(event: Event) {
		const target = event.target as HTMLInputElement;
		const newPos = parseFloat(target.value);
		refs.setPosition(newPos);
		timelinePosition.set(newPos);
	}

	function resetToEnd() {
		stopPlayback();
		refs.setPosition(1.0);
		timelinePosition.set(1.0);
	}

	function resetToStart() {
		stopPlayback();
		refs.setPosition(0.0);
		timelinePosition.set(0.0);
	}

	async function fetchRange() {
		try {
			const result = await getTimelineRange();
			refs.setRange(result);
			timelineRange.set(result);
			refs.setError(null);
		} catch (e) {
			refs.setError(String(e));
			refs.setRange(null);
			timelineRange.set(null);
		}
	}

	return {
		startPlayback,
		stopPlayback,
		togglePlayback,
		toggleEnabled,
		cycleSpeed,
		handleSliderInput,
		resetToStart,
		resetToEnd,
		fetchRange
	};
}

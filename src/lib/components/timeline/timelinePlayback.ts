export type TimelineSpeed = 1 | 2 | 5;

const PLAYBACK_TICK_MS = 50;
const PLAYBACK_BASE_DURATION_MS = 30000;

export function nextTimelineSpeed(speed: TimelineSpeed): TimelineSpeed {
	if (speed === 1) return 2;
	if (speed === 2) return 5;
	return 1;
}

export function nextTimelinePosition(current: number, speed: TimelineSpeed): number {
	const increment = (PLAYBACK_TICK_MS / PLAYBACK_BASE_DURATION_MS) * speed;
	return Math.min(1.0, current + increment);
}

export function startTimelineInterval(tick: () => void): ReturnType<typeof setInterval> {
	return setInterval(tick, PLAYBACK_TICK_MS);
}

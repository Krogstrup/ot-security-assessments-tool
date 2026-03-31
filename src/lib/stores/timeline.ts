import { writable } from 'svelte/store';
import type { TimelineRange } from '$lib/types/analysis';

/** Timeline range from loaded data */
export const timelineRange = writable<TimelineRange | null>(null);

/** Current timeline position as a fraction (0.0 = earliest, 1.0 = latest) */
export const timelinePosition = writable<number>(1.0);

/** Whether timeline playback is active */
export const timelinePlaying = writable<boolean>(false);

/** Whether the timeline scrubber is enabled (filtering active) */
export const timelineEnabled = writable<boolean>(false);

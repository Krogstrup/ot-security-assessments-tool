import type { CaptureStatus } from '$lib/types/capture';
import type { NetworkInterface } from '$lib/types/network';
import { writable } from 'svelte/store';
import type { CaptureStatsEvent } from '$lib/types';

/** Available network interfaces */
export const interfaces = writable<NetworkInterface[]>([]);

/** Current capture status */
export const captureStatus = writable<CaptureStatus>('idle');

/** Live capture statistics */
export const captureStats = writable<CaptureStatsEvent>({
	packets_captured: 0,
	packets_per_second: 0,
	bytes_captured: 0,
	active_connections: 0,
	asset_count: 0,
	elapsed_seconds: 0
});

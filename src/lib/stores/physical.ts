import { writable } from 'svelte/store';
import type { PhysicalTopology } from '$lib/types';

/** Physical topology from Cisco config/CAM/CDP/ARP imports */
export const physicalTopology = writable<PhysicalTopology>({
	switches: [],
	links: [],
	device_locations: {}
});

/** Currently selected device IP in physical view (for cross-reference) */
export const physicalHighlightIp = writable<string | null>(null);

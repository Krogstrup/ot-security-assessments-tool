import { writable } from 'svelte/store';
import type { SessionInfo } from '$lib/types';

/** All saved sessions */
export const sessions = writable<SessionInfo[]>([]);

/** Currently loaded session info (null if no session loaded) */
export const currentSession = writable<SessionInfo | null>(null);

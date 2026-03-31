import type { ThemeMode } from '$lib/types/analysis';
import { writable } from 'svelte/store';
import type { Project } from '$lib/types';

/** Current theme mode */
export const themeMode = writable<ThemeMode>('dark');

/** Currently active view/tab */
export type ViewTab =
	| 'projects'
	| 'topology'
	| 'physical'
	| 'inventory'
	| 'capture'
	| 'signatures'
	| 'protocol_stats'
	| 'export'
	| 'analysis'
	| 'settings'
	| 'comm_patterns'
	| 'segmentation';

const defaultTab: ViewTab =
	typeof window !== 'undefined' && !('__TAURI_INTERNALS__' in window) ? 'topology' : 'projects';

export const activeTab = writable<ViewTab>(defaultTab);

/** Currently active project (null if no project selected) */
export const activeProject = writable<Project | null>(null);

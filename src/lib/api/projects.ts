/**
 * Project management: CRUD + active project.
 */

import type { ProjectSummary } from '$lib/types/analysis';
import { invokeCompat } from './core';
import type { Project } from '$lib/types';

export async function createProject(
	name: string,
	clientName?: string,
	siteName?: string,
	assessorName?: string,
	engagementStart?: string,
	engagementEnd?: string,
	notes?: string
): Promise<Project> {
	return invokeCompat<Project>('create_project', {
		name,
		clientName: clientName ?? null,
		siteName: siteName ?? null,
		assessorName: assessorName ?? null,
		engagementStart: engagementStart ?? null,
		engagementEnd: engagementEnd ?? null,
		notes: notes ?? null
	});
}

export async function listProjects(): Promise<ProjectSummary[]> {
	return invokeCompat<ProjectSummary[]>('list_projects');
}

export async function getProject(id: number): Promise<Project> {
	return invokeCompat<Project>('get_project', { id });
}

export async function updateProject(
	id: number,
	name: string,
	clientName?: string,
	siteName?: string,
	assessorName?: string,
	engagementStart?: string,
	engagementEnd?: string,
	notes?: string
): Promise<Project> {
	return invokeCompat<Project>('update_project', {
		id,
		name,
		clientName: clientName ?? null,
		siteName: siteName ?? null,
		assessorName: assessorName ?? null,
		engagementStart: engagementStart ?? null,
		engagementEnd: engagementEnd ?? null,
		notes: notes ?? null
	});
}

export async function deleteProject(id: number): Promise<void> {
	return invokeCompat('delete_project', { id });
}

export async function setActiveProject(id: number): Promise<Project> {
	return invokeCompat<Project>('set_active_project', { id });
}

export async function clearActiveProject(): Promise<void> {
	return invokeCompat('clear_active_project');
}

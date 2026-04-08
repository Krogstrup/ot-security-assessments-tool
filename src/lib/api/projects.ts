/**
 * Project management: CRUD + active project.
 *
 * Tauri runtime  → invoke() (desktop)
 * Web runtime    → /api/v1/projects/* with Zod runtime validation
 */

import { invoke } from '@tauri-apps/api/core';
import { z } from 'zod';
import { projectSchema, projectSummarySchema } from '$lib/schemas';
import { httpValidated, isTauriRuntime } from './core';
import type { ProjectSummary } from '$lib/types/analysis';
import type { Project } from '$lib/types';

const json = (body: unknown): RequestInit => ({
	method: 'POST',
	headers: { 'Content-Type': 'application/json' },
	body: JSON.stringify(body)
});

export async function createProject(
	name: string,
	clientName?: string,
	siteName?: string,
	assessorName?: string,
	engagementStart?: string,
	engagementEnd?: string,
	notes?: string
): Promise<Project> {
	if (isTauriRuntime()) {
		return invoke<Project>('create_project', {
			name,
			clientName: clientName ?? null,
			siteName: siteName ?? null,
			assessorName: assessorName ?? null,
			engagementStart: engagementStart ?? null,
			engagementEnd: engagementEnd ?? null,
			notes: notes ?? null
		});
	}
	return httpValidated(projectSchema, '/api/v1/projects', {
		...json({ name, clientName, siteName, assessorName, engagementStart, engagementEnd, notes }),
		method: 'POST'
	});
}

export async function listProjects(): Promise<ProjectSummary[]> {
	if (isTauriRuntime()) {
		return invoke<ProjectSummary[]>('list_projects');
	}
	return httpValidated(z.array(projectSummarySchema), '/api/v1/projects');
}

export async function getProject(id: number): Promise<Project> {
	if (isTauriRuntime()) {
		return invoke<Project>('get_project', { id });
	}
	return httpValidated(projectSchema, `/api/v1/projects/${id}`);
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
	if (isTauriRuntime()) {
		return invoke<Project>('update_project', {
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
	return httpValidated(projectSchema, `/api/v1/projects/${id}`, {
		...json({ name, clientName, siteName, assessorName, engagementStart, engagementEnd, notes }),
		method: 'PUT'
	});
}

export async function deleteProject(id: number): Promise<void> {
	if (isTauriRuntime()) {
		return invoke('delete_project', { id });
	}
	await httpValidated(z.unknown(), `/api/v1/projects/${id}`, { method: 'DELETE' });
}

export async function setActiveProject(id: number): Promise<Project> {
	if (isTauriRuntime()) {
		return invoke<Project>('set_active_project', { id });
	}
	return httpValidated(projectSchema, '/api/v1/projects/active', json({ id }));
}

export async function clearActiveProject(): Promise<void> {
	if (isTauriRuntime()) {
		return invoke('clear_active_project');
	}
	await httpValidated(z.unknown(), '/api/v1/projects/active', { method: 'DELETE' });
}

/**
 * Project management: CRUD + active project.
 *
 * Web runtime → /api/v1/projects/* with Zod runtime validation
 */

import { z } from 'zod';
import { projectSchema, projectSummarySchema } from '$lib/schemas';
import { httpValidated } from './core';
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
	return httpValidated(projectSchema, '/api/v1/projects', {
		...json({ name, clientName, siteName, assessorName, engagementStart, engagementEnd, notes }),
		method: 'POST'
	});
}

export async function listProjects(): Promise<ProjectSummary[]> {
	return httpValidated(z.array(projectSummarySchema), '/api/v1/projects');
}

export async function getProject(id: number): Promise<Project> {
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
	return httpValidated(projectSchema, `/api/v1/projects/${id}`, {
		...json({ name, clientName, siteName, assessorName, engagementStart, engagementEnd, notes }),
		method: 'PUT'
	});
}

export async function deleteProject(id: number): Promise<void> {
	await httpValidated(z.unknown(), `/api/v1/projects/${id}`, { method: 'DELETE' });
}

export async function setActiveProject(id: number): Promise<Project> {
	return httpValidated(projectSchema, '/api/v1/projects/active', json({ id }));
}

export async function clearActiveProject(): Promise<void> {
	await httpValidated(z.unknown(), '/api/v1/projects/active', { method: 'DELETE' });
}

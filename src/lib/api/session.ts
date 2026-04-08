/**
 * Session management: save, load, list, delete, export, import.
 *
 * Web runtime → /api/v1/sessions/* with Zod runtime validation
 */

import { z } from 'zod';
import { sessionInfoSchema, baselineDiffSchema } from '$lib/schemas';
import { httpValidated } from './core';
import type { BaselineDiff } from '$lib/types/analysis';
import type { SessionInfo } from '$lib/types';

const json = (body: unknown): RequestInit => ({
	method: 'POST',
	headers: { 'Content-Type': 'application/json' },
	body: JSON.stringify(body)
});

export async function saveSession(name: string, description?: string): Promise<SessionInfo> {
	return httpValidated(sessionInfoSchema, '/api/v1/sessions', json({ name, description }));
}

export async function loadSession(sessionId: string): Promise<SessionInfo> {
	return httpValidated(sessionInfoSchema, `/api/v1/sessions/${sessionId}/load`, { method: 'POST' });
}

export async function listSessions(): Promise<SessionInfo[]> {
	return httpValidated(z.array(sessionInfoSchema), '/api/v1/sessions');
}

export async function deleteSession(sessionId: string): Promise<void> {
	await httpValidated(z.unknown(), `/api/v1/sessions/${sessionId}`, { method: 'DELETE' });
}

export async function exportSessionArchive(sessionId: string, outputPath: string): Promise<string> {
	return httpValidated(z.string(), `/api/v1/sessions/${sessionId}/export`, json({ outputPath }));
}

export async function importSessionArchive(archivePath: string): Promise<SessionInfo> {
	return httpValidated(sessionInfoSchema, '/api/v1/sessions/import', json({ archivePath }));
}

export async function compareSessions(baselineSessionId: string): Promise<BaselineDiff> {
	return httpValidated(baselineDiffSchema, '/api/v1/sessions/compare', json({ baselineSessionId }));
}

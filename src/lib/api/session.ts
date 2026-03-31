/**
 * Session management: save, load, list, delete, export, import.
 */

import { invokeCompat } from './core';
import type { SessionInfo, BaselineDiff } from '/types';

export async function saveSession(name: string, description?: string): Promise<SessionInfo> {
	return invokeCompat<SessionInfo>('save_session', { name, description: description ?? null });
}

export async function loadSession(sessionId: string): Promise<SessionInfo> {
	return invokeCompat<SessionInfo>('load_session', { sessionId });
}

export async function listSessions(): Promise<SessionInfo[]> {
	return invokeCompat<SessionInfo[]>('list_sessions');
}

export async function deleteSession(sessionId: string): Promise<void> {
	return invokeCompat('delete_session', { sessionId });
}

export async function exportSessionArchive(sessionId: string, outputPath: string): Promise<string> {
	return invokeCompat<string>('export_session_archive', { sessionId, outputPath });
}

export async function importSessionArchive(archivePath: string): Promise<SessionInfo> {
	return invokeCompat<SessionInfo>('import_session_archive', { archivePath });
}

export async function compareSessions(baselineSessionId: string): Promise<BaselineDiff> {
	return invokeCompat<BaselineDiff>('compare_sessions', { baselineSessionId });
}

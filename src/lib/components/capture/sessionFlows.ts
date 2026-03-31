import {
	deleteSession,
	exportSessionArchive,
	importSessionArchive,
	listSessions,
	loadSession,
	saveSession
} from '$lib/api';
import { currentSession, sessions } from '$lib/stores/session';
import type { SessionInfo } from '$lib/types/operations';
import { savePathDialog } from '$lib/utils/dialog';
import { defaultSessionArchiveName } from './captureViewFormatters';
import type { ImportPathPickerOptions } from './importPathPicker';

export interface SessionFeedback {
	message: string;
	type: 'success' | 'error';
}

export async function refreshSessionsStore() {
	try {
		sessions.set(await listSessions());
	} catch {
		// DB may not be available in browser/dev mode.
	}
}

export async function saveSessionFlow(name: string, description: string): Promise<SessionFeedback> {
	const info = await saveSession(name.trim(), description.trim() || undefined);
	currentSession.set(info);
	await refreshSessionsStore();
	return {
		message: `Session "${info.name}" saved (${info.asset_count} assets, ${info.connection_count} connections)`,
		type: 'success'
	};
}

export async function loadSessionFlow(id: string): Promise<{ info: SessionInfo; feedback: SessionFeedback }> {
	const info = await loadSession(id);
	currentSession.set(info);
	return {
		info,
		feedback: {
			message: `Session "${info.name}" loaded`,
			type: 'success'
		}
	};
}

export async function deleteSessionFlow(id: string): Promise<SessionFeedback> {
	await deleteSession(id);
	if (getCurrentSessionId() === id) {
		currentSession.set(null);
	}
	await refreshSessionsStore();
	return {
		message: 'Session deleted',
		type: 'success'
	};
}

export async function exportSessionFlow(session: SessionInfo, path: string): Promise<SessionFeedback> {
	await exportSessionArchive(session.id, path);
	return {
		message: `Exported to ${path}`,
		type: 'success'
	};
}

export async function importSessionArchiveFlow(path: string): Promise<{ info: SessionInfo; feedback: SessionFeedback }> {
	const info = await importSessionArchive(path);
	currentSession.set(info);
	await refreshSessionsStore();
	return {
		info,
		feedback: {
			message: `Imported "${info.name}" (${info.asset_count} assets)`,
			type: 'success'
		}
	};
}

export interface SessionHandlerHooks {
	setMessage: (m: string, type: 'success' | 'error') => void;
	setShowSaveForm?: (v: boolean) => void;
	clearSaveForm?: () => void;
	setConfirmDeleteId?: (id: string | null) => void;
}

export async function handleSaveSession(
	name: string,
	desc: string,
	hooks: SessionHandlerHooks
): Promise<void> {
	if (!name.trim()) return;
	try {
		const feedback = await saveSessionFlow(name, desc);
		hooks.setMessage(feedback.message, feedback.type);
		hooks.setShowSaveForm?.(false);
		hooks.clearSaveForm?.();
	} catch (err) {
		hooks.setMessage(`Save failed: ${err}`, 'error');
	}
}

export async function handleLoadSession(
	id: string,
	onDataChanged: () => Promise<void>,
	hooks: SessionHandlerHooks
): Promise<void> {
	try {
		const { feedback } = await loadSessionFlow(id);
		await onDataChanged();
		hooks.setMessage(feedback.message, feedback.type);
	} catch (err) {
		hooks.setMessage(`Load failed: ${err}`, 'error');
	}
}

export async function handleDeleteSession(
	id: string,
	hooks: SessionHandlerHooks
): Promise<void> {
	try {
		const feedback = await deleteSessionFlow(id);
		hooks.setConfirmDeleteId?.(null);
		hooks.setMessage(feedback.message, feedback.type);
	} catch (err) {
		hooks.setMessage(`Delete failed: ${err}`, 'error');
	}
}

export async function handleExportSession(
	session: SessionInfo,
	hooks: SessionHandlerHooks
): Promise<void> {
	try {
		const path = await savePathDialog({
			title: 'Export Session Archive',
			defaultPath: defaultSessionArchiveName(session),
			filters: [
				{ name: 'Kusanagi Kajiki Archive', extensions: ['kkj'] },
				{ name: 'All Files', extensions: ['*'] }
			]
		});
		if (!path) return;
		const feedback = await exportSessionFlow(session, path);
		hooks.setMessage(feedback.message, feedback.type);
	} catch (err) {
		hooks.setMessage(`Export failed: ${err}`, 'error');
	}
}

export async function handleImportArchive(
	pickPaths: (opts: ImportPathPickerOptions) => Promise<string[]>,
	onDataChanged: () => Promise<void>,
	hooks: SessionHandlerHooks
): Promise<void> {
	try {
		const paths = await pickPaths({
			kind: 'session_archive',
			serverTitle: 'Import Session Archive',
			dialogTitle: 'Import Session Archive',
			multiple: false,
			filters: [
				{ name: 'Kusanagi Kajiki Archive', extensions: ['kkj'] },
				{ name: 'All Files', extensions: ['*'] }
			]
		});
		if (paths.length === 0) return;
		const { feedback } = await importSessionArchiveFlow(paths[0]);
		await onDataChanged();
		hooks.setMessage(feedback.message, feedback.type);
	} catch (err) {
		hooks.setMessage(`Import failed: ${err}`, 'error');
	}
}

function getCurrentSessionId() {
	let sessionId: string | null = null;
	const unsubscribe = currentSession.subscribe((session) => {
		sessionId = session?.id ?? null;
	});
	unsubscribe();
	return sessionId;
}

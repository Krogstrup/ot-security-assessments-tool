import { onCaptureError, onCaptureStats, onImportProgress } from '$lib/api';
import type { ImportProgressEvent } from '$lib/api/capture';
import type { CaptureStatsEvent } from '$lib/types/capture';

export interface CaptureListenerHandles {
	unlistenStats: (() => void) | null;
	unlistenError: (() => void) | null;
	unlistenProgress: (() => void) | null;
}

export interface CaptureRuntimeListenerHandles {
	unlistenStats: (() => void) | null;
	unlistenError: (() => void) | null;
}

export async function setupCaptureListeners(handlers: {
	onStats: (stats: CaptureStatsEvent) => void;
	onError: (error: string) => void;
	onProgress: (progress: ImportProgressEvent) => void;
}): Promise<CaptureListenerHandles> {
	const [unlistenStats, unlistenError, unlistenProgress] = await Promise.all([
		onCaptureStats(handlers.onStats),
		onCaptureError(handlers.onError),
		onImportProgress(handlers.onProgress)
	]);

	return {
		unlistenStats,
		unlistenError,
		unlistenProgress
	};
}

export async function setupCaptureRuntimeListeners(handlers: {
	onStats: (stats: CaptureStatsEvent) => void;
	onError: (error: string) => void;
}): Promise<CaptureRuntimeListenerHandles> {
	const [unlistenStats, unlistenError] = await Promise.all([
		onCaptureStats(handlers.onStats),
		onCaptureError(handlers.onError)
	]);

	return {
		unlistenStats,
		unlistenError
	};
}

export async function setupImportProgressListener(
	onProgress: (progress: ImportProgressEvent) => void
): Promise<() => void> {
	return onImportProgress(onProgress);
}

export function cleanupCaptureListeners(handles: CaptureListenerHandles) {
	handles.unlistenStats?.();
	handles.unlistenError?.();
	handles.unlistenProgress?.();
}

export function cleanupCaptureRuntimeListeners(handles: CaptureRuntimeListenerHandles) {
	handles.unlistenStats?.();
	handles.unlistenError?.();
}

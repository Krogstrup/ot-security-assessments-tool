/**
 * PCAP import and live capture commands.
 */

import type { CaptureStatusInfo, StopCaptureResult, PacketEvent, CaptureStatsEvent } from '$lib/types/capture';
import { invokeCompat, httpJson, isTauriRuntime } from './core';
import { listen } from '@tauri-apps/api/event';
import type { ImportResult } from '$lib/types/capture';
import type { HeadlessImportKind, HeadlessImportPcapList } from './system';
import { getProtocolStats as getProtocolStatsFromConnections } from './connections';

export async function importPcap(paths: string[]): Promise<ImportResult> {
	if (!isTauriRuntime()) {
		return httpJson<ImportResult>('/api/capture/import-pcap', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ paths })
		});
	}
	return invokeCompat<ImportResult>('import_pcap', { paths });
}

export async function cancelImport(): Promise<void> {
	return invokeCompat('cancel_import');
}

export interface ImportProgressEvent {
	current_file: string;
	file_index: number;
	file_count: number;
	packets_processed: number;
	bytes_processed: number;
	file_size: number;
	progress_percent: number;
	elapsed_secs: number;
}

export async function onImportProgress(
	callback: (progress: ImportProgressEvent) => void
): Promise<() => void> {
	if (!isTauriRuntime()) {
		return () => {};
	}
	const unlisten = await listen<ImportProgressEvent>('import_progress', (event) => {
		callback(event.payload);
	});
	return unlisten;
}

export async function startCapture(interfaceName: string, bpfFilter?: string): Promise<void> {
	return invokeCompat('start_capture', { interfaceName, bpfFilter: bpfFilter ?? null });
}

export async function stopCapture(savePath?: string): Promise<StopCaptureResult> {
	return invokeCompat<StopCaptureResult>('stop_capture', { savePath: savePath ?? null });
}

export async function getProtocolStats() {
	return getProtocolStatsFromConnections();
}

export async function pauseCapture(): Promise<void> {
	return invokeCompat('pause_capture');
}

export async function resumeCapture(): Promise<void> {
	return invokeCompat('resume_capture');
}

export async function getCaptureStatus(): Promise<CaptureStatusInfo> {
	return invokeCompat<CaptureStatusInfo>('get_capture_status');
}

export async function onPacketEvent(callback: (event: PacketEvent) => void) {
	if (!isTauriRuntime()) {
		return () => {};
	}
	return listen<PacketEvent>('packet-event', (event) => callback(event.payload));
}

export async function onCaptureStats(callback: (stats: CaptureStatsEvent) => void) {
	if (!isTauriRuntime()) {
		return () => {};
	}
	return listen<CaptureStatsEvent>('capture-stats', (event) => callback(event.payload));
}

export async function onCaptureError(callback: (error: string) => void) {
	if (!isTauriRuntime()) {
		return () => {};
	}
	return listen<string>('capture-error', (event) => callback(event.payload));
}

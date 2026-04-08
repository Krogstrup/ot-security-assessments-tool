/**
 * PCAP import and live capture commands.
 *
 * Web runtime → HTTP API + SSE event stream (/api/v1/events)
 */

import type { CaptureStatusInfo, StopCaptureResult, PacketEvent, CaptureStatsEvent } from '$lib/types/capture';
import { httpJson } from './core';
import type { ImportResult } from '$lib/types/capture';
import { getProtocolStats as getProtocolStatsFromConnections } from './connections';

export async function importPcap(paths: string[]): Promise<ImportResult> {
	return httpJson<ImportResult>('/api/capture/import-pcap', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ paths })
	});
}

export async function cancelImport(): Promise<void> {
	await httpJson('/api/v1/capture/cancel', { method: 'POST' });
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
	const source = new EventSource('/api/v1/events');
	source.addEventListener('import_progress', (e) => {
		try {
			callback(JSON.parse((e as MessageEvent).data) as ImportProgressEvent);
		} catch {
			// ignore malformed events
		}
	});
	return () => source.close();
}

export async function startCapture(interfaceName: string, bpfFilter?: string): Promise<void> {
	await httpJson('/api/v1/capture/start', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ interfaceName, bpfFilter: bpfFilter ?? null })
	});
}

export async function stopCapture(savePath?: string): Promise<StopCaptureResult> {
	return httpJson<StopCaptureResult>('/api/v1/capture/stop', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ savePath: savePath ?? null })
	});
}

export async function getProtocolStats() {
	return getProtocolStatsFromConnections();
}

export async function pauseCapture(): Promise<void> {
	await httpJson('/api/v1/capture/pause', { method: 'POST' });
}

export async function resumeCapture(): Promise<void> {
	await httpJson('/api/v1/capture/resume', { method: 'POST' });
}

export async function getCaptureStatus(): Promise<CaptureStatusInfo> {
	return httpJson<CaptureStatusInfo>('/api/v1/capture/status');
}

export async function onPacketEvent(_callback: (event: PacketEvent) => void): Promise<() => void> {
	// No SSE equivalent for per-packet events in web runtime; use capture stats instead.
	return () => {};
}

export async function onCaptureStats(callback: (stats: CaptureStatsEvent) => void): Promise<() => void> {
	const source = new EventSource('/api/v1/events');
	source.addEventListener('capture_stats', (e) => {
		try {
			callback(JSON.parse((e as MessageEvent).data) as CaptureStatsEvent);
		} catch {
			// ignore malformed events
		}
	});
	return () => source.close();
}

export async function onCaptureError(_callback: (error: string) => void): Promise<() => void> {
	// No SSE event for capture errors in web runtime.
	return () => {};
}

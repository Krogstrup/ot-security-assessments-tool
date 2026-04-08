/**
 * Wireshark integration: detection, opening, frame export.
 */

import type { FrameRow } from '$lib/types/operations';
import { httpJson } from './core';
import type { WiresharkInfo } from '$lib/types';

export async function detectWireshark(): Promise<WiresharkInfo> {
	return httpJson<WiresharkInfo>('/api/v1/wireshark/info');
}

export async function openInWireshark(connectionId: string): Promise<void> {
	await httpJson('/api/v1/wireshark/open-connection', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ connectionId })
	});
}

export async function openWiresharkForNode(ipAddress: string): Promise<void> {
	await httpJson('/api/v1/wireshark/open-node', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ ipAddress })
	});
}

export async function getConnectionFrames(connectionId: string): Promise<FrameRow[]> {
	return httpJson<FrameRow[]>(`/api/v1/wireshark/frames/${encodeURIComponent(connectionId)}`);
}

export async function exportFramesCsv(connectionId: string): Promise<string> {
	return httpJson<string>(`/api/v1/wireshark/frames/${encodeURIComponent(connectionId)}/csv`);
}

export async function saveFramesCsv(connectionId: string, outputPath: string): Promise<void> {
	await httpJson(`/api/v1/wireshark/frames/${encodeURIComponent(connectionId)}/csv`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ outputPath })
	});
}

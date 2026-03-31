/**
 * Wireshark integration: detection, opening, frame export.
 */

import { invokeCompat } from './core';
import type { WiresharkInfo, FrameRow } from '/types';

export async function detectWireshark(): Promise<WiresharkInfo> {
	return invokeCompat<WiresharkInfo>('detect_wireshark');
}

export async function openInWireshark(connectionId: string): Promise<void> {
	return invokeCompat('open_in_wireshark', { connectionId });
}

export async function openWiresharkForNode(ipAddress: string): Promise<void> {
	return invokeCompat('open_wireshark_for_node', { ipAddress });
}

export async function getConnectionFrames(connectionId: string): Promise<FrameRow[]> {
	return invokeCompat<FrameRow[]>('get_connection_frames', { connectionId });
}

export async function exportFramesCsv(connectionId: string): Promise<string> {
	return invokeCompat<string>('export_frames_csv', { connectionId });
}

export async function saveFramesCsv(connectionId: string, outputPath: string): Promise<void> {
	return invokeCompat('save_frames_csv', { connectionId, outputPath });
}

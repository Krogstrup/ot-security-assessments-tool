/**
 * System commands: app info, interfaces, settings, plugins, timeline.
 */

import type { UserSettings, TimelineRange, PluginManifest } from '$lib/types/analysis';
import { httpJson } from './core';
import type { NetworkInterface } from '$lib/types';

export async function listInterfaces(): Promise<NetworkInterface[]> {
	return httpJson<NetworkInterface[]>('/api/system/interfaces');
}

export async function getAppInfo(): Promise<{ version: string; rust_version: string }> {
	return httpJson<{ version: string; rust_version: string }>('/api/system/app-info');
}

export async function getSettings(): Promise<UserSettings> {
	return httpJson<UserSettings>('/api/v1/system/settings');
}

export async function saveSettings(settings: UserSettings): Promise<void> {
	await httpJson('/api/v1/system/settings', {
		method: 'PUT',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ settings })
	});
}

// Explicit aliases used during T2 migration.
export async function getUserSettings(): Promise<UserSettings> {
	return getSettings();
}

export async function saveUserSettings(settings: UserSettings): Promise<void> {
	return saveSettings(settings);
}

export async function getTimelineRange(): Promise<TimelineRange> {
	return httpJson<TimelineRange>('/api/v1/data/timeline-range');
}

export async function listPlugins(): Promise<PluginManifest[]> {
	return httpJson<PluginManifest[]>('/api/v1/system/plugins');
}

export interface HeadlessImportPcapFile {
	name: string;
	path: string;
	size_bytes: number;
}

export type HeadlessImportKind =
	| 'pcap'
	| 'physical_config'
	| 'physical_mac'
	| 'physical_neighbor'
	| 'physical_arp'
	| 'zeek'
	| 'suricata'
	| 'nmap'
	| 'masscan'
	| 'wazuh'
	| 'sinema'
	| 'tia'
	| 'session_archive';

export interface HeadlessImportPcapList {
	kind?: HeadlessImportKind;
	base_dir: string;
	files: HeadlessImportPcapFile[];
	list_limit: number;
	truncated: boolean;
}

export async function listHeadlessImportFiles(kind: HeadlessImportKind): Promise<HeadlessImportPcapList> {
	return httpJson<HeadlessImportPcapList>(`/api/system/import-files/${encodeURIComponent(kind)}`);
}

export async function listHeadlessImportPcapFiles(): Promise<HeadlessImportPcapList> {
	return listHeadlessImportFiles('pcap');
}

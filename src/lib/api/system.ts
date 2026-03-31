/**
 * System commands: app info, interfaces, settings, plugins, timeline.
 */

import type { UserSettings, TimelineRange, PluginManifest } from '$lib/types/analysis';
import { invokeCompat, httpJson, isTauriRuntime } from './core';
import type { NetworkInterface } from '$lib/types';

export async function listInterfaces(): Promise<NetworkInterface[]> {
	if (!isTauriRuntime()) {
		return httpJson<NetworkInterface[]>('/api/system/interfaces');
	}
	return invokeCompat<NetworkInterface[]>('list_interfaces');
}

export async function getAppInfo(): Promise<{ version: string; rust_version: string }> {
	if (!isTauriRuntime()) {
		return httpJson<{ version: string; rust_version: string }>('/api/system/app-info');
	}
	return invokeCompat('get_app_info');
}

export async function getSettings(): Promise<UserSettings> {
	return invokeCompat<UserSettings>('get_settings');
}

export async function saveSettings(settings: UserSettings): Promise<void> {
	return invokeCompat('save_settings', { settings });
}

// Explicit aliases used during T2 migration.
export async function getUserSettings(): Promise<UserSettings> {
	return getSettings();
}

export async function saveUserSettings(settings: UserSettings): Promise<void> {
	return saveSettings(settings);
}

export async function getTimelineRange(): Promise<TimelineRange> {
	return invokeCompat<TimelineRange>('get_timeline_range');
}

export async function listPlugins(): Promise<PluginManifest[]> {
	return invokeCompat<PluginManifest[]>('list_plugins');
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
	if (isTauriRuntime()) {
		throw new Error('Server-side import file list is only available in web/headless mode');
	}
	return httpJson<HeadlessImportPcapList>(`/api/system/import-files/${encodeURIComponent(kind)}`);
}

export async function listHeadlessImportPcapFiles(): Promise<HeadlessImportPcapList> {
	return listHeadlessImportFiles('pcap');
}

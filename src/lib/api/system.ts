/**
 * System commands: app info, interfaces, settings, plugins, timeline.
 */

import type { UserSettings, TimelineRange, PluginManifest } from '$lib/types/analysis';
import { z } from 'zod';
import { timelineRangeSchema } from '$lib/schemas';
import { httpValidated } from './core';
import type { NetworkInterface } from '$lib/types';

export async function listInterfaces(): Promise<NetworkInterface[]> {
	return httpValidated(z.unknown(), '/api/system/interfaces') as Promise<NetworkInterface[]>;
}

export async function getAppInfo(): Promise<{ version: string; rust_version: string }> {
	return httpValidated(
		z.object({ version: z.string(), rust_version: z.string() }),
		'/api/system/app-info'
	);
}

export async function getSettings(): Promise<UserSettings> {
	return httpValidated(z.object({ theme: z.enum(['dark', 'light', 'system']) }), '/api/v1/system/settings');
}

export async function saveSettings(settings: UserSettings): Promise<void> {
	await httpValidated(z.unknown(), '/api/v1/system/settings', {
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
	return httpValidated(timelineRangeSchema, '/api/v1/data/timeline-range');
}

export async function listPlugins(): Promise<PluginManifest[]> {
	return httpValidated(z.unknown(), '/api/v1/system/plugins') as Promise<PluginManifest[]>;
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
	return httpValidated(
		z.object({
			kind: z.string().optional(),
			base_dir: z.string(),
			files: z.array(
				z.object({
					name: z.string(),
					path: z.string(),
					size_bytes: z.number()
				})
			),
			list_limit: z.number(),
			truncated: z.boolean()
		}),
		`/api/system/import-files/${encodeURIComponent(kind)}`
	) as Promise<HeadlessImportPcapList>;
}

export async function listHeadlessImportPcapFiles(): Promise<HeadlessImportPcapList> {
	return listHeadlessImportFiles('pcap');
}

/**
 * External tool imports: Zeek, Suricata, Nmap, Masscan, Wazuh, SINEMA, TIA.
 */

import type { IngestImportResult } from '$lib/types/operations';
import { httpJson } from './core';
import type { DeviceZeekEvents } from '$lib/types';

export async function importZeekLogs(paths: string[]): Promise<IngestImportResult> {
	return httpJson<IngestImportResult>('/api/v1/ingest/zeek', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ paths })
	});
}

export async function importSuricataEve(path: string): Promise<IngestImportResult> {
	return httpJson<IngestImportResult>('/api/v1/ingest/suricata', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ path })
	});
}

export async function importNmapXml(path: string): Promise<IngestImportResult> {
	return httpJson<IngestImportResult>('/api/v1/ingest/nmap', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ path })
	});
}

export async function importMasscanJson(path: string): Promise<IngestImportResult> {
	return httpJson<IngestImportResult>('/api/v1/ingest/masscan', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ path })
	});
}

export async function importWazuhAlerts(path: string): Promise<IngestImportResult> {
	return httpJson<IngestImportResult>('/api/v1/ingest/wazuh', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ path })
	});
}

export async function importSinemaCsv(path: string): Promise<IngestImportResult> {
	return httpJson<IngestImportResult>('/api/v1/ingest/sinema', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ path })
	});
}

export async function importTiaXml(path: string): Promise<IngestImportResult> {
	return httpJson<IngestImportResult>('/api/v1/ingest/tia', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ path })
	});
}

export async function getDeviceZeekEvents(deviceIp: string): Promise<DeviceZeekEvents> {
	return httpJson<DeviceZeekEvents>(
		`/api/v1/ingest/zeek-device-events/${encodeURIComponent(deviceIp)}`
	);
}

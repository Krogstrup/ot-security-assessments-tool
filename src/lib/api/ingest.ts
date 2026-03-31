/**
 * External tool imports: Zeek, Suricata, Nmap, Masscan, Wazuh, SINEMA, TIA.
 */

import { invokeCompat } from './core';
import type { IngestImportResult } from '/types';

export async function importZeekLogs(paths: string[]): Promise<IngestImportResult> {
	return invokeCompat<IngestImportResult>('import_zeek_logs', { paths });
}

export async function importSuricataEve(path: string): Promise<IngestImportResult> {
	return invokeCompat<IngestImportResult>('import_suricata_eve', { path });
}

export async function importNmapXml(path: string): Promise<IngestImportResult> {
	return invokeCompat<IngestImportResult>('import_nmap_xml', { path });
}

export async function importMasscanJson(path: string): Promise<IngestImportResult> {
	return invokeCompat<IngestImportResult>('import_masscan_json', { path });
}

export async function importWazuhAlerts(path: string): Promise<IngestImportResult> {
	return invokeCompat<IngestImportResult>('import_wazuh_alerts', { path });
}

export async function importSinemaCsv(path: string): Promise<IngestImportResult> {
	return invokeCompat('import_sinema_csv', { path });
}

export async function importTiaXml(path: string): Promise<IngestImportResult> {
	return invokeCompat('import_tia_xml', { path });
}

import {
	importZeekLogs,
	importSuricataEve,
	importNmapXml,
	importMasscanJson,
	importWazuhAlerts,
	importSinemaCsv,
	importTiaXml
} from '$lib/api';
import type { IngestImportResult } from '$lib/types/operations';
import type { HeadlessImportKind } from '$lib/api/system';
import type { FileDialogFilter, ImportPathPickerOptions } from './importPathPicker';

export interface ExternalImportTask {
	kind: HeadlessImportKind;
	serverTitle: string;
	dialogTitle: string;
	multiple: boolean;
	filters: FileDialogFilter[];
	startMessage: (pathCount: number) => string;
	successMessage: (result: IngestImportResult) => string;
	errorPrefix: string;
	importFn: (input: string | string[]) => Promise<IngestImportResult>;
}

export interface ExternalImportHooks {
	setStatus: (s: 'idle' | 'importing' | 'done' | 'error') => void;
	setMessage: (m: string) => void;
	setLastResult: (r: IngestImportResult | null) => void;
}

export async function runExternalImport(
	task: ExternalImportTask,
	pickPaths: (opts: ImportPathPickerOptions) => Promise<string[]>,
	onDataChanged: () => Promise<void>,
	hooks: ExternalImportHooks
): Promise<void> {
	try {
		const paths = await pickPaths({
			kind: task.kind,
			serverTitle: task.serverTitle,
			dialogTitle: task.dialogTitle,
			multiple: task.multiple,
			filters: task.filters
		});
		if (paths.length === 0) return;

		hooks.setStatus('importing');
		hooks.setMessage(task.startMessage(paths.length));
		hooks.setLastResult(null);

		const payload = task.multiple ? paths : paths[0];
		const result = await task.importFn(payload);
		hooks.setLastResult(result);
		hooks.setStatus('done');
		hooks.setMessage(task.successMessage(result));

		await onDataChanged();
	} catch (err) {
		hooks.setStatus('error');
		hooks.setMessage(`${task.errorPrefix}: ${err}`);
	}
}

export const EXTERNAL_IMPORT_TASKS: Record<string, ExternalImportTask> = {
	zeek: {
		kind: 'zeek',
		serverTitle: 'Import Zeek Logs',
		dialogTitle: 'Import Zeek Logs',
		multiple: true,
		filters: [
			{ name: 'Zeek Logs', extensions: ['log'] },
			{ name: 'All Files', extensions: ['*'] }
		],
		startMessage: (count) => `Importing ${count} Zeek log file${count > 1 ? 's' : ''}...`,
		successMessage: (result) =>
			`Zeek: ${result.new_assets} new + ${result.updated_assets} updated assets, ${result.connection_count} connections (${result.duration_ms}ms)`,
		errorPrefix: 'Zeek import failed',
		importFn: (input) => importZeekLogs(input as string[])
	},
	suricata: {
		kind: 'suricata',
		serverTitle: 'Import Suricata eve.json',
		dialogTitle: 'Import Suricata eve.json',
		multiple: false,
		filters: [
			{ name: 'JSON Files', extensions: ['json'] },
			{ name: 'All Files', extensions: ['*'] }
		],
		startMessage: () => 'Importing Suricata eve.json...',
		successMessage: (result) =>
			`Suricata: ${result.new_assets} new + ${result.updated_assets} updated assets, ${result.connection_count} connections, ${result.alert_count} alerts (${result.duration_ms}ms)`,
		errorPrefix: 'Suricata import failed',
		importFn: (input) => importSuricataEve(input as string)
	},
	nmap: {
		kind: 'nmap',
		serverTitle: 'Import Nmap XML',
		dialogTitle: 'Import Nmap XML',
		multiple: false,
		filters: [
			{ name: 'XML Files', extensions: ['xml'] },
			{ name: 'All Files', extensions: ['*'] }
		],
		startMessage: () => 'Importing Nmap XML...',
		successMessage: (result) =>
			`Nmap: ${result.new_assets} new + ${result.updated_assets} updated assets (${result.duration_ms}ms) [ACTIVE SCAN]`,
		errorPrefix: 'Nmap import failed',
		importFn: (input) => importNmapXml(input as string)
	},
	masscan: {
		kind: 'masscan',
		serverTitle: 'Import Masscan JSON',
		dialogTitle: 'Import Masscan JSON',
		multiple: false,
		filters: [
			{ name: 'JSON Files', extensions: ['json'] },
			{ name: 'All Files', extensions: ['*'] }
		],
		startMessage: () => 'Importing Masscan JSON...',
		successMessage: (result) =>
			`Masscan: ${result.new_assets} new + ${result.updated_assets} updated assets (${result.duration_ms}ms) [ACTIVE SCAN]`,
		errorPrefix: 'Masscan import failed',
		importFn: (input) => importMasscanJson(input as string)
	},
	wazuh: {
		kind: 'wazuh',
		serverTitle: 'Import Wazuh Alert Export',
		dialogTitle: 'Import Wazuh Alert Export',
		multiple: false,
		filters: [
			{ name: 'JSON Files', extensions: ['json', 'jsonl', 'ndjson'] },
			{ name: 'All Files', extensions: ['*'] }
		],
		startMessage: () => 'Importing Wazuh alerts...',
		successMessage: (result) => `Wazuh: ${result.alert_count} alerts imported (${result.duration_ms}ms)`,
		errorPrefix: 'Wazuh import failed',
		importFn: (input) => importWazuhAlerts(input as string)
	},
	sinema: {
		kind: 'sinema',
		serverTitle: 'Import SINEMA Server CSV',
		dialogTitle: 'Import SINEMA Server CSV',
		multiple: false,
		filters: [
			{ name: 'CSV Files', extensions: ['csv', 'txt'] },
			{ name: 'All Files', extensions: ['*'] }
		],
		startMessage: () => 'Importing SINEMA Server CSV...',
		successMessage: (result) =>
			`SINEMA: ${result.new_assets} new + ${result.updated_assets} updated assets (${result.duration_ms}ms)`,
		errorPrefix: 'SINEMA import failed',
		importFn: (input) => importSinemaCsv(input as string)
	},
	tia: {
		kind: 'tia',
		serverTitle: 'Import TIA Portal XML',
		dialogTitle: 'Import TIA Portal XML',
		multiple: false,
		filters: [
			{ name: 'XML Files', extensions: ['xml'] },
			{ name: 'All Files', extensions: ['*'] }
		],
		startMessage: () => 'Importing TIA Portal XML...',
		successMessage: (result) =>
			`TIA Portal: ${result.new_assets} new + ${result.updated_assets} updated assets (${result.duration_ms}ms)`,
		errorPrefix: 'TIA Portal import failed',
		importFn: (input) => importTiaXml(input as string)
	}
};

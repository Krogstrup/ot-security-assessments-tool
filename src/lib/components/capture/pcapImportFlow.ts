import type { FileImportResult, ImportResult } from '$lib/types/capture';
import type { ImportProgressEvent } from '$lib/api/capture';
import { formatImportSuccessMessage } from './captureViewFormatters';
import type { ImportPathPickerOptions } from './importPathPicker';

export interface PcapImportHooks {
	setStatus: (s: 'idle' | 'importing' | 'done' | 'error') => void;
	setProgress: (p: ImportProgressEvent | null) => void;
	setMessage: (m: string) => void;
	setFileResults: (r: FileImportResult[]) => void;
}

export async function runPcapImport(
	importTask: Promise<ImportResult>,
	fileCount: number,
	hooks: PcapImportHooks,
	onDataChanged: () => Promise<void>
): Promise<void> {
	hooks.setStatus('importing');
	hooks.setProgress(null);
	hooks.setMessage(`Importing ${fileCount} file${fileCount > 1 ? 's' : ''}...`);
	hooks.setFileResults([]);

	const result = await importTask;

	hooks.setProgress(null);
	hooks.setStatus('done');
	hooks.setFileResults(result.per_file);
	hooks.setMessage(formatImportSuccessMessage(result));
	await onDataChanged();
}

export async function handleImportPcap(
	pickPaths: (opts: ImportPathPickerOptions) => Promise<string[]>,
	importFn: (paths: string[]) => Promise<ImportResult>,
	hooks: PcapImportHooks,
	onDataChanged: () => Promise<void>
): Promise<void> {
	try {
		const paths = await pickPaths({
			kind: 'pcap',
			serverTitle: 'Select Server PCAP Files',
			dialogTitle: 'Import PCAP Files',
			multiple: true,
			filters: [
				{ name: 'PCAP Files', extensions: ['pcap', 'pcapng', 'cap'] },
				{ name: 'All Files', extensions: ['*'] }
			]
		});
		if (paths.length === 0) return;
		await runPcapImport(importFn(paths), paths.length, hooks, onDataChanged);
	} catch (err) {
		hooks.setProgress(null);
		hooks.setStatus('error');
		hooks.setMessage(`Import failed: ${err}`);
		console.error('PCAP import error:', err);
	}
}

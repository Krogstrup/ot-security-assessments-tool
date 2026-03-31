<script lang="ts">
	import type { FileImportResult } from '$lib/types/capture';
	import type { ImportProgressEvent } from '$lib/api/capture';
	import ServerImportPickerDialog from './ServerImportPickerDialog.svelte';
	import ImportProgressBar from './ImportProgressBar.svelte';
	import FileImportResultsTable from './FileImportResultsTable.svelte';

	interface Props {
		importStatus: 'idle' | 'importing' | 'done' | 'error';
		importMessage: string;
		fileResults: FileImportResult[];
		importProgress: ImportProgressEvent | null;
		showServerPicker: boolean;
		serverPickerLoading: boolean;
		serverPickerError: string;
		serverPickerBaseDir: string;
		serverPickerFiles: Array<{ name: string; path: string; size_bytes: number }>;
		selectedServerPaths: string[];
		serverPickerListLimit: number;
		serverPickerTruncated: boolean;
		serverPickerTitle: string;
		serverPickerAllowMultiple: boolean;
		onImport: () => Promise<void>;
		onToggleServerPath: (path: string, checked: boolean) => void;
		onCloseServerPicker: (result: string[] | null) => void;
	}

	let {
		importStatus,
		importMessage,
		fileResults,
		importProgress,
		showServerPicker,
		serverPickerLoading,
		serverPickerError,
		serverPickerBaseDir,
		serverPickerFiles,
		selectedServerPaths,
		serverPickerListLimit,
		serverPickerTruncated,
		serverPickerTitle,
		serverPickerAllowMultiple,
		onImport,
		onToggleServerPath,
		onCloseServerPicker
	}: Props = $props();
</script>

<section class="capture-section">
	<h3 class="section-title">PCAP Import</h3>
	<p class="section-desc">
		Import one or more PCAP/PCAPNG files captured from an OT network. Multiple files can be
		selected simultaneously — all traffic is merged into a single topology with per-file attribution.
	</p>

	<button class="action-btn primary" onclick={onImport} disabled={importStatus === 'importing'}>
		{importStatus === 'importing' ? 'Importing...' : 'Import PCAP Files'}
	</button>

	<ServerImportPickerDialog
		show={showServerPicker}
		loading={serverPickerLoading}
		error={serverPickerError}
		baseDir={serverPickerBaseDir}
		files={serverPickerFiles}
		selectedPaths={selectedServerPaths}
		listLimit={serverPickerListLimit}
		truncated={serverPickerTruncated}
		title={serverPickerTitle}
		allowMultiple={serverPickerAllowMultiple}
		onTogglePath={onToggleServerPath}
		onClose={onCloseServerPicker}
	/>

	<ImportProgressBar progress={importProgress} />

	{#if importStatus === 'error'}
		<div class="import-result error">{importMessage}</div>
	{:else if importStatus === 'done'}
		<div class="import-result success">{importMessage}</div>
		<FileImportResultsTable results={fileResults} />
	{:else if importMessage}
		<div class="import-result info">{importMessage}</div>
	{/if}
</section>

<style>
	.capture-section {
		padding: 1.5rem;
		background: var(--gm-bg-secondary);
		border: 1px solid var(--gm-border);
		border-radius: 6px;
		margin-bottom: 1rem;
	}

	.section-title {
		font-size: 1.125rem;
		font-weight: 600;
		margin-bottom: 0.5rem;
		color: var(--gm-text-primary);
	}

	.section-desc {
		font-size: 0.8125rem;
		color: var(--gm-text-secondary);
		margin-bottom: 1rem;
	}

	.action-btn {
		padding: 0.5rem 1rem;
		border: none;
		border-radius: 4px;
		font-size: 0.8125rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn.primary {
		background: #6366f1;
		color: white;
	}

	.action-btn.primary:hover:not(:disabled) {
		background: #4f46e5;
	}

	.action-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.import-result {
		margin-top: 1rem;
		padding: 0.75rem;
		border-radius: 4px;
		font-size: 0.8125rem;
	}

	.import-result.success {
		background: rgba(34, 197, 94, 0.1);
		color: #22c55e;
		border: 1px solid rgba(34, 197, 94, 0.3);
	}

	.import-result.error {
		background: rgba(239, 68, 68, 0.1);
		color: #ef4444;
		border: 1px solid rgba(239, 68, 68, 0.3);
	}

	.import-result.info {
		background: rgba(99, 102, 241, 0.1);
		color: #6366f1;
		border: 1px solid rgba(99, 102, 241, 0.3);
	}
</style>

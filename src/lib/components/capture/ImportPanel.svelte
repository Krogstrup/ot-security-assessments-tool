<script lang="ts">
	import type { FileImportResult } from '$lib/types/capture';
	import type { IngestImportResult, SessionInfo } from '$lib/types/operations';
	import type { ImportProgressEvent } from '$lib/api/capture';

	import CaptureImportPanel from './CaptureImportPanel.svelte';
	import ExternalImportPanel from './ExternalImportPanel.svelte';
	import SessionPanel from './SessionPanel.svelte';

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

		sessions: SessionInfo[];
		currentSession: SessionInfo | null;
		sessionMessage: string;
		sessionMessageType: 'success' | 'error' | '';
		showSaveForm: boolean;
		sessionName: string;
		sessionDesc: string;
		confirmDeleteId: string | null;
		onToggleSaveForm: () => void;
		onSaveName: (name: string) => void;
		onSaveDesc: (desc: string) => void;
		onSaveSession: () => Promise<void>;
		onLoadSession: (id: string) => Promise<void>;
		onDeleteSession: (id: string) => Promise<void>;
		onExportSession: (session: SessionInfo) => Promise<void>;
		onImportArchive: () => Promise<void>;
		onConfirmDelete: (id: string | null) => void;

		ingestStatus: 'idle' | 'importing' | 'done' | 'error';
		ingestMessage: string;
		lastIngestResult: IngestImportResult | null;
		onImportZeek: () => Promise<void>;
		onImportSuricata: () => Promise<void>;
		onImportNmap: () => Promise<void>;
		onImportMasscan: () => Promise<void>;
		onImportWazuh: () => Promise<void>;
		onImportSinema: () => Promise<void>;
		onImportTia: () => Promise<void>;
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
		onCloseServerPicker,
		sessions,
		currentSession,
		sessionMessage,
		sessionMessageType,
		showSaveForm,
		sessionName,
		sessionDesc,
		confirmDeleteId,
		onToggleSaveForm,
		onSaveName,
		onSaveDesc,
		onSaveSession,
		onLoadSession,
		onDeleteSession,
		onExportSession,
		onImportArchive,
		onConfirmDelete,
		ingestStatus,
		ingestMessage,
		lastIngestResult,
		onImportZeek,
		onImportSuricata,
		onImportNmap,
		onImportMasscan,
		onImportWazuh,
		onImportSinema,
		onImportTia
	}: Props = $props();
</script>

<CaptureImportPanel
	{importStatus}
	{importMessage}
	{fileResults}
	{importProgress}
	{showServerPicker}
	{serverPickerLoading}
	{serverPickerError}
	{serverPickerBaseDir}
	{serverPickerFiles}
	{selectedServerPaths}
	{serverPickerListLimit}
	{serverPickerTruncated}
	{serverPickerTitle}
	{serverPickerAllowMultiple}
	{onImport}
	onToggleServerPath={onToggleServerPath}
	onCloseServerPicker={onCloseServerPicker}
/>

<SessionPanel
	{sessions}
	{currentSession}
	{sessionMessage}
	{sessionMessageType}
	{showSaveForm}
	{sessionName}
	{sessionDesc}
	{confirmDeleteId}
	onToggleSaveForm={onToggleSaveForm}
	onSaveName={onSaveName}
	onSaveDesc={onSaveDesc}
	onSaveSession={onSaveSession}
	onLoadSession={onLoadSession}
	onDeleteSession={onDeleteSession}
	onExportSession={onExportSession}
	onImportArchive={onImportArchive}
	onConfirmDelete={onConfirmDelete}
/>

<ExternalImportPanel
	{ingestStatus}
	{ingestMessage}
	{lastIngestResult}
	onImportZeek={onImportZeek}
	onImportSuricata={onImportSuricata}
	onImportNmap={onImportNmap}
	onImportMasscan={onImportMasscan}
	onImportWazuh={onImportWazuh}
	onImportSinema={onImportSinema}
	onImportTia={onImportTia}
/>

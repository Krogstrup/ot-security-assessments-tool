<script lang="ts">
	import { currentSession, sessions } from '$lib/stores/session';
	import { importPcap, listHeadlessImportFiles } from '$lib/api';
	import type { FileImportResult } from '$lib/types/capture';
	import type { IngestImportResult, SessionInfo } from '$lib/types/operations';
	import type { ImportProgressEvent } from '$lib/api/capture';
	import type { HeadlessImportKind } from '$lib/api/system';
	import { onDestroy, onMount } from 'svelte';

	import ImportPanel from './ImportPanel.svelte';
	import {
		EXTERNAL_IMPORT_TASKS,
		runExternalImport,
		type ExternalImportHooks
	} from './externalImportTasks';
	import {
		handleImportPcap as handleImportPcapFlow,
		type PcapImportHooks
	} from './pcapImportFlow';
	import {
		handleSaveSession as handleSaveSessionFlow,
		handleLoadSession as handleLoadSessionFlow,
		handleDeleteSession as handleDeleteSessionFlow,
		handleExportSession as handleExportSessionFlow,
		handleImportArchive as handleImportArchiveFlow,
		refreshSessionsStore,
		type SessionHandlerHooks
	} from './sessionFlows';
	import {
		beginServerPicker,
		closeServerPickerState,
		createServerPickerState,
		toggleServerPickerSelection,
		withServerPickerListing
	} from './serverPickerState';
	import { refreshCoreStores } from './captureStoreRefresh';
	import { setupImportProgressListener } from './captureListeners';
	import { pickImportPaths as resolveImportPaths, type ImportPathPickerOptions } from './importPathPicker';

	interface Props {
		onDataChanged?: () => void;
	}

	let { onDataChanged = () => {} }: Props = $props();

	let importStatus = $state<'idle' | 'importing' | 'done' | 'error'>('idle');
	let importMessage = $state('');
	let fileResults = $state<FileImportResult[]>([]);
	let importProgress = $state<ImportProgressEvent | null>(null);
	let unlistenProgress: (() => void) | null = null;
	let serverPicker = $state(createServerPickerState());

	let sessionName = $state('');
	let sessionDesc = $state('');
	let sessionMessage = $state('');
	let sessionMessageType = $state<'success' | 'error' | ''>('');
	let showSaveForm = $state(false);
	let confirmDeleteId = $state<string | null>(null);

	let ingestStatus = $state<'idle' | 'importing' | 'done' | 'error'>('idle');
	let ingestMessage = $state('');
	let lastIngestResult = $state<IngestImportResult | null>(null);

	onMount(() => {
		void refreshSessionsStore();
		void setupProgressListener();
	});

	onDestroy(() => {
		unlistenProgress?.();
	});

	async function setupProgressListener() {
		unlistenProgress = await setupImportProgressListener((progress: ImportProgressEvent) => {
			importProgress = progress;
		});
	}

	async function refreshDataAndNotify() {
		await refreshCoreStores();
		onDataChanged();
	}

	function toggleServerPickerPath(path: string, checked: boolean) {
		serverPicker = toggleServerPickerSelection(serverPicker, path, checked);
	}

	function closeServerPicker(result: string[] | null) {
		serverPicker = closeServerPickerState(serverPicker, result);
	}

	async function openServerImportPicker(
		kind: HeadlessImportKind,
		title: string,
		multiple: boolean
	): Promise<string[] | null> {
		serverPicker = beginServerPicker(serverPicker, title, multiple);

		const pickerResult = new Promise<string[] | null>((resolve) => {
			serverPicker = { ...serverPicker, resolve };
		});

		try {
			const listing = await listHeadlessImportFiles(kind);
			serverPicker = withServerPickerListing(serverPicker, listing);
			if (listing.files.length === 0) {
				serverPicker = {
					...serverPicker,
					error: `No importable files found in ${listing.base_dir}`
				};
			}
		} catch (err) {
			closeServerPicker(null);
			throw err;
		} finally {
			serverPicker = { ...serverPicker, loading: false };
		}

		return pickerResult;
	}

	async function pickImportPaths(options: ImportPathPickerOptions): Promise<string[]> {
		return resolveImportPaths(options, openServerImportPicker);
	}

	const pcapHooks: PcapImportHooks = {
		setStatus: (s) => (importStatus = s),
		setProgress: (p) => (importProgress = p),
		setMessage: (m) => (importMessage = m),
		setFileResults: (r) => (fileResults = r)
	};

	const sessionHooks: SessionHandlerHooks = {
		setMessage: (m, t) => { sessionMessage = m; sessionMessageType = t; },
		setShowSaveForm: (v) => (showSaveForm = v),
		clearSaveForm: () => { sessionName = ''; sessionDesc = ''; },
		setConfirmDeleteId: (id) => (confirmDeleteId = id)
	};

	const externalHooks: ExternalImportHooks = {
		setStatus: (s) => (ingestStatus = s),
		setMessage: (m) => (ingestMessage = m),
		setLastResult: (r) => (lastIngestResult = r)
	};

	const handleImportPcap = () => handleImportPcapFlow(pickImportPaths, importPcap, pcapHooks, refreshDataAndNotify);
	const handleSaveSession = () => handleSaveSessionFlow(sessionName, sessionDesc, sessionHooks);
	const handleLoadSession = (id: string) => handleLoadSessionFlow(id, refreshDataAndNotify, sessionHooks);
	const handleDeleteSession = (id: string) => handleDeleteSessionFlow(id, sessionHooks);
	const handleExportSession = (session: SessionInfo) => handleExportSessionFlow(session, sessionHooks);
	const handleImportArchive = () => handleImportArchiveFlow(pickImportPaths, refreshDataAndNotify, sessionHooks);

	const handleImportZeek = () => runExternalImport(EXTERNAL_IMPORT_TASKS.zeek, pickImportPaths, refreshDataAndNotify, externalHooks);
	const handleImportSuricata = () => runExternalImport(EXTERNAL_IMPORT_TASKS.suricata, pickImportPaths, refreshDataAndNotify, externalHooks);
	const handleImportNmap = () => runExternalImport(EXTERNAL_IMPORT_TASKS.nmap, pickImportPaths, refreshDataAndNotify, externalHooks);
	const handleImportMasscan = () => runExternalImport(EXTERNAL_IMPORT_TASKS.masscan, pickImportPaths, refreshDataAndNotify, externalHooks);
	const handleImportWazuh = () => runExternalImport(EXTERNAL_IMPORT_TASKS.wazuh, pickImportPaths, refreshDataAndNotify, externalHooks);
	const handleImportSinema = () => runExternalImport(EXTERNAL_IMPORT_TASKS.sinema, pickImportPaths, refreshDataAndNotify, externalHooks);
	const handleImportTia = () => runExternalImport(EXTERNAL_IMPORT_TASKS.tia, pickImportPaths, refreshDataAndNotify, externalHooks);
</script>

<ImportPanel
	{importStatus}
	{importMessage}
	{fileResults}
	{importProgress}
	showServerPicker={serverPicker.show}
	serverPickerLoading={serverPicker.loading}
	serverPickerError={serverPicker.error}
	serverPickerBaseDir={serverPicker.baseDir}
	serverPickerFiles={serverPicker.files}
	selectedServerPaths={serverPicker.selectedPaths}
	serverPickerListLimit={serverPicker.listLimit}
	serverPickerTruncated={serverPicker.truncated}
	serverPickerTitle={serverPicker.title}
	serverPickerAllowMultiple={serverPicker.allowMultiple}
	onImport={handleImportPcap}
	onToggleServerPath={toggleServerPickerPath}
	onCloseServerPicker={closeServerPicker}
	sessions={$sessions}
	currentSession={$currentSession}
	{sessionMessage}
	{sessionMessageType}
	{showSaveForm}
	{sessionName}
	{sessionDesc}
	{confirmDeleteId}
	onToggleSaveForm={() => (showSaveForm = !showSaveForm)}
	onSaveName={(name) => (sessionName = name)}
	onSaveDesc={(desc) => (sessionDesc = desc)}
	onSaveSession={handleSaveSession}
	onLoadSession={handleLoadSession}
	onDeleteSession={handleDeleteSession}
	onExportSession={handleExportSession}
	onImportArchive={handleImportArchive}
	onConfirmDelete={(id) => (confirmDeleteId = id)}
	{ingestStatus}
	{ingestMessage}
	{lastIngestResult}
	onImportZeek={handleImportZeek}
	onImportSuricata={handleImportSuricata}
	onImportNmap={handleImportNmap}
	onImportMasscan={handleImportMasscan}
	onImportWazuh={handleImportWazuh}
	onImportSinema={handleImportSinema}
	onImportTia={handleImportTia}
/>

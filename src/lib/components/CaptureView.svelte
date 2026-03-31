<script lang="ts">
	import { interfaces, captureStatus, captureStats, assets, connections, topology, sessions, currentSession, assetCount, connectionCount, protocolStats } from '$lib/stores';
	import {
		importPcap, listHeadlessImportFiles, cancelImport, onImportProgress,
		getAssets, getConnections, getDataCounts, getTopology, getProtocolStats,
		startCapture, stopCapture, pauseCapture, resumeCapture,
		onCaptureStats, onCaptureError,
		saveSession, loadSession, listSessions, deleteSession,
		exportSessionArchive, importSessionArchive,
		importZeekLogs, importSuricataEve, importNmapXml, importMasscanJson, importWazuhAlerts,
		importSinemaCsv, importTiaXml,
		getFindings, isTauriRuntime
	} from '$lib/api';
	import { openPathDialog, savePathDialog } from '$lib/utils/dialog';
	import type { ImportProgressEvent, HeadlessImportKind } from '$lib/types';
	import type { FileImportResult, CaptureStatsEvent, SessionInfo, IngestImportResult } from '$lib/types';
	import { onMount, onDestroy } from 'svelte';
	import { get } from 'svelte/store';

	import CaptureImportPanel from './capture/CaptureImportPanel.svelte';
	import LiveCapturePanel from './capture/LiveCapturePanel.svelte';
	import ProtocolStatsPanel from './capture/ProtocolStatsPanel.svelte';
	import SessionPanel from './capture/SessionPanel.svelte';
	import ExternalImportPanel from './capture/ExternalImportPanel.svelte';

	// ── PCAP Import State ──────────────────────────────────────────
	let importStatus = $state<'idle' | 'importing' | 'done' | 'error'>('idle');
	let importMessage = $state('');
	let fileResults = $state<FileImportResult[]>([]);
	let importProgress = $state<ImportProgressEvent | null>(null);
	let unlistenProgress: (() => void) | null = null;
	let showServerPicker = $state(false);
	let serverPickerLoading = $state(false);
	let serverPickerError = $state('');
	let serverPickerBaseDir = $state('');
	let serverPickerFiles = $state<Array<{ name: string; path: string; size_bytes: number }>>([]);
	let selectedServerPaths = $state<string[]>([]);
	let serverPickerListLimit = $state(0);
	let serverPickerTruncated = $state(false);
	let serverPickerTitle = $state('Select Server Files');
	let serverPickerAllowMultiple = $state(true);
	let serverPickerResolve: ((paths: string[] | null) => void) | null = null;

	// ── Live Capture State ────────────────────────────────────────
	let selectedInterface = $state('');
	let bpfFilter = $state('');
	let captureError = $state('');
	let stopResult = $state<{ packets: number; bytes: number; elapsed: number; saved: boolean; path: string | null } | null>(null);
	let unlistenStats: (() => void) | null = null;
	let unlistenError: (() => void) | null = null;
	let refreshInterval: ReturnType<typeof setInterval> | null = null;

	// ── Session State ──────────────────────────────────────────────
	let sessionName = $state('');
	let sessionDesc = $state('');
	let sessionMessage = $state('');
	let sessionMessageType = $state<'success' | 'error' | ''>('');
	let showSaveForm = $state(false);
	let confirmDeleteId = $state<string | null>(null);

	// ── External Import State ──────────────────────────────────────
	let ingestStatus = $state<'idle' | 'importing' | 'done' | 'error'>('idle');
	let ingestMessage = $state('');
	let lastIngestResult = $state<IngestImportResult | null>(null);

	const isCapturing = $derived($captureStatus === 'capturing' || $captureStatus === 'paused');

	// ────────────────────────────────────────────────────────────────
	// EVENT LISTENERS & LIFECYCLE
	// ────────────────────────────────────────────────────────────────

	onMount(() => {
		setupEventListeners();
		refreshSessions();
	});

	onDestroy(() => {
		cleanupListeners();
	});

	async function setupEventListeners() {
		unlistenStats = await onCaptureStats((stats: CaptureStatsEvent) => {
			captureStats.set(stats);
		});

		unlistenError = await onCaptureError((error: string) => {
			captureError = error;
			captureStatus.set('error');
			cleanupRefreshInterval();
		});

		unlistenProgress = await onImportProgress((progress: ImportProgressEvent) => {
			importProgress = progress;
		});
	}

	function cleanupListeners() {
		unlistenStats?.();
		unlistenError?.();
		unlistenProgress?.();
		cleanupRefreshInterval();
	}

	function cleanupRefreshInterval() {
		if (refreshInterval) {
			clearInterval(refreshInterval);
			refreshInterval = null;
		}
	}

	function startDataRefresh() {
		cleanupRefreshInterval();
		refreshInterval = setInterval(async () => {
			try {
				const [assetPage, connPage, newTopology, newStats, counts] = await Promise.all([
					getAssets(0, 200),
					getConnections(0, 500),
					getTopology(),
					getProtocolStats(),
					getDataCounts()
				]);
				assets.set(assetPage.assets);
				connections.set(connPage.connections);
				topology.set(newTopology);
				protocolStats.set(newStats);
				assetCount.set(counts.asset_count);
				connectionCount.set(counts.connection_count);
			} catch (err) {
				console.error('Data refresh error:', err);
			}
		}, 500);
	}

	// ────────────────────────────────────────────────────────────────
	// PCAP IMPORT HANDLERS
	// ────────────────────────────────────────────────────────────────

	async function runPcapImport(importTask: Promise<import('$lib/types').ImportResult>, fileCount: number) {
		importStatus = 'importing';
		importProgress = null;
		importMessage = `Importing ${fileCount} file${fileCount > 1 ? 's' : ''}...`;
		fileResults = [];

		const result = await importTask;

		importProgress = null;
		importStatus = 'done';
		fileResults = result.per_file;
		importMessage = `Imported ${result.packet_count.toLocaleString()} packets from ${result.file_count} file${result.file_count > 1 ? 's' : ''} → ${result.asset_count} assets, ${result.connection_count} connections (${result.duration_ms}ms)`;

		const [assetPage, connPage, newTopology, newStats, counts] = await Promise.all([
			getAssets(0, 200),
			getConnections(0, 500),
			getTopology(),
			getProtocolStats(),
			getDataCounts()
		]);

		assets.set(assetPage.assets);
		connections.set(connPage.connections);
		topology.set(newTopology);
		protocolStats.set(newStats);
		assetCount.set(counts.asset_count);
		connectionCount.set(counts.connection_count);
	}

	function toggleServerPickerPath(path: string, checked: boolean) {
		if (!serverPickerAllowMultiple) {
			selectedServerPaths = checked ? [path] : [];
			return;
		}
		if (checked) {
			if (!selectedServerPaths.includes(path)) {
				selectedServerPaths = [...selectedServerPaths, path];
			}
			return;
		}
		selectedServerPaths = selectedServerPaths.filter((p) => p !== path);
	}

	function closeServerPicker(result: string[] | null) {
		showServerPicker = false;
		const resolve = serverPickerResolve;
		serverPickerResolve = null;
		if (resolve) resolve(result);
	}

	async function openServerImportPicker(kind: HeadlessImportKind, title: string, multiple: boolean): Promise<string[] | null> {
		serverPickerLoading = true;
		serverPickerError = '';
		serverPickerFiles = [];
		serverPickerBaseDir = '';
		selectedServerPaths = [];
		serverPickerListLimit = 0;
		serverPickerTruncated = false;
		serverPickerTitle = title;
		serverPickerAllowMultiple = multiple;
		showServerPicker = true;

		const pickerResult = new Promise<string[] | null>((resolve) => {
			serverPickerResolve = resolve;
		});

		try {
			const listing = await listHeadlessImportFiles(kind);
			serverPickerBaseDir = listing.base_dir;
			serverPickerFiles = listing.files;
			serverPickerListLimit = listing.list_limit;
			serverPickerTruncated = listing.truncated;
			if (listing.files.length === 0) {
				serverPickerError = `No importable files found in ${listing.base_dir}`;
			}
		} catch (err) {
			closeServerPicker(null);
			throw err;
		} finally {
			serverPickerLoading = false;
		}
		return pickerResult;
	}

	async function handleImportPcap() {
		try {
			if (!isTauriRuntime()) {
				const paths = await openServerImportPicker('pcap', 'Select Server PCAP Files', true);
				if (!paths || paths.length === 0) return;
				await runPcapImport(importPcap(paths), paths.length);
				return;
			}

			const selected = await openPathDialog({
				title: 'Import PCAP Files',
				multiple: true,
				filters: [
					{ name: 'PCAP Files', extensions: ['pcap', 'pcapng', 'cap'] },
					{ name: 'All Files', extensions: ['*'] }
				]
			});
			if (!selected) return;
			const paths = Array.isArray(selected) ? selected : [selected];
			if (paths.length === 0) return;
			await runPcapImport(importPcap(paths), paths.length);
		} catch (err) {
			importProgress = null;
			importStatus = 'error';
			importMessage = `Import failed: ${err}`;
			console.error('PCAP import error:', err);
		}
	}

	async function handleCancelImport() {
		try {
			await cancelImport();
		} catch (err) {
			console.error('Cancel import error:', err);
		}
	}

	// ────────────────────────────────────────────────────────────────
	// LIVE CAPTURE HANDLERS
	// ────────────────────────────────────────────────────────────────

	async function handleStartCapture() {
		if (!selectedInterface) return;
		captureError = '';
		stopResult = null;

		try {
			const filter = bpfFilter.trim() || undefined;
			await startCapture(selectedInterface, filter);
			captureStatus.set('capturing');
			captureStats.set({
				packets_captured: 0,
				packets_per_second: 0,
				bytes_captured: 0,
				active_connections: 0,
				asset_count: 0,
				elapsed_seconds: 0
			});
			startDataRefresh();
		} catch (err) {
			captureError = `${err}`;
			captureStatus.set('error');
		}
	}

	async function handleStopCapture() {
		try {
			const savePath = await savePathDialog({
				title: 'Save Capture as PCAP',
				defaultPath: `capture_${new Date().toISOString().slice(0, 19).replace(/:/g, '-')}.pcap`,
				filters: [
					{ name: 'PCAP Files', extensions: ['pcap'] },
					{ name: 'All Files', extensions: ['*'] }
				]
			});

			const result = await stopCapture(savePath ?? undefined);
			captureStatus.set('idle');
			cleanupRefreshInterval();
			stopResult = {
				packets: Number(result.packets_captured),
				bytes: Number(result.bytes_captured),
				elapsed: result.elapsed_seconds,
				saved: result.pcap_saved,
				path: result.pcap_path
			};

			const [assetPage, connPage, newTopology, newStats, counts] = await Promise.all([
				getAssets(0, 200),
				getConnections(0, 500),
				getTopology(),
				getProtocolStats(),
				getDataCounts()
			]);
			assets.set(assetPage.assets);
			connections.set(connPage.connections);
			topology.set(newTopology);
			protocolStats.set(newStats);
			assetCount.set(counts.asset_count);
			connectionCount.set(counts.connection_count);
		} catch (err) {
			captureError = `Stop failed: ${err}`;
		}
	}

	async function handlePauseResume() {
		try {
			if ($captureStatus === 'paused') {
				await resumeCapture();
				captureStatus.set('capturing');
				startDataRefresh();
			} else {
				await pauseCapture();
				captureStatus.set('paused');
				cleanupRefreshInterval();
			}
		} catch (err) {
			captureError = `${err}`;
		}
	}

	// ────────────────────────────────────────────────────────────────
	// SESSION HANDLERS
	// ────────────────────────────────────────────────────────────────

	async function refreshSessions() {
		try {
			const list = await listSessions();
			sessions.set(list);
		} catch {
			// DB may not be available in dev mode
		}
	}

	async function handleSaveSession() {
		if (!sessionName.trim()) return;
		try {
			const info = await saveSession(sessionName.trim(), sessionDesc.trim() || undefined);
			currentSession.set(info);
			sessionMessage = `Session "${info.name}" saved (${info.asset_count} assets, ${info.connection_count} connections)`;
			sessionMessageType = 'success';
			showSaveForm = false;
			sessionName = '';
			sessionDesc = '';
			await refreshSessions();
		} catch (err) {
			sessionMessage = `Save failed: ${err}`;
			sessionMessageType = 'error';
		}
	}

	async function handleLoadSession(id: string) {
		try {
			const info = await loadSession(id);
			currentSession.set(info);
			const [assetPage, connPage, newTopology, newStats, counts] = await Promise.all([
				getAssets(0, 200), getConnections(0, 500), getTopology(), getProtocolStats(), getDataCounts()
			]);
			assets.set(assetPage.assets);
			connections.set(connPage.connections);
			topology.set(newTopology);
			protocolStats.set(newStats);
			assetCount.set(counts.asset_count);
			connectionCount.set(counts.connection_count);
			sessionMessage = `Session "${info.name}" loaded`;
			sessionMessageType = 'success';
		} catch (err) {
			sessionMessage = `Load failed: ${err}`;
			sessionMessageType = 'error';
		}
	}

	async function handleDeleteSession(id: string) {
		try {
			await deleteSession(id);
			if ($currentSession?.id === id) {
				currentSession.set(null);
			}
			confirmDeleteId = null;
			sessionMessage = 'Session deleted';
			sessionMessageType = 'success';
			await refreshSessions();
		} catch (err) {
			sessionMessage = `Delete failed: ${err}`;
			sessionMessageType = 'error';
		}
	}

	async function handleExportSession(session: SessionInfo) {
		try {
			const path = await savePathDialog({
				title: 'Export Session Archive',
				defaultPath: `${session.name.replace(/[^a-zA-Z0-9_-]/g, '_')}.kkj`,
				filters: [
					{ name: 'Kusanagi Kajiki Archive', extensions: ['kkj'] },
					{ name: 'All Files', extensions: ['*'] }
				]
			});
			if (!path) return;
			await exportSessionArchive(session.id, path);
			sessionMessage = `Exported to ${path}`;
			sessionMessageType = 'success';
		} catch (err) {
			sessionMessage = `Export failed: ${err}`;
			sessionMessageType = 'error';
		}
	}

	async function handleImportArchive() {
		try {
			let path: string | null = null;
			if (!isTauriRuntime()) {
				const paths = await openServerImportPicker('session_archive', 'Import Session Archive', false);
				path = paths && paths.length > 0 ? paths[0] : null;
			} else {
				const selected = await openPathDialog({
					title: 'Import Session Archive',
					multiple: false,
					filters: [
						{ name: 'Kusanagi Kajiki Archive', extensions: ['kkj'] },
						{ name: 'All Files', extensions: ['*'] }
					]
				});
				if (!selected) return;
				path = Array.isArray(selected) ? selected[0] : selected;
			}
			if (!path) return;
			const info = await importSessionArchive(path);
			currentSession.set(info);
			const [assetPage, connPage, newTopology, newStats, counts] = await Promise.all([
				getAssets(0, 200), getConnections(0, 500), getTopology(), getProtocolStats(), getDataCounts()
			]);
			assets.set(assetPage.assets);
			connections.set(connPage.connections);
			topology.set(newTopology);
			protocolStats.set(newStats);
			assetCount.set(counts.asset_count);
			connectionCount.set(counts.connection_count);
			sessionMessage = `Imported "${info.name}" (${info.asset_count} assets)`;
			sessionMessageType = 'success';
			await refreshSessions();
		} catch (err) {
			sessionMessage = `Import failed: ${err}`;
			sessionMessageType = 'error';
		}
	}

	// ────────────────────────────────────────────────────────────────
	// EXTERNAL TOOL IMPORT HANDLERS
	// ────────────────────────────────────────────────────────────────

	async function refreshStores() {
		const [assetPage, connPage, newTopology, newStats, counts] = await Promise.all([
			getAssets(0, 200), getConnections(0, 500), getTopology(), getProtocolStats(), getDataCounts()
		]);
		assets.set(assetPage.assets);
		connections.set(connPage.connections);
		topology.set(newTopology);
		protocolStats.set(newStats);
		assetCount.set(counts.asset_count);
		connectionCount.set(counts.connection_count);
	}

	async function handleImportZeek() {
		try {
			let paths: string[] = [];
			if (!isTauriRuntime()) {
				const selected = await openServerImportPicker('zeek', 'Import Zeek Logs', true);
				paths = selected ?? [];
			} else {
				const selected = await openPathDialog({
					title: 'Import Zeek Logs',
					multiple: true,
					filters: [
						{ name: 'Zeek Logs', extensions: ['log'] },
						{ name: 'All Files', extensions: ['*'] }
					]
				});
				if (!selected) return;
				paths = Array.isArray(selected) ? selected : [selected];
			}
			if (paths.length === 0) return;

			ingestStatus = 'importing';
			ingestMessage = `Importing ${paths.length} Zeek log file${paths.length > 1 ? 's' : ''}...`;
			lastIngestResult = null;

			const result = await importZeekLogs(paths);
			lastIngestResult = result;
			ingestStatus = 'done';
			ingestMessage = `Zeek: ${result.new_assets} new + ${result.updated_assets} updated assets, ${result.connection_count} connections (${result.duration_ms}ms)`;

			await refreshStores();
		} catch (err) {
			ingestStatus = 'error';
			ingestMessage = `Zeek import failed: ${err}`;
		}
	}

	async function handleImportSuricata() {
		try {
			let path: string | null = null;
			if (!isTauriRuntime()) {
				const paths = await openServerImportPicker('suricata', 'Import Suricata eve.json', false);
				path = paths && paths.length > 0 ? paths[0] : null;
			} else {
				const selected = await openPathDialog({
					title: 'Import Suricata eve.json',
					multiple: false,
					filters: [
						{ name: 'JSON Files', extensions: ['json'] },
						{ name: 'All Files', extensions: ['*'] }
					]
				});
				if (!selected) return;
				path = Array.isArray(selected) ? selected[0] : selected;
			}
			if (!path) return;

			ingestStatus = 'importing';
			ingestMessage = 'Importing Suricata eve.json...';
			lastIngestResult = null;

			const result = await importSuricataEve(path);
			lastIngestResult = result;
			ingestStatus = 'done';
			ingestMessage = `Suricata: ${result.new_assets} new + ${result.updated_assets} updated assets, ${result.connection_count} connections, ${result.alert_count} alerts (${result.duration_ms}ms)`;

			await refreshStores();
		} catch (err) {
			ingestStatus = 'error';
			ingestMessage = `Suricata import failed: ${err}`;
		}
	}

	async function handleImportNmap() {
		try {
			let path: string | null = null;
			if (!isTauriRuntime()) {
				const paths = await openServerImportPicker('nmap', 'Import Nmap XML', false);
				path = paths && paths.length > 0 ? paths[0] : null;
			} else {
				const selected = await openPathDialog({
					title: 'Import Nmap XML',
					multiple: false,
					filters: [
						{ name: 'XML Files', extensions: ['xml'] },
						{ name: 'All Files', extensions: ['*'] }
					]
				});
				if (!selected) return;
				path = Array.isArray(selected) ? selected[0] : selected;
			}
			if (!path) return;

			ingestStatus = 'importing';
			ingestMessage = 'Importing Nmap XML...';
			lastIngestResult = null;

			const result = await importNmapXml(path);
			lastIngestResult = result;
			ingestStatus = 'done';
			ingestMessage = `Nmap: ${result.new_assets} new + ${result.updated_assets} updated assets (${result.duration_ms}ms) [ACTIVE SCAN]`;

			await refreshStores();
		} catch (err) {
			ingestStatus = 'error';
			ingestMessage = `Nmap import failed: ${err}`;
		}
	}

	async function handleImportMasscan() {
		try {
			let path: string | null = null;
			if (!isTauriRuntime()) {
				const paths = await openServerImportPicker('masscan', 'Import Masscan JSON', false);
				path = paths && paths.length > 0 ? paths[0] : null;
			} else {
				const selected = await openPathDialog({
					title: 'Import Masscan JSON',
					multiple: false,
					filters: [
						{ name: 'JSON Files', extensions: ['json'] },
						{ name: 'All Files', extensions: ['*'] }
					]
				});
				if (!selected) return;
				path = Array.isArray(selected) ? selected[0] : selected;
			}
			if (!path) return;

			ingestStatus = 'importing';
			ingestMessage = 'Importing Masscan JSON...';
			lastIngestResult = null;

			const result = await importMasscanJson(path);
			lastIngestResult = result;
			ingestStatus = 'done';
			ingestMessage = `Masscan: ${result.new_assets} new + ${result.updated_assets} updated assets (${result.duration_ms}ms) [ACTIVE SCAN]`;

			await refreshStores();
		} catch (err) {
			ingestStatus = 'error';
			ingestMessage = `Masscan import failed: ${err}`;
		}
	}

	async function handleImportWazuh() {
		try {
			let path: string | null = null;
			if (!isTauriRuntime()) {
				const paths = await openServerImportPicker('wazuh', 'Import Wazuh Alert Export', false);
				path = paths && paths.length > 0 ? paths[0] : null;
			} else {
				const selected = await openPathDialog({
					title: 'Import Wazuh Alert Export',
					multiple: false,
					filters: [
						{ name: 'JSON Files', extensions: ['json', 'jsonl', 'ndjson'] },
						{ name: 'All Files', extensions: ['*'] }
					]
				});
				if (!selected) return;
				path = Array.isArray(selected) ? selected[0] : selected;
			}
			if (!path) return;

			ingestStatus = 'importing';
			ingestMessage = 'Importing Wazuh alerts...';
			lastIngestResult = null;

			const result = await importWazuhAlerts(path);
			lastIngestResult = result;
			ingestStatus = 'done';
			ingestMessage = `Wazuh: ${result.alert_count} alerts imported (${result.duration_ms}ms)`;

			await refreshStores();
		} catch (err) {
			ingestStatus = 'error';
			ingestMessage = `Wazuh import failed: ${err}`;
		}
	}

	async function handleImportSinema() {
		try {
			let path: string | null = null;
			if (!isTauriRuntime()) {
				const paths = await openServerImportPicker('sinema', 'Import SINEMA Server CSV', false);
				path = paths && paths.length > 0 ? paths[0] : null;
			} else {
				const selected = await openPathDialog({
					title: 'Import SINEMA Server CSV',
					multiple: false,
					filters: [
						{ name: 'CSV Files', extensions: ['csv', 'txt'] },
						{ name: 'All Files', extensions: ['*'] }
					]
				});
				if (!selected) return;
				path = Array.isArray(selected) ? selected[0] : selected;
			}
			if (!path) return;

			ingestStatus = 'importing';
			ingestMessage = 'Importing SINEMA Server CSV...';
			lastIngestResult = null;

			const result = await importSinemaCsv(path);
			lastIngestResult = result;
			ingestStatus = 'done';
			ingestMessage = `SINEMA: ${result.new_assets} new + ${result.updated_assets} updated assets (${result.duration_ms}ms)`;

			await refreshStores();
		} catch (err) {
			ingestStatus = 'error';
			ingestMessage = `SINEMA import failed: ${err}`;
		}
	}

	async function handleImportTia() {
		try {
			let path: string | null = null;
			if (!isTauriRuntime()) {
				const paths = await openServerImportPicker('tia', 'Import TIA Portal XML', false);
				path = paths && paths.length > 0 ? paths[0] : null;
			} else {
				const selected = await openPathDialog({
					title: 'Import TIA Portal XML',
					multiple: false,
					filters: [
						{ name: 'XML Files', extensions: ['xml'] },
						{ name: 'All Files', extensions: ['*'] }
					]
				});
				if (!selected) return;
				path = Array.isArray(selected) ? selected[0] : selected;
			}
			if (!path) return;

			ingestStatus = 'importing';
			ingestMessage = 'Importing TIA Portal XML...';
			lastIngestResult = null;

			const result = await importTiaXml(path);
			lastIngestResult = result;
			ingestStatus = 'done';
			ingestMessage = `TIA Portal: ${result.new_assets} new + ${result.updated_assets} updated assets (${result.duration_ms}ms)`;

			await refreshStores();
		} catch (err) {
			ingestStatus = 'error';
			ingestMessage = `TIA Portal import failed: ${err}`;
		}
	}
</script>

<div class="capture-container">
	<div class="capture-toolbar">
		<h2 class="view-title">Capture & Import</h2>
	</div>

	<div class="capture-content">
		<CaptureImportPanel
			{importStatus}
			{importMessage}
			{fileResults}
			totalStats={{ packets: 0, assets: 0, connections: 0, ms: 0, files: 0 }}
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
			onImport={handleImportPcap}
			onCancelImport={handleCancelImport}
			onToggleServerPath={toggleServerPickerPath}
			onCloseServerPicker={closeServerPicker}
		/>

		<LiveCapturePanel
			{selectedInterface}
			{bpfFilter}
			{captureError}
			captureStatus={$captureStatus}
			captureStats={$captureStats}
			{stopResult}
			interfaces={$interfaces}
			onInterfaceChange={(iface) => (selectedInterface = iface)}
			onFilterChange={(filter) => (bpfFilter = filter)}
			onStartCapture={handleStartCapture}
			onStopCapture={handleStopCapture}
			onPauseResume={handlePauseResume}
		/>

		<ProtocolStatsPanel stats={$protocolStats} />

		<SessionPanel
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
		/>

		<ExternalImportPanel
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
	</div>
</div>

<style>
	.capture-container {
		display: flex;
		flex-direction: column;
		height: 100%;
		background: var(--gm-bg-primary);
	}

	.capture-toolbar {
		padding: 0.75rem 1rem;
		background: var(--gm-bg-secondary);
		border-bottom: 1px solid var(--gm-border);
		flex-shrink: 0;
	}

	.view-title {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--gm-text-primary);
	}

	.capture-content {
		flex: 1;
		overflow-y: auto;
		padding: 1rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}
</style>

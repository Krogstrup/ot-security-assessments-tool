<script lang="ts">
	import { captureStats, captureStatus, interfaces } from '$lib/stores/capture';
	import { pauseCapture, resumeCapture, startCapture, stopCapture } from '$lib/api';
	import { savePathDialog } from '$lib/utils/dialog';
	import type { CaptureStatsEvent } from '$lib/types/capture';
	import { onDestroy, onMount } from 'svelte';

	import LiveCapturePanel from './LiveCapturePanel.svelte';
	import {
		defaultCaptureFileName,
		emptyCaptureStats,
		mapStopCaptureSummary,
		type CaptureStopSummary
	} from './captureViewFormatters';
	import { refreshCoreStores } from './captureStoreRefresh';
	import {
		cleanupCaptureRuntimeListeners,
		setupCaptureRuntimeListeners
	} from './captureListeners';

	interface Props {
		onDataChanged?: () => void;
	}

	let { onDataChanged = () => {} }: Props = $props();

	let selectedInterface = $state('');
	let bpfFilter = $state('');
	let captureError = $state('');
	let stopResult = $state<CaptureStopSummary | null>(null);
	let unlistenStats: (() => void) | null = null;
	let unlistenError: (() => void) | null = null;
	let refreshInterval: ReturnType<typeof setInterval> | null = null;

	onMount(() => {
		void setupEventListeners();
	});

	onDestroy(() => {
		cleanupCaptureRuntimeListeners({ unlistenStats, unlistenError });
		cleanupRefreshInterval();
	});

	async function setupEventListeners() {
		const handles = await setupCaptureRuntimeListeners({
			onStats: (stats: CaptureStatsEvent) => {
				captureStats.set(stats);
			},
			onError: (error: string) => {
				captureError = error;
				captureStatus.set('error');
				cleanupRefreshInterval();
			}
		});
		unlistenStats = handles.unlistenStats;
		unlistenError = handles.unlistenError;
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
				await refreshCoreStores();
			} catch (err) {
				console.error('Data refresh error:', err);
			}
		}, 500);
	}

	async function handleStartCapture() {
		if (!selectedInterface) return;
		captureError = '';
		stopResult = null;

		try {
			const filter = bpfFilter.trim() || undefined;
			await startCapture(selectedInterface, filter);
			captureStatus.set('capturing');
			captureStats.set(emptyCaptureStats());
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
				defaultPath: defaultCaptureFileName(),
				filters: [
					{ name: 'PCAP Files', extensions: ['pcap'] },
					{ name: 'All Files', extensions: ['*'] }
				]
			});

			const result = await stopCapture(savePath ?? undefined);
			captureStatus.set('idle');
			cleanupRefreshInterval();
			stopResult = mapStopCaptureSummary(result);
			await refreshCoreStores();
			onDataChanged();
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
</script>

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

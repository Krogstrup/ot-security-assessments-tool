<script lang="ts">
	import type { CaptureStatsEvent } from '$lib/types/capture';
	import type { NetworkInterface } from '$lib/types/network';
	import LiveCaptureControls from './LiveCaptureControls.svelte';
	import LiveCaptureStatsGrid from './LiveCaptureStatsGrid.svelte';
	import { formatDuration } from './liveCaptureFormatters';

	interface Props {
		selectedInterface: string;
		bpfFilter: string;
		captureError: string;
		captureStatus: 'idle' | 'capturing' | 'paused' | 'error';
		captureStats: CaptureStatsEvent | null;
		stopResult: { packets: number; bytes: number; elapsed: number; saved: boolean; path: string | null } | null;
		interfaces: NetworkInterface[];
		onInterfaceChange: (iface: string) => void;
		onFilterChange: (filter: string) => void;
		onStartCapture: () => Promise<void>;
		onStopCapture: () => Promise<void>;
		onPauseResume: () => Promise<void>;
	}

	let {
		selectedInterface,
		bpfFilter,
		captureError,
		captureStatus,
		captureStats,
		stopResult,
		interfaces,
		onInterfaceChange,
		onFilterChange,
		onStartCapture,
		onStopCapture,
		onPauseResume
	}: Props = $props();

	const isCapturing = $derived(captureStatus === 'capturing' || captureStatus === 'paused');
</script>

<section class="capture-section">
	<h3 class="section-title">Live Capture</h3>
	<p class="section-desc">
		Perform packet capture directly on the network interface. All traffic is analyzed in real-time for assets, connections, and protocol behaviors.
	</p>

	<LiveCaptureControls
		{selectedInterface}
		{bpfFilter}
		{captureStatus}
		{interfaces}
		{onInterfaceChange}
		{onFilterChange}
		{onStartCapture}
		{onStopCapture}
		{onPauseResume}
	/>

	{#if captureError}
		<div class="capture-error">{captureError}</div>
	{/if}

	{#if isCapturing && captureStats}
		<LiveCaptureStatsGrid stats={captureStats} />
	{/if}

	{#if stopResult}
		<div class="capture-result success">
			<strong>Capture Complete</strong>
			<div class="result-detail">
				{stopResult.packets.toLocaleString()} packets captured in {formatDuration(stopResult.elapsed)}
				{#if stopResult.saved && stopResult.path}
					<br />Saved to: <code>{stopResult.path}</code>
				{/if}
			</div>
		</div>
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

	.capture-error {
		padding: 0.75rem;
		background: rgba(239, 68, 68, 0.12);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 4px;
		color: #fca5a5;
		font-size: 0.8125rem;
		margin-bottom: 1rem;
	}

	.capture-result {
		margin-top: 1rem;
		padding: 0.75rem;
		border-radius: 4px;
		font-size: 0.8125rem;
	}

	.capture-result.success {
		background: rgba(16, 185, 129, 0.12);
		border: 1px solid rgba(16, 185, 129, 0.3);
		color: #a7f3d0;
	}

	.result-detail {
		margin-top: 0.35rem;
		font-size: 0.75rem;
	}
</style>

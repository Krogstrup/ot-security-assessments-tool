<script lang="ts">
	import type { CaptureStatsEvent } from '$lib/types';

	interface Props {
		selectedInterface: string;
		bpfFilter: string;
		captureError: string;
		captureStatus: 'idle' | 'capturing' | 'paused' | 'error';
		captureStats: CaptureStatsEvent | null;
		stopResult: { packets: number; bytes: number; elapsed: number; saved: boolean; path: string | null } | null;
		interfaces: string[];
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

	function formatBytes(bytes: number): string {
		if (bytes < 1024) return `${bytes} B`;
		if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
		if (bytes < 1024 * 1024 * 1024) return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
		return `${(bytes / 1024 / 1024 / 1024).toFixed(2)} GB`;
	}

	function formatDuration(seconds: number): string {
		const h = Math.floor(seconds / 3600);
		const m = Math.floor((seconds % 3600) / 60);
		const s = Math.floor(seconds % 60);
		if (h > 0) return `${h}h ${m}m ${s}s`;
		if (m > 0) return `${m}m ${s}s`;
		return `${s}s`;
	}

	const isCapturing = $derived(captureStatus === 'capturing' || captureStatus === 'paused');
</script>

<section class="capture-section">
	<h3 class="section-title">Live Capture</h3>
	<p class="section-desc">
		Perform packet capture directly on the network interface. All traffic is analyzed in real-time for assets, connections, and protocol behaviors.
	</p>

	<div class="capture-controls">
		<label class="control-group">
			<span class="control-label">Interface</span>
			<select
				class="control-input select"
				value={selectedInterface}
				onchange={(e) => onInterfaceChange((e.target as HTMLSelectElement).value)}
				disabled={isCapturing}
			>
				<option value="">Select interface...</option>
				{#each interfaces as iface}
					<option value={iface}>{iface}</option>
				{/each}
			</select>
		</label>

		<label class="control-group">
			<span class="control-label">BPF Filter (optional)</span>
			<input
				type="text"
				class="control-input"
				placeholder="e.g., 'not tcp port 22' or 'udp port 502'"
				value={bpfFilter}
				onchange={(e) => onFilterChange((e.target as HTMLInputElement).value)}
				disabled={isCapturing}
			/>
		</label>

		<div class="button-group">
			{#if !isCapturing}
				<button class="action-btn primary" onclick={onStartCapture} disabled={!selectedInterface}>
					Start Capture
				</button>
			{:else}
				<button class="action-btn warning" onclick={onPauseResume}>
					{captureStatus === 'paused' ? 'Resume' : 'Pause'}
				</button>
				<button class="action-btn danger" onclick={onStopCapture}>
					Stop & Save
				</button>
			{/if}
		</div>
	</div>

	{#if captureError}
		<div class="capture-error">{captureError}</div>
	{/if}

	{#if isCapturing && captureStats}
		<div class="capture-stats">
			<div class="stat-row">
				<span class="stat-label">Packets</span>
				<span class="stat-value">{captureStats.packets_captured.toLocaleString()}</span>
			</div>
			<div class="stat-row">
				<span class="stat-label">Rate</span>
				<span class="stat-value">{captureStats.packets_per_second.toLocaleString()} pps</span>
			</div>
			<div class="stat-row">
				<span class="stat-label">Bytes</span>
				<span class="stat-value">{formatBytes(captureStats.bytes_captured)}</span>
			</div>
			<div class="stat-row">
				<span class="stat-label">Assets</span>
				<span class="stat-value">{captureStats.asset_count}</span>
			</div>
			<div class="stat-row">
				<span class="stat-label">Connections</span>
				<span class="stat-value">{captureStats.active_connections}</span>
			</div>
			<div class="stat-row">
				<span class="stat-label">Elapsed</span>
				<span class="stat-value">{formatDuration(captureStats.elapsed_seconds)}</span>
			</div>
		</div>
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

	.capture-controls {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.control-group {
		display: flex;
		flex-direction: column;
		gap: 0.35rem;
	}

	.control-label {
		font-size: 0.8125rem;
		font-weight: 600;
		color: var(--gm-text-secondary);
	}

	.control-input {
		padding: 0.5rem;
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		background: var(--gm-bg-tertiary);
		color: var(--gm-text-primary);
		font-size: 0.8125rem;
		font-family: inherit;
	}

	.control-input:focus {
		outline: none;
		border-color: #6366f1;
	}

	.control-input:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.button-group {
		display: flex;
		gap: 0.5rem;
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

	.action-btn.warning {
		background: #f59e0b;
		color: white;
	}

	.action-btn.warning:hover {
		background: #d97706;
	}

	.action-btn.danger {
		background: #ef4444;
		color: white;
	}

	.action-btn.danger:hover {
		background: #dc2626;
	}

	.action-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.capture-error {
		margin-top: 1rem;
		padding: 0.75rem;
		background: rgba(239, 68, 68, 0.1);
		color: #ef4444;
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 4px;
		font-size: 0.8125rem;
	}

	.capture-stats {
		margin-top: 1rem;
		padding: 1rem;
		background: rgba(99, 102, 241, 0.05);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 4px;
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
		gap: 1rem;
	}

	.stat-row {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.stat-label {
		font-size: 0.75rem;
		color: var(--gm-text-secondary);
		font-weight: 600;
	}

	.stat-value {
		font-size: 1rem;
		color: var(--gm-text-primary);
		font-weight: 600;
		font-family: 'JetBrains Mono', monospace;
	}

	.capture-result {
		margin-top: 1rem;
		padding: 0.75rem;
		border-radius: 4px;
		font-size: 0.8125rem;
	}

	.capture-result.success {
		background: rgba(34, 197, 94, 0.1);
		color: #22c55e;
		border: 1px solid rgba(34, 197, 94, 0.3);
	}

	.result-detail {
		margin-top: 0.5rem;
		font-size: 0.75rem;
		opacity: 0.9;
	}

	.result-detail code {
		background: rgba(0, 0, 0, 0.3);
		padding: 0.2rem 0.4rem;
		border-radius: 2px;
		font-family: 'JetBrains Mono', monospace;
	}
</style>

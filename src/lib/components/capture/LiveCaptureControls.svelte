<script lang="ts">
	import type { NetworkInterface } from '$lib/types/network';

	interface Props {
		selectedInterface: string;
		bpfFilter: string;
		captureStatus: 'idle' | 'capturing' | 'paused' | 'error';
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
		captureStatus,
		interfaces,
		onInterfaceChange,
		onFilterChange,
		onStartCapture,
		onStopCapture,
		onPauseResume
	}: Props = $props();

	const isCapturing = $derived(captureStatus === 'capturing' || captureStatus === 'paused');
</script>

<div class="capture-controls">
	<label class="control-group">
		<span class="control-label">Interface</span>
		<select
			class="control-input select"
			value={selectedInterface}
			onchange={(event) => onInterfaceChange((event.target as HTMLSelectElement).value)}
			disabled={isCapturing}
		>
			<option value="">Select interface...</option>
			{#each interfaces as iface}
				<option value={iface.name}>{iface.name}{iface.description ? ` — ${iface.description}` : ''}</option>
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
			onchange={(event) => onFilterChange((event.target as HTMLInputElement).value)}
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
			<button class="action-btn danger" onclick={onStopCapture}>Stop & Save</button>
		{/if}
	</div>
</div>

<style>
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
		font-family: inherit;
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.action-btn.primary {
		background: #10b981;
		color: #fff;
	}

	.action-btn.primary:hover:not(:disabled) {
		background: #059669;
	}

	.action-btn.warning {
		background: #f59e0b;
		color: #111827;
	}

	.action-btn.warning:hover {
		background: #d97706;
	}

	.action-btn.danger {
		background: #ef4444;
		color: #fff;
	}

	.action-btn.danger:hover {
		background: #dc2626;
	}
</style>

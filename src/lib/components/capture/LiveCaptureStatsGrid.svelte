<script lang="ts">
	import type { CaptureStatsEvent } from '$lib/types/capture';
	import { formatBytes, formatDuration } from './liveCaptureFormatters';

	let { stats }: { stats: CaptureStatsEvent } = $props();
</script>

<div class="capture-stats">
	<div class="stat-row">
		<span class="stat-label">Packets</span>
		<span class="stat-value">{stats.packets_captured.toLocaleString()}</span>
	</div>
	<div class="stat-row">
		<span class="stat-label">Rate</span>
		<span class="stat-value">{stats.packets_per_second.toLocaleString()} pps</span>
	</div>
	<div class="stat-row">
		<span class="stat-label">Bytes</span>
		<span class="stat-value">{formatBytes(stats.bytes_captured)}</span>
	</div>
	<div class="stat-row">
		<span class="stat-label">Assets</span>
		<span class="stat-value">{stats.asset_count}</span>
	</div>
	<div class="stat-row">
		<span class="stat-label">Connections</span>
		<span class="stat-value">{stats.active_connections}</span>
	</div>
	<div class="stat-row">
		<span class="stat-label">Elapsed</span>
		<span class="stat-value">{formatDuration(stats.elapsed_seconds)}</span>
	</div>
</div>

<style>
	.capture-stats {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
		gap: 0.75rem;
		margin-top: 1rem;
		padding: 1rem;
		background: var(--gm-bg-tertiary);
		border: 1px solid var(--gm-border);
		border-radius: 4px;
	}

	.stat-row {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.stat-label {
		font-size: 0.75rem;
		color: var(--gm-text-muted);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.stat-value {
		font-size: 0.9375rem;
		font-weight: 600;
		color: var(--gm-text-primary);
	}
</style>

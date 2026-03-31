<script lang="ts">
	import type { BaselineDiff } from '$lib/types/analysis';

	interface Props {
		diff: BaselineDiff;
	}

	let { diff }: Props = $props();
	let summary = $derived(diff.summary);
</script>

<div class="summary-grid">
	<div class="summary-card">
		<div class="card-label">Baseline Assets</div>
		<div class="card-value">{summary.total_baseline_assets}</div>
	</div>
	<div class="summary-card">
		<div class="card-label">Current Assets</div>
		<div class="card-value">{summary.total_current_assets}</div>
	</div>
	<div class="summary-card card-new">
		<div class="card-label">New Devices</div>
		<div class="card-value">{summary.new_asset_count}</div>
	</div>
	<div class="summary-card card-missing">
		<div class="card-label">Missing Devices</div>
		<div class="card-value">{summary.missing_asset_count}</div>
	</div>
	<div class="summary-card card-changed">
		<div class="card-label">Changed Devices</div>
		<div class="card-value">{summary.changed_asset_count}</div>
	</div>
	<div class="summary-card">
		<div class="card-label">New Connections</div>
		<div class="card-value">{summary.new_connection_count}</div>
	</div>
	<div class="summary-card">
		<div class="card-label">Missing Connections</div>
		<div class="card-value">{summary.missing_connection_count}</div>
	</div>
	<div class="summary-card">
		<div class="card-label">Baseline</div>
		<div class="card-value-small">{diff.baseline_session_name}</div>
	</div>
</div>

<style>
	.summary-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 12px;
		padding: 16px 20px;
		border-bottom: 1px solid var(--gm-border);
		flex-shrink: 0;
	}

	.summary-card {
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 8px;
		padding: 14px;
		text-align: center;
	}

	.summary-card.card-new {
		border-color: #10b981;
	}

	.summary-card.card-missing {
		border-color: #ef4444;
	}

	.summary-card.card-changed {
		border-color: #f59e0b;
	}

	.card-label {
		font-size: 10px;
		color: var(--gm-text-muted);
		text-transform: uppercase;
		letter-spacing: 0.5px;
		margin-bottom: 6px;
	}

	.card-value {
		font-size: 22px;
		font-weight: 700;
		color: var(--gm-text-primary);
	}

	.card-value-small {
		font-size: 11px;
		font-weight: 600;
		color: var(--gm-text-secondary);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}
</style>

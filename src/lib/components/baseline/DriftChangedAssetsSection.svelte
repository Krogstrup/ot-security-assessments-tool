<script lang="ts">
	import type { ChangedAsset } from '$lib/types/analysis';

	interface Props {
		assets: ChangedAsset[];
	}

	let { assets }: Props = $props();
</script>

<div class="drift-section">
	<h3 class="section-title section-changed">Changed Devices ({assets.length})</h3>
	<div class="device-list">
		{#each assets as asset}
			<div class="device-card device-changed">
				<div class="device-ip">{asset.ip_address}</div>
				<div class="change-table">
					{#each asset.changes as change}
						<div class="change-row">
							<span class="change-field">{change.field}</span>
							<span class="change-baseline" title="Baseline value">{change.baseline_value}</span>
							<span class="change-arrow">&rarr;</span>
							<span class="change-current" title="Current value">{change.current_value}</span>
						</div>
					{/each}
				</div>
			</div>
		{/each}
	</div>
</div>

<style>
	.drift-section {
		margin-bottom: 24px;
	}

	.section-title {
		font-size: 13px;
		font-weight: 600;
		margin: 0 0 10px;
		padding-left: 10px;
		border-left: 3px solid var(--gm-border);
	}

	.section-title.section-changed {
		color: #f59e0b;
		border-left-color: #f59e0b;
	}

	.device-list {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.device-card {
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 8px;
		padding: 12px 14px;
	}

	.device-card.device-changed {
		border-left: 3px solid #f59e0b;
	}

	.device-ip {
		font-size: 12px;
		font-weight: 600;
		color: var(--gm-text-primary);
		margin-bottom: 6px;
	}

	.change-table {
		margin-top: 6px;
	}

	.change-row {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 4px 0;
		border-bottom: 1px solid var(--gm-border);
		font-size: 10px;
	}

	.change-row:last-child {
		border-bottom: none;
	}

	.change-field {
		font-weight: 600;
		color: var(--gm-text-secondary);
		min-width: 100px;
		text-transform: capitalize;
	}

	.change-baseline {
		color: #ef4444;
		background: rgba(239, 68, 68, 0.08);
		padding: 1px 6px;
		border-radius: 3px;
		max-width: 200px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.change-arrow {
		color: var(--gm-text-muted);
		flex-shrink: 0;
	}

	.change-current {
		color: #10b981;
		background: rgba(16, 185, 129, 0.08);
		padding: 1px 6px;
		border-radius: 3px;
		max-width: 200px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}
</style>

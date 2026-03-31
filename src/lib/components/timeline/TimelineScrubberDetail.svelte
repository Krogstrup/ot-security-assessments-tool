<script lang="ts">
	interface Props {
		startLabel: string;
		endLabel: string;
		connectionCount: number;
		progressPercent: number;
		enabled: boolean;
		onResetStart: () => void;
		onResetEnd: () => void;
		onRefresh: () => void;
	}

	let {
		startLabel,
		endLabel,
		connectionCount,
		progressPercent,
		enabled,
		onResetStart,
		onResetEnd,
		onRefresh
	}: Props = $props();
</script>

<div class="scrubber-detail">
	<div class="detail-row">
		<span class="detail-label">Range:</span>
		<span class="detail-value">{startLabel} to {endLabel}</span>
	</div>
	<div class="detail-row">
		<span class="detail-label">Connections:</span>
		<span class="detail-value">{connectionCount}</span>
	</div>
	<div class="detail-row">
		<span class="detail-label">Position:</span>
		<span class="detail-value">{progressPercent}%</span>
	</div>
	<div class="detail-actions">
		<button class="detail-btn" onclick={onResetStart} disabled={!enabled} title="Jump to start">
			|&lt;
		</button>
		<button class="detail-btn" onclick={onResetEnd} disabled={!enabled} title="Jump to end (show all)">
			&gt;|
		</button>
		<button class="detail-btn" onclick={onRefresh} title="Refresh timeline range">
			Refresh
		</button>
	</div>
</div>

<style>
	.scrubber-detail {
		padding: 8px 12px 10px;
		border-top: 1px solid var(--gm-border);
		display: flex;
		align-items: center;
		gap: 20px;
		flex-wrap: wrap;
		background: var(--gm-bg-panel);
	}

	.detail-row {
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.detail-label {
		font-size: 9px;
		color: var(--gm-text-muted);
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.3px;
	}

	.detail-value {
		font-size: 10px;
		color: var(--gm-text-secondary);
	}

	.detail-actions {
		display: flex;
		align-items: center;
		gap: 6px;
		margin-left: auto;
	}

	.detail-btn {
		background: var(--gm-bg-secondary);
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		color: var(--gm-text-secondary);
		font-family: inherit;
		font-size: 10px;
		padding: 3px 10px;
		cursor: pointer;
		transition: all 0.15s;
	}

	.detail-btn:hover:not(:disabled) {
		border-color: var(--gm-active);
		color: var(--gm-text-primary);
	}

	.detail-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}
</style>

<script lang="ts">
	import type { DriftSummary } from '$lib/types/analysis';

	interface Props {
		summary: DriftSummary;
	}

	let { summary }: Props = $props();

	function formatDriftScore(score: number): string {
		return Math.round(score * 100).toString();
	}

	function getDriftScoreColor(score: number): string {
		if (score >= 0.5) return '#ef4444';
		if (score >= 0.25) return '#f59e0b';
		return '#10b981';
	}

	function getDriftScoreLabel(score: number): string {
		if (score >= 0.5) return 'High Drift';
		if (score >= 0.25) return 'Moderate Drift';
		if (score > 0) return 'Low Drift';
		return 'No Drift';
	}
</script>

<div class="drift-score-section">
	<div class="drift-score-header">
		<span class="drift-score-label">Drift Score</span>
		<span class="drift-score-value" style="color: {getDriftScoreColor(summary.drift_score)}">
			{formatDriftScore(summary.drift_score)}%
		</span>
		<span class="drift-score-tag" style="background: {getDriftScoreColor(summary.drift_score)}">
			{getDriftScoreLabel(summary.drift_score)}
		</span>
	</div>
	<div class="drift-bar-track">
		<div
			class="drift-bar-fill"
			style="width: {Math.min(summary.drift_score * 100, 100)}%; background: {getDriftScoreColor(summary.drift_score)}"
		></div>
	</div>
	<div class="drift-bar-labels">
		<span>0%</span>
		<span>100%</span>
	</div>
</div>

<style>
	.drift-score-section {
		padding: 16px 20px;
		border-bottom: 1px solid var(--gm-border);
		flex-shrink: 0;
	}

	.drift-score-header {
		display: flex;
		align-items: center;
		gap: 10px;
		margin-bottom: 8px;
	}

	.drift-score-label {
		font-size: 12px;
		font-weight: 600;
		color: var(--gm-text-secondary);
	}

	.drift-score-value {
		font-size: 18px;
		font-weight: 700;
	}

	.drift-score-tag {
		padding: 2px 8px;
		border-radius: 4px;
		font-size: 9px;
		font-weight: 700;
		color: #0a0e17;
		letter-spacing: 0.5px;
		text-transform: uppercase;
	}

	.drift-bar-track {
		height: 10px;
		background: var(--gm-bg-panel);
		border-radius: 5px;
		overflow: hidden;
	}

	.drift-bar-fill {
		height: 100%;
		border-radius: 5px;
		transition: width 0.4s ease;
		min-width: 2px;
	}

	.drift-bar-labels {
		display: flex;
		justify-content: space-between;
		margin-top: 4px;
		font-size: 9px;
		color: var(--gm-text-muted);
	}
</style>

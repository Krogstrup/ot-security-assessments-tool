<script lang="ts">
	interface Props {
		lastRunTime: string | null;
		isRunning: boolean;
		hasAssets: boolean;
		onRunAnalysis: () => Promise<void>;
	}

	let { lastRunTime, isRunning, hasAssets, onRunAnalysis }: Props = $props();
</script>

<div class="analysis-header">
	<div class="header-left">
		<h2 class="header-title">Security Analysis</h2>
		{#if lastRunTime}
			<span class="last-run">Last run: {lastRunTime}</span>
		{/if}
	</div>
	<button class="run-btn" onclick={onRunAnalysis} disabled={isRunning || !hasAssets}>
		{isRunning ? 'Analyzing...' : 'Run Analysis'}
	</button>
</div>

<style>
	.analysis-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 16px 20px;
		border-bottom: 1px solid var(--gm-border);
		flex-shrink: 0;
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: 16px;
	}

	.header-title {
		font-size: 15px;
		font-weight: 600;
		color: var(--gm-text-primary);
		margin: 0;
	}

	.last-run {
		font-size: 10px;
		color: var(--gm-text-muted);
	}
</style>

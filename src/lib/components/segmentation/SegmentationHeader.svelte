<script lang="ts">
	interface Props {
		isRunning: boolean;
		hasReport: boolean;
		onRunSegmentation: () => Promise<void>;
	}

	let { isRunning, hasReport, onRunSegmentation }: Props = $props();
</script>

<div class="view-header">
	<h2>Microsegmentation</h2>
	<p class="subtitle">IEC 62443 Zone/Conduit model · Least-privilege matrix · Policy enforcement</p>
	<button class="run-btn" onclick={onRunSegmentation} disabled={isRunning}>
		{#if isRunning}
			Running analysis…
		{:else}
			{hasReport ? 'Re-run Analysis' : 'Run Segmentation Analysis'}
		{/if}
	</button>
</div>

<style>
	.view-header {
		margin-bottom: 1.5rem;
		flex-shrink: 0;
	}

	.view-header h2 {
		margin: 0 0 0.25rem;
		font-size: 1.4rem;
	}

	.subtitle {
		color: var(--text-muted, #888);
		margin: 0 0 1rem;
		font-size: 0.85rem;
	}

	.run-btn {
		background: var(--accent, #0ea5e9);
		color: white;
		border: none;
		padding: 0.5rem 1.2rem;
		border-radius: 4px;
		cursor: pointer;
		font-size: 0.9rem;
	}

	.run-btn:disabled {
		opacity: 0.6;
		cursor: default;
	}
</style>

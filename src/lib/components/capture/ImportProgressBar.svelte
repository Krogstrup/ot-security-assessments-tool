<script lang="ts">
	import type { ImportProgressEvent } from '$lib/api/capture';

	interface Props {
		progress: ImportProgressEvent | null;
	}

	let { progress }: Props = $props();
</script>

{#if progress}
	<div class="import-progress">
		<div class="progress-bar" style="width: {progress.progress_percent}%"></div>
		<span class="progress-text">
			{progress.current_file} ({progress.progress_percent.toFixed(1)}%) —
			{progress.packets_processed.toLocaleString()} packets
		</span>
	</div>
{/if}

<style>
	.import-progress {
		margin-top: 1rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.3);
		border-radius: 4px;
		overflow: hidden;
	}

	.progress-bar {
		height: 4px;
		background: #6366f1;
		transition: width 0.2s;
	}

	.progress-text {
		display: block;
		padding: 0.5rem;
		font-size: 0.75rem;
		color: var(--gm-text-secondary);
	}
</style>

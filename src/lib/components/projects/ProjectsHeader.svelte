<script lang="ts">
	interface Props {
		onCreate: () => void;
	}

	let { onCreate }: Props = $props();
</script>

<div class="view-header">
	<div class="header-left">
		<h1 class="view-title">Projects</h1>
		<span class="view-subtitle">Select or create an engagement project to get started.</span>
	</div>
	<button class="btn-primary" onclick={onCreate}>+ New Project</button>
</div>

<style>
	.view-header {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 16px;
	}

	.header-left {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.view-title {
		font-size: 22px;
		font-weight: 700;
		color: var(--gm-text-primary);
		margin: 0;
		letter-spacing: 0.5px;
	}

	.view-subtitle {
		font-size: 12px;
		color: var(--gm-text-muted);
	}

	.btn-primary {
		padding: 8px 16px;
		background: #10b981;
		color: #0a0e17;
		border: none;
		border-radius: 5px;
		font-family: inherit;
		font-size: 12px;
		font-weight: 600;
		cursor: pointer;
		transition: background 0.15s;
		white-space: nowrap;
	}

	.btn-primary:hover:not(:disabled) {
		background: #059669;
	}
</style>

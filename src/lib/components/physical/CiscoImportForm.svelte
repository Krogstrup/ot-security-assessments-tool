<script lang="ts">
	interface Props {
		importing: boolean;
		onImport: () => Promise<void>;
	}

	let { importing, onImport }: Props = $props();
</script>

<div class="form-card">
	<h4>Cisco/Config Import</h4>
	<p>Import running configuration (vendor auto-detect).</p>
	<button class="import-btn" onclick={onImport} disabled={importing}>
		{importing ? 'Importing...' : 'Import Config'}
	</button>
</div>

<style>
	.form-card {
		padding: 0.75rem;
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		background: var(--gm-bg-tertiary);
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	h4 {
		margin: 0;
		font-size: 0.82rem;
		color: var(--gm-text-primary);
	}

	p {
		margin: 0;
		font-size: 0.75rem;
		color: var(--gm-text-secondary);
	}

	.import-btn {
		padding: 0.45rem;
		border: none;
		border-radius: 4px;
		background: #6366f1;
		color: #fff;
		font-size: 0.75rem;
		font-weight: 600;
		cursor: pointer;
	}

	.import-btn:disabled {
		opacity: 0.55;
		cursor: not-allowed;
	}
</style>


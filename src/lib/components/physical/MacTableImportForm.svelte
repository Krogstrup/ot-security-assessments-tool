<script lang="ts">
	interface Props {
		importing: boolean;
		switchHostname: string;
		switchOptions: string[];
		onSwitchChange: (hostname: string) => void;
		onImport: () => Promise<void>;
	}

	let { importing, switchHostname, switchOptions, onSwitchChange, onImport }: Props = $props();
</script>

<div class="form-card">
	<h4>MAC Table Import</h4>
	<p>Import CAM/MAC address table for a selected switch.</p>
	<select
		class="import-select"
		value={switchHostname}
		onchange={(e) => onSwitchChange((e.target as HTMLSelectElement).value)}
	>
		<option value="">Select switch...</option>
		{#each switchOptions as hostname}
			<option value={hostname}>{hostname}</option>
		{/each}
	</select>
	<button class="import-btn" onclick={onImport} disabled={importing || !switchHostname.trim()}>
		{importing ? 'Importing...' : 'Import MAC Table'}
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

	.import-select {
		padding: 0.45rem;
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		background: var(--gm-bg-primary);
		color: var(--gm-text-primary);
		font-size: 0.75rem;
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


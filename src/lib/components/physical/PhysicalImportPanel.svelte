<script lang="ts">
	interface Props {
		importType: 'config' | 'mac' | 'cdp' | 'arp';
		switchHostname: string;
		switchOptions: string[];
		importError: string;
		importSuccess: string;
		importing: boolean;
		onTypeChange: (type: 'config' | 'mac' | 'cdp' | 'arp') => void;
		onSwitchChange: (hostname: string) => void;
		onImport: () => Promise<void>;
		onClear: () => Promise<void>;
	}

	let {
		importType,
		switchHostname,
		switchOptions,
		importError,
		importSuccess,
		importing,
		onTypeChange,
		onSwitchChange,
		onImport,
		onClear
	}: Props = $props();
</script>

<div class="import-panel">
	<h3 class="panel-title">Import Network Data</h3>

	<div class="import-form">
		<label class="import-label">
			Import Type:
			<select
				class="import-select"
				value={importType}
				onchange={(e) => onTypeChange(e.currentTarget.value as any)}
			>
				<option value="config">Running Config (auto-detect vendor)</option>
				<option value="mac">MAC Address Table</option>
				<option value="cdp">CDP / LLDP Neighbors</option>
				<option value="arp">ARP Table</option>
			</select>
		</label>

		{#if importType === 'mac' || importType === 'cdp'}
			<label class="import-label">
				Switch:
				{#if switchOptions.length > 0}
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
				{:else}
					<input
						class="import-input"
						type="text"
						placeholder="Import a config first"
						value={switchHostname}
						onchange={(e) => onSwitchChange((e.target as HTMLInputElement).value)}
					/>
				{/if}
			</label>
		{/if}

		<button class="import-btn" onclick={onImport} disabled={importing}>
			{importing ? 'Importing...' : 'Import File'}
		</button>
		<button class="import-btn secondary" onclick={onClear} disabled={importing}>
			Clear Topology
		</button>
	</div>

	{#if importError}
		<div class="msg error">{importError}</div>
	{/if}
	{#if importSuccess}
		<div class="msg success">{importSuccess}</div>
	{/if}

	<div class="import-help">
		<h4>Import Order</h4>
		<ol>
			<li><strong>Running Config</strong> — creates the switch &amp; ports</li>
			<li><strong>MAC Address Table</strong> — maps MACs to ports</li>
			<li><strong>ARP Table</strong> — maps IPs to MACs</li>
			<li><strong>CDP Neighbors</strong> — discovers switch links</li>
		</ol>
	</div>
</div>

<style>
	.import-panel {
		flex: 0 0 280px;
		background: var(--gm-bg-secondary);
		border-right: 1px solid var(--gm-border);
		padding: 1rem;
		overflow-y: auto;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.panel-title {
		margin: 0;
		font-size: 0.95rem;
		color: var(--gm-text-primary);
		font-weight: 600;
	}

	.import-form {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.import-label {
		display: flex;
		flex-direction: column;
		gap: 0.35rem;
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--gm-text-secondary);
	}

	.import-select,
	.import-input {
		padding: 0.5rem;
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		background: var(--gm-bg-tertiary);
		color: var(--gm-text-primary);
		font-size: 0.8125rem;
	}

	.import-select:focus,
	.import-input:focus {
		outline: none;
		border-color: #6366f1;
	}

	.import-btn {
		padding: 0.5rem;
		border: none;
		border-radius: 4px;
		background: #6366f1;
		color: white;
		font-size: 0.8125rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.import-btn:hover:not(:disabled) {
		background: #4f46e5;
	}

	.import-btn.secondary {
		background: var(--gm-bg-tertiary);
		color: var(--gm-text-primary);
		border: 1px solid var(--gm-border);
	}

	.import-btn.secondary:hover:not(:disabled) {
		background: rgba(255, 255, 255, 0.05);
	}

	.import-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.msg {
		padding: 0.75rem;
		border-radius: 4px;
		font-size: 0.8125rem;
	}

	.msg.error {
		background: rgba(239, 68, 68, 0.1);
		color: #ef4444;
		border: 1px solid rgba(239, 68, 68, 0.3);
	}

	.msg.success {
		background: rgba(34, 197, 94, 0.1);
		color: #22c55e;
		border: 1px solid rgba(34, 197, 94, 0.3);
	}

	.import-help {
		padding: 0.75rem;
		background: rgba(59, 130, 246, 0.05);
		border: 1px solid rgba(59, 130, 246, 0.2);
		border-radius: 4px;
		font-size: 0.75rem;
	}

	.import-help h4 {
		margin: 0 0 0.5rem 0;
		color: var(--gm-text-primary);
		font-weight: 600;
	}

	.import-help ol {
		margin: 0;
		padding-left: 1.25rem;
		color: var(--gm-text-secondary);
	}

	.import-help li {
		margin-bottom: 0.35rem;
	}
</style>

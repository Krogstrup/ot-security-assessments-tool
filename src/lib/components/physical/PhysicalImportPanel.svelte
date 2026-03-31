<script lang="ts">
	import CiscoImportForm from './CiscoImportForm.svelte';
	import MacTableImportForm from './MacTableImportForm.svelte';
	import CdpImportForm from './CdpImportForm.svelte';
	import ArpImportForm from './ArpImportForm.svelte';

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

	async function runImport(type: 'config' | 'mac' | 'cdp' | 'arp') {
		onTypeChange(type);
		await onImport();
	}
</script>

<div class="import-panel">
	<h3 class="panel-title">Import Network Data</h3>

	<div class="forms-grid">
		<CiscoImportForm importing={importing} onImport={() => runImport('config')} />
		<MacTableImportForm
			importing={importing}
			{switchHostname}
			{switchOptions}
			onSwitchChange={onSwitchChange}
			onImport={() => runImport('mac')}
		/>
		<CdpImportForm
			importing={importing}
			{switchHostname}
			{switchOptions}
			onSwitchChange={onSwitchChange}
			onImport={() => runImport('cdp')}
		/>
		<ArpImportForm importing={importing} onImport={() => runImport('arp')} />
	</div>

	<button class="clear-btn" onclick={onClear} disabled={importing}>Clear Topology</button>

	{#if importError}
		<div class="msg error">{importError}</div>
	{/if}
	{#if importSuccess}
		<div class="msg success">{importSuccess}</div>
	{/if}

	<div class="import-help">
		<h4>Import Order</h4>
		<ol>
			<li><strong>Running Config</strong> — creates switches and ports</li>
			<li><strong>MAC Address Table</strong> — maps MACs to ports</li>
			<li><strong>ARP Table</strong> — maps IPs to MACs</li>
			<li><strong>CDP/LLDP</strong> — discovers switch links</li>
		</ol>
	</div>
</div>

<style>
	.import-panel {
		flex: 0 0 300px;
		background: var(--gm-bg-secondary);
		border-right: 1px solid var(--gm-border);
		padding: 1rem;
		overflow-y: auto;
		display: flex;
		flex-direction: column;
		gap: 0.9rem;
	}

	.panel-title {
		margin: 0;
		font-size: 0.95rem;
		color: var(--gm-text-primary);
		font-weight: 600;
	}

	.forms-grid {
		display: flex;
		flex-direction: column;
		gap: 0.65rem;
	}

	.clear-btn {
		padding: 0.5rem;
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		background: var(--gm-bg-tertiary);
		color: var(--gm-text-primary);
		font-size: 0.8rem;
		font-weight: 600;
		cursor: pointer;
	}

	.clear-btn:disabled {
		opacity: 0.55;
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

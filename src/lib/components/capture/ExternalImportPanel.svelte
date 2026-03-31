<script lang="ts">
	import type { IngestImportResult } from '$lib/types';

	interface Props {
		ingestStatus: 'idle' | 'importing' | 'done' | 'error';
		ingestMessage: string;
		lastIngestResult: IngestImportResult | null;
		onImportZeek: () => Promise<void>;
		onImportSuricata: () => Promise<void>;
		onImportNmap: () => Promise<void>;
		onImportMasscan: () => Promise<void>;
		onImportWazuh: () => Promise<void>;
		onImportSinema: () => Promise<void>;
		onImportTia: () => Promise<void>;
	}

	let {
		ingestStatus,
		ingestMessage,
		lastIngestResult,
		onImportZeek,
		onImportSuricata,
		onImportNmap,
		onImportMasscan,
		onImportWazuh,
		onImportSinema,
		onImportTia
	}: Props = $props();

	const isImporting = $derived(ingestStatus === 'importing');
</script>

<section class="capture-section">
	<h3 class="section-title">External Tool Import</h3>
	<p class="section-desc">
		Import network intelligence from IDS, SIEM, passive discovery, and device configuration tools.
		These enhance asset identification and provide behavioral context.
	</p>

	<div class="import-grid">
		<button class="import-tool-btn" onclick={onImportZeek} disabled={isImporting}>
			<div class="tool-name">Zeek Logs</div>
			<div class="tool-desc">conn, modbus, dnp3 logs</div>
		</button>
		<button class="import-tool-btn" onclick={onImportSuricata} disabled={isImporting}>
			<div class="tool-name">Suricata</div>
			<div class="tool-desc">eve.json alert logs</div>
		</button>
		<button class="import-tool-btn" onclick={onImportNmap} disabled={isImporting}>
			<div class="tool-name">Nmap</div>
			<div class="tool-desc">XML scan results (active)</div>
		</button>
		<button class="import-tool-btn" onclick={onImportMasscan} disabled={isImporting}>
			<div class="tool-name">Masscan</div>
			<div class="tool-desc">JSON scan results (active)</div>
		</button>
		<button class="import-tool-btn" onclick={onImportWazuh} disabled={isImporting}>
			<div class="tool-name">Wazuh</div>
			<div class="tool-desc">Alert export JSON</div>
		</button>
		<button class="import-tool-btn" onclick={onImportSinema} disabled={isImporting}>
			<div class="tool-name">SINEMA Server</div>
			<div class="tool-desc">Device CSV export</div>
		</button>
		<button class="import-tool-btn" onclick={onImportTia} disabled={isImporting}>
			<div class="tool-name">TIA Portal</div>
			<div class="tool-desc">Project XML export</div>
		</button>
	</div>

	{#if ingestStatus === 'error'}
		<div class="import-result error">{ingestMessage}</div>
	{:else if ingestStatus === 'done'}
		<div class="import-result success">{ingestMessage}</div>
		{#if lastIngestResult}
			<div class="result-summary">
				<div class="result-row">
					<span>New Assets</span>
					<span class="result-value">{lastIngestResult.new_assets || '—'}</span>
				</div>
				<div class="result-row">
					<span>Updated Assets</span>
					<span class="result-value">{lastIngestResult.updated_assets || '—'}</span>
				</div>
				{#if lastIngestResult.connection_count}
					<div class="result-row">
						<span>Connections</span>
						<span class="result-value">{lastIngestResult.connection_count}</span>
					</div>
				{/if}
				{#if lastIngestResult.alert_count}
					<div class="result-row">
						<span>Alerts</span>
						<span class="result-value">{lastIngestResult.alert_count}</span>
					</div>
				{/if}
			</div>
		{/if}
	{:else if ingestMessage}
		<div class="import-result info">{ingestMessage}</div>
	{/if}
</section>

<style>
	.capture-section {
		padding: 1.5rem;
		background: var(--gm-bg-secondary);
		border: 1px solid var(--gm-border);
		border-radius: 6px;
		margin-bottom: 1rem;
	}

	.section-title {
		font-size: 1.125rem;
		font-weight: 600;
		margin-bottom: 0.5rem;
		color: var(--gm-text-primary);
	}

	.section-desc {
		font-size: 0.8125rem;
		color: var(--gm-text-secondary);
		margin-bottom: 1rem;
	}

	.import-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
		gap: 0.75rem;
		margin-bottom: 1rem;
	}

	.import-tool-btn {
		padding: 0.75rem;
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		background: var(--gm-bg-tertiary);
		color: var(--gm-text-primary);
		cursor: pointer;
		transition: all 0.2s;
		text-align: center;
		font-size: 0.8125rem;
		font-weight: 600;
		display: flex;
		flex-direction: column;
		gap: 0.35rem;
		align-items: center;
	}

	.import-tool-btn:hover:not(:disabled) {
		background: rgba(99, 102, 241, 0.2);
		border-color: #6366f1;
	}

	.import-tool-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.tool-name {
		font-weight: 600;
		font-size: 0.8125rem;
	}

	.tool-desc {
		font-size: 0.7rem;
		color: var(--gm-text-secondary);
	}

	.import-result {
		padding: 0.75rem;
		border-radius: 4px;
		font-size: 0.8125rem;
		margin-bottom: 0.75rem;
	}

	.import-result.success {
		background: rgba(34, 197, 94, 0.1);
		color: #22c55e;
		border: 1px solid rgba(34, 197, 94, 0.3);
	}

	.import-result.error {
		background: rgba(239, 68, 68, 0.1);
		color: #ef4444;
		border: 1px solid rgba(239, 68, 68, 0.3);
	}

	.import-result.info {
		background: rgba(99, 102, 241, 0.1);
		color: #6366f1;
		border: 1px solid rgba(99, 102, 241, 0.3);
	}

	.result-summary {
		padding: 1rem;
		background: rgba(0, 0, 0, 0.2);
		border-radius: 4px;
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
		gap: 1rem;
	}

	.result-row {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.result-row span:first-child {
		font-size: 0.75rem;
		color: var(--gm-text-secondary);
		font-weight: 600;
	}

	.result-value {
		font-size: 1rem;
		font-weight: 600;
		color: var(--gm-text-primary);
	}
</style>

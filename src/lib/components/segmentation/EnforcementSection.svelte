<script lang="ts">
	import type { EnforcementFormat, SegmentationReport } from '$lib/types/segmentation';

	interface Props {
		report: SegmentationReport;
		selectedFormat: EnforcementFormat;
		exportedContent: string;
		isExporting: boolean;
		onFormatChange: (format: EnforcementFormat) => void;
		onExport: () => void;
		onCopy: (text: string) => void;
	}

	let {
		report,
		selectedFormat,
		exportedContent,
		isExporting,
		onFormatChange,
		onExport,
		onCopy
	}: Props = $props();

	function handleFormatChange(event: Event) {
		onFormatChange((event.target as HTMLSelectElement).value as EnforcementFormat);
	}
</script>

<div class="tab-content">
	<div class="enforcement-controls">
		<label for="fmt-select">Format:</label>
		<select id="fmt-select" value={selectedFormat} onchange={handleFormatChange}>
			<option value="cisco_ios_acl">Cisco IOS ACL</option>
			<option value="cisco_asa_acl">Cisco ASA ACL</option>
			<option value="generic_firewall_table">Generic Firewall Table (TSV)</option>
			<option value="suricata_rules">Suricata Rules</option>
			<option value="json_policy">JSON Policy</option>
		</select>
		<button onclick={onExport} disabled={isExporting}>
			{isExporting ? 'Exporting…' : 'Export'}
		</button>
		{#if exportedContent}
			<button onclick={() => onCopy(exportedContent)}>Copy</button>
		{/if}
	</div>
	<div class="enforcement-summary">
		{#each report.enforcement_configs as cfg}
			<div class="cfg-chip" class:active={cfg.format === selectedFormat}>
				<span>{cfg.format.replace(/_/g, ' ')}</span>
				<span class="rule-count">{cfg.rule_count.toLocaleString()} rules</span>
			</div>
		{/each}
	</div>
	{#if exportedContent}
		<pre class="config-output">{exportedContent}</pre>
	{/if}
</div>

<style>
	.tab-content {
		flex: 1;
		overflow-y: auto;
		min-height: 0;
		animation: fadeIn 0.15s ease;
	}

	@keyframes fadeIn {
		from { opacity: 0; }
		to { opacity: 1; }
	}

	.enforcement-controls {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 1rem;
	}

	.enforcement-controls select {
		background: var(--surface-2, #1e1e1e);
		border: 1px solid var(--border, #333);
		color: var(--text, #eee);
		padding: 0.4rem 0.6rem;
		border-radius: 4px;
		font-size: 0.9rem;
	}

	.enforcement-controls button {
		background: var(--surface-2, #1e1e1e);
		border: 1px solid var(--border, #333);
		color: var(--text, #eee);
		padding: 0.4rem 0.8rem;
		border-radius: 4px;
		cursor: pointer;
		font-size: 0.9rem;
	}

	.enforcement-controls button:disabled {
		opacity: 0.6;
		cursor: default;
	}

	.enforcement-summary {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
		margin-bottom: 1rem;
	}

	.cfg-chip {
		background: var(--surface-2, #1e1e1e);
		border: 1px solid var(--border, #333);
		border-radius: 4px;
		padding: 0.3rem 0.6rem;
		font-size: 0.8rem;
		display: flex;
		gap: 0.5rem;
		align-items: center;
	}

	.cfg-chip.active {
		border-color: var(--accent, #0ea5e9);
		color: var(--accent, #0ea5e9);
	}

	.config-output {
		background: var(--surface-2, #1e1e1e);
		border: 1px solid var(--border, #333);
		border-radius: 4px;
		padding: 1rem;
		font-family: monospace;
		font-size: 0.8rem;
		overflow-x: auto;
		max-height: 500px;
		overflow-y: auto;
		white-space: pre;
	}
</style>

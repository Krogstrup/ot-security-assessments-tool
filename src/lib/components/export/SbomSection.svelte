<script lang="ts">
	type SbomFormat = 'csv' | 'json';

	interface Props {
		hasData: boolean;
		busyAction: string | null;
		sbomFormat: SbomFormat;
		onFormatChange: (format: SbomFormat) => void;
		onExportSbom: () => void;
	}

	let { hasData, busyAction, sbomFormat, onFormatChange, onExportSbom }: Props = $props();

	function handleFormatChange(event: Event) {
		onFormatChange((event.target as HTMLInputElement).value as SbomFormat);
	}
</script>

<section class="export-section">
	<h3 class="section-title">SBOM (CISA BOD 23-01)</h3>
	<p class="section-desc">Export a Software Bill of Materials listing all discovered OT/IT assets with vendor, firmware, and Purdue zone classification.</p>

	<div class="format-selector">
		<label class="radio-row">
			<input
				type="radio"
				name="sbom-format"
				value="json"
				checked={sbomFormat === 'json'}
				onchange={handleFormatChange}
			/>
			<span>JSON</span>
		</label>
		<label class="radio-row">
			<input
				type="radio"
				name="sbom-format"
				value="csv"
				checked={sbomFormat === 'csv'}
				onchange={handleFormatChange}
			/>
			<span>CSV</span>
		</label>
	</div>

	<button
		class="action-btn primary"
		disabled={!hasData || busyAction !== null}
		onclick={onExportSbom}
	>
		{busyAction === 'sbom' ? 'Exporting...' : `Export SBOM (${sbomFormat.toUpperCase()})`}
	</button>
</section>

<style>
	.export-section {
		background: var(--gm-bg-secondary);
		border: 1px solid var(--gm-border);
		border-radius: 8px;
		padding: 20px;
	}

	.section-title {
		font-size: 13px;
		font-weight: 600;
		color: var(--gm-text-primary);
		margin: 0 0 4px 0;
		letter-spacing: 0.5px;
	}

	.section-desc {
		font-size: 11px;
		color: var(--gm-text-muted);
		margin: 0 0 16px 0;
		line-height: 1.5;
	}

	.action-btn {
		padding: 8px 16px;
		border-radius: 6px;
		font-family: inherit;
		font-size: 11px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.15s;
		border: 1px solid transparent;
	}

	.action-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.action-btn.primary {
		background: rgba(59, 130, 246, 0.1);
		border-color: rgba(59, 130, 246, 0.2);
		color: #3b82f6;
	}

	.action-btn.primary:hover:not(:disabled) {
		background: rgba(59, 130, 246, 0.2);
	}

	.format-selector {
		display: flex;
		gap: 16px;
		margin-bottom: 12px;
	}

	.radio-row {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 11px;
		color: var(--gm-text-secondary);
		cursor: pointer;
	}

	.radio-row input[type='radio'] {
		accent-color: #10b981;
	}
</style>

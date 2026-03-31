<script lang="ts">
	interface Props {
		hasData: boolean;
		busyAction: string | null;
		assessorName: string;
		clientName: string;
		assessmentDate: string;
		reportTitle: string;
		includeExecSummary: boolean;
		includeAssetInventory: boolean;
		includeProtocolAnalysis: boolean;
		includeFindings: boolean;
		includeRecommendations: boolean;
		onGeneratePdf: () => Promise<void>;
	}

	let {
		hasData,
		busyAction,
		assessorName = $bindable(),
		clientName = $bindable(),
		assessmentDate = $bindable(),
		reportTitle = $bindable(),
		includeExecSummary = $bindable(),
		includeAssetInventory = $bindable(),
		includeProtocolAnalysis = $bindable(),
		includeFindings = $bindable(),
		includeRecommendations = $bindable(),
		onGeneratePdf
	}: Props = $props();
</script>

<section class="export-section">
	<h3 class="section-title">PDF Assessment Report</h3>
	<p class="section-desc">Generate a professional ICS/SCADA assessment report with executive summary, asset inventory, protocol analysis, and findings.</p>

	<div class="form-grid">
		<div class="form-row">
			<label class="form-label" for="assessor">Assessor Name <span class="required">*</span></label>
			<input type="text" id="assessor" class="form-input" placeholder="e.g., Jane Smith" bind:value={assessorName} />
		</div>
		<div class="form-row">
			<label class="form-label" for="client">Client Name <span class="required">*</span></label>
			<input type="text" id="client" class="form-input" placeholder="e.g., ACME Power Plant" bind:value={clientName} />
		</div>
		<div class="form-row">
			<label class="form-label" for="date">Assessment Date</label>
			<input type="date" id="date" class="form-input" bind:value={assessmentDate} />
		</div>
		<div class="form-row">
			<label class="form-label" for="title">Report Title</label>
			<input type="text" id="title" class="form-input" placeholder="ICS Network Assessment Report" bind:value={reportTitle} />
		</div>
	</div>

	<div class="checkbox-group">
		<h4 class="checkbox-title">Include Sections</h4>
		<label class="checkbox-row">
			<input type="checkbox" bind:checked={includeExecSummary} />
			<span>Executive Summary</span>
		</label>
		<label class="checkbox-row">
			<input type="checkbox" bind:checked={includeAssetInventory} />
			<span>Asset Inventory</span>
		</label>
		<label class="checkbox-row">
			<input type="checkbox" bind:checked={includeProtocolAnalysis} />
			<span>Protocol Analysis</span>
		</label>
		<label class="checkbox-row">
			<input type="checkbox" bind:checked={includeFindings} />
			<span>Findings</span>
		</label>
		<label class="checkbox-row">
			<input type="checkbox" bind:checked={includeRecommendations} />
			<span>Recommendations</span>
		</label>
	</div>

	<button class="action-btn accent" disabled={!hasData || busyAction !== null} onclick={onGeneratePdf}>
		{busyAction === 'pdf' ? 'Generating PDF...' : 'Generate PDF Report'}
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

	.form-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 12px;
		margin-bottom: 16px;
	}

	.form-row {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.form-label {
		font-size: 11px;
		color: var(--gm-text-secondary);
		font-weight: 500;
	}

	.required {
		color: #ef4444;
	}

	.form-input {
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		padding: 7px 10px;
		color: var(--gm-text-primary);
		font-family: inherit;
		font-size: 11px;
		outline: none;
	}

	.form-input:focus {
		border-color: var(--gm-border-active);
	}

	.checkbox-group {
		margin-bottom: 16px;
	}

	.checkbox-title {
		font-size: 11px;
		font-weight: 600;
		color: var(--gm-text-secondary);
		margin: 0 0 8px 0;
	}

	.checkbox-row {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 4px 0;
		font-size: 11px;
		color: var(--gm-text-secondary);
		cursor: pointer;
	}

	.checkbox-row input[type='checkbox'] {
		accent-color: #10b981;
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

	.action-btn.accent {
		background: rgba(16, 185, 129, 0.15);
		border-color: rgba(16, 185, 129, 0.3);
		color: #10b981;
		font-size: 12px;
		padding: 10px 20px;
	}

	.action-btn.accent:hover:not(:disabled) {
		background: rgba(16, 185, 129, 0.25);
	}
</style>

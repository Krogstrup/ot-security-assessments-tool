<script lang="ts">
	export type AnalysisSection =
		| 'findings'
		| 'purdue'
		| 'anomalies'
		| 'summary'
		| 'drift'
		| 'switch_security'
		| 'external_alerts'
		| 'malware'
		| 'compliance';

	interface Props {
		activeSection: AnalysisSection;
		findingsCount: number;
		anomaliesCount: number;
		switchFindingsCount: number;
		correlatedAlertsCount: number;
		malwareFindingsCount: number;
		onSectionChange: (section: AnalysisSection) => void;
	}

	let {
		activeSection,
		findingsCount,
		anomaliesCount,
		switchFindingsCount,
		correlatedAlertsCount,
		malwareFindingsCount,
		onSectionChange
	}: Props = $props();
</script>

<div class="section-tabs">
	<button class="section-tab" class:active={activeSection === 'summary'} onclick={() => onSectionChange('summary')}>
		Summary
	</button>
	<button class="section-tab" class:active={activeSection === 'findings'} onclick={() => onSectionChange('findings')}>
		Findings
		{#if findingsCount > 0}
			<span class="tab-badge">{findingsCount}</span>
		{/if}
	</button>
	<button class="section-tab" class:active={activeSection === 'purdue'} onclick={() => onSectionChange('purdue')}>
		Purdue Model
	</button>
	<button class="section-tab" class:active={activeSection === 'anomalies'} onclick={() => onSectionChange('anomalies')}>
		Anomalies
		{#if anomaliesCount > 0}
			<span class="tab-badge">{anomaliesCount}</span>
		{/if}
	</button>
	<button class="section-tab" class:active={activeSection === 'drift'} onclick={() => onSectionChange('drift')}>
		Baseline Drift
	</button>
	<button class="section-tab" class:active={activeSection === 'switch_security'} onclick={() => onSectionChange('switch_security')}>
		Switch Security
		{#if switchFindingsCount > 0}
			<span class="tab-badge">{switchFindingsCount}</span>
		{/if}
	</button>
	<button class="section-tab" class:active={activeSection === 'external_alerts'} onclick={() => onSectionChange('external_alerts')}>
		Ext. Alerts
		{#if correlatedAlertsCount > 0}
			<span class="tab-badge">{correlatedAlertsCount}</span>
		{/if}
	</button>
	<button class="section-tab" class:active={activeSection === 'malware'} onclick={() => onSectionChange('malware')}>
		Malware
		{#if malwareFindingsCount > 0}
			<span class="tab-badge tab-badge-critical">{malwareFindingsCount}</span>
		{/if}
	</button>
	<button class="section-tab" class:active={activeSection === 'compliance'} onclick={() => onSectionChange('compliance')}>
		Compliance
	</button>
</div>

<style>
	.section-tabs {
		display: flex;
		border-bottom: 1px solid var(--gm-border);
		flex-shrink: 0;
		background: var(--gm-bg-secondary);
	}

	.section-tab {
		padding: 10px 18px;
		background: transparent;
		border: none;
		border-bottom: 2px solid transparent;
		color: var(--gm-text-muted);
		font-family: inherit;
		font-size: 11px;
		font-weight: 500;
		cursor: pointer;
		display: flex;
		align-items: center;
		gap: 6px;
		transition: all 0.15s;
	}

	.section-tab:hover {
		color: var(--gm-text-secondary);
	}

	.section-tab.active {
		color: var(--gm-text-primary);
		border-bottom-color: #10b981;
	}

	.tab-badge {
		background: rgba(239, 68, 68, 0.2);
		color: #ef4444;
		padding: 1px 6px;
		border-radius: 8px;
		font-size: 9px;
		font-weight: 700;
	}

	.tab-badge-critical {
		background: rgba(239, 68, 68, 0.25);
		color: #ef4444;
	}
</style>

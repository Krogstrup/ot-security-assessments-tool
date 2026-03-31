<script lang="ts">
	import type { ComplianceMapping, ComplianceStatus } from '$lib/types/analysis';

	type ComplianceFramework = 'iec62443' | 'nist80082' | 'nerccip';

	interface Props {
		framework: ComplianceFramework;
		mappings: ComplianceMapping[];
		loading: boolean;
		onFrameworkChange: (framework: ComplianceFramework) => void;
		onRefresh: () => void;
	}

	let { framework, mappings, loading, onFrameworkChange, onRefresh }: Props = $props();

	function complianceStatusClass(status: ComplianceStatus): string {
		if (status === 'gap') return 'status-gap';
		if (status === 'partial') return 'status-partial';
		if (status === 'met') return 'status-met';
		return 'status-na';
	}

	function complianceStatusLabel(status: ComplianceStatus): string {
		if (status === 'gap') return 'GAP';
		if (status === 'partial') return 'PARTIAL';
		if (status === 'met') return 'MET';
		return 'N/A';
	}

	function handleFrameworkChange(event: Event) {
		const value = (event.target as HTMLSelectElement).value as ComplianceFramework;
		onFrameworkChange(value);
	}
</script>

<div class="compliance-header">
	<span class="ext-alerts-count">Compliance Mapping</span>
	<select class="framework-select" value={framework} onchange={handleFrameworkChange}>
		<option value="iec62443">IEC 62443</option>
		<option value="nist80082">NIST 800-82 Rev 3</option>
		<option value="nerccip">NERC CIP</option>
	</select>
	<button class="run-btn" onclick={onRefresh} disabled={loading}>
		{loading ? 'Loading...' : 'Refresh'}
	</button>
</div>

{#if loading}
	<div class="empty-panel"><p>Generating compliance report...</p></div>
{:else if mappings.length === 0}
	<div class="empty-panel">
		<div class="empty-icon">&#x1F4CB;</div>
		<p>Select a framework above to generate a compliance mapping.</p>
		<p class="empty-sub">Run analysis first for best results.</p>
	</div>
{:else}
	<div class="compliance-summary">
		<span class="cs-gap">&#x274C; {mappings.filter((m) => m.status === 'gap').length} Gap</span>
		<span class="cs-partial">&#x26A0; {mappings.filter((m) => m.status === 'partial').length} Partial</span>
		<span class="cs-met">&#x2705; {mappings.filter((m) => m.status === 'met').length} Met</span>
		<span class="cs-na">&#x2B1C; {mappings.filter((m) => m.status === 'not_assessed').length} N/A</span>
	</div>
	<div class="compliance-list">
		{#each mappings as mapping}
			<div class="compliance-row cs-row-{mapping.status}">
				<div class="compliance-req-header">
					<span class="req-id">{mapping.requirement_id}</span>
					<span class="req-name">{mapping.requirement_name}</span>
					<span class="status-badge {complianceStatusClass(mapping.status)}">
						{complianceStatusLabel(mapping.status)}
					</span>
				</div>
				<div class="compliance-evidence">{mapping.evidence}</div>
			</div>
		{/each}
	</div>
{/if}

<style>
	.run-btn {
		padding: 8px 20px;
		background: linear-gradient(135deg, #10b981, #059669);
		border: none;
		border-radius: 6px;
		color: #0a0e17;
		font-family: inherit;
		font-size: 12px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.15s;
	}

	.run-btn:hover:not(:disabled) {
		filter: brightness(1.1);
	}

	.run-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.empty-panel {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 40px;
		text-align: center;
		color: var(--gm-text-muted);
		gap: 10px;
	}

	.empty-icon {
		font-size: 36px;
		opacity: 0.4;
	}

	.empty-sub {
		font-size: 10px;
		color: var(--gm-text-muted);
		margin-top: 4px;
	}

	.ext-alerts-count {
		font-size: 11px;
		color: var(--gm-text-muted);
		flex: 1;
	}

	.compliance-header {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 8px 0;
		flex-wrap: wrap;
	}

	.framework-select {
		background: var(--gm-bg-secondary);
		border: 1px solid var(--gm-border);
		color: var(--gm-text-primary);
		border-radius: 4px;
		padding: 4px 8px;
		font-size: 12px;
		cursor: pointer;
	}

	.compliance-summary {
		display: flex;
		gap: 16px;
		padding: 8px 0;
		font-size: 12px;
		font-weight: 600;
		flex-wrap: wrap;
	}

	.cs-gap { color: #ef4444; }
	.cs-partial { color: #f59e0b; }
	.cs-met { color: #10b981; }
	.cs-na { color: var(--gm-text-muted); }

	.compliance-list {
		display: flex;
		flex-direction: column;
		gap: 6px;
		overflow-y: auto;
		padding-bottom: 8px;
	}

	.compliance-row {
		padding: 10px 12px;
		border-radius: 6px;
		border-left: 3px solid transparent;
		background: var(--gm-bg-secondary);
	}

	.cs-row-gap { border-left-color: #ef4444; }
	.cs-row-partial { border-left-color: #f59e0b; }
	.cs-row-met { border-left-color: #10b981; }
	.cs-row-not_assessed { border-left-color: var(--gm-border); }

	.compliance-req-header {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-bottom: 4px;
		flex-wrap: wrap;
	}

	.req-id {
		font-family: monospace;
		font-size: 11px;
		background: var(--gm-bg-tertiary);
		padding: 1px 6px;
		border-radius: 4px;
		white-space: nowrap;
		color: var(--gm-text-muted);
	}

	.req-name {
		font-weight: 600;
		font-size: 12px;
		flex: 1;
	}

	.status-badge {
		display: inline-block;
		padding: 1px 8px;
		border-radius: 4px;
		font-size: 10px;
		font-weight: 700;
		letter-spacing: 0.05em;
		text-transform: uppercase;
		white-space: nowrap;
	}

	.status-gap { background: rgba(239,68,68,0.15); color: #ef4444; }
	.status-partial { background: rgba(245,158,11,0.15); color: #f59e0b; }
	.status-met { background: rgba(16,185,129,0.15); color: #10b981; }
	.status-na { background: rgba(100,116,139,0.15); color: #64748b; }

	.compliance-evidence {
		font-size: 11px;
		color: var(--gm-text-secondary);
		line-height: 1.5;
	}
</style>

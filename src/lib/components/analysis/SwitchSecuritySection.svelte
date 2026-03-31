<script lang="ts">
	import type { SwitchSecurityFinding } from '$lib/types/deep-parse';

	interface Props {
		loading: boolean;
		findings: SwitchSecurityFinding[];
		onRefresh: () => void;
	}

	let { loading, findings, onRefresh }: Props = $props();
</script>

{#if loading}
	<div class="empty-panel">
		<div class="empty-icon">&#x23F3;</div>
		<p>Assessing switch security...</p>
	</div>
{:else if findings.length === 0}
	<div class="empty-panel">
		<div class="empty-icon">&#x1F512;</div>
		<p>No switch security findings. Import a PCAP with managed switch traffic, LLDP frames, or ring redundancy protocols, then click Refresh.</p>
		<button class="run-btn" onclick={onRefresh}>Refresh</button>
	</div>
{:else}
	<div class="switch-findings-list">
		{#each findings as sf}
			<div class="switch-finding sev-{sf.severity}">
				<div class="sf-header">
					<span class="sf-severity sev-badge-{sf.severity}">{sf.severity.toUpperCase()}</span>
					<span class="sf-title">{sf.title}</span>
				</div>
				<p class="sf-description">{sf.description}</p>
				{#if sf.affected_assets.length > 0}
					<div class="sf-detail">
						<span class="sf-label">Affected</span>
						<span class="sf-value">{sf.affected_assets.join(', ')}</span>
					</div>
				{/if}
				<div class="sf-detail">
					<span class="sf-label">Evidence</span>
					<span class="sf-value">{sf.evidence}</span>
				</div>
				<div class="sf-remediation">
					<span class="sf-label">&#x1F527; Remediation</span>
					<p class="sf-rem-text">{sf.remediation}</p>
				</div>
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

	.switch-findings-list {
		display: flex;
		flex-direction: column;
		gap: 12px;
		padding: 4px;
	}

	.switch-finding {
		background: var(--gm-bg-secondary);
		border: 1px solid var(--gm-border);
		border-left: 4px solid var(--gm-border);
		border-radius: 6px;
		padding: 12px 14px;
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.switch-finding.sev-critical { border-left-color: var(--gm-severity-critical); }
	.switch-finding.sev-high { border-left-color: var(--gm-severity-high); }
	.switch-finding.sev-medium { border-left-color: var(--gm-severity-medium); }
	.switch-finding.sev-low { border-left-color: var(--gm-severity-low); }

	.sf-header {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.sf-severity {
		font-size: 9px;
		font-weight: 700;
		padding: 2px 6px;
		border-radius: 3px;
		letter-spacing: 0.5px;
		flex-shrink: 0;
	}

	.sev-badge-critical { background: var(--gm-severity-critical); color: #fff; }
	.sev-badge-high { background: var(--gm-severity-high); color: #fff; }
	.sev-badge-medium { background: var(--gm-severity-medium); color: #fff; }
	.sev-badge-low { background: var(--gm-severity-low); color: #fff; }

	.sf-title {
		font-size: 13px;
		font-weight: 600;
		color: var(--gm-text-primary);
	}

	.sf-description {
		font-size: 12px;
		color: var(--gm-text-secondary);
		line-height: 1.4;
		margin: 0;
	}

	.sf-detail {
		display: flex;
		gap: 8px;
		font-size: 11px;
	}

	.sf-label {
		color: var(--gm-text-muted);
		font-size: 10px;
		font-weight: 600;
		text-transform: uppercase;
		min-width: 60px;
		flex-shrink: 0;
	}

	.sf-value {
		color: var(--gm-text-secondary);
		font-family: 'JetBrains Mono', monospace;
		font-size: 10px;
	}

	.sf-remediation {
		background: var(--gm-bg-panel);
		border-radius: 4px;
		padding: 8px 10px;
		margin-top: 4px;
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.sf-rem-text {
		font-size: 11px;
		color: var(--gm-text-secondary);
		line-height: 1.4;
		margin: 0;
	}
</style>

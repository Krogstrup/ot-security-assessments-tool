<script lang="ts">
	import type { AllowlistEntry } from '$lib/types/analysis';

	interface Props {
		allowlistEntries: AllowlistEntry[];
		showAllowlist: boolean;
		loadingAllowlist: boolean;
		hasData: boolean;
		busyAction: string | null;
		classificationClass: (classification: string) => string;
		onGenerateAllowlist: () => Promise<void>;
		onExportAllowlistCsv: () => Promise<void>;
		onExportFirewallRules: () => Promise<void>;
	}

	let {
		allowlistEntries,
		showAllowlist,
		loadingAllowlist,
		hasData,
		busyAction,
		classificationClass,
		onGenerateAllowlist,
		onExportAllowlistCsv,
		onExportFirewallRules
	}: Props = $props();
</script>

<section class="export-section">
	<h3 class="section-title">Communication Allowlist</h3>
	<p class="section-desc">
		Every observed legitimate flow with frequency, classification, and firewall-ready export.
		Run analysis first for best classification results.
	</p>

	<div class="export-actions">
		<button class="action-btn primary" onclick={onGenerateAllowlist} disabled={loadingAllowlist || !hasData}>
			{loadingAllowlist ? 'Generating...' : 'Generate Allowlist'}
		</button>
		{#if allowlistEntries.length > 0}
			<button class="action-btn" onclick={onExportAllowlistCsv} disabled={busyAction === 'allowlist_csv'}>
				{busyAction === 'allowlist_csv' ? 'Exporting...' : 'Export CSV'}
			</button>
			<button class="action-btn" onclick={onExportFirewallRules} disabled={busyAction === 'fw_rules'}>
				{busyAction === 'fw_rules' ? 'Exporting...' : 'Export Firewall Rules'}
			</button>
		{/if}
	</div>

	{#if showAllowlist && allowlistEntries.length > 0}
		<div class="allowlist-wrap">
			<div class="allowlist-summary">
				{allowlistEntries.length} flows ·
				{allowlistEntries.filter((entry) => entry.classification === 'operational').length} operational ·
				{allowlistEntries.filter((entry) => entry.classification === 'management').length} management ·
				{allowlistEntries.filter((entry) => entry.classification === 'monitoring').length} monitoring ·
				{allowlistEntries.filter((entry) => entry.classification === 'it').length} IT
			</div>
			<table class="allowlist-table">
				<thead>
					<tr>
						<th>Source</th>
						<th>Destination</th>
						<th>Protocol</th>
						<th>Port</th>
						<th>Frequency</th>
						<th>Class</th>
						<th>Justification</th>
					</tr>
				</thead>
				<tbody>
					{#each allowlistEntries as entry}
						<tr class="allowlist-row">
							<td class="col-ip">{entry.src_ip}</td>
							<td class="col-ip">{entry.dst_ip}</td>
							<td class="col-proto">{entry.protocol}</td>
							<td class="col-port">{entry.dst_port}</td>
							<td class="col-freq">{entry.frequency}</td>
							<td class="col-class">
								<span class="cls-badge {classificationClass(entry.classification)}">{entry.classification}</span>
							</td>
							<td class="col-just">{entry.justification}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{:else if showAllowlist}
		<p class="no-findings">No connections to allowlist. Import a PCAP first.</p>
	{/if}
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

	.export-actions {
		display: flex;
		gap: 10px;
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

	.allowlist-wrap {
		overflow-x: auto;
		margin-top: 12px;
		border-radius: 6px;
		border: 1px solid var(--gm-border);
	}

	.allowlist-summary {
		padding: 6px 12px;
		font-size: 11px;
		color: var(--gm-text-muted);
		background: var(--gm-bg-secondary);
		border-bottom: 1px solid var(--gm-border);
	}

	.allowlist-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 11px;
	}

	.allowlist-table th {
		padding: 6px 10px;
		text-align: left;
		font-weight: 600;
		color: var(--gm-text-muted);
		background: var(--gm-bg-secondary);
		border-bottom: 1px solid var(--gm-border);
		white-space: nowrap;
	}

	.allowlist-table td {
		padding: 5px 10px;
		border-bottom: 1px solid var(--gm-border-subtle, var(--gm-border));
		vertical-align: middle;
	}

	.allowlist-row:hover td {
		background: var(--gm-bg-tertiary);
	}

	.col-ip {
		font-family: monospace;
		font-size: 11px;
		white-space: nowrap;
	}

	.col-proto {
		white-space: nowrap;
		font-weight: 600;
	}

	.col-port {
		width: 55px;
		text-align: right;
		font-family: monospace;
	}

	.col-freq {
		white-space: nowrap;
		color: var(--gm-text-secondary);
	}

	.col-class {
		width: 110px;
	}

	.col-just {
		color: var(--gm-text-secondary);
		font-size: 11px;
	}

	.cls-badge {
		display: inline-block;
		padding: 1px 7px;
		border-radius: 4px;
		font-size: 10px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.04em;
	}

	.cls-operational { background: rgba(16,185,129,0.15); color: #10b981; }
	.cls-management { background: rgba(59,130,246,0.15); color: #3b82f6; }
	.cls-monitoring { background: rgba(245,158,11,0.15); color: #f59e0b; }
	.cls-it { background: rgba(100,116,139,0.15); color: #64748b; }

	.no-findings {
		color: var(--gm-text-muted);
		font-size: 11px;
		margin-top: 12px;
	}
</style>

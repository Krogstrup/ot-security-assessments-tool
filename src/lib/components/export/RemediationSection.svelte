<script lang="ts">
	interface RemediationItem {
		rank: number;
		severity: string;
		title: string;
		description: string;
		assets: string[];
		remediation: string;
		technique: string | null;
	}

	interface Props {
		remediationItems: RemediationItem[];
		showRemediationTable: boolean;
		loadingRemediation: boolean;
		busyAction: string | null;
		onGenerateList: () => Promise<void>;
		onExportCsv: () => Promise<void>;
	}

	let {
		remediationItems,
		showRemediationTable,
		loadingRemediation,
		busyAction,
		onGenerateList,
		onExportCsv
	}: Props = $props();
</script>

<section class="export-section">
	<h3 class="section-title">Remediation Priority List</h3>
	<p class="section-desc">Ranked list of security findings with remediation guidance, sorted by severity.</p>
	<div class="btn-row">
		<button class="action-btn primary" onclick={onGenerateList} disabled={loadingRemediation || busyAction !== null}>
			{loadingRemediation ? 'Loading...' : 'Generate List'}
		</button>
		{#if remediationItems.length > 0}
			<button class="action-btn primary" onclick={onExportCsv} disabled={busyAction === 'remediation_csv'}>
				{busyAction === 'remediation_csv' ? 'Copying...' : 'Copy CSV'}
			</button>
		{/if}
	</div>

	{#if showRemediationTable && remediationItems.length > 0}
		<div class="remediation-table-wrap">
			<table class="remediation-table">
				<thead>
					<tr>
						<th class="col-rank">#</th>
						<th class="col-sev">Severity</th>
						<th class="col-title">Finding</th>
						<th class="col-assets">Assets</th>
						<th class="col-remedy">Remediation</th>
						<th class="col-tech">ATT&amp;CK</th>
					</tr>
				</thead>
				<tbody>
					{#each remediationItems as item}
						<tr class="remediation-row">
							<td class="col-rank">{item.rank}</td>
							<td><span class="sev-badge sev-{item.severity}">{item.severity}</span></td>
							<td class="col-title"><div class="finding-title">{item.title}</div></td>
							<td class="col-assets">
								{#each item.assets.slice(0, 3) as ip}
									<div class="asset-ip">{ip}</div>
								{/each}
								{#if item.assets.length > 3}
									<div class="asset-more">+{item.assets.length - 3} more</div>
								{/if}
							</td>
							<td class="col-remedy">{item.remediation}</td>
							<td class="col-tech">
								{#if item.technique}
									<code class="technique-id">{item.technique}</code>
								{/if}
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{:else if showRemediationTable}
		<p class="no-findings">No findings available. Run analysis first.</p>
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

	.btn-row {
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

	.remediation-table-wrap {
		overflow-x: auto;
		margin-top: 16px;
	}

	.remediation-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 11px;
	}

	.remediation-table th {
		text-align: left;
		padding: 8px 10px;
		background: var(--gm-bg-secondary);
		border-bottom: 2px solid var(--gm-border);
		color: var(--gm-text-muted);
		font-weight: 600;
		text-transform: uppercase;
		font-size: 10px;
		letter-spacing: 0.05em;
	}

	.remediation-table td {
		padding: 8px 10px;
		border-bottom: 1px solid var(--gm-border);
		vertical-align: top;
	}

	.remediation-row:hover td {
		background: var(--gm-bg-tertiary);
	}

	.col-rank {
		width: 40px;
		text-align: center;
		font-weight: 700;
		color: var(--gm-text-muted);
	}

	.col-sev {
		width: 90px;
	}

	.col-assets {
		width: 130px;
	}

	.col-tech {
		width: 100px;
	}

	.sev-badge {
		display: inline-block;
		padding: 2px 8px;
		border-radius: 4px;
		font-size: 10px;
		font-weight: 700;
		text-transform: uppercase;
	}

	.sev-badge.sev-critical { background: rgba(239, 68, 68, 0.13); color: #ef4444; }
	.sev-badge.sev-high { background: rgba(249, 115, 22, 0.13); color: #f97316; }
	.sev-badge.sev-medium { background: rgba(245, 158, 11, 0.13); color: #f59e0b; }
	.sev-badge.sev-low { background: rgba(16, 185, 129, 0.13); color: #10b981; }
	.sev-badge.sev-info { background: rgba(59, 130, 246, 0.13); color: #3b82f6; }

	.finding-title {
		font-weight: 500;
		color: var(--gm-text-primary);
	}

	.asset-ip {
		font-family: monospace;
		color: var(--gm-accent, #38bdf8);
		font-size: 10px;
	}

	.asset-more {
		color: var(--gm-text-muted);
		font-size: 10px;
	}

	.col-remedy {
		font-size: 11px;
		color: var(--gm-text-secondary);
	}

	.technique-id {
		font-family: monospace;
		font-size: 11px;
		background: var(--gm-bg-secondary);
		padding: 2px 6px;
		border-radius: 4px;
		color: var(--gm-accent, #38bdf8);
	}

	.no-findings {
		color: var(--gm-text-muted);
		font-size: 11px;
		margin-top: 12px;
	}
</style>

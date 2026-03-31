<script lang="ts">
	import type { CveMatch } from '$lib/types/analysis';

	interface Props {
		cveWarnings: CveMatch[];
	}

	let { cveWarnings }: Props = $props();

	function copyCveId(cveId: string) {
		navigator.clipboard.writeText(cveId);
	}
</script>

{#if cveWarnings.length > 0}
	<div class="detail-section cve-warning-section">
		<h4 class="section-title cve-title">&#128308; Known Vulnerabilities ({cveWarnings.length})</h4>
		{#each cveWarnings as cve}
			<div class="cve-card">
				<div class="cve-header-row">
					<span class="cve-id">{cve.cve_id}</span>
					<span class="cve-cvss-badge sev-{cve.severity_label.toLowerCase()}">
						{cve.severity_label} {cve.cvss.toFixed(1)}
					</span>
					<span class="cve-conf conf-{cve.confidence}">{cve.confidence}</span>
				</div>
				<div class="cve-desc">{cve.description}</div>
				{#if cve.advisory}
					<div class="cve-row">
						<span class="cve-label">Advisory:</span>
						<span>{cve.advisory}</span>
					</div>
				{/if}
				<div class="cve-row cve-remediation">
					<span class="cve-label">Fix:</span>
					<span>{cve.remediation}</span>
				</div>
				<button class="copy-btn cve-copy" onclick={() => copyCveId(cve.cve_id)}>
					Copy CVE ID
				</button>
			</div>
		{/each}
	</div>
{/if}

<style>
	.detail-section {
		margin-top: 1.5rem;
		padding: 1rem;
		border-radius: 6px;
		background: #0f172a;
		border: 1px solid #1e293b;
	}

	.cve-warning-section {
		background: rgba(239, 68, 68, 0.05);
		border: 1px solid rgba(239, 68, 68, 0.2);
	}

	.section-title {
		font-size: 0.875rem;
		font-weight: 600;
		margin-bottom: 1rem;
		color: #e2e8f0;
	}

	.cve-title {
		color: #ef4444;
	}

	.cve-card {
		background: rgba(0, 0, 0, 0.3);
		padding: 0.75rem;
		border-radius: 4px;
		margin-bottom: 0.75rem;
		border-left: 3px solid #ef4444;
	}

	.cve-header-row {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 0.5rem;
		flex-wrap: wrap;
	}

	.cve-id {
		font-family: 'JetBrains Mono', monospace;
		font-size: 0.8125rem;
		font-weight: 600;
		color: #fbbf24;
	}

	.cve-cvss-badge {
		padding: 0.25rem 0.5rem;
		border-radius: 3px;
		font-size: 0.75rem;
		font-weight: 600;
	}

	.cve-cvss-badge.sev-critical {
		background: rgba(239, 68, 68, 0.2);
		color: #ef4444;
	}

	.cve-cvss-badge.sev-high {
		background: rgba(249, 115, 22, 0.2);
		color: #f97316;
	}

	.cve-cvss-badge.sev-medium {
		background: rgba(252, 191, 73, 0.2);
		color: #fcbf49;
	}

	.cve-cvss-badge.sev-low {
		background: rgba(34, 197, 94, 0.2);
		color: #22c55e;
	}

	.cve-conf {
		padding: 0.25rem 0.5rem;
		border-radius: 3px;
		font-size: 0.75rem;
		background: #1e293b;
		color: #cbd5e1;
	}

	.cve-desc {
		font-size: 0.8125rem;
		line-height: 1.4;
		color: #cbd5e1;
		margin-bottom: 0.5rem;
	}

	.cve-row {
		display: flex;
		gap: 0.5rem;
		font-size: 0.8125rem;
		margin-bottom: 0.5rem;
		color: #94a3b8;
	}

	.cve-label {
		font-weight: 600;
		min-width: 70px;
		color: #64748b;
	}

	.cve-remediation {
		color: #10b981;
	}

	.copy-btn {
		margin-top: 0.5rem;
		padding: 0.4rem 0.8rem;
		font-size: 0.75rem;
		background: #ef4444;
		color: white;
		border: none;
		border-radius: 3px;
		cursor: pointer;
		transition: background 0.2s;
	}

	.copy-btn:hover {
		background: #dc2626;
	}
</style>

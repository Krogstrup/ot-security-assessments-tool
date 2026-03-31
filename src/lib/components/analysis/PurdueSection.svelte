<script lang="ts">
	import type { Finding, FindingSeverity, PurdueAssignment } from '$lib/types/analysis';

	interface Props {
		purdueAssignments: PurdueAssignment[];
		findings: Finding[];
		purdueColors: Record<number, string>;
		purdueLabels: Record<number, string>;
		severityColors: Record<FindingSeverity, string>;
		onNavigateAsset: (ip: string) => void;
	}

	let { purdueAssignments, findings, purdueColors, purdueLabels, severityColors, onNavigateAsset }: Props = $props();
	let violations = $derived(findings.filter((finding) => finding.finding_type === 'purdue_violation'));
</script>

{#if purdueAssignments.length === 0}
	<div class="empty-section">
		<p>No Purdue assignments yet. Run analysis to auto-assign Purdue levels.</p>
	</div>
{:else}
	<div class="purdue-overview">
		{#each [5, 4, 3, 2, 1, 0] as level}
			{@const levelAssets = purdueAssignments.filter((assignment) => assignment.level === level)}
			{#if levelAssets.length > 0}
				<div class="purdue-level">
					<div class="purdue-level-header" style="border-left: 3px solid {purdueColors[level]}">
						<span class="purdue-level-label" style="color: {purdueColors[level]}">
							{purdueLabels[level] ?? `L${level}`}
						</span>
						<span class="purdue-level-count">{levelAssets.length} devices</span>
					</div>
					<div class="purdue-devices">
						{#each levelAssets as assignment}
							<div class="purdue-device">
								<button class="device-ip" onclick={() => onNavigateAsset(assignment.ip_address)}>
									{assignment.ip_address}
								</button>
								<span class="device-method" class:manual={assignment.method === 'manual'}>
									{assignment.method}
								</span>
								<span class="device-reason">{assignment.reason}</span>
							</div>
						{/each}
					</div>
				</div>
			{/if}
		{/each}
	</div>

	{#if violations.length > 0}
		<div class="violations-section">
			<h3 class="subsection-title">Cross-Zone Violations ({violations.length})</h3>
			{#each violations as violation}
				<div class="violation-card">
					<span class="severity-badge" style="background: {severityColors[violation.severity]}">
						{violation.severity.toUpperCase()}
					</span>
					<span class="violation-text">{violation.title}</span>
				</div>
			{/each}
		</div>
	{/if}
{/if}

<style>
	.empty-section {
		padding: 40px 20px;
		text-align: center;
		color: var(--gm-text-muted);
		font-size: 12px;
	}

	.subsection-title {
		font-size: 12px;
		font-weight: 600;
		color: var(--gm-text-secondary);
		margin: 0 0 10px;
	}

	.severity-badge {
		display: inline-flex;
		align-items: center;
		gap: 3px;
		padding: 2px 8px;
		border-radius: 4px;
		font-size: 9px;
		font-weight: 700;
		color: #0a0e17;
		letter-spacing: 0.5px;
	}

	.purdue-overview {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.purdue-level {
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 8px;
		overflow: hidden;
	}

	.purdue-level-header {
		padding: 10px 14px;
		background: var(--gm-bg-secondary);
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.purdue-level-label {
		font-size: 12px;
		font-weight: 600;
	}

	.purdue-level-count {
		font-size: 10px;
		color: var(--gm-text-muted);
	}

	.purdue-devices {
		padding: 4px 8px;
	}

	.purdue-device {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 6px 8px;
		border-bottom: 1px solid var(--gm-border);
	}

	.purdue-device:last-child {
		border-bottom: none;
	}

	.device-ip {
		background: none;
		border: none;
		color: #10b981;
		font-family: inherit;
		font-size: 11px;
		font-weight: 600;
		cursor: pointer;
		padding: 0;
		min-width: 120px;
	}

	.device-ip:hover {
		text-decoration: underline;
	}

	.device-method {
		font-size: 9px;
		padding: 1px 6px;
		border-radius: 3px;
		background: var(--gm-bg-hover);
		color: var(--gm-text-muted);
		text-transform: uppercase;
	}

	.device-method.manual {
		background: rgba(59, 130, 246, 0.15);
		color: #3b82f6;
	}

	.device-reason {
		font-size: 10px;
		color: var(--gm-text-muted);
		flex: 1;
	}

	.violations-section {
		margin-top: 24px;
	}

	.violation-card {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 10px 14px;
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 6px;
		margin-bottom: 8px;
	}

	.violation-text {
		font-size: 11px;
		color: var(--gm-text-secondary);
	}
</style>

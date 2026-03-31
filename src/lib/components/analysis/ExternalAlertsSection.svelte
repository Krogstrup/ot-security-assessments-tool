<script lang="ts">
	import type { CorrelatedAlert } from '$lib/types/analysis';

	interface Props {
		loading: boolean;
		alerts: CorrelatedAlert[];
		onRefresh: () => void;
		onClear: () => void;
	}

	let { loading, alerts, onRefresh, onClear }: Props = $props();

	function alertSeverityLabel(severity: number): string {
		if (severity === 1) return 'HIGH';
		if (severity === 2) return 'MED';
		return 'LOW';
	}

	function alertSeverityClass(severity: number): string {
		if (severity === 1) return 'high';
		if (severity === 2) return 'medium';
		return 'low';
	}
</script>

<div class="ext-alerts-header">
	<span class="ext-alerts-count">{alerts.length} alert{alerts.length !== 1 ? 's' : ''}</span>
	{#if alerts.length > 0}
		<button class="clear-btn" onclick={onClear}>Clear All</button>
	{/if}
	<button class="run-btn" onclick={onRefresh} disabled={loading}>
		{loading ? 'Loading...' : 'Refresh'}
	</button>
</div>

{#if loading}
	<div class="empty-panel"><p>Loading alerts...</p></div>
{:else if alerts.length === 0}
	<div class="empty-panel">
		<div class="empty-icon">&#x1F514;</div>
		<p>No external alerts. Import Suricata eve.json or Wazuh alert export via the Capture tab.</p>
	</div>
{:else}
	<div class="alert-list">
		{#each alerts as alert}
			<div class="alert-row sev-{alertSeverityClass(alert.severity)}">
				<div class="alert-row-header">
					<span class="alert-sev sev-badge-{alertSeverityClass(alert.severity)}">{alertSeverityLabel(alert.severity)}</span>
					<span class="alert-source source-badge">{alert.source}</span>
					<span class="alert-sig">{alert.signature}</span>
					<span class="alert-ts">{new Date(alert.timestamp).toLocaleString()}</span>
				</div>
				<div class="alert-flow">
					<span class="flow-ip">
						{alert.src_ip}{alert.src_port ? ':' + alert.src_port : ''}
						{#if alert.src_hostname}<span class="flow-name">({alert.src_hostname})</span>{/if}
						{#if alert.src_device_type}<span class="flow-dtype">[{alert.src_device_type}]</span>{/if}
					</span>
					<span class="flow-arrow">→</span>
					<span class="flow-ip">
						{alert.dst_ip}{alert.dst_port ? ':' + alert.dst_port : ''}
						{#if alert.dst_hostname}<span class="flow-name">({alert.dst_hostname})</span>{/if}
						{#if alert.dst_device_type}<span class="flow-dtype">[{alert.dst_device_type}]</span>{/if}
					</span>
				</div>
				<div class="alert-meta">
					<span class="alert-cat">{alert.category}</span>
					{#if alert.src_purdue_level !== null || alert.dst_purdue_level !== null}
						<span class="alert-purdue">
							L{alert.src_purdue_level ?? '?'} → L{alert.dst_purdue_level ?? '?'}
						</span>
					{/if}
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

	.ext-alerts-header {
		display: flex;
		align-items: center;
		gap: 10px;
		margin-bottom: 12px;
		flex-shrink: 0;
	}

	.ext-alerts-count {
		font-size: 11px;
		color: var(--gm-text-muted);
		flex: 1;
	}

	.clear-btn {
		padding: 4px 10px;
		background: transparent;
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		color: var(--gm-text-muted);
		font-family: inherit;
		font-size: 11px;
		cursor: pointer;
	}

	.clear-btn:hover {
		border-color: #ef4444;
		color: #ef4444;
	}

	.alert-list {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.alert-row {
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-left: 3px solid var(--gm-border);
		border-radius: 6px;
		padding: 10px 14px;
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.alert-row.sev-high { border-left-color: var(--gm-severity-high); }
	.alert-row.sev-medium { border-left-color: var(--gm-severity-medium); }
	.alert-row.sev-low { border-left-color: var(--gm-severity-low); }

	.alert-row-header {
		display: flex;
		align-items: center;
		gap: 8px;
		flex-wrap: wrap;
	}

	.alert-sev {
		font-size: 9px;
		font-weight: 700;
		padding: 2px 6px;
		border-radius: 3px;
		flex-shrink: 0;
	}

	.sev-badge-high { background: var(--gm-severity-high); color: #fff; }
	.sev-badge-medium { background: var(--gm-severity-medium); color: #fff; }
	.sev-badge-low { background: var(--gm-severity-low); color: #fff; }

	.source-badge {
		font-size: 9px;
		padding: 2px 6px;
		border-radius: 3px;
		background: rgba(99, 102, 241, 0.2);
		color: #a5b4fc;
		font-weight: 600;
		flex-shrink: 0;
	}

	.alert-sig {
		font-size: 11px;
		color: var(--gm-text-primary);
		font-weight: 500;
		flex: 1;
	}

	.alert-ts {
		font-size: 10px;
		color: var(--gm-text-muted);
		font-family: 'JetBrains Mono', monospace;
		flex-shrink: 0;
	}

	.alert-flow {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 11px;
		font-family: 'JetBrains Mono', monospace;
		color: var(--gm-text-secondary);
		flex-wrap: wrap;
	}

	.flow-arrow {
		color: var(--gm-text-muted);
	}

	.flow-name {
		color: var(--gm-text-muted);
		font-size: 10px;
	}

	.flow-dtype {
		color: var(--gm-text-muted);
		font-size: 10px;
	}

	.alert-meta {
		display: flex;
		gap: 12px;
		font-size: 10px;
	}

	.alert-cat {
		color: var(--gm-text-muted);
		text-transform: lowercase;
	}

	.alert-purdue {
		color: var(--gm-purdue-l3);
		font-weight: 600;
	}
</style>

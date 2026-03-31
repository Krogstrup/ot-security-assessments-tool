<script lang="ts">
	import type { RedundancyInfo } from '$lib/types';

	interface Props {
		redundancyProtocols: RedundancyInfo[];
		loading: boolean;
	}

	let { redundancyProtocols, loading }: Props = $props();

	const protocolNames = $derived([...new Set(redundancyProtocols.map(r => r.protocol))]);
	const managerCount = $derived(redundancyProtocols.filter(r => r.is_manager).length);
	const tcEventCount = $derived(redundancyProtocols.filter(r => r.topology_change).length);
</script>

<div class="redundancy-container">
	{#if loading}
		<div class="loading">Loading redundancy data...</div>
	{:else if redundancyProtocols.length === 0}
		<div class="empty-state">
			<div class="empty-icon">&#x1F4A1;</div>
			<h3>No Redundancy Detected</h3>
			<p>No spanning tree or redundancy protocols detected in this capture.</p>
		</div>
	{:else}
		<div class="redundancy-content">
			<div class="redundancy-summary">
				<div class="summary-item">
					<span class="summary-label">Devices</span>
					<span class="summary-value">{redundancyProtocols.length}</span>
				</div>
				<div class="summary-item">
					<span class="summary-label">Managers</span>
					<span class="summary-value">{managerCount}</span>
				</div>
				<div class="summary-item">
					<span class="summary-label">TC Events</span>
					<span class="summary-value">{tcEventCount}</span>
				</div>
			</div>

			{#each protocolNames as protocol}
				<div class="protocol-section">
					<h3 class="protocol-title">{protocol}</h3>

					{#each redundancyProtocols.filter(r => r.protocol === protocol) as device}
						<div class="device-card">
							<div class="card-header">
								<span class="device-name">{device.bridge_id}</span>
								{#if device.is_manager}
									<span class="manager-badge">Manager</span>
								{/if}
								{#if device.topology_change}
									<span class="tc-badge">TC Event</span>
								{/if}
							</div>

							<div class="card-detail">
								<span class="detail-label">Priority</span>
								<span class="detail-value">{device.priority}</span>
							</div>

							{#if device.root_port}
								<div class="card-detail">
									<span class="detail-label">Root Port</span>
									<span class="detail-value">{device.root_port}</span>
								</div>
							{/if}

							{#if device.path_cost}
								<div class="card-detail">
									<span class="detail-label">Path Cost</span>
									<span class="detail-value">{device.path_cost}</span>
								</div>
							{/if}

							{#if device.blocked_ports && device.blocked_ports.length > 0}
								<div class="ports-section">
									<span class="ports-label">Blocked Ports ({device.blocked_ports.length})</span>
									<div class="ports-list">
										{#each device.blocked_ports as port}
											<span class="port-tag">{port}</span>
										{/each}
									</div>
								</div>
							{/if}
						</div>
					{/each}
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.redundancy-container {
		flex: 1;
		overflow-y: auto;
		padding: 1rem;
		background: var(--gm-bg-secondary);
	}

	.loading,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100%;
		text-align: center;
		color: var(--gm-text-secondary);
	}

	.empty-icon {
		font-size: 3rem;
		opacity: 0.3;
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		margin: 0 0 0.5rem 0;
		color: var(--gm-text-primary);
		font-size: 1.125rem;
	}

	.empty-state p {
		margin: 0;
		font-size: 0.8125rem;
	}

	.redundancy-content {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.redundancy-summary {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
		gap: 1rem;
		padding: 1rem;
		background: rgba(99, 102, 241, 0.05);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 4px;
	}

	.summary-item {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.summary-label {
		font-size: 0.75rem;
		color: var(--gm-text-secondary);
		font-weight: 600;
	}

	.summary-value {
		font-size: 1.5rem;
		font-weight: 600;
		color: var(--gm-text-primary);
	}

	.protocol-section {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.protocol-title {
		margin: 0;
		font-size: 0.95rem;
		font-weight: 600;
		color: var(--gm-text-primary);
		text-transform: uppercase;
	}

	.device-card {
		padding: 0.75rem;
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		background: rgba(0, 0, 0, 0.2);
	}

	.card-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.5rem;
	}

	.device-name {
		font-family: 'JetBrains Mono', monospace;
		font-size: 0.8125rem;
		color: var(--gm-text-primary);
		font-weight: 600;
		flex: 1;
	}

	.manager-badge,
	.tc-badge {
		padding: 0.2rem 0.4rem;
		border-radius: 2px;
		font-size: 0.7rem;
		font-weight: 600;
	}

	.manager-badge {
		background: rgba(34, 197, 94, 0.2);
		color: #22c55e;
	}

	.tc-badge {
		background: rgba(245, 158, 11, 0.2);
		color: #f59e0b;
	}

	.card-detail {
		display: flex;
		justify-content: space-between;
		margin-bottom: 0.35rem;
		font-size: 0.75rem;
	}

	.detail-label {
		color: var(--gm-text-secondary);
		font-weight: 600;
	}

	.detail-value {
		color: var(--gm-text-primary);
		font-family: 'JetBrains Mono', monospace;
	}

	.ports-section {
		margin-top: 0.5rem;
		padding-top: 0.5rem;
		border-top: 1px solid rgba(0, 0, 0, 0.2);
	}

	.ports-label {
		display: block;
		font-size: 0.7rem;
		color: var(--gm-text-secondary);
		font-weight: 600;
		margin-bottom: 0.35rem;
	}

	.ports-list {
		display: flex;
		flex-wrap: wrap;
		gap: 0.35rem;
	}

	.port-tag {
		padding: 0.2rem 0.4rem;
		background: rgba(239, 68, 68, 0.1);
		color: #ef4444;
		border-radius: 2px;
		font-size: 0.7rem;
		font-family: 'JetBrains Mono', monospace;
		font-weight: 500;
	}
</style>

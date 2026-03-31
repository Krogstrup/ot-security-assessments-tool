<script lang="ts">
	import type { InferredTopology } from '$lib/types';

	interface Props {
		inferredTopology: InferredTopology | null;
		inferring: boolean;
		onRunInference: () => Promise<void>;
	}

	let { inferredTopology, inferring, onRunInference }: Props = $props();
</script>

<div class="inferred-container">
	{#if !inferredTopology}
		<div class="inferred-empty">
			<div class="empty-icon">&#x1F4E1;</div>
			<h3>No Inferred Topology</h3>
			<p>Click <strong>Run Inference</strong> to analyze the current dataset and infer network structure from traffic patterns.</p>
			<button class="action-btn primary" onclick={onRunInference} disabled={inferring}>
				{inferring ? 'Running...' : 'Run Inference'}
			</button>
		</div>
	{:else}
		<div class="inferred-content">
			<!-- Subnets -->
			<div class="inferred-section">
				<h3 class="inferred-section-title">Subnets ({inferredTopology.subnets.length})</h3>
				{#each inferredTopology.subnets as subnet}
					<div class="inferred-card">
						<div class="card-header">
							<span class="card-network">{subnet.network}</span>
							<span class="card-badge">{subnet.member_ips.length} hosts</span>
						</div>
						{#if subnet.gateway_ip}
							<div class="card-detail">
								<span class="card-label">Gateway</span>
								<span class="card-value gw-ip">{subnet.gateway_ip}</span>
							</div>
						{/if}
						<div class="card-ips">
							{#each subnet.member_ips.slice(0, 6) as ip}
								<span class="ip-chip">{ip}</span>
							{/each}
							{#if subnet.member_ips.length > 6}
								<span class="ip-chip more">+{subnet.member_ips.length - 6}</span>
							{/if}
						</div>
					</div>
				{/each}
			</div>

			<!-- Gateways -->
			{#if inferredTopology.gateways.length > 0}
				<div class="inferred-section">
					<h3 class="inferred-section-title">Gateways ({inferredTopology.gateways.length})</h3>
					{#each inferredTopology.gateways as gw}
						<div class="inferred-card">
							<div class="card-header">
								<span class="card-network">{gw.ip_address}</span>
								<span class="card-badge">
									{gw.connected_subnets.length} subnets
								</span>
							</div>
							<div class="card-detail">
								<span class="card-label">Role</span>
								<span class="card-value">{gw.likely_role}</span>
							</div>
						</div>
					{/each}
				</div>
			{/if}

			<!-- Switch Candidates -->
			{#if inferredTopology.switch_candidates.length > 0}
				<div class="inferred-section">
					<h3 class="inferred-section-title">Switch Candidates ({inferredTopology.switch_candidates.length})</h3>
					{#each inferredTopology.switch_candidates as sw}
						<div class="inferred-card">
							<div class="card-header">
								<span class="card-network">{sw.ip_address}</span>
								<span class="card-badge">{sw.forwarded_subnets} subnets</span>
							</div>
							<div class="card-detail">
								<span class="card-label">Confidence</span>
								<span class="card-value">{(sw.confidence * 100).toFixed(0)}%</span>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.inferred-container {
		flex: 1;
		overflow-y: auto;
		padding: 1rem;
		background: var(--gm-bg-secondary);
	}

	.inferred-empty {
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

	.inferred-empty h3 {
		margin: 0 0 0.5rem 0;
		color: var(--gm-text-primary);
		font-size: 1.125rem;
	}

	.inferred-empty p {
		margin: 0 0 1rem 0;
		font-size: 0.8125rem;
		max-width: 300px;
	}

	.action-btn {
		padding: 0.5rem 1rem;
		border: none;
		border-radius: 4px;
		font-size: 0.8125rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn.primary {
		background: #6366f1;
		color: white;
	}

	.action-btn.primary:hover:not(:disabled) {
		background: #4f46e5;
	}

	.action-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.inferred-content {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.inferred-section {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.inferred-section-title {
		margin: 0;
		font-size: 0.95rem;
		font-weight: 600;
		color: var(--gm-text-primary);
		text-transform: uppercase;
	}

	.inferred-card {
		padding: 0.75rem;
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		background: rgba(0, 0, 0, 0.2);
	}

	.card-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.5rem;
	}

	.card-network {
		font-family: 'JetBrains Mono', monospace;
		font-size: 0.8125rem;
		color: var(--gm-text-primary);
		font-weight: 600;
	}

	.card-badge {
		padding: 0.2rem 0.4rem;
		background: rgba(99, 102, 241, 0.2);
		color: #6366f1;
		border-radius: 2px;
		font-size: 0.7rem;
		font-weight: 600;
	}

	.card-detail {
		display: flex;
		justify-content: space-between;
		margin-bottom: 0.5rem;
		font-size: 0.75rem;
	}

	.card-label {
		color: var(--gm-text-secondary);
		font-weight: 600;
	}

	.card-value {
		color: var(--gm-text-primary);
		font-family: 'JetBrains Mono', monospace;
	}

	.card-value.gw-ip {
		font-weight: 600;
	}

	.card-ips {
		display: flex;
		flex-wrap: wrap;
		gap: 0.35rem;
	}

	.ip-chip {
		padding: 0.2rem 0.4rem;
		background: rgba(59, 130, 246, 0.1);
		color: #3b82f6;
		border-radius: 2px;
		font-size: 0.7rem;
		font-family: 'JetBrains Mono', monospace;
		font-weight: 500;
	}

	.ip-chip.more {
		background: transparent;
		border: 1px solid rgba(59, 130, 246, 0.3);
		color: var(--gm-text-secondary);
	}
</style>

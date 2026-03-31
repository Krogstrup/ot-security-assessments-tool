<script lang="ts">
	import type { ProtocolStats } from '$lib/types/protocols';

	interface Props {
		stats: ProtocolStats[];
	}

	let { stats }: Props = $props();

	function formatBytes(bytes: number): string {
		if (bytes < 1024) return `${bytes} B`;
		if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
		if (bytes < 1024 * 1024 * 1024) return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
		return `${(bytes / 1024 / 1024 / 1024).toFixed(2)} GB`;
	}

	const totalPackets = $derived(stats.reduce((sum, s) => sum + s.packet_count, 0));
	const totalBytes = $derived(stats.reduce((sum, s) => sum + s.byte_count, 0));
	const sortedStats = $derived(
		[...stats].sort((a, b) => b.packet_count - a.packet_count)
	);
</script>

<section class="capture-section">
	<h3 class="section-title">Protocol Statistics</h3>
	<p class="section-desc">
		Breakdown of observed protocols by packet count and data volume.
	</p>

	{#if stats.length === 0}
		<div class="empty-state">
			<p>No protocols observed yet. Import a PCAP file or start a capture.</p>
		</div>
	{:else}
		<div class="stats-summary">
			<div class="summary-item">
				<span class="summary-label">Total Packets</span>
				<span class="summary-value">{totalPackets.toLocaleString()}</span>
			</div>
			<div class="summary-item">
				<span class="summary-label">Total Data</span>
				<span class="summary-value">{formatBytes(totalBytes)}</span>
			</div>
		</div>

		<div class="stats-table">
			<div class="table-header">
				<div class="col-name">Protocol</div>
				<div class="col-packets">Packets</div>
				<div class="col-percent">%</div>
				<div class="col-bytes">Data</div>
			</div>

			{#each sortedStats as stat}
				<div class="table-row">
					<div class="col-name">
						<span class="proto-badge">{stat.protocol}</span>
					</div>
					<div class="col-packets">{stat.packet_count.toLocaleString()}</div>
					<div class="col-percent">
						{totalPackets > 0 ? ((stat.packet_count / totalPackets) * 100).toFixed(1) : '0'}%
					</div>
					<div class="col-bytes">{formatBytes(stat.byte_count)}</div>
				</div>
			{/each}
		</div>
	{/if}
</section>

<style>
	.capture-section {
		padding: 1.5rem;
		background: var(--gm-bg-secondary);
		border: 1px solid var(--gm-border);
		border-radius: 6px;
		margin-bottom: 1rem;
	}

	.section-title {
		font-size: 1.125rem;
		font-weight: 600;
		margin-bottom: 0.5rem;
		color: var(--gm-text-primary);
	}

	.section-desc {
		font-size: 0.8125rem;
		color: var(--gm-text-secondary);
		margin-bottom: 1rem;
	}

	.empty-state {
		padding: 2rem;
		text-align: center;
		color: var(--gm-text-secondary);
		font-size: 0.8125rem;
	}

	.stats-summary {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.summary-item {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
		padding: 1rem;
		background: rgba(99, 102, 241, 0.05);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 4px;
	}

	.summary-label {
		font-size: 0.75rem;
		color: var(--gm-text-secondary);
		font-weight: 600;
	}

	.summary-value {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--gm-text-primary);
		font-family: 'JetBrains Mono', monospace;
	}

	.stats-table {
		display: flex;
		flex-direction: column;
		gap: 0;
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		overflow: hidden;
	}

	.table-header {
		display: grid;
		grid-template-columns: 1fr 120px 80px 100px;
		gap: 1rem;
		padding: 0.75rem;
		background: rgba(0, 0, 0, 0.2);
		border-bottom: 1px solid var(--gm-border);
		font-weight: 600;
		font-size: 0.75rem;
		color: var(--gm-text-secondary);
		text-transform: uppercase;
	}

	.table-row {
		display: grid;
		grid-template-columns: 1fr 120px 80px 100px;
		gap: 1rem;
		padding: 0.75rem;
		border-bottom: 1px solid var(--gm-border);
		align-items: center;
		font-size: 0.8125rem;
	}

	.table-row:last-child {
		border-bottom: none;
	}

	.table-row:hover {
		background: rgba(99, 102, 241, 0.05);
	}

	.col-name {
		display: flex;
		align-items: center;
	}

	.proto-badge {
		padding: 0.25rem 0.5rem;
		background: rgba(99, 102, 241, 0.2);
		border-radius: 3px;
		color: #6366f1;
		font-weight: 600;
		font-size: 0.75rem;
		font-family: 'JetBrains Mono', monospace;
	}

	.col-packets,
	.col-percent,
	.col-bytes {
		font-family: 'JetBrains Mono', monospace;
		text-align: right;
	}
</style>

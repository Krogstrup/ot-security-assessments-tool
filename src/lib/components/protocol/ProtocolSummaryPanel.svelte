<script lang="ts">
	import type { ProtocolStats } from '$lib/types/protocols';

	interface Props {
		stats: ProtocolStats[];
		activeProtocol: string | null;
		getColor: (protocol: string) => string;
		formatBytes: (bytes: number) => string;
		onSelectProtocol: (protocol: string | null) => void;
	}

	let { stats, activeProtocol, getColor, formatBytes, onSelectProtocol }: Props = $props();
</script>

<div class="stats-panel">
	<h3 class="panel-title">Protocol Summary</h3>
	<table class="stats-table">
		<thead>
			<tr>
				<th>Protocol</th>
				<th>Packets</th>
				<th>Bytes</th>
				<th>Connections</th>
				<th>Devices</th>
			</tr>
		</thead>
		<tbody>
			{#each stats as stat}
				<tr
					class="stat-row"
					class:active={activeProtocol === stat.protocol}
					onclick={() =>
						onSelectProtocol(activeProtocol === stat.protocol ? null : stat.protocol)}
				>
					<td>
						<span
							class="proto-dot"
							style="background: {getColor(stat.protocol)}"
						></span>
						{stat.protocol}
					</td>
					<td class="cell-numeric">{stat.packet_count.toLocaleString()}</td>
					<td class="cell-numeric">{formatBytes(stat.byte_count)}</td>
					<td class="cell-numeric">{stat.connection_count}</td>
					<td class="cell-numeric">{stat.unique_devices}</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>

<style>
	.stats-panel {
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 6px;
		padding: 14px;
	}

	.panel-title {
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		color: var(--gm-text-muted);
		margin: 0 0 12px;
	}

	.stats-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 11px;
	}

	.stats-table th {
		font-size: 10px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.3px;
		color: var(--gm-text-muted);
		text-align: left;
		padding: 6px 10px;
		border-bottom: 1px solid var(--gm-border);
	}

	.stats-table td {
		padding: 6px 10px;
		color: var(--gm-text-secondary);
		border-bottom: 1px solid rgba(45, 58, 79, 0.3);
	}

	.stat-row {
		cursor: pointer;
		transition: background 0.1s;
	}

	.stat-row:hover {
		background: rgba(255, 255, 255, 0.02);
	}

	.stat-row.active {
		background: rgba(59, 130, 246, 0.08);
	}

	.proto-dot {
		display: inline-block;
		width: 8px;
		height: 8px;
		border-radius: 50%;
		margin-right: 6px;
		vertical-align: middle;
	}

	.cell-numeric {
		text-align: right;
		font-variant-numeric: tabular-nums;
	}
</style>

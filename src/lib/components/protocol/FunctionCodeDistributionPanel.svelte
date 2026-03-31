<script lang="ts">
	import type { FunctionCodeStat } from '$lib/types/deep-parse';

	interface Props {
		functionCodeStats: Record<string, FunctionCodeStat[]>;
		getColor: (protocol: string) => string;
	}

	let { functionCodeStats, getColor }: Props = $props();
</script>

<div class="stats-panel wide">
	<h3 class="panel-title">Function Code Distribution</h3>
	<div class="fc-grid">
		{#each Object.entries(functionCodeStats) as [protocol, fcs]}
			{@const protoLabel = protocol === 'modbus' ? 'Modbus TCP' : protocol === 'dnp3' ? 'DNP3' : protocol.toUpperCase()}
			{@const protoKey = protocol === 'modbus' ? 'Modbus' : protocol === 'dnp3' ? 'Dnp3' : protocol === 'ethernet_ip' ? 'EthernetIp' : protocol === 's7comm' ? 'S7comm' : protocol === 'bacnet' ? 'Bacnet' : protocol === 'iec104' ? 'Iec104' : protocol === 'profinet' ? 'Profinet' : 'Unknown'}
			<div class="fc-section">
				<h4 class="fc-protocol-title" style="color: {getColor(protoKey)}">
					{protoLabel}
				</h4>
				<table class="fc-table">
					<thead>
						<tr>
							<th>FC</th>
							<th>Name</th>
							<th>Count</th>
							<th>Type</th>
						</tr>
					</thead>
					<tbody>
						{#each fcs as fc}
							<tr class:write-fc={fc.is_write}>
								<td class="cell-fc">{fc.code}</td>
								<td>{fc.name}</td>
								<td class="cell-numeric">{fc.count.toLocaleString()}</td>
								<td>
									{#if fc.is_write}
										<span class="fc-badge write">Write</span>
									{:else}
										<span class="fc-badge read">Read</span>
									{/if}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/each}
	</div>
</div>

<style>
	.stats-panel {
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 6px;
		padding: 14px;
	}

	.stats-panel.wide {
		grid-column: 1 / -1;
	}

	.panel-title {
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		color: var(--gm-text-muted);
		margin: 0 0 12px;
	}

	.fc-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
		gap: 16px;
	}

	.fc-section {
		min-width: 0;
	}

	.fc-protocol-title {
		font-size: 12px;
		font-weight: 600;
		margin: 0 0 8px;
	}

	.fc-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 10px;
	}

	.fc-table th {
		font-size: 9px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.3px;
		color: var(--gm-text-muted);
		text-align: left;
		padding: 4px 8px;
		border-bottom: 1px solid var(--gm-border);
	}

	.fc-table td {
		padding: 4px 8px;
		color: var(--gm-text-secondary);
		border-bottom: 1px solid rgba(45, 58, 79, 0.3);
	}

	.cell-fc {
		font-weight: 600;
		color: var(--gm-text-primary);
		width: 30px;
	}

	.cell-numeric {
		text-align: right;
		font-variant-numeric: tabular-nums;
	}

	.write-fc td {
		color: var(--gm-text-primary);
	}

	.fc-badge {
		font-size: 8px;
		font-weight: 600;
		padding: 1px 5px;
		border-radius: 3px;
		letter-spacing: 0.3px;
	}

	.fc-badge.write {
		color: #ef4444;
		background: rgba(239, 68, 68, 0.15);
	}

	.fc-badge.read {
		color: #10b981;
		background: rgba(16, 185, 129, 0.15);
	}
</style>

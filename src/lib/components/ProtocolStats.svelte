<script lang="ts">
	import { protocolStats } from '$lib/stores';
	import { getProtocolStats, getFunctionCodeStats } from '$lib/api';
	import type { FunctionCodeStat } from '$lib/types';
	import { onMount } from 'svelte';
	import TrafficBreakdownPanel from './protocol/TrafficBreakdownPanel.svelte';
	import ProtocolSummaryPanel from './protocol/ProtocolSummaryPanel.svelte';
	import FunctionCodeDistributionPanel from './protocol/FunctionCodeDistributionPanel.svelte';

	let functionCodeStats = $state<Record<string, FunctionCodeStat[]>>({});
	let loading = $state(false);
	let activeProtocol = $state<string | null>(null);

	const protocolColors: Record<string, string> = {
		Modbus: 'var(--gm-modbus, #f59e0b)',
		Dnp3: 'var(--gm-dnp3, #10b981)',
		EthernetIp: 'var(--gm-ethernet-ip, #8b5cf6)',
		Bacnet: 'var(--gm-bacnet, #06b6d4)',
		S7comm: 'var(--gm-s7comm, #ef4444)',
		OpcUa: 'var(--gm-opc-ua, #ec4899)',
		Profinet: '#6366f1',
		Iec104: '#14b8a6',
		Mqtt: '#84cc16',
		HartIp: '#f97316',
		FoundationFieldbus: '#a855f7',
		GeSrtp: '#e879f9',
		WonderwareSuitelink: '#fb923c',
		Http: '#475569',
		Https: '#64748b',
		Dns: '#78716c',
		Ssh: '#525252',
		Rdp: '#737373',
		Snmp: '#a1a1aa',
		Unknown: '#374151'
	};

	onMount(async () => {
		await refresh();
	});

	async function refresh() {
		loading = true;
		try {
			const [stats, fcStats] = await Promise.all([
				getProtocolStats(),
				getFunctionCodeStats()
			]);
			protocolStats.set(stats);
			functionCodeStats = fcStats;
		} catch (err) {
			console.warn('Failed to load protocol stats:', err);
		}
		loading = false;
	}

	function formatBytes(bytes: number): string {
		if (bytes < 1024) return `${bytes} B`;
		if (bytes < 1048576) return `${(bytes / 1024).toFixed(1)} KB`;
		return `${(bytes / 1048576).toFixed(1)} MB`;
	}

	function getColor(protocol: string): string {
		return protocolColors[protocol] ?? '#374151';
	}
</script>

<div class="stats-container">
	<div class="stats-toolbar">
		<h2 class="view-title">Protocol Statistics</h2>
		<button class="refresh-btn" onclick={refresh} disabled={loading}>
			{loading ? 'Loading...' : 'Refresh'}
		</button>
	</div>

	{#if $protocolStats.length === 0}
		<div class="empty-state">
			<p>No protocol data available. Import a PCAP file to see statistics.</p>
		</div>
	{:else}
		<div class="stats-grid">
			<TrafficBreakdownPanel
				stats={$protocolStats}
				{getColor}
			/>

			<ProtocolSummaryPanel
				stats={$protocolStats}
				{activeProtocol}
				{getColor}
				{formatBytes}
				onSelectProtocol={(protocol) => (activeProtocol = protocol)}
			/>

			{#if Object.keys(functionCodeStats).length > 0}
				<FunctionCodeDistributionPanel
					{functionCodeStats}
					{getColor}
				/>
			{/if}
		</div>
	{/if}
</div>

<style>
	.stats-container {
		display: flex;
		flex-direction: column;
		height: 100%;
		overflow: auto;
	}

	.stats-toolbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 10px 16px;
		border-bottom: 1px solid var(--gm-border);
		background: var(--gm-bg-secondary);
		flex-shrink: 0;
	}

	.view-title {
		font-size: 13px;
		font-weight: 600;
		letter-spacing: 1px;
		text-transform: uppercase;
		color: var(--gm-text-primary);
		margin: 0;
	}

	.refresh-btn {
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		padding: 5px 12px;
		color: var(--gm-text-secondary);
		font-family: inherit;
		font-size: 11px;
		cursor: pointer;
		transition: all 0.15s;
	}

	.refresh-btn:hover:not(:disabled) {
		background: var(--gm-bg-hover);
		color: var(--gm-text-primary);
	}

	.refresh-btn:disabled {
		opacity: 0.5;
		cursor: default;
	}

	.empty-state {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100%;
		color: var(--gm-text-muted);
		font-size: 12px;
	}

	.stats-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 16px;
		padding: 16px;
	}
</style>

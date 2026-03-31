<script lang="ts">
	import type { DeepParseInfo, EnipDetail, S7Detail, BacnetDetail, Iec104Detail, ProfinetDcpDetail, LldpDetail } from '$lib/types';

	interface Props {
		deepParseInfo: DeepParseInfo | null;
		loading?: boolean;
	}

	let { deepParseInfo, loading = false }: Props = $props();

	function formatInterval(ms: number): string {
		if (ms < 1000) return `${ms.toFixed(0)}ms`;
		return `${(ms / 1000).toFixed(1)}s`;
	}
</script>

{#if loading}
	<div class="detail-loading">Loading deep parse data...</div>
{:else if deepParseInfo}
	<!-- Modbus Detail -->
	{#if deepParseInfo.modbus}
		<div class="detail-section">
			<h4 class="section-title modbus-title">Modbus TCP</h4>
			<div class="detail-row">
				<span class="detail-label">Role</span>
				<span class="detail-value role-badge">{deepParseInfo.modbus.role}</span>
			</div>
			{#if deepParseInfo.modbus.unit_ids.length > 0}
				<div class="detail-row">
					<span class="detail-label">Unit IDs</span>
					<span class="detail-value">{deepParseInfo.modbus.unit_ids.join(', ')}</span>
				</div>
			{/if}
			{#if deepParseInfo.modbus.function_codes.length > 0}
				<div class="detail-subsection">
					<h5 class="subsection-title">Function Codes</h5>
					<div class="fc-list">
						{#each deepParseInfo.modbus.function_codes as fc}
							<div class="fc-item" class:write={fc.is_write}>
								<span class="fc-code">FC {fc.code}</span>
								<span class="fc-name">{fc.name}</span>
								<span class="fc-count">{fc.count.toLocaleString()}</span>
							</div>
						{/each}
					</div>
				</div>
			{/if}
			{#if deepParseInfo.modbus.register_ranges.length > 0}
				<div class="detail-subsection">
					<h5 class="subsection-title">Register Ranges</h5>
					<div class="reg-list">
						{#each deepParseInfo.modbus.register_ranges as reg}
							<div class="reg-item">
								<span class="reg-range">{reg.start}–{reg.end}</span>
								<span class="reg-type">{reg.register_type}</span>
								<span class="reg-count">{reg.access_count} accesses</span>
							</div>
						{/each}
					</div>
				</div>
			{/if}
		</div>
	{/if}

	<!-- DNP3 Detail -->
	{#if deepParseInfo.dnp3}
		<div class="detail-section">
			<h4 class="section-title dnp3-title">DNP3</h4>
			<div class="detail-row">
				<span class="detail-label">Role</span>
				<span class="detail-value role-badge">{deepParseInfo.dnp3.role}</span>
			</div>
			{#if deepParseInfo.dnp3.has_unsolicited}
				<div class="detail-row">
					<span class="detail-label">Unsolicited</span>
					<span class="detail-value warning">Yes (FC 130 detected)</span>
				</div>
			{/if}
			{#if deepParseInfo.dnp3.function_codes.length > 0}
				<div class="detail-subsection">
					<h5 class="subsection-title">Function Codes</h5>
					<div class="fc-list">
						{#each deepParseInfo.dnp3.function_codes as fc}
							<div class="fc-item" class:write={fc.is_write}>
								<span class="fc-code">FC {fc.code}</span>
								<span class="fc-name">{fc.name}</span>
								<span class="fc-count">{fc.count.toLocaleString()}</span>
							</div>
						{/each}
					</div>
				</div>
			{/if}
		</div>
	{/if}

	<!-- EtherNet/IP Detail -->
	{#if deepParseInfo.enip}
		{@const enip = deepParseInfo.enip as EnipDetail}
		<div class="detail-section">
			<h4 class="section-title ethernet-ip-title">EtherNet/IP + CIP</h4>
			<div class="detail-row">
				<span class="detail-label">Role</span>
				<span class="detail-value role-badge">{enip.role}</span>
			</div>
			{#if enip.cip_writes_to_assembly}
				<div class="detail-row">
					<span class="detail-label">CIP Writes</span>
					<span class="detail-value finding">Assembly object writes (T0855)</span>
				</div>
			{/if}
		</div>
	{/if}

	<!-- S7comm Detail -->
	{#if deepParseInfo.s7}
		{@const s7 = deepParseInfo.s7 as S7Detail}
		<div class="detail-section">
			<h4 class="section-title s7-title">S7comm (Siemens)</h4>
			<div class="detail-row">
				<span class="detail-label">Role</span>
				<span class="detail-value role-badge">{s7.role}</span>
			</div>
			{#if s7.functions_seen.length > 0}
				<div class="detail-subsection">
					<h5 class="subsection-title">Functions Observed</h5>
					<div class="fc-list">
						{#each s7.functions_seen as fn}
							{@const isWrite = fn === 'WriteVar' || fn === 'PlcStop' || fn === 'Download'}
							<div class="fc-item" class:write={isWrite}>
								<span class="fc-name">{fn.replace(/([A-Z])/g, ' $1').trim()}</span>
							</div>
						{/each}
					</div>
				</div>
			{/if}
		</div>
	{/if}

	<!-- BACnet Detail -->
	{#if deepParseInfo.bacnet}
		{@const bacnet = deepParseInfo.bacnet as BacnetDetail}
		<div class="detail-section">
			<h4 class="section-title bacnet-title">BACnet/IP</h4>
			<div class="detail-row">
				<span class="detail-label">Role</span>
				<span class="detail-value role-badge">{bacnet.role}</span>
			</div>
		</div>
	{/if}

	<!-- IEC 104 Detail -->
	{#if deepParseInfo.iec104}
		{@const iec104 = deepParseInfo.iec104 as Iec104Detail}
		<div class="detail-section">
			<h4 class="section-title iec104-title">IEC 60870-5-104</h4>
			<div class="detail-row">
				<span class="detail-label">Role</span>
				<span class="detail-value role-badge">{iec104.role}</span>
			</div>
		</div>
	{/if}

	<!-- PROFINET DCP Detail -->
	{#if deepParseInfo.profinet_dcp}
		{@const pndcp = deepParseInfo.profinet_dcp as ProfinetDcpDetail}
		<div class="detail-section">
			<h4 class="section-title profinet-title">PROFINET DCP</h4>
			<div class="detail-row">
				<span class="detail-label">Role</span>
				<span class="detail-value role-badge">{pndcp.role}</span>
			</div>
		</div>
	{/if}

	<!-- LLDP Detail -->
	{#if deepParseInfo.lldp}
		{@const lldp = deepParseInfo.lldp as LldpDetail}
		<div class="detail-section">
			<h4 class="section-title lldp-title">LLDP Discovery</h4>
			{#if lldp.system_name}
				<div class="detail-row">
					<span class="detail-label">System Name</span>
					<span class="detail-value highlight">{lldp.system_name}</span>
				</div>
			{/if}
			{#if lldp.vendor}
				<div class="detail-row">
					<span class="detail-label">Vendor</span>
					<span class="detail-value">{lldp.vendor}</span>
				</div>
			{/if}
			{#if lldp.model}
				<div class="detail-row">
					<span class="detail-label">Model</span>
					<span class="detail-value highlight">{lldp.model}</span>
				</div>
			{/if}
		</div>
	{/if}

	<!-- SNMP Detail -->
	{#if deepParseInfo.snmp}
		{@const snmp = deepParseInfo.snmp}
		<div class="detail-section">
			<h4 class="section-title snmp-title">SNMP Identity</h4>
			{#if snmp.sys_name}
				<div class="detail-row">
					<span class="detail-label">sysName</span>
					<span class="detail-value">{snmp.sys_name}</span>
				</div>
			{/if}
			{#if snmp.vendor}
				<div class="detail-row">
					<span class="detail-label">Vendor</span>
					<span class="detail-value">{snmp.vendor}</span>
				</div>
			{/if}
		</div>
	{/if}
{/if}

<style>
	.detail-loading {
		padding: 2rem;
		text-align: center;
		color: var(--gm-text-secondary);
		font-style: italic;
	}

	.detail-section {
		margin-top: 1.5rem;
		padding: 1rem;
		border-radius: 6px;
		background: #0f172a;
		border: 1px solid #1e293b;
	}

	.section-title {
		font-size: 0.875rem;
		font-weight: 600;
		margin-bottom: 1rem;
		margin-top: 0;
	}

	.modbus-title { color: var(--gm-modbus, #f59e0b); }
	.dnp3-title { color: var(--gm-dnp3, #10b981); }
	.ethernet-ip-title { color: var(--gm-ethernet-ip, #8b5cf6); }
	.s7-title { color: var(--gm-s7comm, #ef4444); }
	.bacnet-title { color: var(--gm-bacnet, #06b6d4); }
	.iec104-title { color: #14b8a6; }
	.profinet-title { color: #6366f1; }
	.lldp-title { color: #38bdf8; }
	.snmp-title { color: #a78bfa; }

	.detail-row {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.5rem 0;
	}

	.detail-label {
		font-weight: 600;
		color: #94a3b8;
		min-width: 100px;
	}

	.detail-value {
		color: #e2e8f0;
	}

	.role-badge {
		padding: 0.25rem 0.5rem;
		border-radius: 3px;
		background: #1e293b;
		color: #cbd5e1;
		font-weight: 500;
		font-size: 0.75rem;
	}

	.finding {
		color: #ef4444;
		font-weight: 600;
	}

	.warning {
		color: #f59e0b;
		font-weight: 600;
	}

	.highlight {
		color: #fbbf24;
		font-weight: 600;
	}

	.detail-subsection {
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px solid #1e293b;
	}

	.subsection-title {
		font-size: 0.8125rem;
		font-weight: 600;
		color: #cbd5e1;
		margin: 0 0 0.75rem 0;
	}

	.fc-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.fc-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.5rem;
		background: rgba(0, 0, 0, 0.2);
		border-radius: 3px;
		font-size: 0.8125rem;
		border-left: 2px solid #1e293b;
	}

	.fc-item.write {
		border-left-color: #ef4444;
		background: rgba(239, 68, 68, 0.1);
	}

	.fc-code {
		font-family: 'JetBrains Mono', monospace;
		font-weight: 600;
		color: #fbbf24;
		min-width: 50px;
	}

	.fc-name {
		color: #cbd5e1;
		flex: 1;
	}

	.fc-count {
		color: #94a3b8;
		font-size: 0.75rem;
	}

	.reg-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.reg-item {
		display: flex;
		gap: 0.75rem;
		padding: 0.4rem;
		background: rgba(0, 0, 0, 0.2);
		border-radius: 3px;
		font-size: 0.8125rem;
	}

	.reg-range {
		font-family: 'JetBrains Mono', monospace;
		color: #fbbf24;
		font-weight: 600;
		min-width: 80px;
	}

	.reg-type {
		color: #cbd5e1;
		flex: 1;
	}

	.reg-count {
		color: #94a3b8;
		font-size: 0.75rem;
	}
</style>

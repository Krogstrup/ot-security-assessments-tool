<script lang="ts">
	import type { DeepParseInfo, EnipDetail, S7Detail, BacnetDetail, Iec104Detail, ProfinetDcpDetail, LldpDetail } from '$lib/types/deep-parse';
	import ModbusSection from './deep-parse/ModbusSection.svelte';
	import Dnp3Section from './deep-parse/Dnp3Section.svelte';
	import S7Section from './deep-parse/S7Section.svelte';
	import RoleSection from './deep-parse/RoleSection.svelte';
	import LldpSection from './deep-parse/LldpSection.svelte';
	import SnmpSection from './deep-parse/SnmpSection.svelte';

	interface Props {
		deepParseInfo: DeepParseInfo | null;
		loading?: boolean;
	}

	let { deepParseInfo, loading = false }: Props = $props();
</script>

{#if loading}
	<div class="detail-loading">Loading deep parse data...</div>
{:else if deepParseInfo}
	{#if deepParseInfo.modbus}
		<ModbusSection modbus={deepParseInfo.modbus} />
	{/if}

	{#if deepParseInfo.dnp3}
		<Dnp3Section dnp3={deepParseInfo.dnp3} />
	{/if}

	{#if deepParseInfo.enip}
		{@const enip = deepParseInfo.enip as EnipDetail}
		<RoleSection
			title="EtherNet/IP + CIP"
			titleClass="ethernet-ip-title"
			role={enip.role}
			noteLabel={enip.cip_writes_to_assembly ? 'CIP Writes' : undefined}
			noteValue={enip.cip_writes_to_assembly ? 'Assembly object writes (T0855)' : undefined}
			noteClass="finding"
		/>
	{/if}

	{#if deepParseInfo.s7}
		{@const s7 = deepParseInfo.s7 as S7Detail}
		<S7Section {s7} />
	{/if}

	{#if deepParseInfo.bacnet}
		{@const bacnet = deepParseInfo.bacnet as BacnetDetail}
		<RoleSection title="BACnet/IP" titleClass="bacnet-title" role={bacnet.role} />
	{/if}

	{#if deepParseInfo.iec104}
		{@const iec104 = deepParseInfo.iec104 as Iec104Detail}
		<RoleSection title="IEC 60870-5-104" titleClass="iec104-title" role={iec104.role} />
	{/if}

	{#if deepParseInfo.profinet_dcp}
		{@const pndcp = deepParseInfo.profinet_dcp as ProfinetDcpDetail}
		<RoleSection title="PROFINET DCP" titleClass="profinet-title" role={pndcp.role} />
	{/if}

	{#if deepParseInfo.lldp}
		{@const lldp = deepParseInfo.lldp as LldpDetail}
		<LldpSection {lldp} />
	{/if}

	{#if deepParseInfo.snmp}
		<SnmpSection snmp={deepParseInfo.snmp} />
	{/if}
{/if}

<style>
	.detail-loading {
		padding: 2rem;
		text-align: center;
		color: var(--gm-text-secondary);
		font-style: italic;
	}
</style>

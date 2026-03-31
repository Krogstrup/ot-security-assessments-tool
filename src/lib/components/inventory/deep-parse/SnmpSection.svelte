<script lang="ts">
	import type { SnmpDetail } from '$lib/types/deep-parse';

	interface Props {
		snmp: SnmpDetail;
	}

	let { snmp }: Props = $props();
</script>

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

<style>
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

	.snmp-title {
		color: #a78bfa;
	}

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
</style>

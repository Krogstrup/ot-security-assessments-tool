<script lang="ts">
	import type { LldpDetail } from '$lib/types/deep-parse';

	interface Props {
		lldp: LldpDetail;
	}

	let { lldp }: Props = $props();
</script>

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

	.lldp-title {
		color: #38bdf8;
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

	.highlight {
		color: #fbbf24;
		font-weight: 600;
	}
</style>

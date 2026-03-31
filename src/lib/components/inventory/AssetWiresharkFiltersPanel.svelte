<script lang="ts">
	import type { Asset } from '$lib/types/assets';

	interface Props {
		asset: Asset;
	}

	let { asset }: Props = $props();
	let toastMessage = $state('');
	let toastTimer: ReturnType<typeof setTimeout> | null = null;

	function getOtPort(protocols: string[]): number | null {
		const portMap: Record<string, number> = {
			modbus: 502,
			dnp3: 20000,
			ethernet_ip: 44818,
			s7comm: 102,
			bacnet: 47808,
			iec104: 2404,
			profinet: 34964,
			opc_ua: 4840
		};
		for (const p of protocols) {
			const port = portMap[p.toLowerCase()];
			if (port) return port;
		}
		return null;
	}

	function showToast(msg: string) {
		toastMessage = msg;
		if (toastTimer) clearTimeout(toastTimer);
		toastTimer = setTimeout(() => {
			toastMessage = '';
		}, 2000);
	}

	function copyFilter(filter: string) {
		navigator.clipboard.writeText(filter).then(() => showToast('Copied!'));
	}
</script>

<div class="detail-section">
	<h4 class="section-title">Wireshark Filters</h4>
	<div class="filter-row">
		<code class="filter-code">ip.addr == {asset.ip_address}</code>
		<button class="copy-btn" onclick={() => copyFilter(`ip.addr == ${asset.ip_address}`)}>
			Copy
		</button>
	</div>
	{#if getOtPort(asset.protocols) !== null}
		{@const port = getOtPort(asset.protocols)}
		<div class="filter-row">
			<code class="filter-code">ip.addr == {asset.ip_address} && tcp.port == {port}</code>
			<button class="copy-btn" onclick={() => copyFilter(`ip.addr == ${asset.ip_address} && tcp.port == ${port}`)}>
				Copy
			</button>
		</div>
	{/if}
	{#if toastMessage}
		<div class="toast">{toastMessage}</div>
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
		color: #e2e8f0;
	}

	.filter-row {
		display: flex;
		gap: 0.75rem;
		align-items: center;
		margin-bottom: 0.75rem;
	}

	.filter-code {
		flex: 1;
		background: rgba(0, 0, 0, 0.3);
		padding: 0.5rem 0.75rem;
		border-radius: 4px;
		font-family: 'JetBrains Mono', monospace;
		font-size: 0.8125rem;
		color: #fbbf24;
		word-break: break-all;
		border: 1px solid #334155;
	}

	.copy-btn {
		padding: 0.4rem 0.8rem;
		font-size: 0.75rem;
		background: #6366f1;
		color: white;
		border: none;
		border-radius: 3px;
		cursor: pointer;
		transition: background 0.2s;
		white-space: nowrap;
	}

	.copy-btn:hover {
		background: #4f46e5;
	}

	.toast {
		margin-top: 0.75rem;
		padding: 0.5rem;
		background: #10b981;
		color: white;
		border-radius: 3px;
		text-align: center;
		font-size: 0.8125rem;
		animation: slideIn 0.2s ease-out;
	}

	@keyframes slideIn {
		from {
			opacity: 0;
			transform: translateY(-10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}
</style>

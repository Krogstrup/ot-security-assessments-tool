<script lang="ts">
	import type { DriftAsset } from '$lib/types/analysis';

	interface Props {
		title: string;
		variant: 'new' | 'missing';
		assets: DriftAsset[];
	}

	let { title, variant, assets }: Props = $props();
</script>

<div class="drift-section">
	<h3 class="section-title" class:section-new={variant === 'new'} class:section-missing={variant === 'missing'}>
		{title} ({assets.length})
	</h3>
	<div class="device-list">
		{#each assets as asset}
			<div class="device-card" class:device-new={variant === 'new'} class:device-missing={variant === 'missing'}>
				<div class="device-ip">{asset.ip_address}</div>
				<div class="device-meta">
					<span class="device-type">{asset.device_type}</span>
					{#if asset.vendor}
						<span class="device-vendor">{asset.vendor}</span>
					{/if}
					{#if asset.mac_address}
						<span class="device-mac">{asset.mac_address}</span>
					{/if}
				</div>
				{#if asset.protocols.length > 0}
					<div class="device-protocols">
						{#each asset.protocols as proto}
							<span class="protocol-tag">{proto}</span>
						{/each}
					</div>
				{/if}
			</div>
		{/each}
	</div>
</div>

<style>
	.drift-section {
		margin-bottom: 24px;
	}

	.section-title {
		font-size: 13px;
		font-weight: 600;
		margin: 0 0 10px;
		padding-left: 10px;
		border-left: 3px solid var(--gm-border);
	}

	.section-title.section-new {
		color: #10b981;
		border-left-color: #10b981;
	}

	.section-title.section-missing {
		color: #ef4444;
		border-left-color: #ef4444;
	}

	.device-list {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.device-card {
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 8px;
		padding: 12px 14px;
	}

	.device-card.device-new {
		border-left: 3px solid #10b981;
	}

	.device-card.device-missing {
		border-left: 3px solid #ef4444;
	}

	.device-ip {
		font-size: 12px;
		font-weight: 600;
		color: var(--gm-text-primary);
		margin-bottom: 6px;
	}

	.device-meta {
		display: flex;
		align-items: center;
		gap: 10px;
		margin-bottom: 6px;
	}

	.device-type {
		font-size: 10px;
		padding: 1px 6px;
		border-radius: 3px;
		background: var(--gm-bg-hover);
		color: var(--gm-text-secondary);
		text-transform: capitalize;
	}

	.device-vendor,
	.device-mac {
		font-size: 10px;
		color: var(--gm-text-muted);
	}

	.device-protocols {
		display: flex;
		gap: 6px;
		flex-wrap: wrap;
	}

	.protocol-tag {
		font-size: 9px;
		padding: 1px 6px;
		border-radius: 3px;
		background: rgba(16, 185, 129, 0.1);
		color: #10b981;
		text-transform: uppercase;
		letter-spacing: 0.3px;
	}
</style>

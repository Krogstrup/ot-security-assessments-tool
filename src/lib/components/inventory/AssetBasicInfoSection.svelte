<script lang="ts">
	import type { Asset } from '$lib/types';
	import { DEVICE_TYPE_LABELS, PURDUE_LABELS, CONFIDENCE_LABELS, CONFIDENCE_COLORS } from '$lib/constants';

	interface Props {
		asset: Asset;
	}

	let { asset }: Props = $props();

	function countryFlag(code: string): string {
		const base = 0x1f1e6;
		const a = code.charCodeAt(0) - 65;
		const b = code.charCodeAt(1) - 65;
		return String.fromCodePoint(base + a) + String.fromCodePoint(base + b);
	}
</script>

<div class="detail-section">
	<div class="detail-row">
		<span class="detail-label">Type</span>
		<span class="detail-value">{DEVICE_TYPE_LABELS[asset.device_type] ?? asset.device_type}</span>
	</div>
	{#if asset.hostname}
		<div class="detail-row">
			<span class="detail-label">Hostname</span>
			<span class="detail-value">{asset.hostname}</span>
		</div>
	{/if}
	{#if asset.vendor}
		<div class="detail-row">
			<span class="detail-label">Vendor</span>
			<span class="detail-value">{asset.vendor}</span>
		</div>
	{/if}
	{#if asset.oui_vendor}
		<div class="detail-row">
			<span class="detail-label">OUI Vendor</span>
			<span class="detail-value">{asset.oui_vendor}</span>
		</div>
	{/if}
	{#if asset.product_family}
		<div class="detail-row">
			<span class="detail-label">Product</span>
			<span class="detail-value">{asset.product_family}</span>
		</div>
	{/if}
	<div class="detail-row">
		<span class="detail-label">Confidence</span>
		<span class="detail-value">
			{#if asset.confidence > 0}
				<span
					class="confidence-badge"
					style="color: {CONFIDENCE_COLORS[asset.confidence] ?? '#64748b'};
					       background: {(CONFIDENCE_COLORS[asset.confidence] ?? '#64748b')}18"
				>
					{asset.confidence}/5 ({CONFIDENCE_LABELS[asset.confidence]})
				</span>
			{:else}
				—
			{/if}
		</span>
	</div>
	{#if asset.purdue_level != null}
		<div class="detail-row">
			<span class="detail-label">Purdue</span>
			<span class="detail-value">{PURDUE_LABELS[asset.purdue_level] ?? `L${asset.purdue_level}`}</span>
		</div>
	{/if}
	{#if asset.country}
		<div class="detail-row">
			<span class="detail-label">Country</span>
			<span class="detail-value">{countryFlag(asset.country)} {asset.country}</span>
		</div>
	{/if}
	{#if asset.is_public_ip}
		<div class="detail-row">
			<span class="detail-label">Public IP</span>
			<span class="detail-value finding">Yes — unexpected for OT</span>
		</div>
	{/if}
	{#if asset.tags.length > 0}
		<div class="detail-row">
			<span class="detail-label">Tags</span>
			<span class="detail-value">
				{#each asset.tags as tag}
					<span class="tag-badge">{tag}</span>
				{/each}
			</span>
		</div>
	{/if}
	{#if asset.notes}
		<div class="detail-row notes-row">
			<span class="detail-label">Notes</span>
			<span class="detail-value notes-text">{asset.notes}</span>
		</div>
	{/if}

	{#if asset.signature_matches.length > 0}
		<div class="detail-subsection">
			<h5 class="subsection-title">Confidence Breakdown</h5>
			{#each asset.signature_matches as match}
				<div class="confidence-row">
					<span
						class="confidence-badge small"
						style="color: {CONFIDENCE_COLORS[match.confidence] ?? '#64748b'};
						       background: {(CONFIDENCE_COLORS[match.confidence] ?? '#64748b')}18"
					>
						{match.confidence}
					</span>
					<span class="match-name">{match.signature_name}</span>
					{#if match.vendor}
						<span class="match-vendor">{match.vendor}</span>
					{/if}
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.detail-section {
		padding: 1rem;
		border-radius: 6px;
		background: #0f172a;
		border: 1px solid #1e293b;
	}

	.detail-row {
		display: flex;
		align-items: flex-start;
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
		flex: 1;
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		align-items: center;
	}

	.detail-value.finding {
		color: #ef4444;
	}

	.confidence-badge {
		padding: 0.25rem 0.5rem;
		border-radius: 3px;
		font-size: 0.75rem;
		font-weight: 600;
	}

	.confidence-badge.small {
		font-size: 0.7rem;
	}

	.tag-badge {
		padding: 0.25rem 0.5rem;
		border-radius: 3px;
		background: #1e293b;
		color: #cbd5e1;
		font-size: 0.75rem;
		border: 1px solid #334155;
	}

	.notes-row {
		align-items: flex-start;
	}

	.notes-text {
		white-space: pre-wrap;
		word-break: break-word;
		font-size: 0.875rem;
		line-height: 1.5;
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

	.confidence-row {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.4rem 0;
		font-size: 0.8125rem;
	}

	.match-name {
		color: #cbd5e1;
		flex: 1;
	}

	.match-vendor {
		color: #94a3b8;
		font-style: italic;
		font-size: 0.75rem;
	}
</style>

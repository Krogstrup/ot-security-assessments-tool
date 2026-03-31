<script lang="ts">
	import type { SignatureInfo } from '$lib/types/signatures';
	import { CONFIDENCE_COLORS as confidenceColors } from '$lib/constants';

	interface Props {
		signatures: SignatureInfo[];
		selectedSignature: string | null;
		onSelect: (name: string) => void;
	}

	let { signatures, selectedSignature, onSelect }: Props = $props();
</script>

<div class="sig-list-panel">
	<div class="sig-list-header">Loaded Signatures</div>
	<div class="sig-list">
		{#each signatures as sig}
			<button
				class="sig-list-item"
				class:active={selectedSignature === sig.name}
				onclick={() => onSelect(sig.name)}
			>
				<div class="sig-item-name">{sig.name}</div>
				<div class="sig-item-meta">
					<span
						class="confidence-dot"
						style="background: {confidenceColors[sig.confidence] ?? '#64748b'}"
						title="Confidence: {sig.confidence}"
					></span>
					{#if sig.vendor}
						<span class="sig-item-vendor">{sig.vendor}</span>
					{/if}
					{#if sig.protocol}
						<span class="sig-item-proto">{sig.protocol}</span>
					{/if}
				</div>
			</button>
		{/each}
		{#if signatures.length === 0}
			<div class="sig-list-empty">No signatures loaded</div>
		{/if}
	</div>
</div>

<style>
	.sig-list-panel {
		width: 260px;
		min-width: 200px;
		flex-shrink: 0;
		display: flex;
		flex-direction: column;
		border-right: 1px solid var(--gm-border);
		background: var(--gm-bg-secondary);
	}

	.sig-list-header {
		padding: 8px 12px;
		font-size: 10px;
		font-weight: 600;
		letter-spacing: 0.5px;
		text-transform: uppercase;
		color: var(--gm-text-muted);
		border-bottom: 1px solid var(--gm-border);
	}

	.sig-list {
		flex: 1;
		overflow-y: auto;
	}

	.sig-list-item {
		display: block;
		width: 100%;
		padding: 8px 12px;
		background: none;
		border: none;
		border-bottom: 1px solid rgba(45, 58, 79, 0.3);
		text-align: left;
		cursor: pointer;
		transition: background 0.1s;
	}

	.sig-list-item:hover {
		background: var(--gm-bg-hover);
	}

	.sig-list-item.active {
		background: rgba(59, 130, 246, 0.1);
		border-left: 2px solid #3b82f6;
	}

	.sig-item-name {
		font-size: 11px;
		font-weight: 500;
		color: var(--gm-text-primary);
		margin-bottom: 3px;
		font-family: inherit;
	}

	.sig-item-meta {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 9px;
		color: var(--gm-text-muted);
	}

	.confidence-dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		flex-shrink: 0;
	}

	.sig-item-vendor {
		color: var(--gm-text-secondary);
	}

	.sig-item-proto {
		background: rgba(100, 116, 139, 0.15);
		padding: 1px 5px;
		border-radius: 2px;
	}

	.sig-list-empty {
		padding: 20px 12px;
		text-align: center;
		color: var(--gm-text-muted);
		font-size: 11px;
	}
</style>

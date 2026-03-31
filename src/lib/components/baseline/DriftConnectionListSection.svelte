<script lang="ts">
	import type { DriftConnection } from '$lib/types/analysis';

	interface Props {
		title: string;
		variant: 'new' | 'missing';
		connections: DriftConnection[];
	}

	let { title, variant, connections }: Props = $props();
</script>

<div class="drift-section">
	<h3 class="section-title" class:section-new={variant === 'new'} class:section-missing={variant === 'missing'}>
		{title} ({connections.length})
	</h3>
	<div class="connection-list">
		{#each connections as conn}
			<div class="connection-card" class:connection-new={variant === 'new'} class:connection-missing={variant === 'missing'}>
				<span class="conn-src">{conn.src_ip}:{conn.src_port}</span>
				<span class="conn-arrow">&rarr;</span>
				<span class="conn-dst">{conn.dst_ip}:{conn.dst_port}</span>
				<span class="conn-proto">{conn.protocol}</span>
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

	.connection-list {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.connection-card {
		display: flex;
		align-items: center;
		gap: 8px;
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 6px;
		padding: 8px 12px;
		font-size: 11px;
	}

	.connection-card.connection-new {
		border-left: 3px solid #10b981;
	}

	.connection-card.connection-missing {
		border-left: 3px solid #ef4444;
	}

	.conn-src,
	.conn-dst {
		color: var(--gm-text-primary);
		font-weight: 600;
	}

	.conn-arrow {
		color: var(--gm-text-muted);
	}

	.conn-proto {
		margin-left: auto;
		font-size: 9px;
		padding: 1px 6px;
		border-radius: 3px;
		background: var(--gm-bg-hover);
		color: var(--gm-text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.3px;
	}
</style>

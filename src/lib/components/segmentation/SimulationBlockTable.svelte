<script lang="ts">
	import type { BlockedConnection } from '$lib/types/segmentation';

	interface Props {
		title: string;
		items: BlockedConnection[];
		visibleCount: number;
		onShowMore: () => void;
		showReason?: boolean;
		countBadgeClass?: string;
		hint?: string;
	}

	let { title, items, visibleCount, onShowMore, showReason = false, countBadgeClass = '', hint }: Props = $props();
</script>

<h3>{title} <span class="count-badge {countBadgeClass}">{items.length.toLocaleString()}</span></h3>
{#if hint}
	<p class="hint">{hint}</p>
{/if}
<div class="paginated-table-container">
	<table class="data-table">
		<thead>
			<tr>
				<th>Source</th><th>Destination</th><th>Protocol:Port</th>
				{#if showReason}<th>Reason</th>{/if}
			</tr>
		</thead>
		<tbody>
			{#each items.slice(0, visibleCount) as item}
				<tr>
					<td>{item.src_ip}</td>
					<td>{item.dst_ip}</td>
					<td>{item.protocol}:{item.dst_port}</td>
					{#if showReason}<td class="reason">{item.reason}</td>{/if}
				</tr>
			{/each}
			{#if items.length > visibleCount}
				<tr>
					<td colspan={showReason ? 4 : 3} class="show-more-row">
						<button class="show-more-btn" onclick={onShowMore}>
							Show more ({(items.length - visibleCount).toLocaleString()} remaining)
						</button>
					</td>
				</tr>
			{/if}
		</tbody>
	</table>
	{#if items.length > 50}
		<p class="table-count">Showing {Math.min(visibleCount, items.length).toLocaleString()} of {items.length.toLocaleString()}</p>
	{/if}
</div>

<style>
	h3 {
		margin: 1.5rem 0 0.75rem;
		font-size: 1rem;
		font-weight: 600;
	}

	.count-badge {
		background: #ef444422;
		color: #ef4444;
		border-radius: 3px;
		padding: 0.1rem 0.4rem;
		font-size: 0.8rem;
		font-weight: 600;
	}

	.count-badge.fp {
		background: #ca8a0422;
		color: #ca8a04;
	}

	.hint {
		font-size: 0.82rem;
		color: var(--text-muted, #888);
	}

	.data-table {
		width: 100%;
		border-collapse: collapse;
		margin-bottom: 1.5rem;
		font-size: 0.85rem;
	}

	.data-table th {
		text-align: left;
		padding: 0.5rem 0.75rem;
		border-bottom: 1px solid var(--border, #333);
		color: var(--text-muted, #888);
		font-weight: 500;
		position: sticky;
		top: 0;
		background: var(--gm-bg-secondary, #1a2332);
		z-index: 1;
	}

	.data-table td {
		padding: 0.4rem 0.75rem;
		border-bottom: 1px solid var(--border-subtle, #222);
	}

	.reason {
		color: var(--text-muted, #888);
		font-size: 0.82rem;
	}

	.paginated-table-container {
		margin-bottom: 1.5rem;
	}

	.show-more-row {
		text-align: center;
		padding: 0.5rem;
	}

	.show-more-btn {
		background: var(--surface-2, #1e1e1e);
		border: 1px solid var(--border, #333);
		color: var(--text-muted, #888);
		padding: 0.4rem 1rem;
		border-radius: 4px;
		cursor: pointer;
		font-size: 0.82rem;
		transition: all 0.15s;
	}

	.show-more-btn:hover {
		background: var(--gm-bg-hover, #252d3a);
		color: var(--text, #eee);
	}

	.table-count {
		font-size: 0.78rem;
		color: var(--text-muted, #888);
		text-align: right;
		margin: 0.25rem 0 0;
	}
</style>

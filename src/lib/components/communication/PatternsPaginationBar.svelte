<script lang="ts">
	interface Props {
		totalPages: number;
		currentPage: number;
		pageSize: number;
		filteredCount: number;
		onPageChange: (page: number) => void;
	}

	let { totalPages, currentPage, pageSize, filteredCount, onPageChange }: Props = $props();
</script>

{#if totalPages > 1}
	<div class="pagination-bar">
		<button class="page-btn" disabled={currentPage === 0} onclick={() => onPageChange(0)}>«</button>
		<button class="page-btn" disabled={currentPage === 0} onclick={() => onPageChange(currentPage - 1)}>‹</button>
		<span class="page-info">
			Page {currentPage + 1} of {totalPages}
			&nbsp;·&nbsp;
			rows {(currentPage * pageSize + 1).toLocaleString()}–{Math.min((currentPage + 1) * pageSize, filteredCount).toLocaleString()}
			of {filteredCount.toLocaleString()}
		</span>
		<button
			class="page-btn"
			disabled={currentPage >= totalPages - 1}
			onclick={() => onPageChange(currentPage + 1)}
		>
			›
		</button>
		<button
			class="page-btn"
			disabled={currentPage >= totalPages - 1}
			onclick={() => onPageChange(totalPages - 1)}
		>
			»
		</button>
	</div>
{/if}

<style>
	.pagination-bar {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 6px;
		padding: 8px 16px;
		border-top: 1px solid var(--gm-border);
		background: var(--gm-bg-secondary);
		flex-shrink: 0;
	}

	.page-btn {
		padding: 3px 10px;
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		color: var(--gm-text-secondary);
		font-family: inherit;
		font-size: 12px;
		cursor: pointer;
		transition: all 0.15s;
	}

	.page-btn:hover:not(:disabled) {
		background: var(--gm-bg-hover);
		color: var(--gm-text-primary);
	}

	.page-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.page-info {
		font-size: 11px;
		color: var(--gm-text-muted);
		white-space: nowrap;
	}
</style>

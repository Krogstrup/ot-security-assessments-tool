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
	<div class="inv-pagination">
		<button class="inv-page-btn" disabled={currentPage === 0} onclick={() => onPageChange(0)}>«</button>
		<button class="inv-page-btn" disabled={currentPage === 0} onclick={() => onPageChange(currentPage - 1)}>‹</button>
		<span class="inv-page-info">
			{currentPage + 1} / {totalPages}
			&nbsp;·&nbsp;
			{currentPage * pageSize + 1}–{Math.min((currentPage + 1) * pageSize, filteredCount)} of {filteredCount}
		</span>
		<button
			class="inv-page-btn"
			disabled={currentPage >= totalPages - 1}
			onclick={() => onPageChange(currentPage + 1)}
		>
			›
		</button>
		<button
			class="inv-page-btn"
			disabled={currentPage >= totalPages - 1}
			onclick={() => onPageChange(totalPages - 1)}
		>
			»
		</button>
	</div>
{/if}

<style>
	.inv-pagination {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.75rem;
		background: var(--gm-bg-secondary);
		border-top: 1px solid var(--gm-border);
	}

	.inv-page-btn {
		padding: 0.35rem 0.5rem;
		background: transparent;
		border: 1px solid var(--gm-border);
		color: var(--gm-text-primary);
		border-radius: 3px;
		cursor: pointer;
		font-size: 0.75rem;
		transition: all 0.15s;
	}

	.inv-page-btn:hover:not(:disabled) {
		background: rgba(99, 102, 241, 0.2);
		border-color: #6366f1;
	}

	.inv-page-btn:disabled {
		opacity: 0.3;
		cursor: not-allowed;
	}

	.inv-page-info {
		color: var(--gm-text-secondary);
		font-size: 0.8rem;
		min-width: 200px;
		text-align: center;
	}
</style>

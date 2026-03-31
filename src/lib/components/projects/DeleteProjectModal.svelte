<script lang="ts">
	interface Props {
		show: boolean;
		onCancel: () => void;
		onConfirm: () => void | Promise<void>;
	}

	let { show, onCancel, onConfirm }: Props = $props();
</script>

{#if show}
	<div class="modal-backdrop" role="dialog" aria-modal="true">
		<div class="modal modal-sm">
			<div class="modal-header">
				<h2 class="modal-title">Delete Project</h2>
			</div>
			<div class="modal-body">
				<p class="confirm-text">
					This will permanently delete the project and <strong>all its sessions</strong>,
					including captured assets, connections, and findings. This cannot be undone.
				</p>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={onCancel}>Cancel</button>
				<button class="btn-danger" onclick={onConfirm}>Delete</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.modal-backdrop {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}

	.modal {
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 8px;
		max-width: 95vw;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.modal-sm {
		width: 400px;
	}

	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 18px 20px 14px;
		border-bottom: 1px solid var(--gm-border);
	}

	.modal-title {
		font-size: 14px;
		font-weight: 700;
		color: var(--gm-text-primary);
		margin: 0;
	}

	.modal-body {
		padding: 20px;
		display: flex;
		flex-direction: column;
		gap: 14px;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 8px;
		padding: 14px 20px;
		border-top: 1px solid var(--gm-border);
	}

	.btn-secondary {
		padding: 8px 16px;
		background: transparent;
		color: var(--gm-text-secondary);
		border: 1px solid var(--gm-border);
		border-radius: 5px;
		font-family: inherit;
		font-size: 12px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-secondary:hover {
		background: var(--gm-bg-hover);
		color: var(--gm-text-primary);
	}

	.btn-danger {
		padding: 8px 16px;
		background: rgba(239, 68, 68, 0.15);
		color: #ef4444;
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 5px;
		font-family: inherit;
		font-size: 12px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-danger:hover {
		background: rgba(239, 68, 68, 0.25);
	}

	.confirm-text {
		font-size: 13px;
		color: var(--gm-text-secondary);
		line-height: 1.6;
		margin: 0;
	}

	.confirm-text strong {
		color: var(--gm-text-primary);
	}
</style>

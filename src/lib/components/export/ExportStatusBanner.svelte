<script lang="ts">
	interface Props {
		message: string;
		type: 'success' | 'error' | 'info';
		onDismiss: () => void;
	}

	let { message, type, onDismiss }: Props = $props();
</script>

{#if message}
	<div class="status-banner" class:success={type === 'success'} class:error={type === 'error'} class:info={type === 'info'}>
		<span>{message}</span>
		<button class="status-dismiss" onclick={onDismiss}>&times;</button>
	</div>
{/if}

<style>
	.status-banner {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 10px 14px;
		border-radius: 6px;
		font-size: 11px;
		font-weight: 500;
	}

	.status-banner.success {
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.25);
		color: #10b981;
	}

	.status-banner.error {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.25);
		color: #ef4444;
	}

	.status-banner.info {
		background: rgba(59, 130, 246, 0.1);
		border: 1px solid rgba(59, 130, 246, 0.25);
		color: #3b82f6;
	}

	.status-dismiss {
		background: none;
		border: none;
		color: currentColor;
		font-size: 16px;
		font-weight: 400;
		line-height: 1;
		cursor: pointer;
		opacity: 0.7;
		padding: 0;
	}

	.status-dismiss:hover {
		opacity: 1;
	}
</style>

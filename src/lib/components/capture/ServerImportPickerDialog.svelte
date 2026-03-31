<script lang="ts">
	import ServerPickerFileList from './ServerPickerFileList.svelte';

	interface Props {
		show: boolean;
		loading: boolean;
		error: string;
		baseDir: string;
		files: Array<{ name: string; path: string; size_bytes: number }>;
		selectedPaths: string[];
		listLimit: number;
		truncated: boolean;
		title: string;
		allowMultiple: boolean;
		onTogglePath: (path: string, checked: boolean) => void;
		onClose: (result: string[] | null) => void;
	}

	let {
		show,
		loading,
		error,
		baseDir,
		files,
		selectedPaths,
		listLimit,
		truncated,
		title,
		allowMultiple,
		onTogglePath,
		onClose
	}: Props = $props();
</script>

{#if show}
	<div
		class="server-picker-overlay"
		role="button"
		tabindex="0"
		onclick={() => onClose(null)}
		onkeydown={(e) => {
			if (e.key === 'Escape' || e.key === 'Enter' || e.key === ' ') {
				onClose(null);
			}
		}}
	>
		<div
			class="server-picker-dialog"
			role="dialog"
			aria-modal="true"
			tabindex="-1"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
		>
			<div class="server-picker-header">
				<h4 class="server-picker-title">{title}</h4>
				<button class="server-picker-close" onclick={() => onClose(null)}>&times;</button>
			</div>
			<div class="server-picker-subtitle">
				Fixed import directory: <code>{baseDir || '(loading...)'}</code>
			</div>
			{#if truncated}
				<div class="server-picker-warning">
					Showing first {files.length} files (limit {listLimit}). Narrow
					<code>KK_HEADLESS_IMPORTS_ROOT</code> or increase
					<code>KK_HEADLESS_IMPORT_LIST_LIMIT</code> on the server.
				</div>
			{/if}

			{#if loading}
				<div class="server-picker-empty">Loading server files...</div>
			{:else if error}
				<div class="import-result error">{error}</div>
			{:else if files.length === 0}
				<div class="server-picker-empty">No importable PCAP files found.</div>
			{:else}
				<ServerPickerFileList {files} {selectedPaths} {allowMultiple} {onTogglePath} />
			{/if}

			<div class="server-picker-actions">
				<button class="action-btn secondary" onclick={() => onClose(null)}>Cancel</button>
				<button
					class="action-btn primary"
					disabled={selectedPaths.length === 0}
					onclick={() => onClose(selectedPaths)}
				>
					Import {selectedPaths.length > 0 ? `(${selectedPaths.length})` : ''}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.server-picker-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}

	.server-picker-dialog {
		background: var(--gm-bg-secondary);
		border: 1px solid var(--gm-border);
		border-radius: 6px;
		max-width: 600px;
		max-height: 80vh;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.server-picker-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		border-bottom: 1px solid var(--gm-border);
	}

	.server-picker-title {
		margin: 0;
		font-size: 1rem;
		color: var(--gm-text-primary);
	}

	.server-picker-close {
		background: none;
		border: none;
		color: var(--gm-text-secondary);
		cursor: pointer;
		font-size: 1.5rem;
	}

	.server-picker-subtitle {
		padding: 0.75rem 1rem;
		font-size: 0.8125rem;
		color: var(--gm-text-secondary);
		background: rgba(0, 0, 0, 0.2);
	}

	.server-picker-subtitle code {
		background: rgba(0, 0, 0, 0.4);
		padding: 0.2rem 0.4rem;
		border-radius: 2px;
		font-family: 'JetBrains Mono', monospace;
		font-size: 0.75rem;
	}

	.server-picker-warning {
		padding: 0.75rem 1rem;
		background: rgba(245, 158, 11, 0.1);
		color: #f59e0b;
		font-size: 0.8125rem;
		border-top: 1px solid rgba(245, 158, 11, 0.3);
	}

	.server-picker-actions {
		display: flex;
		gap: 0.5rem;
		justify-content: flex-end;
		padding: 1rem;
	}

	.action-btn {
		padding: 0.5rem 1rem;
		border: none;
		border-radius: 4px;
		font-size: 0.8125rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn.primary {
		background: #6366f1;
		color: white;
	}

	.action-btn.primary:hover:not(:disabled) {
		background: #4f46e5;
	}

	.action-btn.secondary {
		background: var(--gm-bg-tertiary);
		color: var(--gm-text-primary);
		border: 1px solid var(--gm-border);
	}

	.action-btn.secondary:hover {
		background: rgba(255, 255, 255, 0.05);
	}

	.action-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.server-picker-empty {
		padding: 2rem;
		text-align: center;
		color: var(--gm-text-secondary);
		font-size: 0.8125rem;
	}

	.import-result {
		margin-top: 1rem;
		padding: 0.75rem;
		border-radius: 4px;
		font-size: 0.8125rem;
	}

	.import-result.error {
		background: rgba(239, 68, 68, 0.1);
		color: #ef4444;
		border: 1px solid rgba(239, 68, 68, 0.3);
	}
</style>

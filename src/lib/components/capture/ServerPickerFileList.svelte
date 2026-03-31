<script lang="ts">
	import { formatBytes } from './captureViewFormatters';

	interface Props {
		files: Array<{ name: string; path: string; size_bytes: number }>;
		selectedPaths: string[];
		allowMultiple: boolean;
		onTogglePath: (path: string, checked: boolean) => void;
	}

	let { files, selectedPaths, allowMultiple, onTogglePath }: Props = $props();
</script>

<div class="server-picker-list">
	{#each files as file}
		<label class="server-picker-row">
			<input
				type={allowMultiple ? 'checkbox' : 'radio'}
				name="server-import-file"
				checked={selectedPaths.includes(file.path)}
				onchange={(e) => onTogglePath(file.path, (e.currentTarget as HTMLInputElement).checked)}
			/>
			<span class="server-picker-name">{file.name}</span>
			<span class="server-picker-size">{formatBytes(file.size_bytes)}</span>
		</label>
	{/each}
</div>

<style>
	.server-picker-list {
		flex: 1;
		overflow: auto;
		border-bottom: 1px solid var(--gm-border);
	}

	.server-picker-row {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.5rem 1rem;
		border-bottom: 1px solid var(--gm-border);
		cursor: pointer;
		transition: background 0.1s;
	}

	.server-picker-row:hover {
		background: rgba(255, 255, 255, 0.05);
	}

	.server-picker-row input {
		cursor: pointer;
	}

	.server-picker-name {
		flex: 1;
		color: var(--gm-text-primary);
		font-size: 0.8125rem;
	}

	.server-picker-size {
		color: var(--gm-text-secondary);
		font-size: 0.75rem;
		min-width: 60px;
		text-align: right;
	}
</style>

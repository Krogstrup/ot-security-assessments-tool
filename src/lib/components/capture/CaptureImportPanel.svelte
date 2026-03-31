<script lang="ts">
	import { isTauriRuntime } from '$lib/api';
	import { openPathDialog, savePathDialog } from '$lib/utils/dialog';
	import type { ImportProgressEvent, HeadlessImportKind, FileImportResult } from '$lib/types';

	interface Props {
		importStatus: 'idle' | 'importing' | 'done' | 'error';
		importMessage: string;
		fileResults: FileImportResult[];
		totalStats: { packets: number; assets: number; connections: number; ms: number; files: number };
		importProgress: ImportProgressEvent | null;
		showServerPicker: boolean;
		serverPickerLoading: boolean;
		serverPickerError: string;
		serverPickerBaseDir: string;
		serverPickerFiles: Array<{ name: string; path: string; size_bytes: number }>;
		selectedServerPaths: string[];
		serverPickerListLimit: number;
		serverPickerTruncated: boolean;
		serverPickerTitle: string;
		serverPickerAllowMultiple: boolean;
		onImport: () => Promise<void>;
		onCancelImport: () => Promise<void>;
		onToggleServerPath: (path: string, checked: boolean) => void;
		onCloseServerPicker: (result: string[] | null) => void;
	}

	let {
		importStatus,
		importMessage,
		fileResults,
		totalStats,
		importProgress,
		showServerPicker,
		serverPickerLoading,
		serverPickerError,
		serverPickerBaseDir,
		serverPickerFiles,
		selectedServerPaths,
		serverPickerListLimit,
		serverPickerTruncated,
		serverPickerTitle,
		serverPickerAllowMultiple,
		onImport,
		onCancelImport,
		onToggleServerPath,
		onCloseServerPicker
	}: Props = $props();

	function formatBytes(bytes: number): string {
		if (bytes < 1024) return `${bytes} B`;
		if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
		if (bytes < 1024 * 1024 * 1024) return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
		return `${(bytes / 1024 / 1024 / 1024).toFixed(2)} GB`;
	}
</script>

<section class="capture-section">
	<h3 class="section-title">PCAP Import</h3>
	<p class="section-desc">
		Import one or more PCAP/PCAPNG files captured from an OT network. Multiple files can be
		selected simultaneously — all traffic is merged into a single topology with per-file attribution.
	</p>

	<button class="action-btn primary" onclick={onImport} disabled={importStatus === 'importing'}>
		{importStatus === 'importing' ? 'Importing...' : 'Import PCAP Files'}
	</button>

	{#if showServerPicker}
		<div
			class="server-picker-overlay"
			role="button"
			tabindex="0"
			onclick={() => onCloseServerPicker(null)}
			onkeydown={(e) => {
				if (e.key === 'Escape' || e.key === 'Enter' || e.key === ' ') {
					onCloseServerPicker(null);
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
					<h4 class="server-picker-title">{serverPickerTitle}</h4>
					<button class="server-picker-close" onclick={() => onCloseServerPicker(null)}>&times;</button>
				</div>
				<div class="server-picker-subtitle">
					Fixed import directory: <code>{serverPickerBaseDir || '(loading...)'}</code>
				</div>
				{#if serverPickerTruncated}
					<div class="server-picker-warning">
						Showing first {serverPickerFiles.length} files (limit {serverPickerListLimit}). Narrow
						<code>KK_HEADLESS_IMPORTS_ROOT</code> or increase
						<code>KK_HEADLESS_IMPORT_LIST_LIMIT</code> on the server.
					</div>
				{/if}

				{#if serverPickerLoading}
					<div class="server-picker-empty">Loading server files...</div>
				{:else if serverPickerError}
					<div class="import-result error">{serverPickerError}</div>
				{:else if serverPickerFiles.length === 0}
					<div class="server-picker-empty">No importable PCAP files found.</div>
				{:else}
					<div class="server-picker-list">
						{#each serverPickerFiles as file}
							<label class="server-picker-row">
								<input
									type={serverPickerAllowMultiple ? 'checkbox' : 'radio'}
									name="server-import-file"
									checked={selectedServerPaths.includes(file.path)}
									onchange={(e) =>
										onToggleServerPath(file.path, (e.currentTarget as HTMLInputElement).checked)}
								/>
								<span class="server-picker-name">{file.name}</span>
								<span class="server-picker-size">{formatBytes(file.size_bytes)}</span>
							</label>
						{/each}
					</div>
				{/if}

				<div class="server-picker-actions">
					<button class="action-btn secondary" onclick={() => onCloseServerPicker(null)}>
						Cancel
					</button>
					<button
						class="action-btn primary"
						disabled={selectedServerPaths.length === 0}
						onclick={() => onCloseServerPicker(selectedServerPaths)}
					>
						Import {selectedServerPaths.length > 0 ? `(${selectedServerPaths.length})` : ''}
					</button>
				</div>
			</div>
		</div>
	{/if}

	{#if importProgress}
		<div class="import-progress">
			<div class="progress-bar" style="width: {importProgress.percent}%"></div>
			<span class="progress-text">
				{importProgress.current_file} ({importProgress.percent}%) — {importProgress.packets_so_far.toLocaleString()} packets
			</span>
		</div>
	{/if}

	{#if importStatus === 'error'}
		<div class="import-result error">{importMessage}</div>
	{:else if importStatus === 'done'}
		<div class="import-result success">{importMessage}</div>
		{#if fileResults.length > 0}
			<div class="file-results">
				<h4>Results</h4>
				<table class="results-table">
					<thead>
						<tr>
							<th>File</th>
							<th>Packets</th>
							<th>Assets</th>
							<th>Connections</th>
							<th>Status</th>
						</tr>
					</thead>
					<tbody>
						{#each fileResults as result}
							<tr>
								<td class="file-name">{result.filename}</td>
								<td class="file-packets">{result.packet_count.toLocaleString()}</td>
								<td class="file-assets">{result.asset_count}</td>
								<td class="file-connections">{result.connection_count}</td>
								<td>
									<span class="status-badge" class:success={result.success} class:error={!result.success}>
										{result.success ? 'OK' : 'FAILED'}
									</span>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	{:else if importMessage}
		<div class="import-result info">{importMessage}</div>
	{/if}
</section>

<style>
	.capture-section {
		padding: 1.5rem;
		background: var(--gm-bg-secondary);
		border: 1px solid var(--gm-border);
		border-radius: 6px;
		margin-bottom: 1rem;
	}

	.section-title {
		font-size: 1.125rem;
		font-weight: 600;
		margin-bottom: 0.5rem;
		color: var(--gm-text-primary);
	}

	.section-desc {
		font-size: 0.8125rem;
		color: var(--gm-text-secondary);
		margin-bottom: 1rem;
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

	.action-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

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

	.server-picker-actions {
		display: flex;
		gap: 0.5rem;
		justify-content: flex-end;
		padding: 1rem;
	}

	.action-btn.secondary {
		background: var(--gm-bg-tertiary);
		color: var(--gm-text-primary);
		border: 1px solid var(--gm-border);
	}

	.action-btn.secondary:hover {
		background: rgba(255, 255, 255, 0.05);
	}

	.import-progress {
		margin-top: 1rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.3);
		border-radius: 4px;
		overflow: hidden;
	}

	.progress-bar {
		height: 4px;
		background: #6366f1;
		transition: width 0.2s;
	}

	.progress-text {
		display: block;
		padding: 0.5rem;
		font-size: 0.75rem;
		color: var(--gm-text-secondary);
	}

	.import-result {
		margin-top: 1rem;
		padding: 0.75rem;
		border-radius: 4px;
		font-size: 0.8125rem;
	}

	.import-result.success {
		background: rgba(34, 197, 94, 0.1);
		color: #22c55e;
		border: 1px solid rgba(34, 197, 94, 0.3);
	}

	.import-result.error {
		background: rgba(239, 68, 68, 0.1);
		color: #ef4444;
		border: 1px solid rgba(239, 68, 68, 0.3);
	}

	.import-result.info {
		background: rgba(99, 102, 241, 0.1);
		color: #6366f1;
		border: 1px solid rgba(99, 102, 241, 0.3);
	}

	.server-picker-empty {
		padding: 2rem;
		text-align: center;
		color: var(--gm-text-secondary);
		font-size: 0.8125rem;
	}

	.file-results {
		margin-top: 1rem;
	}

	.file-results h4 {
		font-size: 0.875rem;
		margin-bottom: 0.5rem;
		color: var(--gm-text-primary);
	}

	.results-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 0.8125rem;
	}

	.results-table thead {
		background: rgba(0, 0, 0, 0.2);
	}

	.results-table th {
		padding: 0.5rem;
		text-align: left;
		color: var(--gm-text-secondary);
		font-weight: 600;
		border-bottom: 1px solid var(--gm-border);
	}

	.results-table td {
		padding: 0.5rem;
		color: var(--gm-text-primary);
		border-bottom: 1px solid var(--gm-border);
	}

	.file-name {
		font-family: 'JetBrains Mono', monospace;
		font-size: 0.75rem;
	}

	.file-packets,
	.file-assets,
	.file-connections {
		text-align: right;
	}

	.status-badge {
		padding: 0.2rem 0.4rem;
		border-radius: 2px;
		font-weight: 600;
		font-size: 0.7rem;
	}

	.status-badge.success {
		background: rgba(34, 197, 94, 0.2);
		color: #22c55e;
	}

	.status-badge.error {
		background: rgba(239, 68, 68, 0.2);
		color: #ef4444;
	}
</style>

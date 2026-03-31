<script lang="ts">
	import type { FrameRow } from '$lib/types/operations';

	interface Props {
		show: boolean;
		frames: FrameRow[];
		formatTime: (iso: string) => string;
		onClose: () => void;
		onExportCsv: () => void;
	}

	let { show, frames, formatTime, onClose, onExportCsv }: Props = $props();

	function handleWindowKeydown(event: KeyboardEvent) {
		if (!show) {
			return;
		}
		if (event.key === 'Escape') {
			event.preventDefault();
			onClose();
		}
	}
</script>

<svelte:window onkeydown={handleWindowKeydown} />

{#if show}
	<div class="frames-overlay" role="presentation">
		<div class="frames-dialog" role="dialog" aria-modal="true" aria-label="View Frames">
			<div class="frames-header">
				<h3 class="frames-title">View Frames</h3>
				<span class="frames-count">{frames.length} packets</span>
				<div class="frames-actions">
					<button class="frames-btn" onclick={onExportCsv}>Export CSV</button>
					<button class="frames-btn close" onclick={onClose} aria-label="Close view frames dialog">&times;</button>
				</div>
			</div>
			<div class="frames-table-wrap">
				<table class="frames-table">
					<thead>
						<tr>
							<th>#</th>
							<th>Timestamp</th>
							<th>Source</th>
							<th>Dest</th>
							<th>Protocol</th>
							<th>Length</th>
							<th>File</th>
						</tr>
					</thead>
					<tbody>
						{#each frames as frame}
							<tr>
								<td class="frame-num">{frame.number}</td>
								<td class="frame-ts">{formatTime(frame.timestamp)}</td>
								<td>{frame.src_ip}:{frame.src_port}</td>
								<td>{frame.dst_ip}:{frame.dst_port}</td>
								<td class="frame-proto">{frame.protocol}</td>
								<td class="frame-len">{frame.length}</td>
								<td class="frame-file">{frame.origin_file}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>
	</div>
{/if}

<style>
	.frames-overlay {
		position: fixed;
		inset: 0;
		z-index: 10000;
		background: rgba(0, 0, 0, 0.6);
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.frames-dialog {
		background: var(--gm-bg-primary);
		border: 1px solid var(--gm-border);
		border-radius: 8px;
		width: 90%;
		max-width: 900px;
		max-height: 80vh;
		display: flex;
		flex-direction: column;
		box-shadow: 0 12px 40px rgba(0, 0, 0, 0.5);
	}

	.frames-header {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 16px;
		border-bottom: 1px solid var(--gm-border);
	}

	.frames-title {
		font-size: 13px;
		font-weight: 600;
		color: var(--gm-text-primary);
		margin: 0;
	}

	.frames-count {
		font-size: 10px;
		color: var(--gm-text-muted);
		background: rgba(59, 130, 246, 0.15);
		padding: 2px 8px;
		border-radius: 10px;
	}

	.frames-actions {
		margin-left: auto;
		display: flex;
		gap: 6px;
	}

	.frames-btn {
		padding: 4px 10px;
		background: var(--gm-bg-secondary);
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		color: var(--gm-text-primary);
		font-family: inherit;
		font-size: 11px;
		cursor: pointer;
		transition: background 0.15s;
	}

	.frames-btn:hover {
		background: var(--gm-bg-hover);
	}

	.frames-btn.close {
		font-size: 16px;
		line-height: 1;
		padding: 2px 8px;
		border: none;
		background: transparent;
	}

	.frames-table-wrap {
		flex: 1;
		overflow: auto;
		padding: 0;
	}

	.frames-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 10px;
	}

	.frames-table thead {
		position: sticky;
		top: 0;
		background: var(--gm-bg-secondary);
		z-index: 1;
	}

	.frames-table th {
		padding: 6px 10px;
		text-align: left;
		font-size: 9px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		color: var(--gm-text-muted);
		border-bottom: 1px solid var(--gm-border);
	}

	.frames-table td {
		padding: 4px 10px;
		color: var(--gm-text-secondary);
		border-bottom: 1px solid rgba(45, 58, 79, 0.2);
	}

	.frames-table tr:hover td {
		background: var(--gm-bg-hover);
	}

	.frame-num {
		font-variant-numeric: tabular-nums;
		color: var(--gm-text-muted);
	}

	.frame-ts {
		color: var(--gm-text-muted);
	}

	.frame-proto {
		font-weight: 600;
		color: var(--gm-modbus);
	}

	.frame-len {
		text-align: right;
		font-variant-numeric: tabular-nums;
	}

	.frame-file {
		color: var(--gm-text-muted);
		max-width: 150px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}
</style>

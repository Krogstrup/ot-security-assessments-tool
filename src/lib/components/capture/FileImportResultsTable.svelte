<script lang="ts">
	import type { FileImportResult } from '$lib/types/capture';

	interface Props {
		results: FileImportResult[];
	}

	let { results }: Props = $props();
</script>

{#if results.length > 0}
	<div class="file-results">
		<h4>Results</h4>
		<table class="results-table">
			<thead>
				<tr>
					<th>File</th>
					<th>Packets</th>
					<th>Status</th>
				</tr>
			</thead>
			<tbody>
				{#each results as result}
					<tr>
						<td class="file-name">{result.filename}</td>
						<td class="file-packets">{result.packet_count.toLocaleString()}</td>
						<td>
							<span
								class="status-badge"
								class:success={result.status === 'ok'}
								class:error={result.status !== 'ok'}
							>
								{result.status}
							</span>
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
{/if}

<style>
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

	.file-packets {
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

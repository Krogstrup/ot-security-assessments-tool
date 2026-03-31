<script lang="ts">
	import type { SignatureTestResult } from '$lib/types/signatures';
	import { CONFIDENCE_COLORS as confidenceColors } from '$lib/constants';

	interface Props {
		testError: string | null;
		testResult: SignatureTestResult | null;
	}

	let { testError, testResult }: Props = $props();
</script>

{#if testError}
	<div class="test-results error">
		<div class="result-header">Error</div>
		<p>{testError}</p>
	</div>
{/if}

{#if testResult}
	<div class="test-results" class:success={testResult.match_count > 0}>
		<div class="result-header">
			Test Results: {testResult.match_count} match{testResult.match_count !== 1 ? 'es' : ''}
		</div>
		{#if testResult.matches.length > 0}
			<table class="result-table">
				<thead>
					<tr>
						<th>#</th>
						<th>Source</th>
						<th>Destination</th>
						<th>Confidence</th>
					</tr>
				</thead>
				<tbody>
					{#each testResult.matches.slice(0, 50) as m}
						<tr>
							<td>{m.packet_index}</td>
							<td>{m.src_ip}:{m.src_port}</td>
							<td>{m.dst_ip}:{m.dst_port}</td>
							<td>
								<span
									class="confidence-badge"
									style="color: {confidenceColors[m.confidence] ?? '#64748b'};
									       background: {(confidenceColors[m.confidence] ?? '#64748b')}18"
								>
									{m.confidence}/5
								</span>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
			{#if testResult.matches.length > 50}
				<p class="result-note">Showing first 50 of {testResult.matches.length} matches</p>
			{/if}
		{:else}
			<p class="result-note">No matches found against loaded PCAP data. Import a PCAP first, then test.</p>
		{/if}
	</div>
{/if}

<style>
	.test-results {
		border-top: 1px solid var(--gm-border);
		padding: 10px 16px;
		max-height: 250px;
		overflow-y: auto;
		background: var(--gm-bg-secondary);
	}

	.test-results.error {
		border-top-color: #ef4444;
	}

	.test-results.success {
		border-top-color: #10b981;
	}

	.result-header {
		font-size: 11px;
		font-weight: 600;
		color: var(--gm-text-primary);
		margin-bottom: 8px;
	}

	.test-results.error .result-header {
		color: #ef4444;
	}

	.test-results.success .result-header {
		color: #10b981;
	}

	.test-results p {
		font-size: 11px;
		color: var(--gm-text-secondary);
		margin: 4px 0;
	}

	.result-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 10px;
	}

	.result-table th {
		text-align: left;
		padding: 4px 8px;
		color: var(--gm-text-muted);
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.3px;
		border-bottom: 1px solid var(--gm-border);
	}

	.result-table td {
		padding: 4px 8px;
		color: var(--gm-text-secondary);
		border-bottom: 1px solid rgba(45, 58, 79, 0.3);
	}

	.confidence-badge {
		font-size: 9px;
		font-weight: 600;
		padding: 1px 6px;
		border-radius: 3px;
	}

	.result-note {
		font-style: italic;
		font-size: 10px !important;
		color: var(--gm-text-muted) !important;
	}
</style>

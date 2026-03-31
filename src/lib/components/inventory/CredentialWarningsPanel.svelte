<script lang="ts">
	import type { DefaultCredential } from '$lib/types';

	interface Props {
		warnings: DefaultCredential[];
	}

	let { warnings }: Props = $props();

	function copyCredentials(username: string, password: string) {
		navigator.clipboard.writeText(`${username}:${password}`);
	}
</script>

{#if warnings.length > 0}
	<div class="detail-section cred-warning-section">
		<h4 class="section-title warning-title">⚠ Default Credentials Detected</h4>
		{#each warnings as cw}
			<div class="cred-warning-card">
				<div class="cred-row">
					<span class="cred-label">Protocol:</span>
					<span>{cw.protocol.toUpperCase()}</span>
				</div>
				{#if cw.username}
					<div class="cred-row">
						<span class="cred-label">Username:</span>
						<code>{cw.username}</code>
					</div>
				{/if}
				{#if cw.password}
					<div class="cred-row">
						<span class="cred-label">Password:</span>
						<code>{cw.password}</code>
					</div>
				{/if}
				<div class="cred-row cred-source">
					<span class="cred-label">Source:</span>
					<span>{cw.source}</span>
				</div>
				<button class="copy-btn cred-copy" onclick={() => copyCredentials(cw.username, cw.password)}>
					Copy Credentials
				</button>
			</div>
		{/each}
	</div>
{/if}

<style>
	.detail-section {
		margin-top: 1.5rem;
		padding: 1rem;
		border-radius: 6px;
		background: #0f172a;
		border: 1px solid #1e293b;
	}

	.cred-warning-section {
		background: rgba(239, 68, 68, 0.05);
		border: 1px solid rgba(239, 68, 68, 0.2);
	}

	.section-title {
		font-size: 0.875rem;
		font-weight: 600;
		margin-bottom: 1rem;
		color: #e2e8f0;
	}

	.warning-title {
		color: #ef4444;
	}

	.cred-warning-card {
		background: rgba(0, 0, 0, 0.3);
		padding: 0.75rem;
		border-radius: 4px;
		margin-bottom: 0.75rem;
		border-left: 3px solid #ef4444;
	}

	.cred-row {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.5rem;
		font-size: 0.8125rem;
	}

	.cred-row:last-of-type {
		margin-bottom: 0;
	}

	.cred-label {
		color: #94a3b8;
		font-weight: 600;
		min-width: 80px;
	}

	.cred-row code {
		background: rgba(0, 0, 0, 0.5);
		padding: 0.25rem 0.5rem;
		border-radius: 3px;
		font-family: 'JetBrains Mono', monospace;
		font-size: 0.75rem;
		color: #fbbf24;
	}

	.cred-source {
		color: #64748b;
		font-style: italic;
	}

	.copy-btn {
		margin-top: 0.5rem;
		padding: 0.4rem 0.8rem;
		font-size: 0.75rem;
		background: #ef4444;
		color: white;
		border: none;
		border-radius: 3px;
		cursor: pointer;
		transition: background 0.2s;
	}

	.copy-btn:hover {
		background: #dc2626;
	}
</style>

<script lang="ts">
	import type { SessionInfo } from '$lib/types/operations';

	interface Props {
		session: SessionInfo | null;
	}

	let { session }: Props = $props();
</script>

{#if session}
	<div class="current-session">
		<h4>Current Session</h4>
		<div class="session-detail">
			<div class="detail-row">
				<span class="detail-label">Name</span>
				<span class="detail-value">{session.name}</span>
			</div>
			<div class="detail-row">
				<span class="detail-label">Assets</span>
				<span class="detail-value">{session.asset_count}</span>
			</div>
			<div class="detail-row">
				<span class="detail-label">Connections</span>
				<span class="detail-value">{session.connection_count}</span>
			</div>
		</div>
	</div>
{/if}

<style>
	.current-session {
		margin-bottom: 1rem;
		padding: 1rem;
		background: rgba(34, 197, 94, 0.05);
		border: 1px solid rgba(34, 197, 94, 0.2);
		border-radius: 4px;
	}

	h4 {
		margin: 0 0 0.75rem 0;
		font-size: 0.875rem;
		color: var(--gm-text-primary);
	}

	.session-detail {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
		gap: 0.75rem;
	}

	.detail-row {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.detail-label {
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--gm-text-secondary);
	}

	.detail-value {
		font-size: 0.875rem;
		color: var(--gm-text-primary);
		font-weight: 500;
	}
</style>

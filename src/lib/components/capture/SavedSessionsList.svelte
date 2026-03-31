<script lang="ts">
	import type { SessionInfo } from '$lib/types/operations';

	interface Props {
		sessions: SessionInfo[];
		confirmDeleteId: string | null;
		onLoadSession: (id: string) => Promise<void>;
		onDeleteSession: (id: string) => Promise<void>;
		onExportSession: (session: SessionInfo) => Promise<void>;
		onConfirmDelete: (id: string | null) => void;
	}

	let {
		sessions,
		confirmDeleteId,
		onLoadSession,
		onDeleteSession,
		onExportSession,
		onConfirmDelete
	}: Props = $props();
</script>

{#if sessions.length > 0}
	<div class="sessions-list">
		<h4>Saved Sessions ({sessions.length})</h4>
		<div class="session-items">
			{#each sessions as session}
				<div class="session-item">
					<div class="session-info">
						<div class="session-name">{session.name}</div>
						<div class="session-stats">
							{session.asset_count} assets, {session.connection_count} connections
						</div>
					</div>
					<div class="session-actions">
						<button class="action-btn small primary" onclick={() => onLoadSession(session.id)}>Load</button>
						<button class="action-btn small secondary" onclick={() => onExportSession(session)}>
							Export
						</button>
						<button class="action-btn small danger" onclick={() => onConfirmDelete(session.id)}>
							Delete
						</button>
					</div>
				</div>

				{#if confirmDeleteId === session.id}
					<div class="delete-confirm">
						<span>Are you sure?</span>
						<button
							class="action-btn small danger"
							onclick={() => {
								onDeleteSession(session.id);
								onConfirmDelete(null);
							}}
						>
							Confirm Delete
						</button>
						<button class="action-btn small secondary" onclick={() => onConfirmDelete(null)}>
							Cancel
						</button>
					</div>
				{/if}
			{/each}
		</div>
	</div>
{/if}

<style>
	.sessions-list {
		margin-bottom: 1rem;
	}

	h4 {
		margin: 0 0 0.75rem 0;
		font-size: 0.875rem;
		color: var(--gm-text-primary);
	}

	.session-items {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.session-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem;
		background: rgba(0, 0, 0, 0.2);
		border-radius: 4px;
		border-left: 3px solid #6366f1;
	}

	.session-info {
		flex: 1;
	}

	.session-name {
		font-weight: 600;
		color: var(--gm-text-primary);
		font-size: 0.8125rem;
	}

	.session-stats {
		font-size: 0.75rem;
		color: var(--gm-text-secondary);
		margin-top: 0.25rem;
	}

	.session-actions {
		display: flex;
		gap: 0.25rem;
	}

	.delete-confirm {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 3px;
		font-size: 0.75rem;
		margin-top: 0.5rem;
		margin-left: 0.75rem;
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

	.action-btn.danger {
		background: #ef4444;
		color: white;
	}

	.action-btn.danger:hover {
		background: #dc2626;
	}

	.action-btn.small {
		padding: 0.3rem 0.6rem;
		font-size: 0.75rem;
	}
</style>

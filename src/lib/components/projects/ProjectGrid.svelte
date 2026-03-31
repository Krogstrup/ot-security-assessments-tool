<script lang="ts">
	import type { ProjectSummary } from '$lib/types/analysis';

	interface Props {
		loading: boolean;
		projects: ProjectSummary[];
		formatDate: (iso: string) => string;
		onCreate: () => void;
		onOpen: (summary: ProjectSummary) => void | Promise<void>;
		onEdit: (summary: ProjectSummary) => void;
		onRequestDelete: (id: number) => void;
	}

	let { loading, projects, formatDate, onCreate, onOpen, onEdit, onRequestDelete }: Props = $props();

	function handleCardKeydown(event: KeyboardEvent, project: ProjectSummary) {
		if (event.key === 'Enter' || event.key === ' ') {
			event.preventDefault();
			void onOpen(project);
		}
	}

	function stopAndEdit(event: MouseEvent, project: ProjectSummary) {
		event.stopPropagation();
		onEdit(project);
	}

	function stopAndRequestDelete(event: MouseEvent, id: number) {
		event.stopPropagation();
		onRequestDelete(id);
	}
</script>

{#if loading}
	<div class="empty-state">Loading projects…</div>
{:else if projects.length === 0}
	<div class="empty-state">
		<div class="empty-icon">&#128193;</div>
		<div class="empty-title">No projects yet</div>
		<div class="empty-sub">Create a project to organize your capture sessions and findings.</div>
		<button class="btn-primary" onclick={onCreate}>Create First Project</button>
	</div>
{:else}
	<div class="project-grid">
		{#each projects as p}
			<div class="project-card" role="button" tabindex="0" onclick={() => onOpen(p)} onkeydown={(event) => handleCardKeydown(event, p)}>
				<div class="card-top">
					<div class="card-icon">&#128193;</div>
					<div class="card-actions">
						<button class="card-btn" title="Edit project" onclick={(event) => stopAndEdit(event, p)}>&#9998;</button>
						<button class="card-btn card-btn-danger" title="Delete project" onclick={(event) => stopAndRequestDelete(event, p.id)}>&#10005;</button>
					</div>
				</div>
				<div class="card-name">{p.name}</div>
				{#if p.client_name}
					<div class="card-meta card-client">{p.client_name}</div>
				{/if}
				{#if p.site_name}
					<div class="card-meta">{p.site_name}</div>
				{/if}
				<div class="card-footer">
					<span class="card-sessions">{p.session_count} {p.session_count === 1 ? 'session' : 'sessions'}</span>
					<span class="card-date">{formatDate(p.updated_at)}</span>
				</div>
			</div>
		{/each}
	</div>
{/if}

<style>
	.btn-primary {
		padding: 8px 16px;
		background: #10b981;
		color: #0a0e17;
		border: none;
		border-radius: 5px;
		font-family: inherit;
		font-size: 12px;
		font-weight: 600;
		cursor: pointer;
		transition: background 0.15s;
		white-space: nowrap;
	}

	.btn-primary:hover:not(:disabled) {
		background: #059669;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 12px;
		padding: 80px 20px;
		color: var(--gm-text-muted);
		font-size: 13px;
		text-align: center;
	}

	.empty-icon {
		font-size: 48px;
		opacity: 0.4;
	}

	.empty-title {
		font-size: 16px;
		font-weight: 600;
		color: var(--gm-text-secondary);
	}

	.empty-sub {
		font-size: 12px;
		max-width: 360px;
	}

	.project-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
		gap: 16px;
	}

	.project-card {
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 8px;
		padding: 20px;
		cursor: pointer;
		display: flex;
		flex-direction: column;
		gap: 6px;
		transition: all 0.15s;
		outline: none;
	}

	.project-card:hover,
	.project-card:focus {
		border-color: #10b981;
		background: rgba(16, 185, 129, 0.04);
		box-shadow: 0 0 0 1px #10b981;
	}

	.card-top {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 4px;
	}

	.card-icon {
		font-size: 24px;
		opacity: 0.7;
	}

	.card-actions {
		display: flex;
		gap: 4px;
		opacity: 0;
		transition: opacity 0.1s;
	}

	.project-card:hover .card-actions,
	.project-card:focus .card-actions {
		opacity: 1;
	}

	.card-btn {
		background: transparent;
		border: 1px solid transparent;
		border-radius: 4px;
		color: var(--gm-text-muted);
		font-size: 12px;
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
		transition: all 0.1s;
	}

	.card-btn:hover {
		background: var(--gm-bg-hover);
		color: var(--gm-text-primary);
		border-color: var(--gm-border);
	}

	.card-btn-danger:hover {
		background: rgba(239, 68, 68, 0.15);
		color: #ef4444;
		border-color: rgba(239, 68, 68, 0.3);
	}

	.card-name {
		font-size: 15px;
		font-weight: 700;
		color: var(--gm-text-primary);
		line-height: 1.2;
	}

	.card-meta {
		font-size: 11px;
		color: var(--gm-text-muted);
	}

	.card-client {
		color: #10b981;
		font-weight: 500;
	}

	.card-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-top: 8px;
		padding-top: 8px;
		border-top: 1px solid var(--gm-border);
	}

	.card-sessions {
		font-size: 10px;
		color: var(--gm-text-muted);
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.card-date {
		font-size: 10px;
		color: var(--gm-text-muted);
	}
</style>

<script lang="ts">
	import type { SessionInfo } from '$lib/types';

	interface Props {
		sessions: SessionInfo[];
		currentSession: SessionInfo | null;
		sessionMessage: string;
		sessionMessageType: 'success' | 'error' | '';
		showSaveForm: boolean;
		sessionName: string;
		sessionDesc: string;
		confirmDeleteId: string | null;
		onToggleSaveForm: () => void;
		onSaveName: (name: string) => void;
		onSaveDesc: (desc: string) => void;
		onSaveSession: () => Promise<void>;
		onLoadSession: (id: string) => Promise<void>;
		onDeleteSession: (id: string) => Promise<void>;
		onExportSession: (session: SessionInfo) => Promise<void>;
		onImportArchive: () => Promise<void>;
		onConfirmDelete: (id: string | null) => void;
	}

	let {
		sessions,
		currentSession,
		sessionMessage,
		sessionMessageType,
		showSaveForm,
		sessionName,
		sessionDesc,
		confirmDeleteId,
		onToggleSaveForm,
		onSaveName,
		onSaveDesc,
		onSaveSession,
		onLoadSession,
		onDeleteSession,
		onExportSession,
		onImportArchive,
		onConfirmDelete
	}: Props = $props();
</script>

<section class="capture-section">
	<h3 class="section-title">Sessions</h3>
	<p class="section-desc">
		Save, load, export, and import capture sessions. A session captures all assets, connections, topology,
		and analysis results at a point in time.
	</p>

	<div class="button-group">
		<button class="action-btn primary" onclick={onToggleSaveForm}>
			{showSaveForm ? 'Cancel' : 'Save New Session'}
		</button>
		<button class="action-btn secondary" onclick={onImportArchive}>
			Import Archive
		</button>
	</div>

	{#if sessionMessage}
		<div class="message" class:success={sessionMessageType === 'success'} class:error={sessionMessageType === 'error'}>
			{sessionMessage}
		</div>
	{/if}

	{#if showSaveForm}
		<div class="save-form">
			<input
				type="text"
				class="form-input"
				placeholder="Session name"
				value={sessionName}
				onchange={(e) => onSaveName((e.target as HTMLInputElement).value)}
			/>
			<textarea
				class="form-textarea"
				placeholder="Optional description..."
				rows="2"
				value={sessionDesc}
				onchange={(e) => onSaveDesc((e.target as HTMLTextAreaElement).value)}
			></textarea>
			<button class="action-btn primary" onclick={onSaveSession} disabled={!sessionName.trim()}>
				Save
			</button>
		</div>
	{/if}

	{#if currentSession}
		<div class="current-session">
			<h4>Current Session</h4>
			<div class="session-detail">
				<div class="detail-row">
					<span class="detail-label">Name</span>
					<span class="detail-value">{currentSession.name}</span>
				</div>
				<div class="detail-row">
					<span class="detail-label">Assets</span>
					<span class="detail-value">{currentSession.asset_count}</span>
				</div>
				<div class="detail-row">
					<span class="detail-label">Connections</span>
					<span class="detail-value">{currentSession.connection_count}</span>
				</div>
			</div>
		</div>
	{/if}

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
							<button class="action-btn small primary" onclick={() => onLoadSession(session.id)}>
								Load
							</button>
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

	.button-group {
		display: flex;
		gap: 0.5rem;
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

	.action-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.message {
		padding: 0.75rem;
		border-radius: 4px;
		font-size: 0.8125rem;
		margin-bottom: 1rem;
	}

	.message.success {
		background: rgba(34, 197, 94, 0.1);
		color: #22c55e;
		border: 1px solid rgba(34, 197, 94, 0.3);
	}

	.message.error {
		background: rgba(239, 68, 68, 0.1);
		color: #ef4444;
		border: 1px solid rgba(239, 68, 68, 0.3);
	}

	.save-form {
		padding: 1rem;
		background: rgba(99, 102, 241, 0.05);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 4px;
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		margin-bottom: 1rem;
	}

	.form-input,
	.form-textarea {
		padding: 0.5rem;
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		background: var(--gm-bg-tertiary);
		color: var(--gm-text-primary);
		font-size: 0.8125rem;
		font-family: inherit;
	}

	.form-input:focus,
	.form-textarea:focus {
		outline: none;
		border-color: #6366f1;
	}

	.current-session {
		margin-bottom: 1rem;
		padding: 1rem;
		background: rgba(34, 197, 94, 0.05);
		border: 1px solid rgba(34, 197, 94, 0.2);
		border-radius: 4px;
	}

	.current-session h4 {
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

	.sessions-list {
		margin-bottom: 1rem;
	}

	.sessions-list h4 {
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
</style>

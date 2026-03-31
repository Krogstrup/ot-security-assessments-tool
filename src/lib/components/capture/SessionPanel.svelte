<script lang="ts">
	import type { SessionInfo } from '$lib/types/operations';
	import SessionSaveForm from './SessionSaveForm.svelte';
	import CurrentSessionCard from './CurrentSessionCard.svelte';
	import SavedSessionsList from './SavedSessionsList.svelte';

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

	<SessionSaveForm
		show={showSaveForm}
		{sessionName}
		{sessionDesc}
		{onSaveName}
		{onSaveDesc}
		{onSaveSession}
	/>

	<CurrentSessionCard session={currentSession} />

	<SavedSessionsList
		{sessions}
		{confirmDeleteId}
		{onLoadSession}
		{onDeleteSession}
		{onExportSession}
		{onConfirmDelete}
	/>
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

</style>

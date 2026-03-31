<script lang="ts">
	interface Props {
		show: boolean;
		editing: boolean;
		formError: string | null;
		formSaving: boolean;
		projectName: string;
		clientName: string;
		siteName: string;
		assessorName: string;
		startDate: string;
		endDate: string;
		notes: string;
		onClose: () => void;
		onSave: () => void | Promise<void>;
	}

	let {
		show,
		editing,
		formError,
		formSaving,
		projectName = $bindable(),
		clientName = $bindable(),
		siteName = $bindable(),
		assessorName = $bindable(),
		startDate = $bindable(),
		endDate = $bindable(),
		notes = $bindable(),
		onClose,
		onSave
	}: Props = $props();
</script>

{#if show}
	<div class="modal-backdrop" role="dialog" aria-modal="true">
		<div class="modal">
			<div class="modal-header">
				<h2 class="modal-title">{editing ? 'Edit Project' : 'New Project'}</h2>
				<button class="modal-close" onclick={onClose} aria-label="Close project form">&#10005;</button>
			</div>
			<div class="modal-body">
				{#if formError}
					<div class="form-error">{formError}</div>
				{/if}
				<div class="form-field">
					<label class="form-label" for="proj-name">Project Name *</label>
					<input id="proj-name" class="form-input" type="text" bind:value={projectName} placeholder="e.g. Site Alpha Assessment" />
				</div>
				<div class="form-row">
					<div class="form-field">
						<label class="form-label" for="proj-client">Client Name</label>
						<input id="proj-client" class="form-input" type="text" bind:value={clientName} placeholder="e.g. Acme Corporation" />
					</div>
					<div class="form-field">
						<label class="form-label" for="proj-site">Site Name</label>
						<input id="proj-site" class="form-input" type="text" bind:value={siteName} placeholder="e.g. Plant 1" />
					</div>
				</div>
				<div class="form-field">
					<label class="form-label" for="proj-assessor">Assessor Name</label>
					<input id="proj-assessor" class="form-input" type="text" bind:value={assessorName} placeholder="Your name" />
				</div>
				<div class="form-row">
					<div class="form-field">
						<label class="form-label" for="proj-start">Start Date</label>
						<input id="proj-start" class="form-input" type="date" bind:value={startDate} />
					</div>
					<div class="form-field">
						<label class="form-label" for="proj-end">End Date</label>
						<input id="proj-end" class="form-input" type="date" bind:value={endDate} />
					</div>
				</div>
				<div class="form-field">
					<label class="form-label" for="proj-notes">Notes</label>
					<textarea id="proj-notes" class="form-textarea" bind:value={notes} rows={3} placeholder="Scope, objectives, special considerations…"></textarea>
				</div>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={onClose}>Cancel</button>
				<button class="btn-primary" onclick={onSave} disabled={formSaving}>
					{formSaving ? 'Saving…' : editing ? 'Save Changes' : 'Create Project'}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.modal-backdrop {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}

	.modal {
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 8px;
		width: 540px;
		max-width: 95vw;
		max-height: 90vh;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 18px 20px 14px;
		border-bottom: 1px solid var(--gm-border);
	}

	.modal-title {
		font-size: 14px;
		font-weight: 700;
		color: var(--gm-text-primary);
		margin: 0;
	}

	.modal-close {
		background: transparent;
		border: none;
		color: var(--gm-text-muted);
		font-size: 14px;
		cursor: pointer;
		padding: 4px;
		line-height: 1;
	}

	.modal-close:hover {
		color: var(--gm-text-primary);
	}

	.modal-body {
		padding: 20px;
		overflow-y: auto;
		display: flex;
		flex-direction: column;
		gap: 14px;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 8px;
		padding: 14px 20px;
		border-top: 1px solid var(--gm-border);
	}

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

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-secondary {
		padding: 8px 16px;
		background: transparent;
		color: var(--gm-text-secondary);
		border: 1px solid var(--gm-border);
		border-radius: 5px;
		font-family: inherit;
		font-size: 12px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-secondary:hover {
		background: var(--gm-bg-hover);
		color: var(--gm-text-primary);
	}

	.form-error {
		padding: 8px 12px;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 4px;
		color: #ef4444;
		font-size: 12px;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 12px;
	}

	.form-field {
		display: flex;
		flex-direction: column;
		gap: 5px;
	}

	.form-label {
		font-size: 11px;
		font-weight: 600;
		color: var(--gm-text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.form-input,
	.form-textarea {
		background: var(--gm-bg-secondary);
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		color: var(--gm-text-primary);
		font-family: inherit;
		font-size: 12px;
		padding: 8px 10px;
		outline: none;
		transition: border-color 0.15s;
		width: 100%;
		box-sizing: border-box;
	}

	.form-input:focus,
	.form-textarea:focus {
		border-color: #10b981;
	}

	.form-textarea {
		resize: vertical;
	}
</style>

<script lang="ts">
	interface Props {
		show: boolean;
		sessionName: string;
		sessionDesc: string;
		onSaveName: (name: string) => void;
		onSaveDesc: (desc: string) => void;
		onSaveSession: () => Promise<void>;
	}

	let { show, sessionName, sessionDesc, onSaveName, onSaveDesc, onSaveSession }: Props = $props();
</script>

{#if show}
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

<style>
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

	.action-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>

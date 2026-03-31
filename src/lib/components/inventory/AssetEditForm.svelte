<script lang="ts">
	import type { Asset } from '$lib/types/assets';
	import { DEVICE_TYPE_LABELS, DEVICE_TYPE_OPTIONS, PURDUE_LABELS } from '$lib/constants';

	interface Props {
		isEditing: boolean;
		asset: Asset | null;
		editDeviceType: string;
		editHostname: string;
		editNotes: string;
		editPurdueLevel: number | null;
		editTags: string;
		editSaving: boolean;
		editMessage: string;
		onSave: () => void;
		onCancel: () => void;
		onFieldChange: (field: string, value: unknown) => void;
	}

	let {
		isEditing,
		asset,
		editDeviceType,
		editHostname,
		editNotes,
		editPurdueLevel,
		editTags,
		editSaving,
		editMessage,
		onSave,
		onCancel,
		onFieldChange
	}: Props = $props();
</script>

{#if isEditing && asset}
	<div class="detail-section edit-section">
		<h4 class="section-title">Edit Asset</h4>
		{#if editMessage}
			<div class="edit-message" class:success={editMessage === 'Saved'} class:error={editMessage.startsWith('Error')}>
				{editMessage}
			</div>
		{/if}
		<div class="edit-group">
			<label class="edit-label" for="edit-device-type">Device Type</label>
			<select
				id="edit-device-type"
				class="edit-select"
				value={editDeviceType}
				onchange={(e) => onFieldChange('deviceType', (e.target as HTMLSelectElement).value)}
			>
				{#each DEVICE_TYPE_OPTIONS as dt}
					<option value={dt}>{DEVICE_TYPE_LABELS[dt]}</option>
				{/each}
			</select>
		</div>
		<div class="edit-group">
			<label class="edit-label" for="edit-hostname">Hostname</label>
			<input
				id="edit-hostname"
				class="edit-input"
				type="text"
				value={editHostname}
				placeholder="e.g., PLC-BOILER-01"
				onchange={(e) => onFieldChange('hostname', (e.target as HTMLInputElement).value)}
			/>
		</div>
		<div class="edit-group">
			<label class="edit-label" for="edit-purdue">Purdue Level</label>
			<select
				id="edit-purdue"
				class="edit-select"
				value={editPurdueLevel === null ? '' : editPurdueLevel}
				onchange={(e) => {
					const val = (e.target as HTMLSelectElement).value;
					onFieldChange('purdueLevel', val === '' ? null : parseInt(val));
				}}
			>
				<option value="">Not set</option>
				{#each [0, 1, 2, 3, 4, 5] as level}
					<option value={level}>{PURDUE_LABELS[level]}</option>
				{/each}
			</select>
		</div>
		<div class="edit-group">
			<label class="edit-label" for="edit-tags">Tags (comma separated)</label>
			<input
				id="edit-tags"
				class="edit-input"
				type="text"
				value={editTags}
				placeholder="e.g., critical, zone-a"
				onchange={(e) => onFieldChange('tags', (e.target as HTMLInputElement).value)}
			/>
		</div>
		<div class="edit-group">
			<label class="edit-label" for="edit-notes">Notes</label>
			<textarea
				id="edit-notes"
				class="edit-textarea"
				value={editNotes}
				rows="3"
				placeholder="Freeform notes about this asset..."
				onchange={(e) => onFieldChange('notes', (e.target as HTMLTextAreaElement).value)}
			></textarea>
		</div>
		<div class="edit-actions">
			<button class="action-btn primary small" onclick={onSave} disabled={editSaving}>
				{editSaving ? 'Saving...' : 'Save'}
			</button>
			<button class="action-btn secondary small" onclick={onCancel}>
				Cancel
			</button>
		</div>
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

	.edit-section {
		background: rgba(99, 102, 241, 0.05);
		border: 1px solid rgba(99, 102, 241, 0.2);
	}

	.section-title {
		font-size: 0.875rem;
		font-weight: 600;
		margin-bottom: 1rem;
		color: #e2e8f0;
	}

	.edit-message {
		padding: 0.5rem;
		border-radius: 3px;
		margin-bottom: 1rem;
		font-size: 0.8125rem;
	}

	.edit-message.success {
		background: rgba(34, 197, 94, 0.1);
		color: #22c55e;
		border: 1px solid rgba(34, 197, 94, 0.3);
	}

	.edit-message.error {
		background: rgba(239, 68, 68, 0.1);
		color: #ef4444;
		border: 1px solid rgba(239, 68, 68, 0.3);
	}

	.edit-group {
		margin-bottom: 1rem;
	}

	.edit-label {
		display: block;
		font-size: 0.8125rem;
		font-weight: 600;
		margin-bottom: 0.35rem;
		color: #cbd5e1;
	}

	.edit-input,
	.edit-select,
	.edit-textarea {
		width: 100%;
		padding: 0.5rem;
		border: 1px solid #334155;
		border-radius: 4px;
		background: #1e293b;
		color: #e2e8f0;
		font-size: 0.8125rem;
		font-family: inherit;
	}

	.edit-textarea {
		resize: vertical;
		font-family: inherit;
	}

	.edit-input:focus,
	.edit-select:focus,
	.edit-textarea:focus {
		outline: none;
		border-color: #6366f1;
		box-shadow: 0 0 0 2px rgba(99, 102, 241, 0.1);
	}

	.edit-actions {
		display: flex;
		gap: 0.5rem;
		margin-top: 1rem;
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
		background: #1e293b;
		color: #cbd5e1;
		border: 1px solid #334155;
	}

	.action-btn.secondary:hover {
		background: #334155;
	}

	.action-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.action-btn.small {
		padding: 0.4rem 0.8rem;
		font-size: 0.75rem;
	}
</style>

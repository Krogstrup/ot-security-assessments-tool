<script lang="ts">
	import { DEVICE_TYPE_LABELS, DEVICE_TYPE_OPTIONS, PURDUE_LABELS } from '$lib/constants';

	interface Props {
		selectedCount: number;
		bulkDeviceType: string;
		bulkPurdueLevel: string;
		bulkTag: string;
		bulkNotes: string;
		bulkSaving: boolean;
		onApply: () => void;
		onClear: () => void;
		onFieldChange: (field: string, value: string) => void;
	}

	let { selectedCount, bulkDeviceType, bulkPurdueLevel, bulkTag, bulkNotes, bulkSaving, onApply, onClear, onFieldChange }: Props = $props();

	const canApply = $derived(
		!bulkSaving && !!(bulkDeviceType || bulkPurdueLevel || bulkTag.trim() || bulkNotes.trim())
	);
</script>

<div class="bulk-bar">
	<span class="bulk-count">{selectedCount} selected</span>
	<select
		class="bulk-select"
		value={bulkDeviceType}
		onchange={(e) => onFieldChange('deviceType', (e.target as HTMLSelectElement).value)}
	>
		<option value="">Set Type...</option>
		{#each DEVICE_TYPE_OPTIONS as dt}
			<option value={dt}>{DEVICE_TYPE_LABELS[dt]}</option>
		{/each}
	</select>
	<select
		class="bulk-select"
		value={bulkPurdueLevel}
		onchange={(e) => onFieldChange('purdueLevel', (e.target as HTMLSelectElement).value)}
	>
		<option value="">Set Purdue...</option>
		{#each [0, 1, 2, 3, 4, 5] as level}
			<option value={level.toString()}>{PURDUE_LABELS[level]}</option>
		{/each}
	</select>
	<input
		class="bulk-input"
		type="text"
		placeholder="Add tag..."
		value={bulkTag}
		onchange={(e) => onFieldChange('tag', (e.target as HTMLInputElement).value)}
	/>
	<input
		class="bulk-input"
		type="text"
		placeholder="Set notes..."
		value={bulkNotes}
		onchange={(e) => onFieldChange('notes', (e.target as HTMLInputElement).value)}
	/>
	<button class="bulk-apply" onclick={onApply} disabled={!canApply}>
		{bulkSaving ? 'Applying...' : 'Apply'}
	</button>
	<button class="bulk-cancel" onclick={onClear}>
		Clear
	</button>
</div>

<style>
	.bulk-bar {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: #1e293b;
		border-bottom: 1px solid #334155;
		flex-wrap: wrap;
	}

	.bulk-count {
		font-weight: 600;
		color: #e2e8f0;
		font-size: 0.875rem;
		min-width: 100px;
	}

	.bulk-select,
	.bulk-input {
		padding: 0.5rem 0.75rem;
		border: 1px solid #334155;
		border-radius: 4px;
		background: #0f172a;
		color: #e2e8f0;
		font-size: 0.8125rem;
	}

	.bulk-select:focus,
	.bulk-input:focus {
		outline: none;
		border-color: #6366f1;
	}

	.bulk-apply,
	.bulk-cancel {
		padding: 0.5rem 1rem;
		border: none;
		border-radius: 4px;
		font-size: 0.8125rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.bulk-apply {
		background: #10b981;
		color: white;
	}

	.bulk-apply:hover:not(:disabled) {
		background: #059669;
	}

	.bulk-apply:disabled {
		background: #6b7280;
		cursor: not-allowed;
		opacity: 0.6;
	}

	.bulk-cancel {
		background: #6b7280;
		color: white;
	}

	.bulk-cancel:hover {
		background: #4b5563;
	}
</style>

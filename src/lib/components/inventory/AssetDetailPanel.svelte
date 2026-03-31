<script lang="ts">
	import type { CorrelatedAlert, CveMatch, DefaultCredential, DeviceZeekEvents } from '$lib/types/analysis';
	import type { Asset } from '$lib/types/assets';
	import type { DeepParseInfo } from '$lib/types/deep-parse';

	import AssetEditForm from './AssetEditForm.svelte';
	import AssetBasicInfoSection from './AssetBasicInfoSection.svelte';
	import AssetWiresharkFiltersPanel from './AssetWiresharkFiltersPanel.svelte';
	import CredentialWarningsPanel from './CredentialWarningsPanel.svelte';
	import CveWarningsPanel from './CveWarningsPanel.svelte';
	import ZeekEventsPanel from './ZeekEventsPanel.svelte';
	import AlertsPanel from './AlertsPanel.svelte';
	import DeepParsePanel from './DeepParsePanel.svelte';

	interface Props {
		asset: Asset | null;
		deepParseInfo: DeepParseInfo | null;
		loadingDeepParse: boolean;
		credWarnings: DefaultCredential[];
		cveWarnings: CveMatch[];
		zeekEvents: DeviceZeekEvents | null;
		assetAlerts: CorrelatedAlert[];
		isEditing: boolean;
		editDeviceType: string;
		editHostname: string;
		editNotes: string;
		editPurdueLevel: number | null;
		editTags: string;
		editSaving: boolean;
		editMessage: string;
		onStartEditing: (asset: Asset) => void;
		onClose: () => void;
		onSave: () => void;
		onCancelEdit: () => void;
		onFieldChange: (field: string, value: unknown) => void;
	}

	let {
		asset,
		deepParseInfo,
		loadingDeepParse,
		credWarnings,
		cveWarnings,
		zeekEvents,
		assetAlerts,
		isEditing,
		editDeviceType,
		editHostname,
		editNotes,
		editPurdueLevel,
		editTags,
		editSaving,
		editMessage,
		onStartEditing,
		onClose,
		onSave,
		onCancelEdit,
		onFieldChange
	}: Props = $props();
</script>

{#if asset}
	<div class="detail-panel">
		<div class="detail-header">
			<h3 class="detail-title">{asset.ip_address}</h3>
			<div class="detail-header-actions">
				{#if !isEditing}
					<button class="edit-btn" onclick={() => onStartEditing(asset)} title="Edit asset">Edit</button>
				{/if}
				<button class="detail-close" onclick={onClose}>&times;</button>
			</div>
		</div>

		<div class="detail-body">
			<AssetEditForm
				{isEditing}
				{asset}
				{editDeviceType}
				{editHostname}
				{editNotes}
				{editPurdueLevel}
				{editTags}
				{editSaving}
				{editMessage}
				{onSave}
				onCancel={onCancelEdit}
				{onFieldChange}
			/>

			{#if !isEditing}
				<AssetBasicInfoSection {asset} />
			{/if}

			<AssetWiresharkFiltersPanel {asset} />
			<CredentialWarningsPanel warnings={credWarnings} />
			<CveWarningsPanel {cveWarnings} />
			<ZeekEventsPanel {zeekEvents} />
			<AlertsPanel alerts={assetAlerts} />
			<DeepParsePanel {deepParseInfo} loading={loadingDeepParse} />
		</div>
	</div>
{/if}

<style>
	.detail-panel {
		width: 360px;
		min-width: 320px;
		border-left: 1px solid var(--gm-border);
		background: var(--gm-bg-secondary);
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.detail-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 10px 14px;
		border-bottom: 1px solid var(--gm-border);
		flex-shrink: 0;
	}

	.detail-header-actions {
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.detail-title {
		font-size: 13px;
		font-weight: 600;
		color: var(--gm-text-primary);
		margin: 0;
	}

	.edit-btn {
		padding: 3px 10px;
		background: rgba(59, 130, 246, 0.1);
		border: 1px solid rgba(59, 130, 246, 0.3);
		border-radius: 4px;
		color: #3b82f6;
		font-family: inherit;
		font-size: 10px;
		font-weight: 600;
		cursor: pointer;
	}

	.edit-btn:hover {
		background: rgba(59, 130, 246, 0.2);
	}

	.detail-close {
		background: none;
		border: none;
		color: var(--gm-text-muted);
		font-size: 18px;
		cursor: pointer;
		padding: 0 4px;
		line-height: 1;
	}

	.detail-close:hover {
		color: var(--gm-text-primary);
	}

	.detail-body {
		flex: 1;
		overflow-y: auto;
		padding: 12px 14px;
	}
</style>


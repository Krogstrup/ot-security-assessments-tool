<script lang="ts">
	import { selectedAsset, selectedAssetId } from '$lib/stores/core';
	import { updateAsset } from '$lib/api';
	import type {
		CorrelatedAlert,
		CveMatch,
		DefaultCredential,
		DeviceZeekEvents
	} from '$lib/types/analysis';
	import type { DeepParseInfo } from '$lib/types/deep-parse';
	import type { Asset } from '$lib/types/assets';

	import AssetDetailPanel from './AssetDetailPanel.svelte';
	import { loadAssetDetailData } from './assetDetailLoaders';
	import {
		buildAssetEditUpdate,
		createAssetEditFields,
		refreshAssetsStore
	} from './inventoryMutations';

	let deepParseInfo = $state<DeepParseInfo | null>(null);
	let loadingDeepParse = $state(false);
	let lastLoadedIp = $state<string | null>(null);

	let isEditing = $state(false);
	let editDeviceType = $state('');
	let editHostname = $state('');
	let editNotes = $state('');
	let editPurdueLevel = $state<number | null>(null);
	let editTags = $state('');
	let editSaving = $state(false);
	let editMessage = $state('');

	let credWarnings = $state<DefaultCredential[]>([]);
	let cveWarnings = $state<CveMatch[]>([]);
	let zeekEvents = $state<DeviceZeekEvents | null>(null);
	let assetAlerts = $state<CorrelatedAlert[]>([]);
	let detailRequestId = 0;

	$effect(() => {
		const asset = $selectedAsset;
		if (asset) {
			loadingDeepParse = true;
			const requestId = ++detailRequestId;
			void (async () => {
				const details = await loadAssetDetailData(asset.ip_address, asset);
				if (requestId !== detailRequestId) return;
				deepParseInfo = details.deepParseInfo;
				credWarnings = details.credWarnings;
				cveWarnings = details.cveWarnings;
				zeekEvents = details.zeekEvents;
				assetAlerts = details.assetAlerts;
				loadingDeepParse = false;
			})();
		} else {
			detailRequestId++;
			deepParseInfo = null;
			loadingDeepParse = false;
			lastLoadedIp = null;
			isEditing = false;
			editMessage = '';
			credWarnings = [];
			cveWarnings = [];
			zeekEvents = null;
			assetAlerts = [];
		}
	});

	$effect(() => {
		const asset = $selectedAsset;
		if (asset && asset.ip_address !== lastLoadedIp) {
			lastLoadedIp = asset.ip_address;
			isEditing = false;
			editMessage = '';
		}
	});

	function startEditing(asset: Asset) {
		const fields = createAssetEditFields(asset);
		editDeviceType = fields.deviceType;
		editHostname = fields.hostname;
		editNotes = fields.notes;
		editPurdueLevel = fields.purdueLevel;
		editTags = fields.tags;
		editMessage = '';
		isEditing = true;
	}

	async function saveEdits() {
		const asset = $selectedAsset;
		if (!asset) return;
		editSaving = true;
		editMessage = '';

		const updates = buildAssetEditUpdate(asset, {
			deviceType: editDeviceType,
			hostname: editHostname,
			notes: editNotes,
			purdueLevel: editPurdueLevel,
			tags: editTags
		});
		if (!updates) {
			isEditing = false;
			editSaving = false;
			return;
		}

		try {
			await updateAsset(asset.id, updates);
			await refreshAssetsStore();
			editMessage = 'Saved';
			isEditing = false;
		} catch (err) {
			editMessage = `Error: ${err}`;
		}
		editSaving = false;
	}

	function updateEditField(field: string, value: unknown) {
		if (field === 'deviceType') editDeviceType = value as string;
		else if (field === 'hostname') editHostname = value as string;
		else if (field === 'purdueLevel') editPurdueLevel = value as number | null;
		else if (field === 'tags') editTags = value as string;
		else if (field === 'notes') editNotes = value as string;
	}
</script>

<AssetDetailPanel
	asset={$selectedAsset}
	{deepParseInfo}
	{loadingDeepParse}
	{credWarnings}
	{cveWarnings}
	{zeekEvents}
	{assetAlerts}
	{isEditing}
	{editDeviceType}
	{editHostname}
	{editNotes}
	{editPurdueLevel}
	{editTags}
	{editSaving}
	{editMessage}
	onStartEditing={startEditing}
	onClose={() => selectedAssetId.set(null)}
	onSave={saveEdits}
	onCancelEdit={() => {
		isEditing = false;
		editMessage = '';
	}}
	onFieldChange={updateEditField}
/>

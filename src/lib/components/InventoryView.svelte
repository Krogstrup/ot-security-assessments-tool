<script lang="ts">
	import {
		assetFilter,
		filteredAssets,
		protocolFilter,
		selectedAssetId
	} from '$lib/stores/core';
	import { bulkUpdateAssets } from '$lib/api';
	import type { IcsProtocol } from '$lib/types/protocols';

	import AssetBulkEditPanel from './inventory/AssetBulkEditPanel.svelte';
	import AssetDetailContainer from './inventory/AssetDetailContainer.svelte';
	import AssetListPanel from './inventory/AssetListPanel.svelte';
	import InventoryToolbar from './inventory/InventoryToolbar.svelte';
	import {
		DEFAULT_VISIBLE_COLUMNS,
		INVENTORY_PAGE_SIZE,
		sortAssetsByColumn
	} from './inventory/assetTableConfig';
	import type { AssetColKey } from './inventory/assetTableConfig';
	import {
		buildBulkAssetUpdate,
		refreshAssetsStore
	} from './inventory/inventoryMutations';

	const protocols: IcsProtocol[] = ['modbus', 'dnp3', 'ethernet_ip', 'bacnet', 's7comm', 'opc_ua'];

	let selectedIds = $state<Set<string>>(new Set());
	let showBulkPanel = $state(false);
	let bulkDeviceType = $state('');
	let bulkPurdueLevel = $state('');
	let bulkTag = $state('');
	let bulkNotes = $state('');
	let bulkSaving = $state(false);

	let invPage = $state(0);
	let visibleColumns = $state<Set<AssetColKey>>(new Set(DEFAULT_VISIBLE_COLUMNS));
	let showColumnPicker = $state(false);
	let sortColumn = $state<AssetColKey | ''>('');
	let sortDirection = $state<'asc' | 'desc'>('asc');

	function toggleSelect(id: string) {
		const next = new Set(selectedIds);
		if (next.has(id)) {
			next.delete(id);
		} else {
			next.add(id);
		}
		selectedIds = next;
		showBulkPanel = next.size > 0;
	}

	function selectAll() {
		if (selectedIds.size === $filteredAssets.length) {
			selectedIds = new Set();
			showBulkPanel = false;
		} else {
			selectedIds = new Set($filteredAssets.map((a) => a.id));
			showBulkPanel = true;
		}
	}

	function handleFilterInput(event: Event) {
		const target = event.target as HTMLInputElement;
		assetFilter.set(target.value);
		invPage = 0;
	}

	function toggleSort(key: AssetColKey) {
		if (sortColumn === key) {
			if (sortDirection === 'asc') {
				sortDirection = 'desc';
			} else {
				sortColumn = '';
				sortDirection = 'asc';
			}
		} else {
			sortColumn = key;
			sortDirection = 'asc';
		}
		invPage = 0;
	}

	function toggleColumnVisibility(key: AssetColKey) {
		const next = new Set(visibleColumns);
		if (next.has(key)) next.delete(key);
		else next.add(key);
		visibleColumns = next;
	}

	function updateBulkField(field: string, value: string) {
		if (field === 'deviceType') bulkDeviceType = value;
		else if (field === 'purdueLevel') bulkPurdueLevel = value;
		else if (field === 'tag') bulkTag = value;
		else if (field === 'notes') bulkNotes = value;
	}

	async function applyBulkUpdate() {
		if (selectedIds.size === 0) return;
		bulkSaving = true;
		const updates = buildBulkAssetUpdate({
			deviceType: bulkDeviceType,
			purdueLevel: bulkPurdueLevel,
			tag: bulkTag,
			notes: bulkNotes
		});
		if (!updates) {
			bulkSaving = false;
			return;
		}

		try {
			await bulkUpdateAssets(Array.from(selectedIds), updates);
			await refreshAssetsStore();
			selectedIds = new Set();
			showBulkPanel = false;
			bulkDeviceType = '';
			bulkPurdueLevel = '';
			bulkTag = '';
			bulkNotes = '';
		} catch (err) {
			console.error('Bulk update failed:', err);
		}
		bulkSaving = false;
	}

	let sortedAssets = $derived.by(() => {
		return sortAssetsByColumn($filteredAssets, sortColumn, sortDirection);
	});

	let pagedAssets = $derived(
		sortedAssets.slice(invPage * INVENTORY_PAGE_SIZE, (invPage + 1) * INVENTORY_PAGE_SIZE)
	);

	$effect(() => {
		$filteredAssets;
		invPage = 0;
	});
</script>

<div class="inventory-container">
	<InventoryToolbar
		{protocols}
		filteredCount={$filteredAssets.length}
		onFilterInput={handleFilterInput}
		onProtocolChange={(protocol) => protocolFilter.set(protocol)}
	/>

	{#if showBulkPanel}
		<AssetBulkEditPanel
			selectedCount={selectedIds.size}
			{bulkDeviceType}
			{bulkPurdueLevel}
			{bulkTag}
			{bulkNotes}
			{bulkSaving}
			onApply={applyBulkUpdate}
			onClear={() => {
				selectedIds = new Set();
				showBulkPanel = false;
			}}
			onFieldChange={updateBulkField}
		/>
	{/if}

	<div class="inventory-body">
		<AssetListPanel
			assets={pagedAssets}
			filteredCount={$filteredAssets.length}
			selectedAssetId={$selectedAssetId}
			{selectedIds}
			{sortColumn}
			{sortDirection}
			{visibleColumns}
			{showColumnPicker}
			onSelectAsset={(id) => selectedAssetId.set(id)}
			onToggleSelect={toggleSelect}
			onSelectAll={selectAll}
			onSort={toggleSort}
			onToggleColumnVisibility={toggleColumnVisibility}
			onToggleColumnPicker={() => {
				showColumnPicker = !showColumnPicker;
			}}
			onPageChange={(page) => {
				invPage = page;
			}}
		/>

		<AssetDetailContainer />
	</div>
</div>

<style>
	.inventory-container {
		display: flex;
		flex-direction: column;
		height: 100%;
	}

	.inventory-body {
		flex: 1;
		display: flex;
		overflow: hidden;
	}
</style>

<script lang="ts">
	import type { BaselineDiff } from '$lib/types/analysis';
	import DriftScoreSection from './DriftScoreSection.svelte';
	import DriftSummaryGrid from './DriftSummaryGrid.svelte';
	import DriftAssetListSection from './DriftAssetListSection.svelte';
	import DriftChangedAssetsSection from './DriftChangedAssetsSection.svelte';
	import DriftConnectionListSection from './DriftConnectionListSection.svelte';

	interface Props {
		diff: BaselineDiff;
	}

	let { diff }: Props = $props();
	let summary = $derived(diff.summary);
</script>

<DriftScoreSection summary={summary} />
<DriftSummaryGrid {diff} />

<div class="drift-sections">
	{#if diff.new_assets.length > 0}
		<DriftAssetListSection title="New Devices" variant="new" assets={diff.new_assets} />
	{/if}

	{#if diff.missing_assets.length > 0}
		<DriftAssetListSection title="Missing Devices" variant="missing" assets={diff.missing_assets} />
	{/if}

	{#if diff.changed_assets.length > 0}
		<DriftChangedAssetsSection assets={diff.changed_assets} />
	{/if}

	{#if diff.new_connections.length > 0}
		<DriftConnectionListSection title="New Connections" variant="new" connections={diff.new_connections} />
	{/if}

	{#if diff.missing_connections.length > 0}
		<DriftConnectionListSection title="Missing Connections" variant="missing" connections={diff.missing_connections} />
	{/if}

	{#if diff.new_assets.length === 0 &&
		diff.missing_assets.length === 0 &&
		diff.changed_assets.length === 0 &&
		diff.new_connections.length === 0 &&
		diff.missing_connections.length === 0}
		<div class="no-drift">
			<p>No drift detected. The current network state matches the baseline.</p>
		</div>
	{/if}
</div>

<style>
	.drift-sections {
		flex: 1;
		overflow-y: auto;
		padding: 16px 20px;
	}

	.no-drift {
		padding: 40px 20px;
		text-align: center;
		color: var(--gm-text-muted);
		font-size: 12px;
	}
</style>

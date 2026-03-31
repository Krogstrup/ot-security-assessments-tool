<script lang="ts">
	import { protocolStats } from '$lib/stores/core';

	import CaptureHeader from './capture/CaptureHeader.svelte';
	import ImportSessionSection from './capture/ImportSessionSection.svelte';
	import LiveCaptureSection from './capture/LiveCaptureSection.svelte';
	import LiveAlertsSection from './capture/LiveAlertsSection.svelte';
	import ProtocolStatsPanel from './capture/ProtocolStatsPanel.svelte';

	let alertsRefreshToken = $state(0);

	function notifyDataChanged() {
		alertsRefreshToken += 1;
	}
</script>

<div class="capture-container">
	<CaptureHeader />

	<div class="capture-content">
		<ImportSessionSection onDataChanged={notifyDataChanged} />
		<LiveCaptureSection onDataChanged={notifyDataChanged} />
		<ProtocolStatsPanel stats={$protocolStats} />
		<LiveAlertsSection refreshToken={alertsRefreshToken} />
	</div>
</div>

<style>
	.capture-container {
		display: flex;
		flex-direction: column;
		height: 100%;
		background: var(--gm-bg-primary);
	}

	.capture-content {
		flex: 1;
		overflow-y: auto;
		padding: 1rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}
</style>

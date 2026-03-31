<script lang="ts">
	import type { AllowlistEntry } from '$lib/types/analysis';
	import { generateCommunicationAllowlist } from '$lib/api';

	import AllowlistSection from './AllowlistSection.svelte';
	import { allowlistClassificationClass } from './exportUtils';

	interface Props {
		hasData: boolean;
		busyAction: string | null;
		onShowStatus: (message: string, type?: 'success' | 'error' | 'info') => void;
		onExportAllowlistCsv: () => Promise<void>;
		onExportFirewallRules: () => Promise<void>;
	}

	let {
		hasData,
		busyAction,
		onShowStatus,
		onExportAllowlistCsv,
		onExportFirewallRules
	}: Props = $props();

	let allowlistEntries = $state<AllowlistEntry[]>([]);
	let showAllowlist = $state(false);
	let loadingAllowlist = $state(false);

	async function loadAllowlist() {
		loadingAllowlist = true;
		try {
			allowlistEntries = await generateCommunicationAllowlist();
			showAllowlist = true;
		} catch (err) {
			onShowStatus(`Failed to generate allowlist: ${err}`, 'error');
		} finally {
			loadingAllowlist = false;
		}
	}
</script>

<AllowlistSection
	{allowlistEntries}
	{showAllowlist}
	{loadingAllowlist}
	{hasData}
	{busyAction}
	classificationClass={allowlistClassificationClass}
	onGenerateAllowlist={loadAllowlist}
	onExportAllowlistCsv={onExportAllowlistCsv}
	onExportFirewallRules={onExportFirewallRules}
/>

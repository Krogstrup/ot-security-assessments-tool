<script lang="ts">
	import { getFindings } from '$lib/api';

	import RemediationSection from './RemediationSection.svelte';
	import {
		buildRemediationCsv,
		mapFindingsToRemediation,
		type RemediationItem
	} from './exportUtils';

	interface Props {
		busyAction: string | null;
		onBusyActionChange: (value: string | null) => void;
		onShowStatus: (message: string, type?: 'success' | 'error' | 'info') => void;
	}

	let { busyAction, onBusyActionChange, onShowStatus }: Props = $props();

	let remediationItems = $state<RemediationItem[]>([]);
	let showRemediationTable = $state(false);
	let loadingRemediation = $state(false);

	async function loadRemediationList() {
		loadingRemediation = true;
		try {
			remediationItems = mapFindingsToRemediation(await getFindings());
			showRemediationTable = true;
		} catch (err) {
			onShowStatus(`Failed to load findings: ${err}`, 'error');
		} finally {
			loadingRemediation = false;
		}
	}

	async function exportRemediationCsv() {
		const csv = buildRemediationCsv(remediationItems);
		try {
			onBusyActionChange('remediation_csv');
			await navigator.clipboard.writeText(csv);
			onShowStatus('Remediation CSV copied to clipboard (paste into a .csv file)', 'info');
		} catch (err) {
			onShowStatus(`Export failed: ${err}`, 'error');
		} finally {
			onBusyActionChange(null);
		}
	}
</script>

<RemediationSection
	{remediationItems}
	{showRemediationTable}
	{loadingRemediation}
	{busyAction}
	onGenerateList={loadRemediationList}
	onExportCsv={exportRemediationCsv}
/>

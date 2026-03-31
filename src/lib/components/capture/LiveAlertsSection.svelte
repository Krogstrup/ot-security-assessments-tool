<script lang="ts">
	import { getFindings } from '$lib/api';
	import type { Finding } from '$lib/types/analysis';

	import LiveAlertsPanel from './LiveAlertsPanel.svelte';

	interface Props {
		refreshToken?: number;
	}

	let { refreshToken = 0 }: Props = $props();

	let liveFindings = $state<Finding[]>([]);
	let liveAlertsLoading = $state(false);
	let liveAlertsError = $state('');
	let requestId = 0;

	$effect(() => {
		refreshToken;
		const currentRequest = ++requestId;
		liveAlertsLoading = true;
		liveAlertsError = '';

		void (async () => {
			try {
				const findings = await getFindings();
				if (currentRequest !== requestId) return;
				liveFindings = findings;
			} catch (err) {
				if (currentRequest !== requestId) return;
				liveFindings = [];
				liveAlertsError = String(err);
			} finally {
				if (currentRequest === requestId) {
					liveAlertsLoading = false;
				}
			}
		})();
	});
</script>

<LiveAlertsPanel findings={liveFindings} loading={liveAlertsLoading} error={liveAlertsError} />

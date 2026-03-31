import {
	getAlertsForIp,
	getCredentialWarnings,
	getCveWarnings,
	getDeepParseInfo,
	getDeviceZeekEvents
} from '$lib/api';
import type {
	CorrelatedAlert,
	CveMatch,
	DefaultCredential,
	DeviceZeekEvents
} from '$lib/types/analysis';
import type { Asset } from '$lib/types/assets';
import type { DeepParseInfo } from '$lib/types/deep-parse';
import { filterCredentialWarningsForAsset, hasZeekActivity } from './assetDetails';

export interface LoadedAssetDetailData {
	deepParseInfo: DeepParseInfo | null;
	credWarnings: DefaultCredential[];
	cveWarnings: CveMatch[];
	zeekEvents: DeviceZeekEvents | null;
	assetAlerts: CorrelatedAlert[];
}

export async function loadAssetDetailData(
	ip: string,
	selectedAsset: Asset | null
): Promise<LoadedAssetDetailData> {
	const [deepParseInfo, credentialWarnings, cveWarnings, zeekEvents, assetAlerts] = await Promise.all([
		getDeepParseInfo(ip).catch(() => null),
		getCredentialWarnings().catch(() => [] as DefaultCredential[]),
		getCveWarnings(ip).catch(() => [] as CveMatch[]),
		getDeviceZeekEvents(ip).catch(() => null),
		getAlertsForIp(ip).catch(() => [] as CorrelatedAlert[])
	]);

	const filteredCredWarnings = filterCredentialWarningsForAsset(credentialWarnings, selectedAsset);
	const filteredZeekEvents = zeekEvents && hasZeekActivity(zeekEvents) ? zeekEvents : null;

	return {
		deepParseInfo,
		credWarnings: filteredCredWarnings,
		cveWarnings,
		zeekEvents: filteredZeekEvents,
		assetAlerts
	};
}

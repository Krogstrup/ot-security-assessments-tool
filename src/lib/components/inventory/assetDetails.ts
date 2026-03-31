import type { DefaultCredential, DeviceZeekEvents } from '$lib/types/analysis';
import type { Asset } from '$lib/types/assets';

export function filterCredentialWarningsForAsset(
	warnings: DefaultCredential[],
	asset: Asset | null
): DefaultCredential[] {
	if (!asset) return [];

	const vendor = (asset.vendor ?? asset.oui_vendor ?? '').toLowerCase();
	const product = (asset.product_family ?? '').toLowerCase();
	if (!vendor && !product) return [];

	return warnings.filter((warning) => {
		const warningVendor = warning.vendor.toLowerCase();
		return vendor ? warningVendor.includes(vendor) || vendor.includes(warningVendor) : false;
	});
}

export function hasZeekActivity(events: DeviceZeekEvents): boolean {
	const total =
		events.conn_log_entries +
		events.modbus_events +
		events.dnp3_events +
		events.dns_queries +
		events.http_requests;
	return total > 0;
}

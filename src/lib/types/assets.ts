import type { IcsProtocol } from './protocols';

// AssetSignatureMatch crosses the IPC boundary — re-export from generated binding.
import type { AssetSignatureMatch } from '../../../backend/bindings/gen/types/AssetSignatureMatch';
export type { AssetSignatureMatch };

export type DeviceType =
	| 'plc'
	| 'rtu'
	| 'hmi'
	| 'historian'
	| 'engineering_workstation'
	| 'scada_server'
	| 'it_device'
	| 'unknown';

export type PurdueLevel = 0 | 1 | 2 | 3 | 4 | 5;

export interface Asset {
	id: string;
	ip_address: string;
	mac_address: string | null;
	hostname: string | null;
	device_type: DeviceType;
	vendor: string | null;
	protocols: IcsProtocol[];
	first_seen: string;
	last_seen: string;
	notes: string;
	purdue_level: PurdueLevel | null;
	tags: string[];
	packet_count: number;
	/** Overall confidence score (1-5), highest from any signature match. */
	confidence: number;
	/** Vendor-specific product identification from signatures. */
	product_family: string | null;
	/** All signature matches for this asset. */
	signature_matches: AssetSignatureMatch[];
	/** Vendor from IEEE OUI database (MAC prefix lookup). */
	oui_vendor: string | null;
	/** ISO 3166-1 alpha-2 country code (public IPs only). */
	country: string | null;
	/** Whether this IP is a public (routable) address. */
	is_public_ip: boolean;
}


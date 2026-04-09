import type { DeviceType } from './assets';

// PacketSummary crosses the IPC boundary — re-export from generated binding.
export type { PacketSummary } from '../../../backend/bindings/gen/types/PacketSummary';

export interface Connection {
	id: string;
	src_ip: string;
	src_port: number;
	src_mac: string | null;
	dst_ip: string;
	dst_port: number;
	dst_mac: string | null;
	protocol: string;
	transport: string;
	packet_count: number;
	byte_count: number;
	first_seen: string;
	last_seen: string;
	origin_files: string[];
}

/** Connection tree node grouped by source IP. */
export interface ConnectionTreeNode {
	ip: string;
	device_type: DeviceType;
	mac_address: string | null;
	packet_count: number;
	connections: Connection[];
}

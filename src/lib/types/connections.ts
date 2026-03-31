import type { DeviceType } from './assets';

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

/** Packet summary row for connection drilldown. */
export interface PacketSummary {
	timestamp: string;
	src_ip: string;
	dst_ip: string;
	src_port: number;
	dst_port: number;
	protocol: string;
	length: number;
	origin_file: string;
}

/** Connection tree node grouped by source IP. */
export interface ConnectionTreeNode {
	ip: string;
	device_type: DeviceType;
	mac_address: string | null;
	packet_count: number;
	connections: Connection[];
}

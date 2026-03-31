import type { IcsProtocol } from './protocols';

export type CaptureStatus = 'idle' | 'capturing' | 'paused' | 'error';

export interface CaptureConfig {
	interface_name: string;
	bpf_filter: string | null;
	promiscuous: boolean;
}

export interface ImportResult {
	file_count: number;
	packet_count: number;
	connection_count: number;
	asset_count: number;
	protocols_detected: string[];
	duration_ms: number;
	per_file: FileImportResult[];
}

export interface FileImportResult {
	filename: string;
	packet_count: number;
	status: string;
}

/** Capture stream packet event payload. */
export interface PacketEvent {
	timestamp: string;
	src_ip: string;
	dst_ip: string;
	protocol: IcsProtocol;
	length: number;
}

/** Capture stream aggregate stats payload. */
export interface CaptureStatsEvent {
	packets_captured: number;
	packets_per_second: number;
	bytes_captured: number;
	active_connections: number;
	asset_count: number;
	elapsed_seconds: number;
}

/** Result of stopping a capture run. */
export interface StopCaptureResult {
	packets_captured: number;
	bytes_captured: number;
	elapsed_seconds: number;
	pcap_saved: boolean;
	pcap_path: string | null;
	packets_saved: number;
}

/** Current capture status snapshot from backend. */
export interface CaptureStatusInfo {
	is_running: boolean;
	is_paused: boolean;
	packets_captured: number;
	bytes_captured: number;
	elapsed_seconds: number;
}

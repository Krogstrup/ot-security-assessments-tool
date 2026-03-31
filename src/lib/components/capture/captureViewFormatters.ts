import type { CaptureStatsEvent, ImportResult, StopCaptureResult } from '$lib/types/capture';
import type { SessionInfo } from '$lib/types/operations';

export interface CaptureStopSummary {
	packets: number;
	bytes: number;
	elapsed: number;
	saved: boolean;
	path: string | null;
}

export function formatImportSuccessMessage(result: ImportResult) {
	return `Imported ${result.packet_count.toLocaleString()} packets from ${result.file_count} file${result.file_count > 1 ? 's' : ''} -> ${result.asset_count} assets, ${result.connection_count} connections (${result.duration_ms}ms)`;
}

export function emptyCaptureStats(): CaptureStatsEvent {
	return {
		packets_captured: 0,
		packets_per_second: 0,
		bytes_captured: 0,
		active_connections: 0,
		asset_count: 0,
		elapsed_seconds: 0
	};
}

export function mapStopCaptureSummary(result: StopCaptureResult): CaptureStopSummary {
	return {
		packets: Number(result.packets_captured),
		bytes: Number(result.bytes_captured),
		elapsed: result.elapsed_seconds,
		saved: result.pcap_saved,
		path: result.pcap_path
	};
}

export function defaultCaptureFileName(date = new Date()) {
	return `capture_${date.toISOString().slice(0, 19).replace(/:/g, '-')}.pcap`;
}

export function defaultSessionArchiveName(session: SessionInfo) {
	return `${session.name.replace(/[^a-zA-Z0-9_-]/g, '_')}.kkj`;
}

export function formatBytes(bytes: number): string {
	if (bytes < 1024) return `${bytes} B`;
	if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
	if (bytes < 1024 * 1024 * 1024) return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
	return `${(bytes / 1024 / 1024 / 1024).toFixed(2)} GB`;
}

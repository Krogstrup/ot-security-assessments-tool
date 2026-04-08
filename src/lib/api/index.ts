/**
 * API module barrel export.
 *
 * Provides domain-organized access to all Tauri backend commands.
 * Prefer importing specific functions from domain modules (e.g., `import { getAssets } from '$lib/api/assets'`)
 * for better tree-shaking, but this barrel export enables migration from the legacy `$lib/api`.
 */

// Core utilities
export { invokeCompat, httpJson, isAppError, invokeValidated } from './core';

// Domain modules
export * from './system';
export {
	importPcap,
	cancelImport,
	onImportProgress,
	startCapture,
	stopCapture,
	pauseCapture,
	resumeCapture,
	getCaptureStatus,
	onPacketEvent,
	onCaptureStats,
	onCaptureError,
	getProtocolStats as getCaptureProtocolStats
} from './capture';
export * from './assets';
export {
	getConnections,
	getDataCounts,
	getConnectionPackets,
	getProtocolStats,
	getConnectionStats,
	getPatternAnomalies,
	getRedundancyProtocols
} from './connections';
export * from './analysis';
export * from './session';
export * from './physical';
export * from './ingest';
export * from './correlation';
export * from './export';
export * from './projects';
export * from './wireshark';
export * from './signatures';

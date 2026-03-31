/**
 * API module barrel export.
 *
 * Provides domain-organized access to all Tauri backend commands.
 * Prefer importing specific functions from domain modules (e.g., `import { getAssets } from '$lib/api/assets'`)
 * for better tree-shaking, but this barrel export enables migration from the legacy `$lib/utils/tauri`.
 */

// Core utilities
export { invokeCompat, httpJson, isTauriRuntime, isAppError, invokeValidated } from './core';

// Domain modules
export * from './system';
export * from './capture';
export * from './assets';
export * from './connections';
export * from './analysis';
export * from './session';
export * from './physical';
export * from './ingest';
export * from './export';
export * from './projects';
export * from './wireshark';

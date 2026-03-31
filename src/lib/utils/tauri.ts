/**
 * DEPRECATED: Use `$lib/api` instead.
 *
 * This module is maintained for backward compatibility only.
 * All functions have been moved to domain-organized modules under `$lib/api/`.
 *
 * Migration path:
 *   Old: import { getAssets, runAnalysis } from '$lib/utils/tauri'
 *   New: import { getAssets } from '$lib/api/assets'
 *        import { runAnalysis } from '$lib/api/analysis'
 *
 * Or use the barrel export:
 *   import { getAssets, runAnalysis } from '$lib/api'
 */

// Re-export all public functions from the new api module for backward compatibility
export * from '../api';

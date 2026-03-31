/**
 * @deprecated — use domain imports (e.g. `$lib/types/assets`, `$lib/types/analysis`)
 *
 * Compatibility barrel for frontend domain types. All consumers have been
 * migrated to direct domain imports. This barrel is retained for any external
 * code that may still reference it, but should not be used in new code.
 */

export * from './errors';
export * from './network';
export * from './capture';
export * from './protocols';
export * from './assets';
export * from './connections';
export * from './pagination';
export * from './topology';
export * from './signatures';

export * from './deep-parse';
export * from './operations';
export * from './analysis';
export * from './segmentation';

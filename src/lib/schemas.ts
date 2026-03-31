/**
 * Runtime Zod schemas for the five highest-traffic IPC response types.
 *
 * These schemas mirror the TypeScript types in `$lib/types/index.ts` and the
 * Rust structs in `src-tauri/src/commands/`. When a Rust struct is renamed or
 * a field is added/removed, update the corresponding schema here so that
 * mismatches are surfaced as runtime ZodErrors instead of silent `undefined`
 * field values.
 *
 * Usage:
 *   import { assetSchema } from '$lib/schemas';
 *   const asset = assetSchema.parse(raw);        // throws ZodError on mismatch
 *   const result = assetSchema.safeParse(raw);   // { success, data | error }
 */

import { z } from 'zod';

// ─── AssetSignatureMatch ──────────────────────────────────────────────────────

export const assetSignatureMatchSchema = z.object({
	signature_name: z.string(),
	confidence: z.number(),
	vendor: z.string().nullable(),
	product_family: z.string().nullable(),
	device_type: z.string().nullable(),
	role: z.string().nullable()
});

// ─── AssetInfo ────────────────────────────────────────────────────────────────

export const assetSchema = z.object({
	id: z.string(),
	ip_address: z.string(),
	mac_address: z.string().nullable(),
	hostname: z.string().nullable(),
	device_type: z.string(),
	vendor: z.string().nullable(),
	protocols: z.array(z.string()),
	first_seen: z.string(),
	last_seen: z.string(),
	notes: z.string(),
	purdue_level: z.number().nullable(),
	tags: z.array(z.string()),
	packet_count: z.union([z.number(), z.bigint()]),
	confidence: z.number(),
	product_family: z.string().nullable(),
	signature_matches: z.array(assetSignatureMatchSchema),
	oui_vendor: z.string().nullable().optional(),
	country: z.string().nullable().optional(),
	is_public_ip: z.boolean().default(false)
});

export type AssetSchema = z.infer<typeof assetSchema>;

// ─── ConnectionInfo ───────────────────────────────────────────────────────────

export const connectionSchema = z.object({
	id: z.string(),
	src_ip: z.string(),
	src_port: z.number(),
	src_mac: z.string().nullable(),
	dst_ip: z.string(),
	dst_port: z.number(),
	dst_mac: z.string().nullable(),
	protocol: z.string(),
	transport: z.string(),
	packet_count: z.union([z.number(), z.bigint()]),
	byte_count: z.union([z.number(), z.bigint()]),
	first_seen: z.string(),
	last_seen: z.string(),
	origin_files: z.array(z.string())
});

export type ConnectionSchema = z.infer<typeof connectionSchema>;

// ─── Finding ──────────────────────────────────────────────────────────────────

const findingTypeSchema = z.enum(['attack_technique', 'purdue_violation', 'anomaly']);
const severitySchema = z.enum(['info', 'low', 'medium', 'high', 'critical']);

export const findingSchema = z.object({
	id: z.string(),
	finding_type: findingTypeSchema,
	severity: severitySchema,
	title: z.string(),
	description: z.string(),
	affected_assets: z.array(z.string()),
	evidence: z.string(),
	technique_id: z.string().nullable(),
	created_at: z.string()
});

export type FindingSchema = z.infer<typeof findingSchema>;

// ─── PurdueAssignment ─────────────────────────────────────────────────────────

export const purdueAssignmentSchema = z.object({
	ip_address: z.string(),
	level: z.number(),
	method: z.enum(['auto', 'manual']),
	reason: z.string()
});

export type PurdueAssignmentSchema = z.infer<typeof purdueAssignmentSchema>;

// ─── AnomalyScore ─────────────────────────────────────────────────────────────

export const anomalyScoreSchema = z.object({
	anomaly_type: z.string(),
	severity: severitySchema,
	confidence: z.number(),
	affected_asset: z.string(),
	evidence: z.string()
});

export type AnomalyScoreSchema = z.infer<typeof anomalyScoreSchema>;

// ─── AppError ─────────────────────────────────────────────────────────────────

export const appErrorSchema = z.object({
	code: z.enum([
		'state_lock',
		'no_session',
		'no_project',
		'invalid_input',
		'parse_failure',
		'db_error',
		'io_error',
		'no_capture_running',
		'external_process'
	]),
	message: z.string().optional()
});

export type AppErrorSchema = z.infer<typeof appErrorSchema>;

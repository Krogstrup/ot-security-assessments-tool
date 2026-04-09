/**
 * Runtime Zod schemas for the five highest-traffic IPC response types.
 *
 * These schemas mirror the TypeScript types in `$lib/types/index.ts` and the
 * Rust structs in `backend/src/commands/`. When a Rust struct is renamed or
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
	anomaly_type: z.enum(['polling_deviation', 'role_reversal', 'new_device', 'unexpected_public_ip']),
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
		'external_process',
		'bad_request',
		'internal_error',
		'http_error'
	]),
	message: z.string().optional()
});

export type AppErrorSchema = z.infer<typeof appErrorSchema>;

// ─── Project ──────────────────────────────────────────────────────────────────

export const projectSchema = z.object({
	id: z.number(),
	name: z.string(),
	client_name: z.string(),
	site_name: z.string(),
	assessor_name: z.string(),
	engagement_start: z.string(),
	engagement_end: z.string(),
	notes: z.string(),
	created_at: z.string(),
	updated_at: z.string()
});

export type ProjectSchema = z.infer<typeof projectSchema>;

export const projectSummarySchema = z.object({
	id: z.number(),
	name: z.string(),
	client_name: z.string(),
	site_name: z.string(),
	session_count: z.number(),
	created_at: z.string(),
	updated_at: z.string()
});

export type ProjectSummarySchema = z.infer<typeof projectSummarySchema>;

// ─── Session ──────────────────────────────────────────────────────────────────

export const sessionInfoSchema = z.object({
	id: z.string(),
	name: z.string(),
	description: z.string(),
	created_at: z.string(),
	updated_at: z.string(),
	asset_count: z.number(),
	connection_count: z.number()
});

export type SessionInfoSchema = z.infer<typeof sessionInfoSchema>;

// ─── BaselineDiff ─────────────────────────────────────────────────────────────

const driftAssetSchema = z.object({
	ip_address: z.string(),
	mac_address: z.string().nullable(),
	device_type: z.string(),
	vendor: z.string().nullable(),
	protocols: z.array(z.string()),
	confidence: z.number()
});

const assetChangeSchema = z.object({
	field: z.string(),
	baseline_value: z.string(),
	current_value: z.string()
});

const changedAssetSchema = z.object({
	ip_address: z.string(),
	changes: z.array(assetChangeSchema)
});

const driftConnectionSchema = z.object({
	src_ip: z.string(),
	dst_ip: z.string(),
	src_port: z.number(),
	dst_port: z.number(),
	protocol: z.string()
});

const driftSummarySchema = z.object({
	total_baseline_assets: z.number(),
	total_current_assets: z.number(),
	new_asset_count: z.number(),
	missing_asset_count: z.number(),
	changed_asset_count: z.number(),
	new_connection_count: z.number(),
	missing_connection_count: z.number(),
	drift_score: z.number()
});

export const baselineDiffSchema = z.object({
	baseline_session_name: z.string(),
	new_assets: z.array(driftAssetSchema),
	missing_assets: z.array(driftAssetSchema),
	changed_assets: z.array(changedAssetSchema),
	new_connections: z.array(driftConnectionSchema),
	missing_connections: z.array(driftConnectionSchema),
	summary: driftSummarySchema
});

export type BaselineDiffSchema = z.infer<typeof baselineDiffSchema>;

// ─── AnalysisResult ───────────────────────────────────────────────────────────

const analysisSummarySchema = z.object({
	total_findings: z.number(),
	critical_count: z.number(),
	high_count: z.number(),
	medium_count: z.number(),
	low_count: z.number(),
	info_count: z.number(),
	purdue_violations: z.number(),
	anomaly_count: z.number(),
	assets_analyzed: z.number(),
	connections_analyzed: z.number(),
	unencrypted_ot_percent: z.number()
});

export const analysisResultSchema = z.object({
	findings: z.array(findingSchema),
	purdue_assignments: z.array(purdueAssignmentSchema),
	anomalies: z.array(anomalyScoreSchema),
	summary: analysisSummarySchema
});

export type AnalysisResultSchema = z.infer<typeof analysisResultSchema>;

// ─── DefaultCredential ────────────────────────────────────────────────────────

export const defaultCredentialSchema = z.object({
	vendor: z.string(),
	product_pattern: z.string(),
	protocol: z.string(),
	username: z.string(),
	password: z.string(),
	source: z.string(),
	severity: z.string()
});

export type DefaultCredentialSchema = z.infer<typeof defaultCredentialSchema>;

// ─── CriticalityAssessment ────────────────────────────────────────────────────

export const criticalityAssessmentSchema = z.object({
	ip_address: z.string(),
	level: z.enum(['critical', 'high', 'medium', 'low', 'unknown']),
	reason: z.string()
});

export type CriticalityAssessmentSchema = z.infer<typeof criticalityAssessmentSchema>;

// ─── NamingSuggestion ─────────────────────────────────────────────────────────

export const namingSuggestionSchema = z.object({
	ip_address: z.string(),
	suggested_name: z.string(),
	reason: z.string()
});

export type NamingSuggestionSchema = z.infer<typeof namingSuggestionSchema>;

// ─── MalwareFinding ───────────────────────────────────────────────────────────

export const malwareFindingSchema = z.object({
	malware_name: z.string(),
	confidence: z.string(),
	severity: z.string(),
	source_ip: z.string(),
	target_ips: z.array(z.string()),
	evidence: z.string(),
	attack_techniques: z.array(z.string()),
	pattern_description: z.string()
});

export type MalwareFindingSchema = z.infer<typeof malwareFindingSchema>;

// ─── CveMatch ─────────────────────────────────────────────────────────────────

export const cveMatchSchema = z.object({
	cve_id: z.string(),
	cvss: z.number(),
	description: z.string(),
	advisory: z.string(),
	remediation: z.string(),
	matched_product: z.string(),
	matched_firmware: z.string().nullable(),
	confidence: z.string(),
	severity_label: z.string()
});

export type CveMatchSchema = z.infer<typeof cveMatchSchema>;

// ─── ComplianceMapping ────────────────────────────────────────────────────────

export const complianceMappingSchema = z.object({
	framework: z.string(),
	requirement_id: z.string(),
	requirement_name: z.string(),
	status: z.enum(['gap', 'partial', 'met', 'not_assessed']),
	evidence: z.string(),
	description: z.string()
});

export type ComplianceMappingSchema = z.infer<typeof complianceMappingSchema>;

// ─── SwitchSecurityFinding ────────────────────────────────────────────────────

export const switchSecurityFindingSchema = z.object({
	finding_type: z.string(),
	title: z.string(),
	severity: z.enum(['info', 'low', 'medium', 'high', 'critical']),
	description: z.string(),
	affected_assets: z.array(z.string()),
	evidence: z.string(),
	remediation: z.string()
});

export type SwitchSecurityFindingSchema = z.infer<typeof switchSecurityFindingSchema>;

// ─── Data/Capture/Export/Ingest ──────────────────────────────────────────────

export const assetPageSchema = z.object({
	assets: z.array(assetSchema),
	total: z.number(),
	page: z.number(),
	page_size: z.number(),
	has_more: z.boolean()
});

export const connectionPageSchema = z.object({
	connections: z.array(connectionSchema),
	total: z.number(),
	page: z.number(),
	page_size: z.number(),
	has_more: z.boolean()
});

export const dataCountsSchema = z.object({
	asset_count: z.number(),
	connection_count: z.number()
});

export const protocolStatSchema = z.object({
	protocol: z.string(),
	packet_count: z.union([z.number(), z.bigint()]),
	byte_count: z.union([z.number(), z.bigint()]),
	connection_count: z.union([z.number(), z.bigint()]),
	unique_devices: z.union([z.number(), z.bigint()])
});

export const timelineRangeSchema = z.object({
	earliest: z.string().nullable(),
	latest: z.string().nullable(),
	connection_count: z.number()
});

export const fileImportResultSchema = z.object({
	filename: z.string(),
	packet_count: z.number(),
	status: z.string()
});

export const importResultSchema = z.object({
	file_count: z.number(),
	packet_count: z.number(),
	connection_count: z.number(),
	asset_count: z.number(),
	protocols_detected: z.array(z.string()),
	duration_ms: z.number(),
	per_file: z.array(fileImportResultSchema)
});

export const stopCaptureResultSchema = z.object({
	packets_captured: z.number(),
	bytes_captured: z.number(),
	elapsed_seconds: z.number(),
	pcap_saved: z.boolean(),
	pcap_path: z.string().nullable(),
	packets_saved: z.number()
});

export const captureStatusInfoSchema = z.object({
	is_running: z.boolean(),
	is_paused: z.boolean(),
	packets_captured: z.number(),
	bytes_captured: z.number(),
	elapsed_seconds: z.number()
});

export const ingestImportResultSchema = z.object({
	source: z.string(),
	files_processed: z.number(),
	asset_count: z.number(),
	connection_count: z.number(),
	alert_count: z.number(),
	new_assets: z.number(),
	updated_assets: z.number(),
	duration_ms: z.number(),
	errors: z.array(z.string())
});

export const filteredPcapResultSchema = z.object({
	output_path: z.string(),
	packets_written: z.number(),
	source_files: z.number()
});

export const allowlistEntrySchema = z.object({
	src_ip: z.string(),
	dst_ip: z.string(),
	protocol: z.string(),
	dst_port: z.number(),
	direction: z.string(),
	frequency: z.string(),
	avg_interval_ms: z.number().nullable(),
	classification: z.string(),
	justification: z.string()
});

export const wiresharkInfoSchema = z.object({
	found: z.boolean(),
	path: z.string().nullable(),
	version: z.string().nullable()
});

export const frameRowSchema = z.object({
	number: z.number(),
	timestamp: z.string(),
	src_ip: z.string(),
	dst_ip: z.string(),
	src_port: z.number(),
	dst_port: z.number(),
	protocol: z.string(),
	length: z.number(),
	origin_file: z.string()
});

export const deviceZeekEventsSchema = z.object({
	device_ip: z.string(),
	conn_log_entries: z.number(),
	modbus_events: z.number(),
	dnp3_events: z.number(),
	dns_queries: z.number(),
	http_requests: z.number(),
	unique_peers: z.number(),
	alert_count: z.number(),
	sample_events: z.array(
		z.object({
			timestamp: z.string(),
			log_type: z.string(),
			peer_ip: z.string(),
			summary: z.string()
		})
	)
});

// ─── Physical Topology & Inference ───────────────────────────────────────────

const cdpNeighborSchema = z.object({
	device_id: z.string(),
	remote_port: z.string(),
	platform: z.string().nullable(),
	ip_address: z.string().nullable(),
	capabilities: z.array(z.string())
});

const physicalPortSchema = z.object({
	name: z.string(),
	short_name: z.string(),
	description: z.string().nullable(),
	vlans: z.array(z.number()),
	mode: z.string(),
	shutdown: z.boolean(),
	ip_address: z.string().nullable(),
	subnet_mask: z.string().nullable(),
	mac_addresses: z.array(z.string()),
	ip_addresses: z.array(z.string()),
	cdp_neighbor: cdpNeighborSchema.nullable(),
	speed: z.string().nullable(),
	duplex: z.string().nullable()
});

const physicalSwitchSchema = z.object({
	hostname: z.string(),
	management_ip: z.string().nullable(),
	model: z.string().nullable(),
	ios_version: z.string().nullable(),
	ports: z.array(physicalPortSchema),
	vlans: z.record(z.string(), z.string())
});

const physicalLinkSchema = z.object({
	src_switch: z.string(),
	src_port: z.string(),
	dst_switch: z.string(),
	dst_port: z.string()
});

const deviceLocationSchema = z.object({
	ip_address: z.string(),
	mac_address: z.string().nullable(),
	switch_hostname: z.string(),
	port_name: z.string(),
	vlan: z.number().nullable()
});

export const physicalTopologySchema = z.object({
	switches: z.array(physicalSwitchSchema),
	links: z.array(physicalLinkSchema),
	device_locations: z.record(z.string(), deviceLocationSchema)
});

export const inferredTopologySchema = z.object({
	subnets: z.array(
		z.object({
			network: z.string(),
			member_ips: z.array(z.string()),
			gateway_ip: z.string().nullable()
		})
	),
	gateways: z.array(
		z.object({
			ip_address: z.string(),
			mac_address: z.string().nullable(),
			connected_subnets: z.array(z.string()),
			confidence: z.number()
		})
	),
	switch_candidates: z.array(
		z.object({
			ip_address: z.string().nullable(),
			mac_address: z.string().nullable(),
			connected_ips: z.array(z.string()),
			confidence: z.number()
		})
	),
	broadcast_domains: z.array(
		z.object({
			id: z.string(),
			network: z.string(),
			member_ips: z.array(z.string()),
			gateway_ip: z.string().nullable(),
			inferred_from: z.string()
		})
	)
});

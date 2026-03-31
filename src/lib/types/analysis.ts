// ─── Security Analysis (Phase 10) ────────────────────────

/** Finding type classification */
export type FindingType = 'attack_technique' | 'purdue_violation' | 'anomaly';

/** Severity levels for findings */
export type FindingSeverity = 'info' | 'low' | 'medium' | 'high' | 'critical';

/** A security finding from analysis */
export interface Finding {
	id: string;
	finding_type: FindingType;
	severity: FindingSeverity;
	title: string;
	description: string;
	affected_assets: string[];
	evidence: string;
	technique_id: string | null;
	created_at: string;
}

/** Purdue level assignment method */
export type PurdueMethod = 'auto' | 'manual';

/** Purdue level assignment for a device */
export interface PurdueAssignment {
	ip_address: string;
	level: number;
	method: PurdueMethod;
	reason: string;
}

/** Anomaly type classification */
export type AnomalyType = 'polling_deviation' | 'role_reversal' | 'new_device' | 'unexpected_public_ip';

/** An anomaly score from analysis */
export interface AnomalyScore {
	anomaly_type: AnomalyType;
	severity: FindingSeverity;
	confidence: number;
	affected_asset: string;
	evidence: string;
}

/** Full analysis result from the backend */
export interface AnalysisResult {
	findings: Finding[];
	purdue_assignments: PurdueAssignment[];
	anomalies: AnomalyScore[];
	summary: AnalysisSummary;
}

/** Summary statistics from an analysis run */
export interface AnalysisSummary {
	total_findings: number;
	critical_count: number;
	high_count: number;
	medium_count: number;
	low_count: number;
	info_count: number;
	purdue_violations: number;
	anomaly_count: number;
	assets_analyzed: number;
	connections_analyzed: number;
	unencrypted_ot_percent: number;
}

// ─── Phase 13A Quick-Win Features ───────────────────────

/** A known default credential entry for an ICS/SCADA product */
export interface DefaultCredential {
	vendor: string;
	product_pattern: string;
	protocol: string;
	username: string;
	password: string;
	source: string;
	severity: string;
}

/** Criticality level for an asset */
export type CriticalityLevel = 'critical' | 'high' | 'medium' | 'low' | 'unknown';

/** Criticality assessment for a single device */
export interface CriticalityAssessment {
	ip_address: string;
	level: CriticalityLevel;
	reason: string;
}

/** A naming suggestion for a discovered device */
export interface NamingSuggestion {
	ip_address: string;
	suggested_name: string;
	reason: string;
}

// ─── Baseline Drift (Phase 11) ───────────────────────────

/** Full diff result between current state and a baseline session */
export interface BaselineDiff {
	baseline_session_name: string;
	new_assets: DriftAsset[];
	missing_assets: DriftAsset[];
	changed_assets: ChangedAsset[];
	new_connections: DriftConnection[];
	missing_connections: DriftConnection[];
	summary: DriftSummary;
}

/** A device in the drift report (new or missing) */
export interface DriftAsset {
	ip_address: string;
	mac_address: string | null;
	device_type: string;
	vendor: string | null;
	protocols: string[];
	confidence: number;
}

/** A device with changed properties */
export interface ChangedAsset {
	ip_address: string;
	changes: AssetChange[];
}

/** A single field change between baseline and current */
export interface AssetChange {
	field: string;
	baseline_value: string;
	current_value: string;
}

/** A connection in the drift report */
export interface DriftConnection {
	src_ip: string;
	dst_ip: string;
	src_port: number;
	dst_port: number;
	protocol: string;
}

/** Summary of drift statistics */
export interface DriftSummary {
	total_baseline_assets: number;
	total_current_assets: number;
	new_asset_count: number;
	missing_asset_count: number;
	changed_asset_count: number;
	new_connection_count: number;
	missing_connection_count: number;
	drift_score: number;
}

// ─── Theme (Phase 11) ────────────────────────────────────

/** Theme mode: dark, light, or follow system preference */
export type ThemeMode = 'dark' | 'light' | 'system';

/** Persistent user settings */
export interface UserSettings {
	theme: ThemeMode;
}

// ─── Timeline (Phase 11) ─────────────────────────────────

/** Timeline range for the scrubber */
export interface TimelineRange {
	earliest: string | null;
	latest: string | null;
	connection_count: number;
}

// ─── Plugins (Phase 11) ──────────────────────────────────

/** A plugin manifest */
export interface PluginManifest {
	name: string;
	version: string;
	plugin_type: string;
	description: string;
	author: string | null;
}

// ─── Communication Patterns ──────────────────────────────

/** Per-connection timing and traffic statistics */
export interface ConnectionStats {
	src_ip: string;
	dst_ip: string;
	protocol: string;
	port: number;
	packet_count: number;
	byte_count: number;
	first_seen: number;
	last_seen: number;
	duration_secs: number;
	avg_interval_ms: number;
	std_interval_ms: number;
	min_interval_ms: number;
	max_interval_ms: number;
	is_periodic: boolean;
	packets_per_sec: number;
}

/** A detected communication pattern anomaly */
export interface PatternAnomaly {
	anomaly_type: string;
	src_ip: string;
	dst_ip: string;
	port: number;
	protocol: string;
	description: string;
	severity: string;
}

// ─── Projects ─────────────────────────────────────────────

/** A named engagement project (top-level container for sessions). */
export interface Project {
	id: number;
	name: string;
	client_name: string;
	site_name: string;
	assessor_name: string;
	engagement_start: string;
	engagement_end: string;
	notes: string;
	created_at: string;
	updated_at: string;
}

/** Lightweight project summary for the project list view. */
export interface ProjectSummary {
	id: number;
	name: string;
	client_name: string;
	site_name: string;
	session_count: number;
	created_at: string;
	updated_at: string;
}

// ─── External Alerts (Phase 14D) ─────────────────────────

/** An IDS/SIEM alert imported from Suricata or Wazuh. */
export interface StoredAlert {
	timestamp: string;
	src_ip: string;
	src_port: number;
	dst_ip: string;
	dst_port: number;
	signature_id: number;
	signature: string;
	category: string;
	/** 1 = high, 2 = medium, 3 = low */
	severity: number;
	source: string;
}

/** An IDS/SIEM alert enriched with device inventory data. */
export interface CorrelatedAlert extends StoredAlert {
	src_hostname: string | null;
	src_device_type: string | null;
	src_purdue_level: number | null;
	dst_hostname: string | null;
	dst_device_type: string | null;
	dst_purdue_level: number | null;
}

/** Real-time ATT&CK alert emitted during live capture. */
export interface LiveAttackAlert {
	technique_id: string;
	title: string;
	severity: string;
	description: string;
	affected_assets: string[];
	evidence: string;
	timestamp: string;
}

/** Result of a filtered PCAP export. */
export interface FilteredPcapResult {
	output_path: string;
	packets_written: number;
	source_files: number;
}

// ─── Phase 14E: ICS Malware, Allowlist, Compliance ───────────

/** A detected ICS malware behavioral signature match. */
export interface MalwareFinding {
	malware_name: string;
	confidence: string;
	severity: string;
	source_ip: string;
	target_ips: string[];
	evidence: string;
	attack_techniques: string[];
	pattern_description: string;
}

/** An entry in the communication allowlist. */
export interface AllowlistEntry {
	src_ip: string;
	dst_ip: string;
	protocol: string;
	dst_port: number;
	direction: string;
	frequency: string;
	avg_interval_ms: number | null;
	classification: string;
	justification: string;
}

// ─── Phase 14F: CVE Matching, SINEMA, Zeek Drill-Down ─────────

/** A CVE match result for a discovered OT infrastructure device. */
export interface CveMatch {
	cve_id: string;
	cvss: number;
	description: string;
	advisory: string;
	remediation: string;
	matched_product: string;
	matched_firmware: string | null;
	/** "high" = product + firmware known, "medium" = product match, "low" = vendor only */
	confidence: string;
	/** CRITICAL / HIGH / MEDIUM / LOW */
	severity_label: string;
}

/** A single Zeek-observed event summarised for a device. */
export interface ZeekEventSummary {
	timestamp: string;
	log_type: string;
	peer_ip: string;
	summary: string;
}

/** Per-device aggregate of Zeek-observed events. */
export interface DeviceZeekEvents {
	device_ip: string;
	conn_log_entries: number;
	modbus_events: number;
	dnp3_events: number;
	dns_queries: number;
	http_requests: number;
	unique_peers: number;
	alert_count: number;
	sample_events: ZeekEventSummary[];
}

/** Compliance status for a single framework requirement. */
export type ComplianceStatus = 'gap' | 'partial' | 'met' | 'not_assessed';

/** A compliance mapping for a single framework requirement. */
export interface ComplianceMapping {
	framework: string;
	requirement_id: string;
	requirement_name: string;
	status: ComplianceStatus;
	evidence: string;
	description: string;
}

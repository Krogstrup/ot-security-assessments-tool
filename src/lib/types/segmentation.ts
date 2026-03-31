// ─── Phase 15: Microsegmentation ─────────────────────────

/** IEC 62443 Security Level for a zone. */
export type SecurityLevel = 'sl1' | 'sl2' | 'sl3' | 'sl4';

/** Risk level for a policy rule. */
export type RuleRisk = 'low' | 'medium' | 'high';

/** Output format for enforcement config export. */
export type EnforcementFormat =
	| 'cisco_ios_acl'
	| 'cisco_asa_acl'
	| 'generic_firewall_table'
	| 'suricata_rules'
	| 'json_policy';

/** Identity-based cluster of assets (Phase 15A). */
export interface PolicyGroup {
	id: string;
	name: string;
	member_ips: string[];
	purdue_level: number | null;
	device_category: string;
	security_level: SecurityLevel;
	criticality: string;
}

/** A single allow rule within a zone policy. */
export interface PolicyRule {
	protocol: string;
	dst_port: number | null;
	risk: RuleRisk;
	justification: string;
	packet_count: number;
}

/** Allowed communication rules between a zone pair. */
export interface ZonePairPolicy {
	src_zone_id: string;
	dst_zone_id: string;
	rules: PolicyRule[];
}

/** Per-zone-pair least-privilege communication matrix (Phase 15C). */
export interface CommunicationMatrix {
	zone_pairs: ZonePairPolicy[];
	default_action: string;
	coverage_percent: number;
}

/** IEC 62443 security zone (Phase 15B). */
export interface Zone {
	id: string;
	name: string;
	purdue_levels: number[];
	policy_group_ids: string[];
	security_level: SecurityLevel;
	asset_count: number;
}

/** Direction of conduit traffic flow. */
export type ConduitDirection = 'unidirectional' | 'bidirectional';

/** A single allow rule within a conduit definition. */
export interface ConduitRule {
	protocol: string;
	dst_port: number | null;
	has_write_ops: boolean;
	has_config_ops: boolean;
	attack_techniques: string[];
	risk_note: string | null;
}

/** IEC 62443 conduit — a controlled channel between two zones. */
export interface Conduit {
	id: string;
	src_zone_id: string;
	dst_zone_id: string;
	direction: ConduitDirection;
	rules: ConduitRule[];
	cross_purdue_risk: boolean;
}

/** IEC 62443 zone model (Phase 15B). */
export interface ZoneModel {
	zones: Zone[];
	conduits: Conduit[];
	zone_score: number;
	recommendations: string[];
}

/** Enforcement configuration in a specific output format (Phase 15D). */
export interface EnforcementConfig {
	format: EnforcementFormat;
	content: string;
	generated_at: string;
	rule_count: number;
}

/** A connection that would be blocked by the proposed policy. */
export interface BlockedConnection {
	src_ip: string;
	dst_ip: string;
	protocol: string;
	dst_port: number;
	is_false_positive_candidate: boolean;
	reason: string;
}

/** Per-zone-pair summary of blocked connections. */
export interface ZoneBlockSummary {
	src_zone_id: string;
	dst_zone_id: string;
	blocked_count: number;
}

/** Policy simulation result (Phase 15E). */
export interface SimulationResult {
	allowed: number;
	blocked: number;
	blocked_percent: number;
	risk_reduction_score: number;
	deployment_score: number;
	critical_blocks: BlockedConnection[];
	false_positive_candidates: BlockedConnection[];
	zone_block_summaries: ZoneBlockSummary[];
}

/** Complete segmentation analysis output (Phases 15A–15E). */
export interface SegmentationReport {
	policy_groups: PolicyGroup[];
	zone_model: ZoneModel;
	communication_matrix: CommunicationMatrix;
	enforcement_configs: EnforcementConfig[];
	simulation: SimulationResult;
	generated_at: string;
}

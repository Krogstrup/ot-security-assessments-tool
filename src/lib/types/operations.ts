// ─── Sessions (Phase 6) ─────────────────────────────────

/** Session info returned from the backend */
export interface SessionInfo {
	id: string;
	name: string;
	description: string;
	created_at: string;
	updated_at: string;
	asset_count: number;
	connection_count: number;
}

/** Partial updates for an asset's editable fields */
export interface AssetUpdate {
	device_type?: string;
	hostname?: string;
	notes?: string;
	purdue_level?: number;
	tags?: string[];
}

// ─── Physical Topology (Phase 7) ─────────────────────────

/** A physical network switch with ports and metadata */
export interface PhysicalSwitch {
	hostname: string;
	management_ip: string | null;
	model: string | null;
	ios_version: string | null;
	ports: PhysicalPort[];
	vlans: Record<number, string>;
}

/** A physical switch port with devices and config */
export interface PhysicalPort {
	name: string;
	short_name: string;
	description: string | null;
	vlans: number[];
	mode: string;
	shutdown: boolean;
	ip_address: string | null;
	subnet_mask: string | null;
	mac_addresses: string[];
	ip_addresses: string[];
	cdp_neighbor: CdpNeighbor | null;
	speed: string | null;
	duplex: string | null;
}

/** CDP/LLDP neighbor on a port */
export interface CdpNeighbor {
	device_id: string;
	remote_port: string;
	platform: string | null;
	ip_address: string | null;
	capabilities: string[];
}

/** Where a device is physically located */
export interface DeviceLocation {
	ip_address: string;
	mac_address: string | null;
	switch_hostname: string;
	port_name: string;
	vlan: number | null;
}

/** A link between two physical switches */
export interface PhysicalLink {
	src_switch: string;
	src_port: string;
	dst_switch: string;
	dst_port: string;
}

/** Full physical topology */
export interface PhysicalTopology {
	switches: PhysicalSwitch[];
	links: PhysicalLink[];
	device_locations: Record<string, DeviceLocation>;
}

// ─── Traffic-Inferred Topology ────────────────────────────

/** An IPv4 /24 subnet inferred from observed traffic */
export interface InferredSubnet {
	network: string;
	member_ips: string[];
	gateway_ip: string | null;
}

/** A device inferred to be a gateway/router */
export interface InferredGateway {
	ip_address: string;
	mac_address: string | null;
	connected_subnets: string[];
	confidence: number;
}

/** A device that may be a switch/hub based on high fan-out */
export interface SwitchCandidate {
	ip_address: string | null;
	mac_address: string | null;
	connected_ips: string[];
	confidence: number;
}

/** A broadcast domain inferred from subnet structure */
export interface BroadcastDomain {
	id: string;
	network: string;
	member_ips: string[];
	gateway_ip: string | null;
	inferred_from: string;
}

/** Full traffic-inferred topology */
export interface InferredTopology {
	subnets: InferredSubnet[];
	gateways: InferredGateway[];
	switch_candidates: SwitchCandidate[];
	broadcast_domains: BroadcastDomain[];
}

// ─── External Tool Ingest (Phase 8) ─────────────────────

/** Data source for ingested results */
export type IngestSource = 'zeek' | 'suricata' | 'nmap' | 'masscan';

/** Result of importing external tool data */
export interface IngestImportResult {
	source: string;
	files_processed: number;
	asset_count: number;
	connection_count: number;
	alert_count: number;
	new_assets: number;
	updated_assets: number;
	duration_ms: number;
	errors: string[];
}

// ─── Wireshark Integration (Phase 8) ─────────────────────

/** Wireshark installation info */
export interface WiresharkInfo {
	found: boolean;
	path: string | null;
	version: string | null;
}

/** A packet frame row for the View Frames dialog */
export interface FrameRow {
	number: number;
	timestamp: string;
	src_ip: string;
	dst_ip: string;
	src_port: number;
	dst_port: number;
	protocol: string;
	length: number;
	origin_file: string;
}

// ─── Export & Reporting (Phase 9) ─────────────────────────

/** Configuration for PDF report generation */
export interface ReportConfig {
	assessor_name: string;
	client_name: string;
	assessment_date?: string;
	title?: string;
	include_executive_summary: boolean;
	include_asset_inventory: boolean;
	include_protocol_analysis: boolean;
	include_findings: boolean;
	include_recommendations: boolean;
}

/** SBOM entry (CISA BOD 23-01 format) */
export interface SbomEntry {
	ip_address: string;
	mac_address: string;
	hostname: string;
	vendor: string;
	product: string;
	firmware_version: string;
	protocols: string;
	purdue_zone: string;
	device_type: string;
	confidence: number;
	first_seen: string;
	last_seen: string;
	country: string;
	tags: string;
}

/** Export format options */
export type ExportFormat = 'csv' | 'json' | 'pdf' | 'sbom_csv' | 'sbom_json' | 'stix';

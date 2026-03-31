// ─── Deep Parse (Phase 4) ────────────────────────────────────

/** Aggregated deep parse info for a single device */
export interface DeepParseInfo {
	modbus: ModbusDetail | null;
	dnp3: Dnp3Detail | null;
	enip: EnipDetail | null;
	s7: S7Detail | null;
	bacnet: BacnetDetail | null;
	iec104: Iec104Detail | null;
	profinet_dcp: ProfinetDcpDetail | null;
	lldp: LldpDetail | null;
	snmp: SnmpDetail | null;
}

/** EtherNet/IP aggregated details for a device */
export interface EnipDetail {
	/** "scanner" (client) or "adapter" (server) */
	role: string;
	/** IP sent CIP Write or ReadModifyWrite to an Assembly object (T0855) */
	cip_writes_to_assembly: boolean;
	/** IP accessed CIP File class — firmware/program operations (T0836) */
	cip_file_access: boolean;
	/** IP sent ListIdentity requests — network discovery */
	list_identity_requests: boolean;
}

/** S7comm aggregated details for a device */
export interface S7Detail {
	/** "client" or "server" */
	role: string;
	/** S7 PDU functions observed from this device (snake_case names) */
	functions_seen: string[];
}

/** BACnet aggregated details for a device */
export interface BacnetDetail {
	/** "client" or "server" */
	role: string;
	/** WriteProperty to AnalogOutput or BinaryOutput was seen (T0855) */
	write_to_output: boolean;
	/** WriteProperty to NotificationClass was seen — alarm suppression (T0856) */
	write_to_notification_class: boolean;
	/** ReinitializeDevice service was seen (T0816) */
	reinitialize_device: boolean;
	/** DeviceCommunicationControl service was seen (T0811) */
	device_communication_control: boolean;
}

/** IEC 60870-5-104 aggregated details for a device */
export interface Iec104Detail {
	/** "master" or "outstation" */
	role: string;
	/** Device sent control command ASDUs (type IDs 45–69) (T0855) */
	has_control_commands: boolean;
	/** Device sent Reset Process command (type ID 105) (T0816) */
	has_reset_process: boolean;
	/** Device sent General Interrogation (type ID 100) */
	has_interrogation: boolean;
}

/** PROFINET DCP aggregated details for a device */
export interface ProfinetDcpDetail {
	/** "io_device", "io_controller", "io_supervisor", or "unknown" */
	role: string;
	/** Station name from DCP Name-of-Station block */
	device_name: string | null;
}

/** LLDP (Link Layer Discovery Protocol) aggregated details for a device */
export interface LldpDetail {
	/** System name (hostname) from LLDP Type 5 */
	system_name: string | null;
	/** System description (model + OS + firmware) from LLDP Type 6 */
	system_description: string | null;
	/** Chassis identifier (MAC address or local string) */
	chassis_id: string | null;
	/** Port identifier */
	port_id: string | null;
	/** Human-readable capability summary, e.g. "Bridge, Router" */
	capability_summary: string | null;
	/** Management addresses advertised by the device */
	management_addresses: string[];
	/** VLAN IDs from IEEE 802.1 org-specific TLVs */
	vlan_ids: number[];
	/** Vendor name inferred from description or OUI */
	vendor: string | null;
	/** Model name inferred from description */
	model: string | null;
	/** Firmware/software version inferred from description */
	firmware: string | null;
}

/** SNMP device identity extracted from GET-Response packets */
export interface SnmpDetail {
	/** sysDescr — free-text description of the device */
	sys_descr: string | null;
	/** sysName — administratively assigned hostname */
	sys_name: string | null;
	/** sysLocation — physical location string */
	sys_location: string | null;
	/** sysObjectID — vendor's authoritative OID for this device type */
	sys_object_id: string | null;
	/** sysUpTime in centiseconds */
	sys_uptime_cs: number | null;
	/** sysContact — contact person / email */
	sys_contact: string | null;
	/** Vendor name inferred from enterprise OID */
	vendor: string | null;
}

/** Redundancy protocol frame observed in the capture */
export interface RedundancyInfo {
	/** Protocol family: "mrp" | "rstp" | "hsr" | "prp" | "dlr" */
	protocol: 'mrp' | 'rstp' | 'hsr' | 'prp' | 'dlr';
	/** Device role in the ring (e.g. "ring-manager", "root-bridge") */
	role: string | null;
	/** Ring / domain identifier */
	ring_id: number | null;
	/** Priority for manager election */
	priority: number | null;
	/** Source MAC address of the sending device */
	source_mac: string;
	/** Human-readable frame summary */
	details: string;
	/** True if this device is the ring manager / root bridge */
	is_manager: boolean;
	/** True if a topology change event was observed */
	topology_change: boolean;
}

/** Switch port security finding */
export interface SwitchSecurityFinding {
	/** Finding category */
	finding_type: string;
	/** Human-readable title */
	title: string;
	/** Severity level */
	severity: 'info' | 'low' | 'medium' | 'high' | 'critical';
	/** Description explaining the risk */
	description: string;
	/** IPs of affected switches / assets */
	affected_assets: string[];
	/** Evidence collected */
	evidence: string;
	/** Recommended remediation step */
	remediation: string;
}

/** Modbus protocol details for a device */
export interface ModbusDetail {
	role: string;
	unit_ids: number[];
	function_codes: FunctionCodeStat[];
	register_ranges: RegisterRangeInfo[];
	device_id: ModbusDeviceIdInfo | null;
	relationships: ModbusRelationship[];
	polling_intervals: PollingInterval[];
}

/** DNP3 protocol details for a device */
export interface Dnp3Detail {
	role: string;
	addresses: number[];
	function_codes: FunctionCodeStat[];
	has_unsolicited: boolean;
	relationships: Dnp3Relationship[];
}

/** Function code usage statistics */
export interface FunctionCodeStat {
	code: number;
	name: string;
	count: number;
	is_write: boolean;
}

/** Modbus register range */
export interface RegisterRangeInfo {
	start: number;
	count: number;
	register_type: string;
	access_count: number;
}

/** Modbus device identification from FC 43/14 */
export interface ModbusDeviceIdInfo {
	vendor_name: string | null;
	product_code: string | null;
	revision: string | null;
	vendor_url: string | null;
	product_name: string | null;
	model_name: string | null;
}

/** Modbus master/slave relationship */
export interface ModbusRelationship {
	remote_ip: string;
	remote_role: string;
	unit_ids: number[];
	packet_count: number;
}

/** DNP3 master/outstation relationship */
export interface Dnp3Relationship {
	remote_ip: string;
	remote_role: string;
	packet_count: number;
}

/** Detected polling interval */
export interface PollingInterval {
	remote_ip: string;
	unit_id: number | null;
	function_code: number;
	avg_interval_ms: number;
	min_interval_ms: number;
	max_interval_ms: number;
	sample_count: number;
}

export type IcsProtocol =
	| 'modbus'
	| 'dnp3'
	| 'ethernet_ip'
	| 'bacnet'
	| 's7comm'
	| 'opc_ua'
	| 'profinet'
	| 'iec104'
	| 'mqtt'
	| 'hart_ip'
	| 'foundation_fieldbus'
	| 'ge_srtp'
	| 'wonderware_suitelink'
	| 'http'
	| 'https'
	| 'dns'
	| 'ssh'
	| 'rdp'
	| 'snmp'
	| 'unknown';

export interface ProtocolStats {
	protocol: IcsProtocol;
	packet_count: number;
	byte_count: number;
	connection_count: number;
	unique_devices: number;
}

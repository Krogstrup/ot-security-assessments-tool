// ─── Protocol detail types (Rust-first, auto-generated) ───────
// These types cross the IPC boundary. Their source of truth is the
// Rust struct with #[derive(TS)]. Do not redefine them here.
// To regenerate: cargo test --manifest-path src-tauri/Cargo.toml
export type { DeepParseInfo } from '../../../src-tauri/bindings/gen/types/DeepParseInfo';
export type { EnipDetail } from '../../../src-tauri/bindings/gen/types/EnipDetail';
export type { S7Detail } from '../../../src-tauri/bindings/gen/types/S7Detail';
export type { BacnetDetail } from '../../../src-tauri/bindings/gen/types/BacnetDetail';
export type { Iec104Detail } from '../../../src-tauri/bindings/gen/types/Iec104Detail';
export type { ProfinetDcpDetail } from '../../../src-tauri/bindings/gen/types/ProfinetDcpDetail';
export type { LldpDetail } from '../../../src-tauri/bindings/gen/types/LldpDetail';
export type { SnmpDetail } from '../../../src-tauri/bindings/gen/types/SnmpDetail';
export type { ModbusDetail } from '../../../src-tauri/bindings/gen/types/ModbusDetail';
export type { Dnp3Detail } from '../../../src-tauri/bindings/gen/types/Dnp3Detail';
export type { FunctionCodeStat } from '../../../src-tauri/bindings/gen/types/FunctionCodeStat';
export type { RegisterRangeInfo } from '../../../src-tauri/bindings/gen/types/RegisterRangeInfo';
export type { ModbusDeviceIdInfo } from '../../../src-tauri/bindings/gen/types/ModbusDeviceIdInfo';
export type { ModbusRelationship } from '../../../src-tauri/bindings/gen/types/ModbusRelationship';
export type { Dnp3Relationship } from '../../../src-tauri/bindings/gen/types/Dnp3Relationship';
export type { PollingInterval } from '../../../src-tauri/bindings/gen/types/PollingInterval';

// ─── Manual-only types (pure UI / no Rust counterpart) ────────

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

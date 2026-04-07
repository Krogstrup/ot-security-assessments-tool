//! Aggregated per-device parse results for each ICS protocol.
//!
//! These types are the output of the deep-parse stage: after the packet
//! processor accumulates raw parser results, it collapses them into these
//! view models for the command layer and the UI.

#[cfg(test)]
use ts_rs::TS;

use serde::{Deserialize, Serialize};

/// Aggregated deep parse information for a single device (IP address).
///
/// Collects all Modbus/DNP3/EtherNet-IP/S7comm/BACnet details observed across
/// every packet for a given IP, including function codes, roles, and
/// security-relevant flags for ATT&CK detection.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export, export_to = "../../bindings/gen/types/"))]
pub struct DeepParseInfo {
    /// Modbus-specific details (present if device speaks Modbus)
    pub modbus: Option<ModbusDetail>,
    /// DNP3-specific details (present if device speaks DNP3)
    pub dnp3: Option<Dnp3Detail>,
    /// EtherNet/IP details (present if device speaks EtherNet/IP)
    pub enip: Option<EnipDetail>,
    /// S7comm details (present if device speaks S7comm)
    pub s7: Option<S7Detail>,
    /// BACnet details (present if device speaks BACnet)
    pub bacnet: Option<BacnetDetail>,
    /// IEC 60870-5-104 details (present if device speaks IEC 104)
    pub iec104: Option<Iec104Detail>,
    /// PROFINET DCP details (present if device speaks PROFINET DCP)
    pub profinet_dcp: Option<ProfinetDcpDetail>,
    /// LLDP details (present if device advertised itself via LLDP)
    pub lldp: Option<LldpDetail>,
    /// SNMP device identity (present if device responded to SNMP GET)
    pub snmp: Option<SnmpDetail>,
}

/// EtherNet/IP aggregated details for a device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export, export_to = "../../bindings/gen/types/"))]
pub struct EnipDetail {
    /// Detected role: "scanner" (client) or "adapter" (server)
    pub role: String,
    /// IP sent CIP Write or ReadModifyWrite to an Assembly object
    pub cip_writes_to_assembly: bool,
    /// IP accessed CIP File class (firmware/program operations)
    pub cip_file_access: bool,
    /// IP sent ListIdentity requests (network discovery)
    pub list_identity_requests: bool,
}

/// S7comm aggregated details for a device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export, export_to = "../../bindings/gen/types/"))]
pub struct S7Detail {
    /// Detected role: "client" or "server"
    pub role: String,
    /// S7 functions observed from this device (snake_case names, sorted)
    pub functions_seen: Vec<String>,
}

/// BACnet aggregated details for a device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export, export_to = "../../bindings/gen/types/"))]
pub struct BacnetDetail {
    /// Detected role: "client" or "server"
    pub role: String,
    /// WriteProperty to AnalogOutput or BinaryOutput was seen
    pub write_to_output: bool,
    /// WriteProperty to NotificationClass was seen (alarm suppression)
    pub write_to_notification_class: bool,
    /// ReinitializeDevice service was seen
    pub reinitialize_device: bool,
    /// DeviceCommunicationControl service was seen
    pub device_communication_control: bool,
}

/// PROFINET DCP aggregated details for a device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export, export_to = "../../bindings/gen/types/"))]
pub struct ProfinetDcpDetail {
    /// Detected role: "io_device", "io_controller", "io_supervisor", or "unknown"
    pub role: String,
    /// Station name from DCP Name-of-Station block
    pub device_name: Option<String>,
}

/// LLDP (Link Layer Discovery Protocol) details for a device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export, export_to = "../../bindings/gen/types/"))]
pub struct LldpDetail {
    /// System name (hostname) from LLDP Type 5
    pub system_name: Option<String>,
    /// System description from LLDP Type 6
    pub system_description: Option<String>,
    /// Chassis identifier (MAC or string)
    pub chassis_id: Option<String>,
    /// Port identifier
    pub port_id: Option<String>,
    /// Capability summary string, e.g. "Bridge, Router"
    pub capability_summary: Option<String>,
    /// Management addresses advertised
    pub management_addresses: Vec<String>,
    /// VLAN IDs from 802.1 org-specific TLVs
    pub vlan_ids: Vec<u16>,
    /// Vendor inferred from description
    pub vendor: Option<String>,
    /// Model inferred from description
    pub model: Option<String>,
    /// Firmware version inferred from description
    pub firmware: Option<String>,
}

/// SNMP device identity extracted from GET-Response packets.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export, export_to = "../../bindings/gen/types/"))]
pub struct SnmpDetail {
    /// sysDescr — free-text description of the device
    pub sys_descr: Option<String>,
    /// sysName — administratively assigned hostname
    pub sys_name: Option<String>,
    /// sysLocation — physical location string
    pub sys_location: Option<String>,
    /// sysObjectID — vendor's authoritative OID for this device type
    pub sys_object_id: Option<String>,
    /// sysUpTime in centiseconds
    pub sys_uptime_cs: Option<u32>,
    /// sysContact — contact person / email
    pub sys_contact: Option<String>,
    /// Vendor name inferred from enterprise OID
    pub vendor: Option<String>,
}

/// IEC 60870-5-104 aggregated details for a device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export, export_to = "../../bindings/gen/types/"))]
pub struct Iec104Detail {
    /// Detected role: "master" or "outstation"
    pub role: String,
    /// Device sent control command ASDUs (type IDs 45–69)
    pub has_control_commands: bool,
    /// Device sent Reset Process command (type ID 105)
    pub has_reset_process: bool,
    /// Device sent General Interrogation (type ID 100)
    pub has_interrogation: bool,
}

/// Aggregated Modbus details for a device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export, export_to = "../../bindings/gen/types/"))]
pub struct ModbusDetail {
    /// Detected role: "master", "slave", or "both"
    pub role: String,
    /// Unit IDs seen on this device (slave → responds as; master → polls)
    pub unit_ids: Vec<u8>,
    /// Function codes observed (code → count)
    pub function_codes: Vec<FunctionCodeStat>,
    /// Register ranges accessed
    pub register_ranges: Vec<RegisterRangeInfo>,
    /// Device identification from FC 43/14 (if extracted)
    pub device_id: Option<ModbusDeviceIdInfo>,
    /// IPs this device communicates with, with roles
    pub relationships: Vec<ModbusRelationship>,
    /// Polling intervals detected (in milliseconds)
    pub polling_intervals: Vec<PollingInterval>,
}

/// DNP3 aggregated details for a device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export, export_to = "../../bindings/gen/types/"))]
pub struct Dnp3Detail {
    /// Detected role: "master", "outstation", or "both"
    pub role: String,
    /// DNP3 addresses used by this device
    pub addresses: Vec<u16>,
    /// Function codes observed (code → count)
    pub function_codes: Vec<FunctionCodeStat>,
    /// Whether unsolicited responses were detected from this device
    pub has_unsolicited: bool,
    /// IPs this device communicates with
    pub relationships: Vec<Dnp3Relationship>,
}

/// Function code usage statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export, export_to = "../../bindings/gen/types/"))]
pub struct FunctionCodeStat {
    pub code: u8,
    pub name: String,
    pub count: u64,
    /// Whether this is a write/control operation (security-relevant)
    pub is_write: bool,
}

/// Register range accessed by a Modbus device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export, export_to = "../../bindings/gen/types/"))]
pub struct RegisterRangeInfo {
    pub start: u16,
    pub count: u16,
    pub register_type: String,
    /// How many times this range was accessed
    pub access_count: u64,
}

/// Modbus device identification from FC 43/14.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export, export_to = "../../bindings/gen/types/"))]
pub struct ModbusDeviceIdInfo {
    pub vendor_name: Option<String>,
    pub product_code: Option<String>,
    pub revision: Option<String>,
    pub vendor_url: Option<String>,
    pub product_name: Option<String>,
    pub model_name: Option<String>,
}

/// A relationship between Modbus master/slave.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export, export_to = "../../bindings/gen/types/"))]
pub struct ModbusRelationship {
    pub remote_ip: String,
    /// "master" or "slave" — what the REMOTE device is
    pub remote_role: String,
    pub unit_ids: Vec<u8>,
    pub packet_count: u64,
}

/// A relationship between DNP3 master/outstation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export, export_to = "../../bindings/gen/types/"))]
pub struct Dnp3Relationship {
    pub remote_ip: String,
    /// "master" or "outstation"
    pub remote_role: String,
    pub packet_count: u64,
}

/// Detected polling interval for a master→slave relationship.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export, export_to = "../../bindings/gen/types/"))]
pub struct PollingInterval {
    pub remote_ip: String,
    pub unit_id: Option<u8>,
    pub function_code: u8,
    /// Average interval in milliseconds
    pub avg_interval_ms: f64,
    /// Minimum interval observed
    pub min_interval_ms: f64,
    /// Maximum interval observed
    pub max_interval_ms: f64,
    /// Number of samples used to compute the interval
    pub sample_count: u64,
}

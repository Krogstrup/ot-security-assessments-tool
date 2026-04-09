//! Shared OT classification helpers used across analysis modules.
//!
//! Device type strings use the constants from `gm_constants::DEVICE_TYPE_*`.
//!
//! Centralises protocol and device-type lists so that every module draws from
//! a single source of truth. Previously each module kept its own copy of these
//! `matches!` arms, which led to lists drifting out of sync.
//!
//! ## Naming convention
//!
//! Protocol strings here are **PascalCase** (e.g. `"Modbus"`, `"EthernetIp"`),
//! matching the naming used in `ConnectionSnapshot::protocol` and
//! `AssetSnapshot::protocols` when populated from the connection layer.
//!
//! `anomaly.rs` uses a separate local helper for lowercase asset-protocol
//! strings that originate from a different ingestion path.

/// Returns `true` if the device type string represents an OT field device.
///
/// Covers PLCs, RTUs, HMIs, historians, SCADA servers, engineering workstations,
/// I/O servers, generic field devices, and controllers.
pub(crate) fn is_ot_device_type(device_type: &str) -> bool {
    use gm_constants::{
        DEVICE_TYPE_CONTROLLER, DEVICE_TYPE_EWS, DEVICE_TYPE_FIELD_DEVICE, DEVICE_TYPE_HISTORIAN,
        DEVICE_TYPE_HMI, DEVICE_TYPE_IO_SERVER, DEVICE_TYPE_PLC, DEVICE_TYPE_RTU,
        DEVICE_TYPE_SCADA_SERVER,
    };
    matches!(
        device_type,
        _ if device_type == DEVICE_TYPE_PLC
            || device_type == DEVICE_TYPE_RTU
            || device_type == DEVICE_TYPE_HMI
            || device_type == DEVICE_TYPE_HISTORIAN
            || device_type == DEVICE_TYPE_EWS
            || device_type == DEVICE_TYPE_SCADA_SERVER
            || device_type == DEVICE_TYPE_IO_SERVER
            || device_type == DEVICE_TYPE_FIELD_DEVICE
            || device_type == DEVICE_TYPE_CONTROLLER
    )
}

/// Returns `true` for controller-class field devices used in command/reporting checks.
///
/// This intentionally stays narrower than [`is_ot_device_type`] to avoid
/// broadening detections that are designed for PLC/RTU-style endpoints.
pub(crate) fn is_plc_or_rtu_family_device_type(device_type: &str) -> bool {
    use gm_constants::{DEVICE_TYPE_FIELD_DEVICE, DEVICE_TYPE_PLC, DEVICE_TYPE_RTU};

    device_type == DEVICE_TYPE_PLC
        || device_type == DEVICE_TYPE_RTU
        || device_type == DEVICE_TYPE_FIELD_DEVICE
}

/// Returns `true` if the PascalCase protocol string is a known OT/ICS protocol.
///
/// This is the canonical list used by connection-layer checks.
/// Includes both `"Profinet"` and `"ProfinetDcp"` since parsers may emit either,
/// and both `"FoundationFieldbus"` and `"FfHse"` for the same reason.
pub(crate) fn is_ot_protocol_name(protocol: &str) -> bool {
    matches!(
        protocol,
        "Modbus"
            | "Dnp3"
            | "EthernetIp"
            | "S7comm"
            | "Bacnet"
            | "OpcUa"
            | "Iec104"
            | "Profinet"
            | "ProfinetDcp"
            | "HartIp"
            | "GeSrtp"
            | "WonderwareSuitelink"
            | "FoundationFieldbus"
            | "FfHse"
            | "Mqtt"
            | "Snmp"
    )
}

/// Returns the canonical port(s) for a known ICS protocol name.
///
/// Returns an empty slice for unknown protocols. Used by T0885 (Commonly Used
/// Port) to decide whether a connection is using the expected port for its
/// stated protocol.
pub(crate) fn canonical_ot_ports(protocol: &str) -> &'static [u16] {
    match protocol {
        "Modbus" => &[502],
        "Dnp3" => &[20000],
        "EthernetIp" => &[44818, 2222],
        "S7comm" => &[102],
        "Bacnet" => &[47808],
        "OpcUa" => &[4840],
        "Iec104" => &[2404],
        "HartIp" => &[5094],
        "GeSrtp" => &[18245, 18246],
        "WonderwareSuitelink" => &[5007],
        "FfHse" | "FoundationFieldbus" => &[1089, 1090, 1091],
        "Profinet" | "ProfinetDcp" => &[34962, 34963, 34964],
        "Mqtt" => &[1883, 8883],
        _ => &[],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ot_device_type() {
        assert!(is_ot_device_type("plc"));
        assert!(is_ot_device_type("rtu"));
        assert!(is_ot_device_type("hmi"));
        assert!(is_ot_device_type("historian"));
        assert!(is_ot_device_type("scada_server"));
        assert!(!is_ot_device_type("it_device"));
        assert!(!is_ot_device_type("unknown"));
        assert!(!is_ot_device_type(""));
    }

    #[test]
    fn test_is_ot_protocol_name() {
        assert!(is_ot_protocol_name("Modbus"));
        assert!(is_ot_protocol_name("Dnp3"));
        assert!(is_ot_protocol_name("S7comm"));
        assert!(is_ot_protocol_name("Profinet"));
        assert!(is_ot_protocol_name("ProfinetDcp"));
        assert!(is_ot_protocol_name("FoundationFieldbus"));
        assert!(is_ot_protocol_name("FfHse"));
        assert!(!is_ot_protocol_name("Http"));
        assert!(!is_ot_protocol_name("modbus")); // lowercase is not PascalCase
        assert!(!is_ot_protocol_name("Unknown"));
    }

    #[test]
    fn test_is_plc_or_rtu_family_device_type() {
        assert!(is_plc_or_rtu_family_device_type("plc"));
        assert!(is_plc_or_rtu_family_device_type("rtu"));
        assert!(is_plc_or_rtu_family_device_type("field_device"));
        assert!(!is_plc_or_rtu_family_device_type("hmi"));
        assert!(!is_plc_or_rtu_family_device_type("controller"));
    }

    #[test]
    fn test_canonical_ot_ports() {
        assert_eq!(canonical_ot_ports("Modbus"), &[502]);
        assert_eq!(canonical_ot_ports("Dnp3"), &[20000]);
        assert_eq!(canonical_ot_ports("EthernetIp"), &[44818, 2222]);
        assert_eq!(canonical_ot_ports("S7comm"), &[102]);
        assert_eq!(canonical_ot_ports("ProfinetDcp"), &[34962, 34963, 34964]);
        assert_eq!(canonical_ot_ports("Profinet"), &[34962, 34963, 34964]);
        assert_eq!(canonical_ot_ports("Unknown"), &[] as &[u16]);
    }
}

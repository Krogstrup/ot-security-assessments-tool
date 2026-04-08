//! Step 2: Role-based partitioning within each Purdue level.

use std::collections::{HashMap, HashSet};

use crate::{AssetProfile, DeviceCategory, SegmentationInput};

/// Dispatch to level-specific role-split logic.
pub(super) fn split_by_role(
    level: u8,
    assets: &[&AssetProfile],
    input: &SegmentationInput,
) -> Vec<(String, Vec<String>, DeviceCategory)> {
    match level {
        0 | 1 => split_l1_by_protocol_role(level, assets),
        2 => split_l2_hmi_engineering(assets, input),
        3 => split_l3_by_server_type(assets),
        _ => split_l4_by_it_protocol(assets),
    }
}

/// L0/L1: group by primary OT protocol server role.
///
/// Uses `protocol_roles` from deep parse (preferred) then falls back to
/// checking the `protocols` list.
fn split_l1_by_protocol_role(
    level: u8,
    assets: &[&AssetProfile],
) -> Vec<(String, Vec<String>, DeviceCategory)> {
    let category = if level == 0 {
        DeviceCategory::Sensor
    } else {
        DeviceCategory::Plc
    };

    let mut role_buckets: HashMap<String, Vec<String>> = HashMap::new();
    for asset in assets {
        let role = detect_l1_role(asset);
        role_buckets.entry(role).or_default().push(asset.ip.clone());
    }

    role_buckets
        .into_iter()
        .map(|(role, ips)| (role, ips, category))
        .collect()
}

/// Identify the primary OT server role for an L0/L1 asset.
fn detect_l1_role(asset: &AssetProfile) -> String {
    // Prefer explicit protocol_roles (from deep parse).
    for pr in &asset.protocol_roles {
        let role = pr.role.as_str();
        let proto = pr.protocol.as_str();
        let is_server_role = matches!(
            role,
            "slave" | "server" | "outstation" | "adapter" | "io_device"
        );
        if !is_server_role {
            continue;
        }
        let label = match proto {
            "modbus" => "Modbus",
            "dnp3" => "DNP3",
            "s7comm" => "S7",
            "ethernet_ip" | "enip" => "EtherNetIP",
            "iec104" => "IEC104",
            "profinet" | "profinet_dcp" => "PROFINET",
            "bacnet" => "BACnet",
            _ => continue,
        };
        return label.to_string();
    }

    // Fallback: use protocols list.
    for proto in &asset.protocols {
        let label = match proto.as_str() {
            "modbus" => "Modbus",
            "dnp3" => "DNP3",
            "s7comm" => "S7",
            "ethernet_ip" | "enip" => "EtherNetIP",
            "iec104" => "IEC104",
            "profinet" | "profinet_dcp" => "PROFINET",
            "bacnet" => "BACnet",
            _ => continue,
        };
        return label.to_string();
    }

    // Default role label if no specific protocol identified.
    "Control".to_string()
}

/// L2: split HMIs (read clients) from Engineering workstations (config ops).
fn split_l2_hmi_engineering(
    assets: &[&AssetProfile],
    input: &SegmentationInput,
) -> Vec<(String, Vec<String>, DeviceCategory)> {
    let config_ips: HashSet<&str> = input
        .connections
        .iter()
        .filter(|c| c.has_config_operations)
        .flat_map(|c| [c.src_ip.as_str(), c.dst_ip.as_str()])
        .collect();

    let mut hmi_ips: Vec<String> = Vec::new();
    let mut eng_ips: Vec<String> = Vec::new();

    for asset in assets {
        let is_engineering = config_ips.contains(asset.ip.as_str())
            || asset.device_type.contains("engineering")
            || asset.device_type.contains("ews");

        if is_engineering {
            eng_ips.push(asset.ip.clone());
        } else {
            hmi_ips.push(asset.ip.clone());
        }
    }

    let mut result = Vec::new();
    if !hmi_ips.is_empty() {
        result.push(("HMI".to_string(), hmi_ips, DeviceCategory::Hmi));
    }
    if !eng_ips.is_empty() {
        result.push((
            "Engineering".to_string(),
            eng_ips,
            DeviceCategory::EngineeringStation,
        ));
    }
    result
}

/// L3: split Historian, SCADA, and DMZ gateways (dual-homed assets).
fn split_l3_by_server_type(
    assets: &[&AssetProfile],
) -> Vec<(String, Vec<String>, DeviceCategory)> {
    let mut dmz_ips: Vec<String> = Vec::new();
    let mut historian_ips: Vec<String> = Vec::new();
    let mut scada_ips: Vec<String> = Vec::new();
    let mut other_ips: Vec<String> = Vec::new();

    for asset in assets {
        if asset.is_dual_homed {
            dmz_ips.push(asset.ip.clone());
        } else if asset.device_type.contains("historian")
            || asset.protocols.iter().any(|p| p == "opc_ua")
        {
            historian_ips.push(asset.ip.clone());
        } else if asset.device_type.contains("scada") {
            scada_ips.push(asset.ip.clone());
        } else {
            other_ips.push(asset.ip.clone());
        }
    }

    let mut result = Vec::new();
    if !dmz_ips.is_empty() {
        result.push(("DMZ".to_string(), dmz_ips, DeviceCategory::DmzGateway));
    }
    if !historian_ips.is_empty() {
        result.push((
            "Historian".to_string(),
            historian_ips,
            DeviceCategory::Historian,
        ));
    }
    if !scada_ips.is_empty() {
        result.push(("SCADA".to_string(), scada_ips, DeviceCategory::ScadaServer));
    }
    if !other_ips.is_empty() {
        result.push((
            "Operations".to_string(),
            other_ips,
            DeviceCategory::ScadaServer,
        ));
    }
    result
}

/// L4+: split by IT protocol type (web, remote access, general IT).
fn split_l4_by_it_protocol(
    assets: &[&AssetProfile],
) -> Vec<(String, Vec<String>, DeviceCategory)> {
    let mut web_ips: Vec<String> = Vec::new();
    let mut remote_ips: Vec<String> = Vec::new();
    let mut other_ips: Vec<String> = Vec::new();

    for asset in assets {
        if asset
            .protocols
            .iter()
            .any(|p| matches!(p.as_str(), "http" | "https"))
        {
            web_ips.push(asset.ip.clone());
        } else if asset
            .protocols
            .iter()
            .any(|p| matches!(p.as_str(), "rdp" | "ssh" | "vnc" | "telnet"))
        {
            remote_ips.push(asset.ip.clone());
        } else {
            other_ips.push(asset.ip.clone());
        }
    }

    let mut result = Vec::new();
    if !web_ips.is_empty() {
        result.push(("Web".to_string(), web_ips, DeviceCategory::ItEndpoint));
    }
    if !remote_ips.is_empty() {
        result.push((
            "RemoteAccess".to_string(),
            remote_ips,
            DeviceCategory::ItEndpoint,
        ));
    }
    if !other_ips.is_empty() {
        result.push(("IT".to_string(), other_ips, DeviceCategory::ItEndpoint));
    }
    result
}

//! Physical-topology use-cases operating on snapshots/topology state only.

use std::fmt;
use std::path::Path;

use gm_physical::inference::{AssetSnapshot as InfAssetSnapshot, ConnSnapshot, InferenceInput};
use gm_physical::{aruba, cisco, inference, juniper, InferredTopology, PhysicalTopology};
use gm_types::{AssetInfo, ConnectionInfo};

#[derive(Debug)]
pub enum PhysicalUseCaseError {
    ParseFailure(String),
    InvalidInput(String),
    Io(std::io::Error),
}

impl PhysicalUseCaseError {
    pub fn parse_failure(message: impl Into<String>) -> Self {
        PhysicalUseCaseError::ParseFailure(message.into())
    }

    pub fn invalid_input(message: impl Into<String>) -> Self {
        PhysicalUseCaseError::InvalidInput(message.into())
    }
}

impl fmt::Display for PhysicalUseCaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PhysicalUseCaseError::ParseFailure(message) => write!(f, "{message}"),
            PhysicalUseCaseError::InvalidInput(message) => write!(f, "{message}"),
            PhysicalUseCaseError::Io(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for PhysicalUseCaseError {}

impl From<std::io::Error> for PhysicalUseCaseError {
    fn from(value: std::io::Error) -> Self {
        PhysicalUseCaseError::Io(value)
    }
}

pub fn import_cisco_config(
    topology: &mut PhysicalTopology,
    path: &str,
) -> Result<PhysicalTopology, PhysicalUseCaseError> {
    let file_path = Path::new(path);
    let switch = cisco::parse_running_config_file(file_path)
        .map_err(|err| PhysicalUseCaseError::parse_failure(err.to_string()))?;
    let hostname = switch.hostname.clone();
    topology.switches.retain(|s| s.hostname != hostname);
    topology.switches.push(switch);
    topology.build_links();
    topology.correlate_arp_to_ports();
    Ok(topology.clone())
}

pub fn import_mac_table(
    topology: &mut PhysicalTopology,
    path: &str,
    switch_hostname: &str,
) -> Result<PhysicalTopology, PhysicalUseCaseError> {
    let file_path = Path::new(path);
    let entries = cisco::parse_mac_table_file(file_path)
        .map_err(|err| PhysicalUseCaseError::parse_failure(err.to_string()))?;

    if !topology
        .switches
        .iter()
        .any(|s| s.hostname == switch_hostname)
    {
        return Err(PhysicalUseCaseError::invalid_input(format!(
            "Switch '{}' not found. Import its running-config first.",
            switch_hostname
        )));
    }

    topology.apply_mac_table(switch_hostname, &entries);
    topology.correlate_arp_to_ports();
    Ok(topology.clone())
}

pub fn import_cdp_neighbors(
    topology: &mut PhysicalTopology,
    path: &str,
    switch_hostname: &str,
) -> Result<PhysicalTopology, PhysicalUseCaseError> {
    let file_path = Path::new(path);
    let neighbors = cisco::parse_cdp_neighbors_file(file_path)
        .map_err(|err| PhysicalUseCaseError::parse_failure(err.to_string()))?;

    if !topology
        .switches
        .iter()
        .any(|s| s.hostname == switch_hostname)
    {
        return Err(PhysicalUseCaseError::invalid_input(format!(
            "Switch '{}' not found. Import its running-config first.",
            switch_hostname
        )));
    }

    topology.apply_cdp_neighbors(switch_hostname, &neighbors);
    topology.build_links();
    Ok(topology.clone())
}

pub fn import_arp_table(
    topology: &mut PhysicalTopology,
    path: &str,
) -> Result<PhysicalTopology, PhysicalUseCaseError> {
    let file_path = Path::new(path);
    let entries = cisco::parse_arp_table_file(file_path)
        .map_err(|err| PhysicalUseCaseError::parse_failure(err.to_string()))?;
    topology.apply_arp_entries(&entries);
    topology.correlate_arp_to_ports();
    Ok(topology.clone())
}

pub fn import_network_config(
    topology: &mut PhysicalTopology,
    path: &str,
) -> Result<PhysicalTopology, PhysicalUseCaseError> {
    let content = std::fs::read_to_string(path)?;
    let file_path = Path::new(path);

    let switch = if content.contains("set system host-name")
        || content.contains("set interfaces ge-")
        || content.contains("set interfaces xe-")
    {
        juniper::parse_junos_config(&content)
            .map_err(|err| PhysicalUseCaseError::parse_failure(err.to_string()))?
    } else if content.contains("hostname \"") && content.contains("untagged") {
        aruba::parse_aruba_config(&content)
            .map_err(|err| PhysicalUseCaseError::parse_failure(err.to_string()))?
    } else {
        cisco::parse_running_config_file(file_path)
            .map_err(|err| PhysicalUseCaseError::parse_failure(err.to_string()))?
    };

    let hostname = switch.hostname.clone();
    topology.switches.retain(|s| s.hostname != hostname);
    topology.switches.push(switch);
    topology.build_links();
    topology.correlate_arp_to_ports();
    Ok(topology.clone())
}

pub fn import_mac_table_auto(
    topology: &mut PhysicalTopology,
    path: &str,
    switch_hostname: &str,
) -> Result<PhysicalTopology, PhysicalUseCaseError> {
    let content = std::fs::read_to_string(path)?;
    let file_path = Path::new(path);

    let entries =
        if content.contains("ethernet-switching") || content.contains("Ethernet switching") {
            juniper::parse_ethernet_switching_table(&content, switch_hostname)
        } else if content.to_lowercase().contains("mac address") && content.contains('-') {
            aruba::parse_aruba_mac_table(&content)
        } else {
            cisco::parse_mac_table_file(file_path)
                .map_err(|err| PhysicalUseCaseError::parse_failure(err.to_string()))?
        };

    if !topology
        .switches
        .iter()
        .any(|s| s.hostname == switch_hostname)
    {
        return Err(PhysicalUseCaseError::invalid_input(format!(
            "Switch '{}' not found. Import its config first.",
            switch_hostname
        )));
    }

    topology.apply_mac_table(switch_hostname, &entries);
    topology.correlate_arp_to_ports();
    Ok(topology.clone())
}

pub fn import_neighbor_table(
    topology: &mut PhysicalTopology,
    path: &str,
    switch_hostname: &str,
) -> Result<PhysicalTopology, PhysicalUseCaseError> {
    let content = std::fs::read_to_string(path)?;
    let file_path = Path::new(path);

    let neighbors = if content.contains("ge-") || content.contains("xe-") || content.contains("et-")
    {
        juniper::parse_lldp_neighbors(&content)
    } else if content.contains("ChassisId") || content.contains("LocalPort") {
        aruba::parse_aruba_lldp_neighbors(&content)
    } else {
        cisco::parse_cdp_neighbors_file(file_path)
            .map_err(|err| PhysicalUseCaseError::parse_failure(err.to_string()))?
    };

    if !topology
        .switches
        .iter()
        .any(|s| s.hostname == switch_hostname)
    {
        return Err(PhysicalUseCaseError::invalid_input(format!(
            "Switch '{}' not found. Import its config first.",
            switch_hostname
        )));
    }

    topology.apply_cdp_neighbors(switch_hostname, &neighbors);
    topology.build_links();
    Ok(topology.clone())
}

pub fn run_topology_inference(
    assets: &[AssetInfo],
    connections: &[ConnectionInfo],
    inferred_topology: &mut Option<InferredTopology>,
) -> InferredTopology {
    let input = InferenceInput {
        assets: assets
            .iter()
            .map(|a| InfAssetSnapshot {
                ip_address: a.ip_address.clone(),
                mac_address: a.mac_address.clone(),
            })
            .collect(),
        connections: connections
            .iter()
            .map(|c| ConnSnapshot {
                src_ip: c.src_ip.clone(),
                dst_ip: c.dst_ip.clone(),
                src_mac: c.src_mac.clone(),
                dst_mac: c.dst_mac.clone(),
                packet_count: c.packet_count,
            })
            .collect(),
    };

    let result = inference::infer_topology(&input);
    *inferred_topology = Some(result.clone());
    result
}

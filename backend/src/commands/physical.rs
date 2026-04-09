//! Physical topology command adapters.
//!
//! Adapter responsibilities:
//! - Read/write runtime physical state
//! - Delegate topology import/inference behavior to application use-cases

use gm_physical::{InferredTopology, PhysicalTopology};

use crate::application::use_cases::physical as use_case;

use super::{
    error::AppError,
    support::{read_state, write_state},
    AppState,
};

pub fn import_cisco_config(path: String, state: &AppState) -> Result<PhysicalTopology, AppError> {
    let mut physical = write_state(&state.physical, "physical").map_err(AppError::state_lock)?;
    let topology = use_case::import_cisco_config(&mut physical.physical_topology, &path)
        .map_err(AppError::parse_failure)?;
    log::info!("Imported Cisco config from {}", path);
    Ok(topology)
}

pub fn import_mac_table(
    path: String,
    switch_hostname: String,
    state: &AppState,
) -> Result<PhysicalTopology, AppError> {
    let mut physical = write_state(&state.physical, "physical").map_err(AppError::state_lock)?;
    let topology =
        use_case::import_mac_table(&mut physical.physical_topology, &path, &switch_hostname)
            .map_err(AppError::invalid_input)?;
    log::info!("Imported MAC table from {} for {}", path, switch_hostname);
    Ok(topology)
}

pub fn import_cdp_neighbors(
    path: String,
    switch_hostname: String,
    state: &AppState,
) -> Result<PhysicalTopology, AppError> {
    let mut physical = write_state(&state.physical, "physical").map_err(AppError::state_lock)?;
    let topology =
        use_case::import_cdp_neighbors(&mut physical.physical_topology, &path, &switch_hostname)
            .map_err(AppError::invalid_input)?;
    log::info!(
        "Imported neighbor table from {} for {}",
        path,
        switch_hostname
    );
    Ok(topology)
}

pub fn import_arp_table(path: String, state: &AppState) -> Result<PhysicalTopology, AppError> {
    let mut physical = write_state(&state.physical, "physical").map_err(AppError::state_lock)?;
    let topology = use_case::import_arp_table(&mut physical.physical_topology, &path)
        .map_err(AppError::parse_failure)?;
    log::info!("Imported ARP table from {}", path);
    Ok(topology)
}

pub fn get_physical_topology(state: &AppState) -> Result<PhysicalTopology, AppError> {
    let physical = read_state(&state.physical, "physical").map_err(AppError::state_lock)?;
    Ok(physical.physical_topology.clone())
}

pub fn clear_physical_topology(state: &AppState) -> Result<(), AppError> {
    let mut physical = write_state(&state.physical, "physical").map_err(AppError::state_lock)?;
    physical.physical_topology = PhysicalTopology::default();
    log::info!("Cleared physical topology");
    Ok(())
}

pub fn import_network_config(path: String, state: &AppState) -> Result<PhysicalTopology, AppError> {
    let mut physical = write_state(&state.physical, "physical").map_err(AppError::state_lock)?;
    let topology = use_case::import_network_config(&mut physical.physical_topology, &path)
        .map_err(AppError::parse_failure)?;
    log::info!("Imported network config from {}", path);
    Ok(topology)
}

pub fn import_mac_table_auto(
    path: String,
    switch_hostname: String,
    state: &AppState,
) -> Result<PhysicalTopology, AppError> {
    let mut physical = write_state(&state.physical, "physical").map_err(AppError::state_lock)?;
    let topology =
        use_case::import_mac_table_auto(&mut physical.physical_topology, &path, &switch_hostname)
            .map_err(AppError::invalid_input)?;
    log::info!(
        "Auto-imported MAC table from {} for {}",
        path,
        switch_hostname
    );
    Ok(topology)
}

pub fn import_neighbor_table(
    path: String,
    switch_hostname: String,
    state: &AppState,
) -> Result<PhysicalTopology, AppError> {
    let mut physical = write_state(&state.physical, "physical").map_err(AppError::state_lock)?;
    let topology =
        use_case::import_neighbor_table(&mut physical.physical_topology, &path, &switch_hostname)
            .map_err(AppError::invalid_input)?;
    log::info!(
        "Auto-imported neighbor table from {} for {}",
        path,
        switch_hostname
    );
    Ok(topology)
}

pub fn run_topology_inference(state: &AppState) -> Result<InferredTopology, AppError> {
    let capture = read_state(&state.capture, "capture").map_err(AppError::state_lock)?;
    let inventory = read_state(&state.inventory, "inventory").map_err(AppError::state_lock)?;
    let mut physical = write_state(&state.physical, "physical").map_err(AppError::state_lock)?;

    let result = use_case::run_topology_inference(
        &inventory.assets,
        &capture.connections,
        &mut physical.inferred_topology,
    );

    log::info!(
        "Inferred topology: {} subnets, {} gateways, {} switch candidates",
        result.subnets.len(),
        result.gateways.len(),
        result.switch_candidates.len()
    );

    Ok(result)
}

pub fn get_inferred_topology(state: &AppState) -> Result<Option<InferredTopology>, AppError> {
    let physical = read_state(&state.physical, "physical").map_err(AppError::state_lock)?;
    Ok(physical.inferred_topology.clone())
}

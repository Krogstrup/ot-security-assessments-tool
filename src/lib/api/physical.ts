/**
 * Physical topology: device config, MAC tables, neighbor discovery, inference.
 */

import { invokeCompat } from './core';
import type { PhysicalTopology, InferredTopology } from '/types';

export async function importCiscoConfig(path: string): Promise<PhysicalTopology> {
	return invokeCompat<PhysicalTopology>('import_cisco_config', { path });
}

export async function importMacTable(path: string, switchHostname: string): Promise<PhysicalTopology> {
	return invokeCompat<PhysicalTopology>('import_mac_table', { path, switchHostname });
}

export async function importCdpNeighbors(path: string, switchHostname: string): Promise<PhysicalTopology> {
	return invokeCompat<PhysicalTopology>('import_cdp_neighbors', { path, switchHostname });
}

export async function importArpTable(path: string): Promise<PhysicalTopology> {
	return invokeCompat<PhysicalTopology>('import_arp_table', { path });
}

export async function getPhysicalTopology(): Promise<PhysicalTopology> {
	return invokeCompat<PhysicalTopology>('get_physical_topology');
}

export async function clearPhysicalTopology(): Promise<void> {
	return invokeCompat('clear_physical_topology');
}

export async function importNetworkConfig(path: string): Promise<PhysicalTopology> {
	return invokeCompat<PhysicalTopology>('import_network_config', { path });
}

export async function importMacTableAuto(path: string, switchHostname: string): Promise<PhysicalTopology> {
	return invokeCompat<PhysicalTopology>('import_mac_table_auto', { path, switchHostname });
}

export async function importNeighborTable(path: string, switchHostname: string): Promise<PhysicalTopology> {
	return invokeCompat<PhysicalTopology>('import_neighbor_table', { path, switchHostname });
}

export async function runTopologyInference(): Promise<InferredTopology> {
	return invokeCompat<InferredTopology>('run_topology_inference');
}

export async function getInferredTopology(): Promise<InferredTopology | null> {
	return invokeCompat<InferredTopology | null>('get_inferred_topology');
}

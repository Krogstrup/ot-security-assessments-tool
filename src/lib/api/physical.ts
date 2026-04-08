/**
 * Physical topology: device config, MAC tables, neighbor discovery, inference.
 */

import type { InferredTopology } from '$lib/types/operations';
import { httpJson } from './core';
import type { PhysicalTopology } from '$lib/types';

export async function importCiscoConfig(path: string): Promise<PhysicalTopology> {
	return httpJson<PhysicalTopology>('/api/v1/physical/cisco-config', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ path })
	});
}

export async function importMacTable(path: string, switchHostname: string): Promise<PhysicalTopology> {
	return httpJson<PhysicalTopology>('/api/v1/physical/mac-table', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ path, switchHostname })
	});
}

export async function importCdpNeighbors(path: string, switchHostname: string): Promise<PhysicalTopology> {
	return httpJson<PhysicalTopology>('/api/v1/physical/cdp-neighbors', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ path, switchHostname })
	});
}

export async function importArpTable(path: string): Promise<PhysicalTopology> {
	return httpJson<PhysicalTopology>('/api/v1/physical/arp-table', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ path })
	});
}

export async function getPhysicalTopology(): Promise<PhysicalTopology> {
	return httpJson<PhysicalTopology>('/api/v1/physical/topology');
}

export async function clearPhysicalTopology(): Promise<void> {
	await httpJson('/api/v1/physical/topology', { method: 'DELETE' });
}

export async function importNetworkConfig(path: string): Promise<PhysicalTopology> {
	return httpJson<PhysicalTopology>('/api/v1/physical/network-config', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ path })
	});
}

export async function importMacTableAuto(path: string, switchHostname: string): Promise<PhysicalTopology> {
	return httpJson<PhysicalTopology>('/api/v1/physical/mac-table-auto', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ path, switchHostname })
	});
}

export async function importNeighborTable(path: string, switchHostname: string): Promise<PhysicalTopology> {
	return httpJson<PhysicalTopology>('/api/v1/physical/neighbor-table', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ path, switchHostname })
	});
}

export async function runTopologyInference(): Promise<InferredTopology> {
	return httpJson<InferredTopology>('/api/v1/physical/inference/run', { method: 'POST' });
}

export async function getInferredTopology(): Promise<InferredTopology | null> {
	return httpJson<InferredTopology | null>('/api/v1/physical/inference');
}

/**
 * Physical topology: device config, MAC tables, neighbor discovery, inference.
 */

import type { InferredTopology } from '$lib/types/operations';
import { z } from 'zod';
import { inferredTopologySchema, physicalTopologySchema } from '$lib/schemas';
import { httpValidated } from './core';
import type { PhysicalTopology } from '$lib/types';

export async function importCiscoConfig(path: string): Promise<PhysicalTopology> {
	return httpValidated(physicalTopologySchema, '/api/v1/physical/cisco-config', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ path })
	});
}

export async function importMacTable(path: string, switchHostname: string): Promise<PhysicalTopology> {
	return httpValidated(physicalTopologySchema, '/api/v1/physical/mac-table', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ path, switchHostname })
	});
}

export async function importCdpNeighbors(path: string, switchHostname: string): Promise<PhysicalTopology> {
	return httpValidated(physicalTopologySchema, '/api/v1/physical/cdp-neighbors', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ path, switchHostname })
	});
}

export async function importArpTable(path: string): Promise<PhysicalTopology> {
	return httpValidated(physicalTopologySchema, '/api/v1/physical/arp-table', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ path })
	});
}

export async function getPhysicalTopology(): Promise<PhysicalTopology> {
	return httpValidated(physicalTopologySchema, '/api/v1/physical/topology');
}

export async function clearPhysicalTopology(): Promise<void> {
	await httpValidated(z.unknown(), '/api/v1/physical/topology', { method: 'DELETE' });
}

export async function importNetworkConfig(path: string): Promise<PhysicalTopology> {
	return httpValidated(physicalTopologySchema, '/api/v1/physical/network-config', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ path })
	});
}

export async function importMacTableAuto(path: string, switchHostname: string): Promise<PhysicalTopology> {
	return httpValidated(physicalTopologySchema, '/api/v1/physical/mac-table-auto', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ path, switchHostname })
	});
}

export async function importNeighborTable(path: string, switchHostname: string): Promise<PhysicalTopology> {
	return httpValidated(physicalTopologySchema, '/api/v1/physical/neighbor-table', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ path, switchHostname })
	});
}

export async function runTopologyInference(): Promise<InferredTopology> {
	return httpValidated(inferredTopologySchema, '/api/v1/physical/inference/run', { method: 'POST' });
}

export async function getInferredTopology(): Promise<InferredTopology | null> {
	return httpValidated(inferredTopologySchema.nullable(), '/api/v1/physical/inference');
}

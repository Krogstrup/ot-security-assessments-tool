import {
	importArpTable,
	importMacTableAuto,
	importNeighborTable,
	importNetworkConfig
} from '$lib/api';
import type { PhysicalTopology } from '$lib/types/operations';

export type PhysicalImportType = 'config' | 'mac' | 'cdp' | 'arp';

export interface PhysicalImportResult {
	topology: PhysicalTopology;
	successMessage: string;
}

function fileName(path: string): string {
	return path.split(/[/\\]/).pop() ?? path;
}

export async function runPhysicalImport(
	importType: PhysicalImportType,
	filePath: string,
	switchHostname: string
): Promise<PhysicalImportResult> {
	switch (importType) {
		case 'config': {
			const topology = await importNetworkConfig(filePath);
			return {
				topology,
				successMessage: `Imported config from ${fileName(filePath)}`
			};
		}
		case 'mac': {
			if (!switchHostname.trim()) {
				throw new Error('Select a switch hostname for MAC table import');
			}
			const topology = await importMacTableAuto(filePath, switchHostname.trim());
			return {
				topology,
				successMessage: `Imported MAC table for ${switchHostname}`
			};
		}
		case 'cdp': {
			if (!switchHostname.trim()) {
				throw new Error('Select a switch hostname for CDP/LLDP import');
			}
			const topology = await importNeighborTable(filePath, switchHostname.trim());
			return {
				topology,
				successMessage: `Imported neighbors for ${switchHostname}`
			};
		}
		case 'arp': {
			const topology = await importArpTable(filePath);
			return {
				topology,
				successMessage: `Imported ARP table from ${fileName(filePath)}`
			};
		}
	}
}

import {
	clearPhysicalTopology,
	getInferredTopology,
	getPhysicalTopology,
	getRedundancyProtocols,
	runTopologyInference
} from '$lib/api';
import { physicalTopology } from '$lib/stores/physical';
import type { RedundancyInfo } from '$lib/types/deep-parse';
import type { InferredTopology, PhysicalPort, PhysicalSwitch } from '$lib/types/operations';
import { runPhysicalImport, type PhysicalImportType } from './physicalImport';

export interface PhysicalViewHooks {
	getImportType: () => PhysicalImportType;
	getSwitchHostname: () => string;
	setImporting: (v: boolean) => void;
	setImportError: (v: string) => void;
	setImportSuccess: (v: string) => void;
	setInferring: (v: boolean) => void;
	setInferredTopology: (v: InferredTopology | null) => void;
	setRedundancyProtocols: (v: RedundancyInfo[]) => void;
	setLoadingRedundancy: (v: boolean) => void;
	setSelectedSwitch: (v: PhysicalSwitch | null) => void;
	setSelectedPort: (v: PhysicalPort | null) => void;
}

export async function initializePhysicalView(hooks: Pick<PhysicalViewHooks, 'setInferredTopology'>): Promise<void> {
	try {
		const topo = await getPhysicalTopology();
		physicalTopology.set(topo);
	} catch {
		// Expected in browser dev mode
	}
	try {
		const inferred = await getInferredTopology();
		if (inferred) hooks.setInferredTopology(inferred);
	} catch {
		// Expected in browser dev mode
	}
}

export async function handleImportFlow(
	getFilePath: () => Promise<string | string[] | null>,
	hooks: PhysicalViewHooks
): Promise<void> {
	hooks.setImportError('');
	hooks.setImportSuccess('');

	const result = await getFilePath();
	if (!result) return;
	const filePath = result as string;

	hooks.setImporting(true);
	try {
		const importResult = await runPhysicalImport(
			hooks.getImportType(),
			filePath,
			hooks.getSwitchHostname()
		);
		physicalTopology.set(importResult.topology);
		hooks.setImportSuccess(importResult.successMessage);
	} catch (err) {
		hooks.setImportError(`Import failed: ${err}`);
	} finally {
		hooks.setImporting(false);
	}
}

export async function handleClearFlow(hooks: PhysicalViewHooks): Promise<void> {
	try {
		await clearPhysicalTopology();
		physicalTopology.set({ switches: [], links: [], device_locations: {} });
		hooks.setSelectedSwitch(null);
		hooks.setSelectedPort(null);
		hooks.setImportSuccess('Physical topology cleared');
	} catch (err) {
		hooks.setImportError(`Clear failed: ${err}`);
	}
}

export async function handleRunInferenceFlow(hooks: PhysicalViewHooks): Promise<void> {
	hooks.setInferring(true);
	try {
		hooks.setInferredTopology(await runTopologyInference());
	} catch (err) {
		hooks.setImportError(`Inference failed: ${err}`);
	} finally {
		hooks.setInferring(false);
	}
}

export async function loadRedundancyFlow(hooks: PhysicalViewHooks): Promise<void> {
	hooks.setLoadingRedundancy(true);
	try {
		hooks.setRedundancyProtocols(await getRedundancyProtocols());
	} catch {
		hooks.setRedundancyProtocols([]);
	} finally {
		hooks.setLoadingRedundancy(false);
	}
}

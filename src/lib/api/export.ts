/**
 * Export: CSV, JSON, PDF, SBOM, STIX, PCAP, firewall rules.
 */

import type { FilteredPcapResult, AllowlistEntry } from '$lib/types/analysis';
import type { EnforcementFormat, SegmentationReport } from '$lib/types/segmentation';
import { z } from 'zod';
import { allowlistEntrySchema, filteredPcapResultSchema } from '$lib/schemas';
import { httpValidated } from './core';
import type { ReportConfig } from '$lib/types';

export async function exportAssetsCsv(outputPath: string): Promise<string> {
	return httpValidated(z.string(), '/api/v1/exports/assets/csv', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ outputPath })
	});
}

export async function exportConnectionsCsv(outputPath: string): Promise<string> {
	return httpValidated(z.string(), '/api/v1/exports/connections/csv', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ outputPath })
	});
}

export async function exportTopologyJson(outputPath: string): Promise<string> {
	return httpValidated(z.string(), '/api/v1/exports/topology/json', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ outputPath })
	});
}

export async function exportAssetsJson(outputPath: string): Promise<string> {
	return httpValidated(z.string(), '/api/v1/exports/assets/json', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ outputPath })
	});
}

export async function generatePdfReport(config: ReportConfig, outputPath: string): Promise<string> {
	return httpValidated(z.string(), '/api/v1/exports/report/pdf', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ config, outputPath })
	});
}

export async function exportSbom(format: 'csv' | 'json', outputPath: string): Promise<string> {
	return httpValidated(z.string(), '/api/v1/exports/sbom', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ format, outputPath })
	});
}

export async function exportStixBundle(outputPath: string): Promise<string> {
	return httpValidated(z.string(), '/api/v1/exports/stix', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ outputPath })
	});
}

export async function saveTopologyImage(imageData: string, outputPath: string): Promise<string> {
	return httpValidated(z.string(), '/api/v1/exports/topology/image', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ imageData, outputPath })
	});
}

export async function generateCommunicationAllowlist(): Promise<AllowlistEntry[]> {
	return httpValidated(z.array(allowlistEntrySchema), '/api/v1/exports/allowlist');
}

export async function exportAllowlistCsv(outputPath: string): Promise<string> {
	return httpValidated(z.string(), '/api/v1/exports/allowlist/csv', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ outputPath })
	});
}

export async function exportFirewallRules(outputPath: string): Promise<string> {
	return httpValidated(z.string(), '/api/v1/exports/firewall-rules', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ outputPath })
	});
}

export async function exportFilteredPcap(
	filterIps: string[],
	filterPorts: number[],
	outputPath: string
): Promise<FilteredPcapResult> {
	return httpValidated(filteredPcapResultSchema, '/api/v1/exports/pcap/filtered', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({
			filterIps,
			filterPorts,
			outputPath
		})
	});
}

export async function runSegmentation(): Promise<SegmentationReport> {
	return httpValidated(z.unknown(), '/api/v1/segmentation/run', { method: 'POST' }) as Promise<SegmentationReport>;
}

export async function exportEnforcementConfig(format: EnforcementFormat): Promise<string> {
	return httpValidated(z.string(), '/api/v1/segmentation/enforcement-config', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ format })
	});
}

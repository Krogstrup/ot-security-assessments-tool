/**
 * Export: CSV, JSON, PDF, SBOM, STIX, PCAP, firewall rules.
 */

import type { FilteredPcapResult, AllowlistEntry } from '$lib/types/analysis';
import type { EnforcementFormat, SegmentationReport } from '$lib/types/segmentation';
import { invokeCompat } from './core';
import type { ReportConfig } from '$lib/types';

export async function exportAssetsCsv(outputPath: string): Promise<string> {
	return invokeCompat<string>('export_assets_csv', { outputPath });
}

export async function exportConnectionsCsv(outputPath: string): Promise<string> {
	return invokeCompat<string>('export_connections_csv', { outputPath });
}

export async function exportTopologyJson(outputPath: string): Promise<string> {
	return invokeCompat<string>('export_topology_json', { outputPath });
}

export async function exportAssetsJson(outputPath: string): Promise<string> {
	return invokeCompat<string>('export_assets_json', { outputPath });
}

export async function generatePdfReport(config: ReportConfig, outputPath: string): Promise<string> {
	return invokeCompat<string>('generate_pdf_report', { config, outputPath });
}

export async function exportSbom(format: 'csv' | 'json', outputPath: string): Promise<string> {
	return invokeCompat<string>('export_sbom', { format, outputPath });
}

export async function exportStixBundle(outputPath: string): Promise<string> {
	return invokeCompat<string>('export_stix_bundle', { outputPath });
}

export async function saveTopologyImage(imageData: string, outputPath: string): Promise<string> {
	return invokeCompat<string>('save_topology_image', { imageData, outputPath });
}

export async function generateCommunicationAllowlist(): Promise<AllowlistEntry[]> {
	return invokeCompat<AllowlistEntry[]>('generate_communication_allowlist');
}

export async function exportAllowlistCsv(outputPath: string): Promise<string> {
	return invokeCompat<string>('export_allowlist_csv', { outputPath });
}

export async function exportFirewallRules(outputPath: string): Promise<string> {
	return invokeCompat<string>('export_firewall_rules', { outputPath });
}

export async function exportFilteredPcap(
	filterIps: string[],
	filterPorts: number[],
	outputPath: string
): Promise<FilteredPcapResult> {
	return invokeCompat<FilteredPcapResult>('export_filtered_pcap', {
		filterIps,
		filterPorts,
		outputPath
	});
}

export async function runSegmentation(): Promise<SegmentationReport> {
	return invokeCompat<SegmentationReport>('run_segmentation');
}

export async function exportEnforcementConfig(format: EnforcementFormat): Promise<string> {
	return invokeCompat<string>('export_enforcement_config', { format });
}

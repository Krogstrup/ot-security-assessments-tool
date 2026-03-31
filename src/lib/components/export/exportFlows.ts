import {
	generatePdfReport,
	saveTopologyImage,
	exportAssetsCsv,
	exportConnectionsCsv,
	exportTopologyJson,
	exportAssetsJson,
	exportStixBundle,
	exportAllowlistCsv,
	exportFirewallRules
} from '$lib/api';
import type { ReportConfig } from '$lib/types/operations';
import { saveExportPathDialog } from './exportUtils';

export interface RunPathExportOptions {
	action: string;
	dialogTitle: string;
	defaultName: string;
	filterName: string;
	extensions: string[];
	errorPrefix: string;
	successPrefix?: string;
	exporter: (path: string) => Promise<string>;
}

export interface ExportFlowHooks {
	setBusyAction: (action: string | null) => void;
	showStatus: (message: string, type: 'success' | 'error' | 'info') => void;
}

export const SIMPLE_EXPORTS = {
	assets_csv: {
		action: 'assets_csv',
		dialogTitle: 'Export Assets CSV',
		defaultName: 'assets.csv',
		filterName: 'CSV Files',
		extensions: ['csv'],
		errorPrefix: 'CSV export failed',
		exporter: exportAssetsCsv
	},
	conn_csv: {
		action: 'conn_csv',
		dialogTitle: 'Export Connections CSV',
		defaultName: 'connections.csv',
		filterName: 'CSV Files',
		extensions: ['csv'],
		errorPrefix: 'CSV export failed',
		exporter: exportConnectionsCsv
	},
	topo_json: {
		action: 'topo_json',
		dialogTitle: 'Export Topology JSON',
		defaultName: 'topology.json',
		filterName: 'JSON Files',
		extensions: ['json'],
		errorPrefix: 'JSON export failed',
		exporter: exportTopologyJson
	},
	assets_json: {
		action: 'assets_json',
		dialogTitle: 'Export Assets JSON',
		defaultName: 'assets.json',
		filterName: 'JSON Files',
		extensions: ['json'],
		errorPrefix: 'JSON export failed',
		exporter: exportAssetsJson
	},
	stix: {
		action: 'stix',
		dialogTitle: 'Export STIX Bundle',
		defaultName: 'stix_bundle.json',
		filterName: 'JSON Files',
		extensions: ['json'],
		errorPrefix: 'STIX export failed',
		exporter: exportStixBundle
	},
	allowlist_csv: {
		action: 'allowlist_csv',
		dialogTitle: 'Export Communication Allowlist',
		defaultName: 'allowlist.csv',
		filterName: 'CSV Files',
		extensions: ['csv'],
		errorPrefix: 'Allowlist export failed',
		successPrefix: 'Allowlist exported: ',
		exporter: exportAllowlistCsv
	},
	fw_rules: {
		action: 'fw_rules',
		dialogTitle: 'Export Firewall Rules',
		defaultName: 'firewall_rules.txt',
		filterName: 'Text Files',
		extensions: ['txt'],
		errorPrefix: 'Firewall rules export failed',
		successPrefix: 'Firewall rules exported: ',
		exporter: exportFirewallRules
	}
} satisfies Record<string, RunPathExportOptions>;

export async function runPathExportFlow(options: RunPathExportOptions, hooks: ExportFlowHooks) {
	try {
		hooks.setBusyAction(options.action);
		const path = await saveExportPathDialog(
			options.dialogTitle,
			options.defaultName,
			options.filterName,
			options.extensions
		);
		if (!path) return;
		const result = await options.exporter(path);
		hooks.showStatus(options.successPrefix ? `${options.successPrefix}${result}` : result, 'success');
	} catch (err) {
		hooks.showStatus(`${options.errorPrefix}: ${err}`, 'error');
	} finally {
		hooks.setBusyAction(null);
	}
}

export async function generatePdfFlow(
	config: ReportConfig,
	defaultName: string,
	hooks: ExportFlowHooks
) {
	try {
		hooks.setBusyAction('pdf');
		const path = await saveExportPathDialog('Save PDF Report', defaultName, 'PDF Files', ['pdf']);
		if (!path) return;
		hooks.showStatus(await generatePdfReport(config, path), 'success');
	} catch (err) {
		hooks.showStatus(`PDF generation failed: ${err}`, 'error');
	} finally {
		hooks.setBusyAction(null);
	}
}

export async function exportTopologyImageFlow(
	format: 'png' | 'svg',
	imageData: string | null,
	hooks: ExportFlowHooks
) {
	try {
		hooks.setBusyAction(`image_${format}`);
		if (!imageData) {
			hooks.showStatus('No topology graph available for export.', 'error');
			return;
		}

		const path = await saveExportPathDialog(
			`Save Topology ${format.toUpperCase()}`,
			`topology.${format}`,
			`${format.toUpperCase()} Files`,
			[format]
		);
		if (!path) return;
		hooks.showStatus(await saveTopologyImage(imageData, path), 'success');
	} catch (err) {
		hooks.showStatus(`Image export failed: ${err}`, 'error');
	} finally {
		hooks.setBusyAction(null);
	}
}

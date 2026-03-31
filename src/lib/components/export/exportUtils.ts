import type { AllowlistEntry, Finding } from '$lib/types/analysis';
import { savePathDialog } from '$lib/utils/dialog';

export async function saveExportPathDialog(
	title: string,
	defaultName: string,
	filterName: string,
	extensions: string[]
): Promise<string | null> {
	return savePathDialog({
		title,
		defaultPath: defaultName,
		filters: [
			{ name: filterName, extensions },
			{ name: 'All Files', extensions: ['*'] }
		]
	});
}

export function remediationForFinding(finding: Finding): string {
	const technique = finding.technique_id ?? '';
	const title = finding.title.toLowerCase();
	if (technique === 'T0855')
		return 'Restrict write access, implement application whitelisting per IEC 62443-3-3 SR 3.3';
	if (technique === 'T0886') return 'Implement network segmentation between Purdue levels using firewalls/DMZ';
	if (technique === 'T0843')
		return 'Enable program download protection, require authentication for S7 program transfers';
	if (technique === 'T0816')
		return 'Restrict PLC stop commands to authorized engineering workstations only';
	if (technique === 'T0836')
		return 'Enable firmware verification, implement change management for firmware updates';
	if (technique === 'T0846') return 'Investigate and block unauthorized device from accessing OT network';
	if (technique === 'T0814') return 'Restrict diagnostic commands (FC8) to authorized engineering workstations';
	if (technique === 'T0856')
		return 'Configure authorized master list; block unsolicited DNP3 from unknown sources';
	if (technique === 'T0811') return 'Block DeviceCommunicationControl from unauthorized hosts';
	if (title.includes('flat network'))
		return 'Implement network segmentation per IEC 62443-2-1 and NIST SP 800-82';
	if (title.includes('cleartext') || title.includes('unencrypted'))
		return 'Evaluate encrypted protocol alternatives: OPC UA with TLS, DNP3-SA, or TLS-wrapped tunnels';
	if (title.includes('internet') || title.includes('public ip'))
		return 'Remove public IP or place device behind NAT/DMZ; verify firewall rules immediately';
	return 'Review finding and apply vendor-recommended hardening guidelines';
}

export function mapFindingsToRemediation(
	findings: Finding[]
): RemediationItem[] {
	const severityOrder: Record<string, number> = {
		critical: 0,
		high: 1,
		medium: 2,
		low: 3,
		info: 4
	};
	const sorted = [...findings].sort(
		(a, b) => (severityOrder[a.severity] ?? 5) - (severityOrder[b.severity] ?? 5)
	);

	return sorted.map((finding, index) => ({
		rank: index + 1,
		severity: finding.severity,
		title: finding.title,
		description: finding.description,
		assets: finding.affected_assets,
		remediation: remediationForFinding(finding),
		technique: finding.technique_id ?? null
	}));
}

export interface RemediationItem {
	rank: number;
	severity: string;
	title: string;
	description: string;
	assets: string[];
	remediation: string;
	technique: string | null;
}

export function buildRemediationCsv(items: RemediationItem[]): string {
	const header = 'Rank,Severity,Title,Affected Assets,Remediation,ATT&CK Technique';
	const rows = items.map(
		(item) =>
			`${item.rank},"${item.severity}","${item.title.replace(/"/g, '""')}","${item.assets.join('; ')}","${item.remediation.replace(/"/g, '""')}","${item.technique ?? ''}"`
	);
	return [header, ...rows].join('\n');
}

export function allowlistClassificationClass(classification: AllowlistEntry['classification']): string {
	if (classification === 'operational') return 'cls-operational';
	if (classification === 'management') return 'cls-management';
	if (classification === 'monitoring') return 'cls-monitoring';
	return 'cls-it';
}

export function extractTopologyImageData(format: 'png' | 'svg'): string | null {
	if (format === 'png') {
		const canvas = document.querySelector('.cy-container canvas') as HTMLCanvasElement | null;
		if (!canvas) return null;
		return canvas.toDataURL('image/png').replace(/^data:image\/png;base64,/, '');
	}

	const svgElement = document.querySelector('.cy-container svg');
	if (!svgElement) return null;
	return btoa(new XMLSerializer().serializeToString(svgElement));
}

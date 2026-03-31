/**
 * Security analysis: ATT&CK, Purdue, anomalies, malware, compliance, CVE.
 */

import type { Finding, PurdueAssignment, AnomalyScore, DefaultCredential, CriticalityAssessment, NamingSuggestion, MalwareFinding, ComplianceMapping, CveMatch } from '$lib/types/analysis';
import type { SwitchSecurityFinding } from '$lib/types/deep-parse';
import { invokeCompat } from './core';
import type { AnalysisResult } from '$lib/types';

export async function runAnalysis(): Promise<AnalysisResult> {
	return invokeCompat<AnalysisResult>('run_analysis');
}

export async function getFindings(): Promise<Finding[]> {
	return invokeCompat<Finding[]>('get_findings');
}

export async function getPurdueAssignments(): Promise<PurdueAssignment[]> {
	return invokeCompat<PurdueAssignment[]>('get_purdue_assignments');
}

export async function getAnomalies(): Promise<AnomalyScore[]> {
	return invokeCompat<AnomalyScore[]>('get_anomalies');
}

export async function getCredentialWarnings(): Promise<DefaultCredential[]> {
	return invokeCompat<DefaultCredential[]>('get_credential_warnings');
}

export async function getCriticality(): Promise<CriticalityAssessment[]> {
	return invokeCompat<CriticalityAssessment[]>('get_criticality');
}

export async function getNamingSuggestions(): Promise<NamingSuggestion[]> {
	return invokeCompat<NamingSuggestion[]>('get_naming_suggestions');
}

export async function getMalwareFindings(): Promise<MalwareFinding[]> {
	return invokeCompat<MalwareFinding[]>('get_malware_findings');
}

export async function getComplianceReport(framework: string): Promise<ComplianceMapping[]> {
	return invokeCompat<ComplianceMapping[]>('get_compliance_report', { framework });
}

export async function getCveWarnings(ip: string): Promise<CveMatch[]> {
	return invokeCompat<CveMatch[]>('get_cve_warnings', { ip });
}

export async function getSwitchSecurityFindings(): Promise<SwitchSecurityFinding[]> {
	return invokeCompat<SwitchSecurityFinding[]>('get_switch_security_findings');
}

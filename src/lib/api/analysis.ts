/**
 * Security analysis: ATT&CK, Purdue, anomalies, malware, compliance, CVE.
 *
 * Web runtime → /api/v1/analysis/* with Zod runtime validation
 */

import { z } from 'zod';
import {
	analysisResultSchema,
	findingSchema,
	purdueAssignmentSchema,
	anomalyScoreSchema,
	defaultCredentialSchema,
	criticalityAssessmentSchema,
	namingSuggestionSchema,
	malwareFindingSchema,
	cveMatchSchema,
	complianceMappingSchema,
	switchSecurityFindingSchema
} from '$lib/schemas';
import { httpValidated } from './core';
import type {
	Finding,
	PurdueAssignment,
	AnomalyScore,
	DefaultCredential,
	CriticalityAssessment,
	NamingSuggestion,
	MalwareFinding,
	ComplianceMapping,
	CveMatch
} from '$lib/types/analysis';
import type { SwitchSecurityFinding } from '$lib/types/deep-parse';
import type { AnalysisResult } from '$lib/types';

export async function runAnalysis(): Promise<AnalysisResult> {
	return httpValidated(analysisResultSchema, '/api/v1/analysis/run', { method: 'POST' });
}

export async function getFindings(): Promise<Finding[]> {
	return httpValidated(z.array(findingSchema), '/api/v1/analysis/findings');
}

export async function getPurdueAssignments(): Promise<PurdueAssignment[]> {
	return httpValidated(z.array(purdueAssignmentSchema), '/api/v1/analysis/purdue');
}

export async function getAnomalies(): Promise<AnomalyScore[]> {
	return httpValidated(z.array(anomalyScoreSchema), '/api/v1/analysis/anomalies');
}

export async function getCredentialWarnings(): Promise<DefaultCredential[]> {
	return httpValidated(z.array(defaultCredentialSchema), '/api/v1/analysis/credentials');
}

export async function getCriticality(): Promise<CriticalityAssessment[]> {
	return httpValidated(z.array(criticalityAssessmentSchema), '/api/v1/analysis/criticality');
}

export async function getNamingSuggestions(): Promise<NamingSuggestion[]> {
	return httpValidated(z.array(namingSuggestionSchema), '/api/v1/analysis/naming-suggestions');
}

export async function getMalwareFindings(): Promise<MalwareFinding[]> {
	return httpValidated(z.array(malwareFindingSchema), '/api/v1/analysis/malware');
}

export async function getComplianceReport(framework: string): Promise<ComplianceMapping[]> {
	const params = new URLSearchParams({ framework });
	return httpValidated(z.array(complianceMappingSchema), `/api/v1/analysis/compliance?${params}`);
}

export async function getCveWarnings(ip: string): Promise<CveMatch[]> {
	const params = new URLSearchParams({ ip });
	return httpValidated(z.array(cveMatchSchema), `/api/v1/analysis/cve?${params}`);
}

export async function getSwitchSecurityFindings(): Promise<SwitchSecurityFinding[]> {
	return httpValidated(z.array(switchSecurityFindingSchema), '/api/v1/analysis/switch-security');
}

<script lang="ts">
	import type {
		AnalysisSummary,
		AnomalyScore,
		ComplianceMapping,
		CorrelatedAlert,
		Finding,
		FindingSeverity,
		MalwareFinding,
		PurdueAssignment
	} from '$lib/types/analysis';
	import type { SwitchSecurityFinding } from '$lib/types/deep-parse';
	import BaselineDriftView from '../BaselineDriftView.svelte';
	import AnomaliesSection from './AnomaliesSection.svelte';
	import ComplianceSection from './ComplianceSection.svelte';
	import ExternalAlertsSection from './ExternalAlertsSection.svelte';
	import FindingsSection from './FindingsSection.svelte';
	import MalwareSection from './MalwareSection.svelte';
	import PurdueSection from './PurdueSection.svelte';
	import SummaryDashboardSection from './SummaryDashboardSection.svelte';
	import SwitchSecuritySection from './SwitchSecuritySection.svelte';
	import type { AnalysisSection } from './AnalysisSectionTabs.svelte';

	interface Props {
		activeSection: AnalysisSection;
		analysisSummary: AnalysisSummary | null;
		findings: Finding[];
		purdueAssignments: PurdueAssignment[];
		anomalies: AnomalyScore[];
		switchFindings: SwitchSecurityFinding[];
		loadingSwitchFindings: boolean;
		correlatedAlerts: CorrelatedAlert[];
		loadingAlerts: boolean;
		malwareFindings: MalwareFinding[];
		loadingMalware: boolean;
		complianceFramework: 'iec62443' | 'nist80082' | 'nerccip';
		complianceMappings: ComplianceMapping[];
		loadingCompliance: boolean;
		severityOrder: FindingSeverity[];
		severityColors: Record<FindingSeverity, string>;
		purdueColors: Record<number, string>;
		purdueLabels: Record<number, string>;
		getSeverityIcon: (severity: FindingSeverity) => string;
		getTypeLabel: (type: string) => string;
		onNavigateAsset: (ip: string) => void;
		onRefreshSwitchFindings: () => Promise<void>;
		onRefreshAlerts: () => Promise<void>;
		onClearAlerts: () => Promise<void>;
		onRefreshMalware: () => Promise<void>;
		onRefreshCompliance: () => Promise<void>;
		onComplianceFrameworkChange: (framework: 'iec62443' | 'nist80082' | 'nerccip') => void;
	}

	let {
		activeSection,
		analysisSummary,
		findings,
		purdueAssignments,
		anomalies,
		switchFindings,
		loadingSwitchFindings,
		correlatedAlerts,
		loadingAlerts,
		malwareFindings,
		loadingMalware,
		complianceFramework,
		complianceMappings,
		loadingCompliance,
		severityOrder,
		severityColors,
		purdueColors,
		purdueLabels,
		getSeverityIcon,
		getTypeLabel,
		onNavigateAsset,
		onRefreshSwitchFindings,
		onRefreshAlerts,
		onClearAlerts,
		onRefreshMalware,
		onRefreshCompliance,
		onComplianceFrameworkChange
	}: Props = $props();
</script>

<div class="section-content" class:drift-mode={activeSection === 'drift'}>
	{#if activeSection === 'summary'}
		<SummaryDashboardSection {analysisSummary} {severityOrder} {severityColors} />
	{:else if activeSection === 'findings'}
		<FindingsSection {findings} {severityColors} {getSeverityIcon} {getTypeLabel} onNavigateAsset={onNavigateAsset} />
	{:else if activeSection === 'purdue'}
		<PurdueSection
			{purdueAssignments}
			{findings}
			{purdueColors}
			{purdueLabels}
			{severityColors}
			onNavigateAsset={onNavigateAsset}
		/>
	{:else if activeSection === 'anomalies'}
		<AnomaliesSection {anomalies} {severityColors} onNavigateAsset={onNavigateAsset} />
	{:else if activeSection === 'drift'}
		<BaselineDriftView />
	{:else if activeSection === 'switch_security'}
		<SwitchSecuritySection loading={loadingSwitchFindings} findings={switchFindings} onRefresh={onRefreshSwitchFindings} />
	{:else if activeSection === 'external_alerts'}
		<ExternalAlertsSection loading={loadingAlerts} alerts={correlatedAlerts} onRefresh={onRefreshAlerts} onClear={onClearAlerts} />
	{:else if activeSection === 'malware'}
		<MalwareSection loading={loadingMalware} findings={malwareFindings} onRefresh={onRefreshMalware} />
	{:else if activeSection === 'compliance'}
		<ComplianceSection
			framework={complianceFramework}
			mappings={complianceMappings}
			loading={loadingCompliance}
			onFrameworkChange={onComplianceFrameworkChange}
			onRefresh={onRefreshCompliance}
		/>
	{/if}
</div>

<style>
	.section-content {
		flex: 1;
		overflow-y: auto;
		padding: 16px 20px;
	}

	.section-content.drift-mode {
		padding: 0;
		overflow: hidden;
	}
</style>

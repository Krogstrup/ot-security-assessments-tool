import type { AnalysisSection } from './AnalysisSectionTabs.svelte';

export interface AnalysisSectionLoaders {
	loadSwitchFindings: () => Promise<void>;
	loadCorrelatedAlerts: () => Promise<void>;
	loadMalwareFindings: () => Promise<void>;
	loadComplianceReport: () => Promise<void>;
}

export function loadAnalysisSection(section: AnalysisSection, loaders: AnalysisSectionLoaders) {
	switch (section) {
		case 'switch_security':
			void loaders.loadSwitchFindings();
			break;
		case 'external_alerts':
			void loaders.loadCorrelatedAlerts();
			break;
		case 'malware':
			void loaders.loadMalwareFindings();
			break;
		case 'compliance':
			void loaders.loadComplianceReport();
			break;
		default:
			break;
	}
}

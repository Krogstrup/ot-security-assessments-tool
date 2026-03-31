<script lang="ts">
	interface Props {
		hasData: boolean;
		busyAction: string | null;
		onExportStix: () => void;
	}

	let { hasData, busyAction, onExportStix }: Props = $props();
</script>

<section class="export-section">
	<h3 class="section-title">STIX 2.1 Bundle</h3>
	<p class="section-desc">Export a STIX 2.1 compliant bundle with Identity, Infrastructure, IPv4/MAC observables, Network-Traffic, and Relationship objects for threat intelligence sharing.</p>

	<button
		class="action-btn primary"
		disabled={!hasData || busyAction !== null}
		onclick={onExportStix}
	>
		{busyAction === 'stix' ? 'Exporting...' : 'Export STIX 2.1 Bundle'}
	</button>
</section>

<style>
	.export-section {
		background: var(--gm-bg-secondary);
		border: 1px solid var(--gm-border);
		border-radius: 8px;
		padding: 20px;
	}

	.section-title {
		font-size: 13px;
		font-weight: 600;
		color: var(--gm-text-primary);
		margin: 0 0 4px 0;
		letter-spacing: 0.5px;
	}

	.section-desc {
		font-size: 11px;
		color: var(--gm-text-muted);
		margin: 0 0 16px 0;
		line-height: 1.5;
	}

	.action-btn {
		padding: 8px 16px;
		border-radius: 6px;
		font-family: inherit;
		font-size: 11px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.15s;
		border: 1px solid transparent;
	}

	.action-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.action-btn.primary {
		background: rgba(59, 130, 246, 0.1);
		border-color: rgba(59, 130, 246, 0.2);
		color: #3b82f6;
	}

	.action-btn.primary:hover:not(:disabled) {
		background: rgba(59, 130, 246, 0.2);
	}
</style>

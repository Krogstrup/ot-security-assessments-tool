<script lang="ts">
	interface Props {
		assetCount: number;
		connectionCount: number;
		hasData: boolean;
		busyAction: string | null;
		onExportAssetsCsv: () => Promise<void>;
		onExportConnectionsCsv: () => Promise<void>;
		onExportTopologyJson: () => Promise<void>;
		onExportAssetsJson: () => Promise<void>;
	}

	let {
		assetCount,
		connectionCount,
		hasData,
		busyAction,
		onExportAssetsCsv,
		onExportConnectionsCsv,
		onExportTopologyJson,
		onExportAssetsJson
	}: Props = $props();
</script>

<section class="export-section">
	<h3 class="section-title">Data Exports</h3>
	<p class="section-desc">Export raw asset and connection data for external analysis.</p>

	<div class="export-grid">
		<div class="export-card">
			<div class="card-header">
				<span class="card-icon">CSV</span>
				<span class="card-label">Assets</span>
			</div>
			<p class="card-desc">{assetCount.toLocaleString()} assets — IP, MAC, vendor, protocols, Purdue level, confidence</p>
			<button class="action-btn primary" disabled={!hasData || busyAction !== null} onclick={onExportAssetsCsv}>
				{busyAction === 'assets_csv' ? 'Exporting...' : 'Export Assets CSV'}
			</button>
		</div>

		<div class="export-card">
			<div class="card-header">
				<span class="card-icon">CSV</span>
				<span class="card-label">Connections</span>
			</div>
			<p class="card-desc">{connectionCount.toLocaleString()} connections — source, destination, protocol, packets, bytes</p>
			<button class="action-btn primary" disabled={!hasData || busyAction !== null} onclick={onExportConnectionsCsv}>
				{busyAction === 'conn_csv' ? 'Exporting...' : 'Export Connections CSV'}
			</button>
		</div>

		<div class="export-card">
			<div class="card-header">
				<span class="card-icon">JSON</span>
				<span class="card-label">Topology</span>
			</div>
			<p class="card-desc">Full topology with assets, connections, protocol stats, and metadata</p>
			<button class="action-btn primary" disabled={!hasData || busyAction !== null} onclick={onExportTopologyJson}>
				{busyAction === 'topo_json' ? 'Exporting...' : 'Export Topology JSON'}
			</button>
		</div>

		<div class="export-card">
			<div class="card-header">
				<span class="card-icon">JSON</span>
				<span class="card-label">Assets</span>
			</div>
			<p class="card-desc">Asset inventory with all fields — vendor, OUI, country, confidence</p>
			<button class="action-btn primary" disabled={!hasData || busyAction !== null} onclick={onExportAssetsJson}>
				{busyAction === 'assets_json' ? 'Exporting...' : 'Export Assets JSON'}
			</button>
		</div>
	</div>
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

	.export-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 12px;
	}

	.export-card {
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 6px;
		padding: 14px;
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.card-header {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.card-icon {
		font-size: 9px;
		font-weight: 700;
		letter-spacing: 0.5px;
		padding: 2px 6px;
		border-radius: 3px;
		background: rgba(16, 185, 129, 0.15);
		color: #10b981;
	}

	.card-label {
		font-size: 12px;
		font-weight: 600;
		color: var(--gm-text-primary);
	}

	.card-desc {
		font-size: 10px;
		color: var(--gm-text-muted);
		margin: 0;
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

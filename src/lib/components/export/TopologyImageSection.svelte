<script lang="ts">
	interface Props {
		busyAction: string | null;
		onExportPng: () => void;
		onExportSvg: () => void;
	}

	let { busyAction, onExportPng, onExportSvg }: Props = $props();
</script>

<section class="export-section">
	<h3 class="section-title">Topology Image</h3>
	<p class="section-desc">Export the logical topology graph as an image. The topology must be visible in the Topology tab first.</p>
	<div class="btn-row">
		<button
			class="action-btn primary"
			disabled={busyAction !== null}
			onclick={onExportPng}
		>
			{busyAction === 'image_png' ? 'Capturing...' : 'Export as PNG'}
		</button>
		<button
			class="action-btn primary"
			disabled={busyAction !== null}
			onclick={onExportSvg}
		>
			{busyAction === 'image_svg' ? 'Capturing...' : 'Export as SVG'}
		</button>
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

	.btn-row {
		display: flex;
		gap: 10px;
	}
</style>

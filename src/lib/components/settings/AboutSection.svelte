<script lang="ts">
	interface Props {
		appVersion: string;
		rustVersion: string;
	}

	let { appVersion, rustVersion }: Props = $props();
</script>

<section class="settings-section">
	<h3 class="section-title">About</h3>
	<div class="about-grid">
		<div class="about-row">
			<span class="about-label">Application</span>
			<span class="about-value">Kusanagi Kajiki</span>
		</div>
		<div class="about-row">
			<span class="about-label">Version</span>
			<span class="about-value">{appVersion}</span>
		</div>
		<div class="about-row">
			<span class="about-label">Rust Backend</span>
			<span class="about-value">{rustVersion}</span>
		</div>
		<div class="about-row">
			<span class="about-label">License</span>
			<span class="about-value">Apache 2.0</span>
		</div>
	</div>
	<p class="about-desc">
		Kusanagi Kajiki — a modern ICS/SCADA passive network discovery tool.
		Rebuilt with Tauri 2.0 (Rust) and SvelteKit for performance, security, and cross-platform support.
	</p>
</section>

<style>
	.settings-section {
		background: var(--gm-bg-secondary);
		border: 1px solid var(--gm-border);
		border-radius: 8px;
		padding: 20px;
	}

	.section-title {
		font-size: 13px;
		font-weight: 600;
		color: var(--gm-text-primary);
		margin: 0 0 14px 0;
		letter-spacing: 0.5px;
	}

	.about-grid {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.about-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		font-size: 11px;
	}

	.about-label {
		color: var(--gm-text-muted);
	}

	.about-value {
		color: var(--gm-text-secondary);
		font-weight: 500;
	}

	.about-desc {
		font-size: 11px;
		color: var(--gm-text-muted);
		margin-top: 14px;
		line-height: 1.6;
	}
</style>

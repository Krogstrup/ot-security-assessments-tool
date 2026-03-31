<script lang="ts">
	import type { PluginManifest } from '$lib/types/analysis';

	interface Props {
		plugins: PluginManifest[];
	}

	let { plugins }: Props = $props();
</script>

<section class="settings-section">
	<h3 class="section-title">Plugins</h3>
	<div class="about-row">
		<span class="about-label">Directory</span>
		<span class="about-value mono">~/.kusanaginokajiki/plugins/</span>
	</div>
	{#if plugins.length === 0}
		<p class="section-desc">No plugins installed. Place plugin directories with a manifest.json in the plugins directory.</p>
	{:else}
		<div class="plugin-list">
			{#each plugins as plugin}
				<div class="plugin-item">
					<div class="plugin-header">
						<span class="plugin-name">{plugin.name}</span>
						<span class="plugin-version">v{plugin.version}</span>
						<span class="plugin-type">{plugin.plugin_type}</span>
					</div>
					<p class="plugin-desc">{plugin.description}</p>
					{#if plugin.author}
						<span class="plugin-author">by {plugin.author}</span>
					{/if}
				</div>
			{/each}
		</div>
	{/if}
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

	.section-desc {
		font-size: 11px;
		color: var(--gm-text-muted);
		margin: 0 0 12px 0;
		line-height: 1.5;
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

	.about-value.mono {
		font-size: 10px;
	}

	.plugin-list {
		display: flex;
		flex-direction: column;
		gap: 8px;
		margin-top: 12px;
	}

	.plugin-item {
		padding: 10px 12px;
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 6px;
	}

	.plugin-header {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-bottom: 4px;
	}

	.plugin-name {
		font-size: 12px;
		font-weight: 600;
		color: var(--gm-text-primary);
	}

	.plugin-version {
		font-size: 10px;
		color: var(--gm-text-muted);
	}

	.plugin-type {
		font-size: 9px;
		padding: 2px 6px;
		background: rgba(16, 185, 129, 0.1);
		color: #10b981;
		border-radius: 3px;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		font-weight: 600;
	}

	.plugin-desc {
		font-size: 10px;
		color: var(--gm-text-muted);
		margin: 0;
		line-height: 1.4;
	}

	.plugin-author {
		font-size: 9px;
		color: var(--gm-text-muted);
	}
</style>

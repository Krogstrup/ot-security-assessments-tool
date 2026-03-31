<script lang="ts">
	import type { PluginManifest } from '$lib/types/analysis';
	import { getAppInfo, getSettings, saveSettings, listPlugins } from '$lib/api';
	import { themeMode } from '$lib/stores';
	import { onMount } from 'svelte';
	import type { ThemeMode } from '$lib/types';
	import AboutSection from './settings/AboutSection.svelte';
	import ThemeSection from './settings/ThemeSection.svelte';
	import CaptureDefaultsSection from './settings/CaptureDefaultsSection.svelte';
	import DatabaseSection from './settings/DatabaseSection.svelte';
	import PluginsSection from './settings/PluginsSection.svelte';
	import CliUsageSection from './settings/CliUsageSection.svelte';

	let appVersion = $state('—');
	let rustVersion = $state('—');
	let currentTheme = $state<ThemeMode>('dark');
	let plugins = $state<PluginManifest[]>([]);

	themeMode.subscribe((v) => (currentTheme = v));

	function applyTheme(mode: ThemeMode) {
		if (mode === 'system') {
			const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
			document.documentElement.setAttribute('data-theme', prefersDark ? 'dark' : 'light');
		} else {
			document.documentElement.setAttribute('data-theme', mode);
		}
	}

	async function setTheme(mode: ThemeMode) {
		themeMode.set(mode);
		applyTheme(mode);
		try {
			await saveSettings({ theme: mode });
		} catch {
			// May fail in dev
		}
	}

	onMount(async () => {
		try {
			const info = await getAppInfo();
			appVersion = info.version;
			rustVersion = info.rust_version;
		} catch {
			// Expected in browser dev mode
		}

		try {
			const settings = await getSettings();
			currentTheme = settings.theme as ThemeMode;
		} catch {
			// Expected in browser dev mode
		}

		try {
			plugins = await listPlugins();
		} catch {
			// Expected in browser dev mode
		}
	});
</script>

<div class="settings-container">
	<div class="settings-toolbar">
		<h2 class="view-title">Settings</h2>
	</div>

	<div class="settings-content">
		<AboutSection {appVersion} {rustVersion} />
		<ThemeSection {currentTheme} onSetTheme={setTheme} />
		<CaptureDefaultsSection />
		<DatabaseSection />
		<PluginsSection {plugins} />
		<CliUsageSection />
	</div>
</div>

<style>
	.settings-container {
		display: flex;
		flex-direction: column;
		height: 100%;
	}

	.settings-toolbar {
		padding: 10px 16px;
		border-bottom: 1px solid var(--gm-border);
		background: var(--gm-bg-secondary);
	}

	.view-title {
		font-size: 13px;
		font-weight: 600;
		letter-spacing: 1px;
		text-transform: uppercase;
		color: var(--gm-text-primary);
		margin: 0;
	}

	.settings-content {
		flex: 1;
		overflow-y: auto;
		padding: 20px 24px;
		display: flex;
		flex-direction: column;
		gap: 24px;
		max-width: 640px;
	}
</style>

<script lang="ts">
	import type { ThemeMode } from '$lib/types/analysis';

	interface Props {
		currentTheme: ThemeMode;
		onSetTheme: (mode: ThemeMode) => void;
	}

	let { currentTheme, onSetTheme }: Props = $props();
</script>

<section class="settings-section">
	<h3 class="section-title">Theme</h3>
	<p class="section-desc">Light theme is recommended for control room environments where dark screens cause glare.</p>
	<div class="theme-buttons">
		<button class="theme-btn" class:active={currentTheme === 'dark'} onclick={() => onSetTheme('dark')}>
			<span class="theme-btn-icon">&#9789;</span>
			Dark
		</button>
		<button class="theme-btn" class:active={currentTheme === 'light'} onclick={() => onSetTheme('light')}>
			<span class="theme-btn-icon">&#9788;</span>
			Light
		</button>
		<button class="theme-btn" class:active={currentTheme === 'system'} onclick={() => onSetTheme('system')}>
			<span class="theme-btn-icon">&#9681;</span>
			System
		</button>
	</div>
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

	.theme-buttons {
		display: flex;
		gap: 8px;
	}

	.theme-btn {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 8px 16px;
		border: 1px solid var(--gm-border);
		border-radius: 6px;
		background: transparent;
		color: var(--gm-text-secondary);
		font-family: inherit;
		font-size: 11px;
		cursor: pointer;
		transition: all 0.15s;
	}

	.theme-btn:hover {
		background: var(--gm-bg-hover);
	}

	.theme-btn.active {
		background: rgba(16, 185, 129, 0.1);
		border-color: #10b981;
		color: #10b981;
	}

	.theme-btn-icon {
		font-size: 16px;
	}
</style>

<section class="settings-section">
	<h3 class="section-title">Database</h3>
	<div class="about-row">
		<span class="about-label">Location</span>
		<span class="about-value mono">~/.kusanaginokajiki/data.db</span>
	</div>
	<button class="action-btn danger" style="margin-top: 12px">Reset Database</button>
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

	.action-btn.danger {
		padding: 8px 16px;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.2);
		border-radius: 6px;
		color: #ef4444;
		font-family: inherit;
		font-size: 11px;
		font-weight: 600;
		cursor: pointer;
	}
</style>

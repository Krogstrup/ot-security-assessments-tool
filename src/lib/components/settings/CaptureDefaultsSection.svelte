<section class="settings-section">
	<h3 class="section-title">Capture Defaults</h3>
	<div class="setting-row">
		<label class="setting-label" for="promiscuous">Promiscuous Mode</label>
		<input type="checkbox" id="promiscuous" checked />
	</div>
	<div class="setting-row">
		<label class="setting-label" for="bpf">Default BPF Filter</label>
		<input
			type="text"
			id="bpf"
			class="setting-input"
			placeholder="e.g., port 502 or port 20000 or port 44818"
		/>
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

	.setting-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 8px 0;
	}

	.setting-label {
		font-size: 11px;
		color: var(--gm-text-secondary);
	}

	.setting-input {
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		padding: 6px 10px;
		color: var(--gm-text-primary);
		font-family: inherit;
		font-size: 11px;
		width: 300px;
		outline: none;
	}

	.setting-input:focus {
		border-color: var(--gm-border-active);
	}
</style>

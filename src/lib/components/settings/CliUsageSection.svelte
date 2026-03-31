<section class="settings-section">
	<h3 class="section-title">CLI Usage</h3>
	<div class="cli-examples">
		<code>kusanaginokajiki --open capture.pcap</code>
		<code>kusanaginokajiki --import-pcap /path/to/file.pcap</code>
		<code>kusanaginokajiki --open session.kkj</code>
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

	.cli-examples {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.cli-examples code {
		display: block;
		padding: 6px 10px;
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 4px;
		font-size: 10px;
		color: var(--gm-text-secondary);
		user-select: text;
		-webkit-user-select: text;
	}
</style>

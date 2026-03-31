<div class="empty-state">
	<div class="empty-icon">&#x2B21;</div>
	<h3>No Topology Data</h3>
	<p>Import a PCAP file or start a live capture to visualize network topology.</p>
	<p class="hint">Go to <strong>Capture</strong> &rarr; Import PCAP to get started.</p>
</div>

<style>
	.empty-state {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		text-align: center;
		color: var(--gm-text-muted);
		z-index: 1;
	}

	.empty-icon {
		font-size: 48px;
		margin-bottom: 12px;
		opacity: 0.3;
	}

	.empty-state h3 {
		font-size: 14px;
		font-weight: 600;
		color: var(--gm-text-secondary);
		margin: 0 0 8px 0;
	}

	.empty-state p {
		font-size: 12px;
		margin: 4px 0;
		line-height: 1.5;
	}

	.hint {
		margin-top: 12px !important;
		color: var(--gm-text-muted);
	}
</style>

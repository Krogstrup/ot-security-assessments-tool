<div class="physical-legend">
	<span class="legend-title">PORTS</span>
	<span class="legend-item">
		<span class="legend-dot" style="background: #475569"></span> Empty
	</span>
	<span class="legend-item">
		<span class="legend-dot" style="background: #3b82f6"></span> Has Devices
	</span>
	<span class="legend-item">
		<span class="legend-dot" style="background: #f59e0b"></span> CDP Link
	</span>
	<span class="legend-item">
		<span class="legend-dot" style="background: #8b5cf6; border-radius: 2px"></span> Trunk
	</span>
	<span class="legend-item">
		<span class="legend-dot" style="background: #374151; opacity: 0.5"></span> Shutdown
	</span>
	<span class="legend-item">
		<span class="legend-dot" style="background: #ef4444"></span> Highlighted
	</span>
</div>

<style>
	.physical-legend {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.5rem 1rem;
		background: var(--gm-bg-secondary);
		border-top: 1px solid var(--gm-border);
		flex-wrap: wrap;
		flex-shrink: 0;
		font-size: 0.75rem;
	}

	.legend-title {
		font-weight: 600;
		color: var(--gm-text-secondary);
		margin-right: 0.5rem;
	}

	.legend-item {
		display: flex;
		align-items: center;
		gap: 0.35rem;
		color: var(--gm-text-secondary);
	}

	.legend-dot {
		width: 12px;
		height: 12px;
		border-radius: 2px;
	}
</style>

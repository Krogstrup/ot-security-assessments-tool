<script lang="ts">
	import type { Dnp3Detail } from '$lib/types/deep-parse';

	interface Props {
		dnp3: Dnp3Detail;
	}

	let { dnp3 }: Props = $props();
</script>

<div class="detail-section">
	<h4 class="section-title dnp3-title">DNP3</h4>
	<div class="detail-row">
		<span class="detail-label">Role</span>
		<span class="detail-value role-badge">{dnp3.role}</span>
	</div>
	{#if dnp3.has_unsolicited}
		<div class="detail-row">
			<span class="detail-label">Unsolicited</span>
			<span class="detail-value warning">Yes (FC 130 detected)</span>
		</div>
	{/if}
	{#if dnp3.function_codes.length > 0}
		<div class="detail-subsection">
			<h5 class="subsection-title">Function Codes</h5>
			<div class="fc-list">
				{#each dnp3.function_codes as fc}
					<div class="fc-item" class:write={fc.is_write}>
						<span class="fc-code">FC {fc.code}</span>
						<span class="fc-name">{fc.name}</span>
						<span class="fc-count">{fc.count.toLocaleString()}</span>
					</div>
				{/each}
			</div>
		</div>
	{/if}
</div>

<style>
	.detail-section {
		margin-top: 1.5rem;
		padding: 1rem;
		border-radius: 6px;
		background: #0f172a;
		border: 1px solid #1e293b;
	}

	.section-title {
		font-size: 0.875rem;
		font-weight: 600;
		margin-bottom: 1rem;
		margin-top: 0;
	}

	.dnp3-title {
		color: var(--gm-dnp3, #10b981);
	}

	.detail-row {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.5rem 0;
	}

	.detail-label {
		font-weight: 600;
		color: #94a3b8;
		min-width: 100px;
	}

	.detail-value {
		color: #e2e8f0;
	}

	.role-badge {
		padding: 0.25rem 0.5rem;
		border-radius: 3px;
		background: #1e293b;
		color: #cbd5e1;
		font-weight: 500;
		font-size: 0.75rem;
	}

	.warning {
		color: #f59e0b;
		font-weight: 600;
	}

	.detail-subsection {
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px solid #1e293b;
	}

	.subsection-title {
		font-size: 0.8125rem;
		font-weight: 600;
		color: #cbd5e1;
		margin: 0 0 0.75rem 0;
	}

	.fc-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.fc-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.5rem;
		background: rgba(0, 0, 0, 0.2);
		border-radius: 3px;
		font-size: 0.8125rem;
		border-left: 2px solid #1e293b;
	}

	.fc-item.write {
		border-left-color: #ef4444;
		background: rgba(239, 68, 68, 0.1);
	}

	.fc-code {
		font-family: 'JetBrains Mono', monospace;
		font-weight: 600;
		color: #fbbf24;
		min-width: 50px;
	}

	.fc-name {
		color: #cbd5e1;
		flex: 1;
	}

	.fc-count {
		color: #94a3b8;
		font-size: 0.75rem;
	}
</style>

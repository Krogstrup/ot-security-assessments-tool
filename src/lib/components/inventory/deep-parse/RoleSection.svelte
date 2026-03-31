<script lang="ts">
	interface Props {
		title: string;
		titleClass: string;
		role: string;
		noteLabel?: string;
		noteValue?: string;
		noteClass?: string;
	}

	let { title, titleClass, role, noteLabel, noteValue, noteClass = '' }: Props = $props();
</script>

<div class="detail-section">
	<h4 class="section-title {titleClass}">{title}</h4>
	<div class="detail-row">
		<span class="detail-label">Role</span>
		<span class="detail-value role-badge">{role}</span>
	</div>
	{#if noteLabel && noteValue}
		<div class="detail-row">
			<span class="detail-label">{noteLabel}</span>
			<span class="detail-value {noteClass}">{noteValue}</span>
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

	.ethernet-ip-title {
		color: var(--gm-ethernet-ip, #8b5cf6);
	}

	.bacnet-title {
		color: var(--gm-bacnet, #06b6d4);
	}

	.iec104-title {
		color: #14b8a6;
	}

	.profinet-title {
		color: #6366f1;
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

	.finding {
		color: #ef4444;
		font-weight: 600;
	}

	.warning {
		color: #f59e0b;
		font-weight: 600;
	}
</style>

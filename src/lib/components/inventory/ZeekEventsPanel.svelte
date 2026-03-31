<script lang="ts">
	import type { DeviceZeekEvents } from '$lib/types';

	interface Props {
		zeekEvents: DeviceZeekEvents | null;
	}

	let { zeekEvents }: Props = $props();

	let expanded = $state(false);

	function toggleExpand() {
		expanded = !expanded;
	}
</script>

{#if zeekEvents}
	<div class="detail-section zeek-section">
		<div class="zeek-header">
			<h4 class="section-title zeek-title">&#128269; Zeek Events</h4>
			<button class="zeek-expand-btn" onclick={toggleExpand}>
				{expanded ? 'Hide' : 'Show'} samples
			</button>
		</div>
		<div class="zeek-badges">
			{#if zeekEvents.conn_log_entries > 0}
				<span class="zeek-badge">conn: {zeekEvents.conn_log_entries}</span>
			{/if}
			{#if zeekEvents.modbus_events > 0}
				<span class="zeek-badge zeek-ot">modbus: {zeekEvents.modbus_events}</span>
			{/if}
			{#if zeekEvents.dnp3_events > 0}
				<span class="zeek-badge zeek-ot">dnp3: {zeekEvents.dnp3_events}</span>
			{/if}
			{#if zeekEvents.dns_queries > 0}
				<span class="zeek-badge">dns: {zeekEvents.dns_queries}</span>
			{/if}
			{#if zeekEvents.http_requests > 0}
				<span class="zeek-badge">http: {zeekEvents.http_requests}</span>
			{/if}
			<span class="zeek-badge zeek-peers">peers: {zeekEvents.unique_peers}</span>
			{#if zeekEvents.alert_count > 0}
				<span class="zeek-badge zeek-alert">alerts: {zeekEvents.alert_count}</span>
			{/if}
		</div>
		{#if expanded && zeekEvents.sample_events.length > 0}
			<div class="zeek-events-list">
				{#each zeekEvents.sample_events as ev}
					<div class="zeek-event-row">
						<span class="zeek-ev-time">{ev.timestamp.slice(0, 19).replace('T', ' ')}</span>
						<span class="zeek-ev-type tag-{ev.log_type}">{ev.log_type}</span>
						<span class="zeek-ev-summary">{ev.summary}</span>
					</div>
				{/each}
			</div>
		{/if}
	</div>
{/if}

<style>
	.detail-section {
		margin-top: 1.5rem;
		padding: 1rem;
		border-radius: 6px;
		background: #0f172a;
		border: 1px solid #1e293b;
	}

	.zeek-section {
		background: rgba(99, 102, 241, 0.05);
		border: 1px solid rgba(99, 102, 241, 0.2);
	}

	.zeek-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.section-title {
		font-size: 0.875rem;
		font-weight: 600;
		color: #e2e8f0;
		margin: 0;
	}

	.zeek-title {
		color: #6366f1;
	}

	.zeek-expand-btn {
		padding: 0.25rem 0.75rem;
		font-size: 0.75rem;
		background: #1e293b;
		color: #cbd5e1;
		border: 1px solid #334155;
		border-radius: 3px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.zeek-expand-btn:hover {
		background: #334155;
		color: #e2e8f0;
	}

	.zeek-badges {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-bottom: 1rem;
	}

	.zeek-badge {
		padding: 0.35rem 0.75rem;
		border-radius: 12px;
		font-size: 0.75rem;
		font-weight: 500;
		background: #1e293b;
		color: #cbd5e1;
		border: 1px solid #334155;
	}

	.zeek-badge.zeek-ot {
		background: rgba(251, 146, 60, 0.1);
		color: #fb923c;
		border-color: rgba(251, 146, 60, 0.3);
	}

	.zeek-badge.zeek-peers {
		background: rgba(34, 197, 94, 0.1);
		color: #22c55e;
		border-color: rgba(34, 197, 94, 0.3);
	}

	.zeek-badge.zeek-alert {
		background: rgba(239, 68, 68, 0.1);
		color: #ef4444;
		border-color: rgba(239, 68, 68, 0.3);
	}

	.zeek-events-list {
		background: rgba(0, 0, 0, 0.2);
		border-radius: 4px;
		padding: 0.75rem;
		max-height: 300px;
		overflow-y: auto;
	}

	.zeek-event-row {
		display: flex;
		gap: 0.75rem;
		align-items: center;
		padding: 0.5rem 0;
		border-bottom: 1px solid #1e293b;
		font-size: 0.75rem;
	}

	.zeek-event-row:last-child {
		border-bottom: none;
	}

	.zeek-ev-time {
		color: #94a3b8;
		font-family: 'JetBrains Mono', monospace;
		min-width: 150px;
	}

	.zeek-ev-type {
		padding: 0.2rem 0.4rem;
		border-radius: 2px;
		font-weight: 600;
		background: #1e293b;
	}

	.tag-http {
		color: #3b82f6;
	}

	.tag-dns {
		color: #8b5cf6;
	}

	.tag-modbus {
		color: #f59e0b;
	}

	.zeek-ev-summary {
		color: #cbd5e1;
		flex: 1;
		overflow: hidden;
		text-overflow: ellipsis;
	}
</style>

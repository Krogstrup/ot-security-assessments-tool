<script lang="ts">
	import type { SessionInfo } from '$lib/types/operations';

	interface Props {
		sessions: SessionInfo[];
		selectedSessionId: string;
		isComparing: boolean;
		lastCompareTime: string | null;
		onSessionChange: (sessionId: string) => void;
		onCompare: () => void;
	}

	let {
		sessions,
		selectedSessionId,
		isComparing,
		lastCompareTime,
		onSessionChange,
		onCompare
	}: Props = $props();

	function handleSessionChange(event: Event) {
		onSessionChange((event.target as HTMLSelectElement).value);
	}
</script>

<div class="drift-header">
	<div class="header-left">
		<h2 class="header-title">Baseline Drift</h2>
		{#if lastCompareTime}
			<span class="last-run">Last compared: {lastCompareTime}</span>
		{/if}
	</div>
	<div class="header-controls">
		<select
			class="session-select"
			value={selectedSessionId}
			onchange={handleSessionChange}
			disabled={isComparing}
		>
			<option value="">-- Select baseline session --</option>
			{#each sessions as session}
				<option value={session.id}>
					{session.name} ({session.asset_count} assets, {session.connection_count} conns)
				</option>
			{/each}
		</select>
		<button
			class="compare-btn"
			onclick={onCompare}
			disabled={isComparing || !selectedSessionId}
		>
			{#if isComparing}
				Comparing...
			{:else}
				Compare
			{/if}
		</button>
	</div>
</div>

<style>
	.drift-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 16px 20px;
		border-bottom: 1px solid var(--gm-border);
		flex-shrink: 0;
		gap: 16px;
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: 16px;
		flex-shrink: 0;
	}

	.header-title {
		font-size: 15px;
		font-weight: 600;
		color: var(--gm-text-primary);
		margin: 0;
	}

	.last-run {
		font-size: 10px;
		color: var(--gm-text-muted);
	}

	.header-controls {
		display: flex;
		align-items: center;
		gap: 10px;
		flex-shrink: 1;
		min-width: 0;
	}

	.session-select {
		padding: 7px 10px;
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 6px;
		color: var(--gm-text-primary);
		font-family: inherit;
		font-size: 11px;
		min-width: 260px;
		max-width: 400px;
		cursor: pointer;
	}

	.session-select:focus {
		outline: none;
		border-color: #10b981;
	}

	.session-select:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.compare-btn {
		padding: 8px 20px;
		background: linear-gradient(135deg, #10b981, #059669);
		border: none;
		border-radius: 6px;
		color: #0a0e17;
		font-family: inherit;
		font-size: 12px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.15s;
		white-space: nowrap;
	}

	.compare-btn:hover:not(:disabled) {
		filter: brightness(1.1);
	}

	.compare-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>

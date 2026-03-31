<script lang="ts">
	import { onMount } from 'svelte';
	import type { BaselineDiff } from '$lib/types';
	import { sessions, baselineDiff } from '$lib/stores';
	import { listSessions, compareSessions } from '$lib/api';
	import BaselineCompareHeader from './baseline/BaselineCompareHeader.svelte';
	import BaselineDriftResults from './baseline/BaselineDriftResults.svelte';

	let selectedSessionId = $state<string>('');
	let isComparing = $state(false);
	let error = $state<string | null>(null);
	let lastCompareTime = $state<string | null>(null);

	onMount(async () => {
		try {
			const list = await listSessions();
			sessions.set(list);
		} catch {
			// No sessions available
		}
	});

	async function handleCompare() {
		if (!selectedSessionId) return;
		isComparing = true;
		error = null;
		try {
			const result: BaselineDiff = await compareSessions(selectedSessionId);
			baselineDiff.set(result);
			lastCompareTime = new Date().toLocaleTimeString();
		} catch (e) {
			error = String(e);
		} finally {
			isComparing = false;
		}
	}
</script>

<div class="drift-container">
	<BaselineCompareHeader
		sessions={$sessions}
		{selectedSessionId}
		{isComparing}
		{lastCompareTime}
		onSessionChange={(sessionId) => (selectedSessionId = sessionId)}
		onCompare={handleCompare}
	/>

	{#if error}
		<div class="error-banner">{error}</div>
	{/if}

	{#if $sessions.length === 0}
		<div class="empty-state">
			<div class="empty-icon">~</div>
			<p>No saved sessions available. Save a session first to use as a baseline.</p>
		</div>
	{:else if !$baselineDiff}
		<div class="empty-state">
			<div class="empty-icon">~</div>
			<p>Select a baseline session and click "Compare" to detect drift from the saved baseline.</p>
		</div>
	{:else}
		<BaselineDriftResults diff={$baselineDiff} />
	{/if}
</div>

<style>
	.drift-container {
		height: 100%;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.error-banner {
		margin: 12px 20px;
		padding: 10px 14px;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 6px;
		color: #ef4444;
		font-size: 11px;
	}

	.empty-state {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		color: var(--gm-text-muted);
		gap: 12px;
	}

	.empty-icon {
		font-size: 36px;
		opacity: 0.4;
	}

	.empty-state p {
		font-size: 12px;
		text-align: center;
		max-width: 400px;
		line-height: 1.6;
	}
</style>

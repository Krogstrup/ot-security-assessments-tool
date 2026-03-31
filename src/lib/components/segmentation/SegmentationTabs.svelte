<script lang="ts">
	export type SubTab = 'groups' | 'zones' | 'matrix' | 'enforcement' | 'simulation';

	interface Props {
		activeTab: SubTab;
		onTabChange: (tab: SubTab) => void;
	}

	let { activeTab, onTabChange }: Props = $props();
</script>

<div class="sub-tabs">
	{#each [
		{ id: 'groups', label: 'Policy Groups' },
		{ id: 'zones', label: 'Zones & Conduits' },
		{ id: 'matrix', label: 'Comm Matrix' },
		{ id: 'enforcement', label: 'Enforcement' },
		{ id: 'simulation', label: 'Simulation' }
	] as tab}
		<button
			class="sub-tab"
			class:active={activeTab === tab.id}
			onclick={() => onTabChange(tab.id as SubTab)}
		>
			{tab.label}
		</button>
	{/each}
</div>

<style>
	.sub-tabs {
		display: flex;
		gap: 0.25rem;
		border-bottom: 1px solid var(--border, #333);
		margin-bottom: 1.5rem;
		flex-shrink: 0;
	}

	.sub-tab {
		background: none;
		border: none;
		border-bottom: 2px solid transparent;
		padding: 0.5rem 1rem;
		cursor: pointer;
		color: var(--text-muted, #888);
		font-size: 0.9rem;
		margin-bottom: -1px;
	}

	.sub-tab.active {
		color: var(--text, #eee);
		border-bottom-color: var(--accent, #0ea5e9);
	}
</style>

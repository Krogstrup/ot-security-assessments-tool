<script lang="ts">
	import type { GroupingMode } from '$lib/types/topology';

	interface GroupingOption {
		mode: GroupingMode;
		label: string;
	}

	interface Props {
		show: boolean;
		x: number;
		y: number;
		hasNode: boolean;
		wiresharkAvailable: boolean;
		groupSubmenu: boolean;
		groupingOptions: GroupingOption[];
		selectedGroupingMode: GroupingMode;
		onWatch: () => void;
		onShowInPhysical: () => void;
		onOpenInWireshark: () => void;
		onCreateFilteredView: () => void;
		onToggleGroupSubmenu: () => void;
		onGroupBy: (mode: GroupingMode) => void;
		onClose: () => void;
	}

	let {
		show,
		x,
		y,
		hasNode,
		wiresharkAvailable,
		groupSubmenu,
		groupingOptions,
		selectedGroupingMode,
		onWatch,
		onShowInPhysical,
		onOpenInWireshark,
		onCreateFilteredView,
		onToggleGroupSubmenu,
		onGroupBy,
		onClose
	}: Props = $props();

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			onClose();
		}
	}
</script>

{#if show}
	<div
		class="context-menu"
		role="menu"
		tabindex="0"
		style={`left: ${x}px; top: ${y}px;`}
		onclick={(event) => event.stopPropagation()}
		oncontextmenu={(event) => event.preventDefault()}
		onkeydown={handleKeydown}
	>
		{#if hasNode}
			<button class="ctx-item" role="menuitem" onclick={onWatch}>
				Watch Node
			</button>
			<button class="ctx-item" role="menuitem" onclick={onShowInPhysical}>
				Show in Physical
			</button>
			{#if wiresharkAvailable}
				<button class="ctx-item" role="menuitem" onclick={onOpenInWireshark}>
					Open in Wireshark
				</button>
			{/if}
			<div class="ctx-sep"></div>
		{/if}
		<button class="ctx-item" role="menuitem" onclick={onCreateFilteredView}>
			Create Filtered View
		</button>
		<div class="ctx-sep"></div>
		<button
			class="ctx-item has-sub"
			role="menuitem"
			onclick={(event) => {
				event.stopPropagation();
				onToggleGroupSubmenu();
			}}
		>
			Group By &rsaquo;
		</button>
		{#if groupSubmenu}
			<div class="ctx-submenu">
				{#each groupingOptions as opt}
					<button
						class="ctx-item"
						class:ctx-active={selectedGroupingMode === opt.mode}
						role="menuitem"
						onclick={() => onGroupBy(opt.mode)}
					>
						{opt.label}
					</button>
				{/each}
			</div>
		{/if}
	</div>
{/if}

<style>
	.context-menu {
		position: fixed;
		z-index: 100;
		min-width: 180px;
		background: var(--gm-bg-panel);
		border: 1px solid var(--gm-border);
		border-radius: 6px;
		padding: 4px 0;
		box-shadow: 0 8px 24px rgba(0, 0, 0, 0.5);
	}

	.ctx-item {
		display: block;
		width: 100%;
		padding: 7px 14px;
		background: none;
		border: none;
		color: var(--gm-text-secondary);
		font-family: inherit;
		font-size: 11px;
		text-align: left;
		cursor: pointer;
		transition: background 0.1s;
	}

	.ctx-item:hover {
		background: var(--gm-bg-hover);
		color: var(--gm-text-primary);
	}

	.ctx-item.has-sub {
		display: flex;
		justify-content: space-between;
	}

	.ctx-item.ctx-active {
		color: #10b981;
	}

	.ctx-sep {
		height: 1px;
		background: var(--gm-border);
		margin: 4px 0;
	}

	.ctx-submenu {
		border-top: 1px solid var(--gm-border);
		padding: 2px 0;
		margin-top: 2px;
	}
</style>

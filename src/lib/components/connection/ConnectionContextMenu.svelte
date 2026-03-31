<script lang="ts">
	interface Props {
		show: boolean;
		x: number;
		y: number;
		wiresharkAvailable: boolean;
		onViewFrames: () => void;
		onOpenInWireshark: () => void;
		onClose: () => void;
	}

	let { show, x, y, wiresharkAvailable, onViewFrames, onOpenInWireshark, onClose }: Props = $props();

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			onClose();
		}
	}
</script>

{#if show}
	<div
		class="conn-ctx-menu"
		role="menu"
		tabindex="0"
		style={`left: ${x}px; top: ${y}px;`}
		onclick={(event) => event.stopPropagation()}
		onkeydown={handleKeydown}
	>
		<button class="conn-ctx-item" role="menuitem" onclick={onViewFrames}>
			View Frames
		</button>
		{#if wiresharkAvailable}
			<button class="conn-ctx-item" role="menuitem" onclick={onOpenInWireshark}>
				Open in Wireshark
			</button>
		{/if}
	</div>
{/if}

<style>
	.conn-ctx-menu {
		position: fixed;
		z-index: 9999;
		background: var(--gm-bg-secondary);
		border: 1px solid var(--gm-border);
		border-radius: 6px;
		padding: 4px 0;
		min-width: 160px;
		box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
	}

	.conn-ctx-item {
		display: block;
		width: 100%;
		padding: 6px 12px;
		background: transparent;
		border: none;
		color: var(--gm-text-primary);
		font-family: inherit;
		font-size: 11px;
		text-align: left;
		cursor: pointer;
		transition: background 0.1s;
	}

	.conn-ctx-item:hover {
		background: var(--gm-bg-hover);
	}
</style>

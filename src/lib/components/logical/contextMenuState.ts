export interface LogicalContextMenuState {
	x: number;
	y: number;
	nodeId: string | null;
	show: boolean;
}

export function closedContextMenu(): LogicalContextMenuState {
	return {
		x: 0,
		y: 0,
		nodeId: null,
		show: false
	};
}

export function hideContextMenu(): LogicalContextMenuState {
	return {
		...closedContextMenu()
	};
}

export function openNodeContextMenu(
	position: { x: number; y: number },
	containerRect: DOMRect,
	nodeId: string
): LogicalContextMenuState {
	return {
		x: position.x + containerRect.left,
		y: position.y + containerRect.top,
		nodeId,
		show: true
	};
}

export function openCanvasContextMenu(
	position: { x: number; y: number },
	containerRect: DOMRect
): LogicalContextMenuState {
	return {
		x: position.x + containerRect.left,
		y: position.y + containerRect.top,
		nodeId: null,
		show: true
	};
}

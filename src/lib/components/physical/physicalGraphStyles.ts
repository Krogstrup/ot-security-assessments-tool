export const physicalGraphStyles: any[] = [
	{
		selector: 'node.switch',
		style: {
			'background-color': 'rgba(16, 185, 129, 0.08)',
			'background-opacity': 0.6,
			'border-color': '#10b981',
			'border-width': 2,
			'border-style': 'solid' as any,
			label: 'data(label)',
			color: '#e2e8f0',
			'font-size': '11px',
			'font-family': 'JetBrains Mono, monospace',
			'font-weight': '600' as any,
			'text-valign': 'top',
			'text-halign': 'center',
			'text-margin-y': -6,
			padding: '20px',
			shape: 'roundrectangle'
		}
	},
	{
		selector: 'node.port',
		style: {
			'background-color': '#1e293b',
			'border-color': 'data(color)',
			'border-width': 1.5,
			label: 'data(label)',
			color: '#94a3b8',
			'font-size': '8px',
			'font-family': 'JetBrains Mono, monospace',
			'text-valign': 'bottom',
			'text-margin-y': 4,
			width: 20,
			height: 20,
			'text-wrap': 'wrap' as any,
			'text-max-width': '80px'
		}
	},
	{
		selector: 'node.port.has-device',
		style: {
			'background-color': '#0f2942',
			'border-color': '#3b82f6',
			'border-width': 2,
			width: 24,
			height: 24
		}
	},
	{
		selector: 'node.port.has-cdp',
		style: {
			'border-color': '#f59e0b',
			'border-width': 2
		}
	},
	{
		selector: 'node.port.shutdown',
		style: {
			'background-color': '#1a1a2e',
			'border-color': '#374151',
			'border-style': 'dashed' as any,
			opacity: 0.5
		}
	},
	{
		selector: 'node.port.trunk',
		style: {
			'border-color': '#8b5cf6',
			shape: 'diamond'
		}
	},
	{
		selector: 'node.port.highlighted',
		style: {
			'background-color': '#1e3a5f',
			'border-color': '#ef4444',
			'border-width': 3,
			width: 28,
			height: 28
		}
	},
	{
		selector: 'node:selected',
		style: {
			'border-color': '#3b82f6',
			'border-width': 3
		}
	},
	{
		selector: 'edge.cdp-link',
		style: {
			width: 2.5,
			'line-color': '#f59e0b',
			'line-style': 'solid' as any,
			'target-arrow-shape': 'none',
			'curve-style': 'bezier',
			opacity: 0.8,
			label: 'data(label)',
			color: '#f59e0b',
			'font-size': '8px',
			'text-rotation': 'autorotate' as any,
			'text-background-color': '#0a0e17',
			'text-background-opacity': 0.9,
			'text-background-padding': '2px' as any
		}
	}
];

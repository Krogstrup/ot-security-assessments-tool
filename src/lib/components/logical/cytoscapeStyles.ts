export const logicalCytoscapeStyle: any[] = [
	{
		selector: 'node.compound',
		style: {
			'background-color': 'rgba(30, 41, 59, 0.4)',
			'background-opacity': 0.4,
			'border-color': '#334155',
			'border-width': 1,
			'border-style': 'dashed' as any,
			label: 'data(label)',
			color: '#64748b',
			'font-size': '9px',
			'font-family': 'JetBrains Mono, monospace',
			'text-valign': 'top',
			'text-halign': 'center',
			'text-margin-y': -4,
			padding: '16px',
			shape: 'roundrectangle'
		}
	},
	{
		selector: 'node.device',
		style: {
			'background-color': '#1e293b',
			'border-color': 'data(color)',
			'border-width': 2,
			label: 'data(label)',
			color: '#e2e8f0',
			'font-size': '10px',
			'font-family': 'JetBrains Mono, monospace',
			'text-valign': 'bottom',
			'text-margin-y': 6,
			'text-wrap': 'wrap' as any,
			'text-max-width': '120px',
			width: 32,
			height: 32
		}
	},
	{
		selector: 'node.device.ot',
		style: {
			'background-color': '#0f1d2e',
			'border-width': 2.5
		}
	},
	{
		selector: 'node.device:selected',
		style: {
			'border-color': '#3b82f6',
			'border-width': 3,
			'background-color': '#1e3a5f'
		}
	},
	{
		selector: 'node.drift-new',
		style: {
			'border-color': '#10b981',
			'border-width': 3,
			'border-style': 'double' as any
		}
	},
	{
		selector: 'node.drift-changed',
		style: {
			'border-color': '#f59e0b',
			'border-width': 3,
			'border-style': 'double' as any
		}
	},
	{
		selector: 'edge',
		style: {
			width: 'data(weight)',
			'line-color': 'data(color)',
			'target-arrow-color': 'data(color)',
			'target-arrow-shape': 'triangle',
			'arrow-scale': 0.8,
			'curve-style': 'bezier',
			opacity: 0.7
		}
	},
	{
		selector: 'edge.bidirectional',
		style: {
			'source-arrow-color': 'data(color)',
			'source-arrow-shape': 'triangle',
			'target-arrow-shape': 'triangle'
		}
	},
	{
		selector: 'edge.cross-zone',
		style: {
			'line-color': '#ef4444',
			'target-arrow-color': '#ef4444',
			'source-arrow-color': '#ef4444',
			'line-style': 'dashed' as any,
			width: 2,
			opacity: 0.85
		}
	},
	{
		selector: 'edge:selected',
		style: {
			'line-color': '#3b82f6',
			'target-arrow-color': '#3b82f6',
			'source-arrow-color': '#3b82f6',
			opacity: 1
		}
	}
];

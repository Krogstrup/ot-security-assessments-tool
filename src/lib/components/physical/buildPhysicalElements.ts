import type { PhysicalTopology } from '$lib/types/operations';

function shortenName(name: string): string {
	const prefixes: [string, string][] = [
		['TenGigabitEthernet', 'Te'],
		['GigabitEthernet', 'Gi'],
		['FastEthernet', 'Fa'],
		['Ethernet', 'Et'],
		['Loopback', 'Lo'],
		['Tunnel', 'Tu'],
		['Port-channel', 'Po'],
		['Vlan', 'Vl']
	];
	for (const [longName, shortName] of prefixes) {
		if (name.startsWith(longName)) {
			return shortName + name.slice(longName.length);
		}
	}
	return name;
}

function isVisiblePortName(name: string): boolean {
	const lower = name.toLowerCase();
	return (
		lower.startsWith('gigabitethernet') ||
		lower.startsWith('fastethernet') ||
		lower.startsWith('tengigabitethernet') ||
		lower.startsWith('port-channel')
	);
}

export function buildPhysicalElements(topology: PhysicalTopology, highlightIp: string | null): any[] {
	const elements: any[] = [];

	for (const sw of topology.switches) {
		const switchId = `sw-${sw.hostname}`;
		let switchLabel = sw.hostname;
		if (sw.management_ip) {
			switchLabel += `\n${sw.management_ip}`;
		}

		elements.push({
			group: 'nodes',
			data: {
				id: switchId,
				label: switchLabel,
				hostname: sw.hostname
			},
			classes: 'switch'
		});

		const visiblePorts = sw.ports.filter(
			(port) => isVisiblePortName(port.name) || (port.name.toLowerCase().startsWith('vlan') && port.ip_address)
		);

		for (const port of visiblePorts) {
			const portId = `${switchId}-${port.short_name}`;
			const hasDevice = port.mac_addresses.length > 0 || port.ip_addresses.length > 0;
			const hasCdp = port.cdp_neighbor !== null;
			const isTrunk = port.mode === 'trunk';
			const isHighlighted = highlightIp !== null && port.ip_addresses.includes(highlightIp);

			let classes = 'port';
			if (hasDevice) classes += ' has-device';
			if (hasCdp) classes += ' has-cdp';
			if (port.shutdown) classes += ' shutdown';
			if (isTrunk) classes += ' trunk';
			if (isHighlighted) classes += ' highlighted';

			let label = port.short_name;
			if (port.description) label += `\n${port.description}`;
			if (port.ip_addresses.length > 0) label += `\n${port.ip_addresses[0]}`;

			let color = '#475569';
			if (isHighlighted) color = '#ef4444';
			else if (hasCdp) color = '#f59e0b';
			else if (hasDevice) color = '#3b82f6';
			else if (isTrunk) color = '#8b5cf6';
			else if (port.shutdown) color = '#374151';

			elements.push({
				group: 'nodes',
				data: {
					id: portId,
					label,
					parent: switchId,
					portName: port.name,
					switchHostname: sw.hostname,
					color,
					vlans: port.vlans.join(', '),
					macCount: port.mac_addresses.length,
					ipCount: port.ip_addresses.length
				},
				classes
			});
		}
	}

	const addedLinks = new Set<string>();
	for (const link of topology.links) {
		const key = [link.src_switch, link.dst_switch].sort().join('---');
		if (addedLinks.has(key)) continue;
		addedLinks.add(key);

		const srcPortId = `sw-${link.src_switch}-${shortenName(link.src_port)}`;
		const dstPortId = `sw-${link.dst_switch}-${shortenName(link.dst_port)}`;

		const srcExists = elements.some((element) => element.data?.id === srcPortId);
		const dstExists = elements.some((element) => element.data?.id === dstPortId);

		if (srcExists && dstExists) {
			elements.push({
				group: 'edges',
				data: {
					id: `link-${link.src_switch}-${link.dst_switch}`,
					source: srcPortId,
					target: dstPortId,
					label: 'CDP'
				},
				classes: 'cdp-link'
			});
		}
	}

	return elements;
}

/**
 * Shared display constants for the UI.
 *
 * Single source of truth for device type labels, colors, Purdue level labels,
 * and confidence scale labels. Import from here instead of defining local
 * const objects in individual components.
 */

import type { DeviceType } from '$lib/types/assets';

// ── Device type display ───────────────────────────────────────────────────────

export const DEVICE_TYPE_LABELS: Record<DeviceType, string> = {
	plc: 'PLC',
	rtu: 'RTU',
	hmi: 'HMI',
	historian: 'Historian',
	engineering_workstation: 'Eng. WS',
	scada_server: 'SCADA Server',
	it_device: 'IT Device',
	unknown: 'Unknown'
};

export const DEVICE_TYPE_COLORS: Record<DeviceType, string> = {
	plc: '#f59e0b',
	rtu: '#10b981',
	hmi: '#3b82f6',
	historian: '#8b5cf6',
	engineering_workstation: '#06b6d4',
	scada_server: '#ec4899',
	it_device: '#475569',
	unknown: '#64748b'
};

export const DEVICE_TYPE_OPTIONS: DeviceType[] = [
	'plc',
	'rtu',
	'hmi',
	'historian',
	'engineering_workstation',
	'scada_server',
	'it_device',
	'unknown'
];

// ── Purdue Model levels ───────────────────────────────────────────────────────

export const PURDUE_LABELS: Record<number, string> = {
	0: 'L0 — Process',
	1: 'L1 — Basic Control',
	2: 'L2 — Supervisory',
	3: 'L3 — Site Operations',
	4: 'L4 — Enterprise IT',
	5: 'L5 — Enterprise Network'
};

// ── Device identification confidence scale (0–5) ──────────────────────────────

export const CONFIDENCE_LABELS: Record<number, string> = {
	0: '—',
	1: 'Port',
	2: 'Pattern',
	3: 'MAC OUI',
	4: 'Payload',
	5: 'Deep'
};

export const CONFIDENCE_COLORS: Record<number, string> = {
	5: 'var(--gm-confidence-5, #10b981)',
	4: 'var(--gm-confidence-4, #3b82f6)',
	3: 'var(--gm-confidence-3, #f59e0b)',
	2: 'var(--gm-confidence-2, #f97316)',
	1: 'var(--gm-confidence-1, #ef4444)',
	0: '#64748b'
};

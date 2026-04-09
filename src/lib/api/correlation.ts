/**
 * Alert correlation and live ATT&CK alert events.
 */

import type { LiveAttackAlert } from '$lib/types/analysis';
import { z } from 'zod';
import { httpValidated } from './core';
import type { CorrelatedAlert } from '$lib/types/analysis';

export async function getCorrelatedAlerts(): Promise<CorrelatedAlert[]> {
	return httpValidated(z.unknown(), '/api/v1/correlation/alerts') as Promise<CorrelatedAlert[]>;
}

export async function getAlertsForIp(ip: string): Promise<CorrelatedAlert[]> {
	return httpValidated(z.unknown(), `/api/v1/correlation/alerts/${encodeURIComponent(ip)}`) as Promise<CorrelatedAlert[]>;
}

export async function clearAlerts(): Promise<void> {
	await httpValidated(z.unknown(), '/api/v1/correlation/alerts', { method: 'DELETE' });
}

export async function onLiveAttackAlert(
	_callback: (alert: LiveAttackAlert) => void
): Promise<() => void> {
	// No SSE equivalent for live attack alerts in web runtime.
	return () => {};
}

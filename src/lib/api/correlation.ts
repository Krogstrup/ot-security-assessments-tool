/**
 * Alert correlation and live ATT&CK alert events.
 */

import type { LiveAttackAlert } from '$lib/types/analysis';
import { listen } from '@tauri-apps/api/event';
import { invokeCompat, isTauriRuntime } from './core';
import type { CorrelatedAlert } from '$lib/types/analysis';

export async function getCorrelatedAlerts(): Promise<CorrelatedAlert[]> {
	return invokeCompat<CorrelatedAlert[]>('get_correlated_alerts');
}

export async function getAlertsForIp(ip: string): Promise<CorrelatedAlert[]> {
	return invokeCompat<CorrelatedAlert[]>('get_alerts_for_ip', { ip });
}

export async function clearAlerts(): Promise<void> {
	return invokeCompat('clear_alerts');
}

export async function onLiveAttackAlert(
	callback: (alert: LiveAttackAlert) => void
): Promise<() => void> {
	if (!isTauriRuntime()) {
		return () => {};
	}
	return listen<LiveAttackAlert>('live_attack_alert', (event) => callback(event.payload));
}


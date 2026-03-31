export function formatTimelineTimestamp(date: Date): string {
	return date.toISOString().replace('T', ' ').replace(/\.\d{3}Z$/, 'Z');
}

/** Compact time for slider labels. */
export function formatTimelineCompact(date: Date | null): string {
	if (!date) return '--';
	const hours = date.getUTCHours().toString().padStart(2, '0');
	const mins = date.getUTCMinutes().toString().padStart(2, '0');
	const secs = date.getUTCSeconds().toString().padStart(2, '0');
	return `${hours}:${mins}:${secs}`;
}

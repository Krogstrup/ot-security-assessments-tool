/**
 * Core HTTP utilities shared by all API modules.
 *
 * All API calls go through the HTTP server; there is no Tauri/desktop path.
 */

import type { AppError } from '$lib/types';

/**
 * Type guard: checks if a thrown value is a structured AppError.
 */
export function isAppError(err: unknown): err is AppError {
	return (
		typeof err === 'object' &&
		err !== null &&
		'code' in err &&
		typeof (err as Record<string, unknown>).code === 'string'
	);
}

function getHeadlessApiBase(): string {
	const raw = (import.meta.env.VITE_HEADLESS_API_BASE as string | undefined) ?? '';
	return raw.endsWith('/') ? raw.slice(0, -1) : raw;
}

export async function httpJson<T>(path: string, init?: RequestInit): Promise<T> {
	const base = getHeadlessApiBase();
	const url = `${base}${path}`;
	const response = await fetch(url, init);
	if (!response.ok) {
		let message = `${response.status} ${response.statusText}`;
		try {
			const body = (await response.json()) as { error?: string };
			if (body.error) message = body.error;
		} catch {
			// ignore JSON parse errors
		}
		throw new Error(message);
	}
	return response.json() as Promise<T>;
}

/**
 * Fetch a resource endpoint and validate the response against a Zod schema.
 *
 * Use this for versioned resource endpoints (e.g. GET /api/v1/projects).
 */
export async function httpValidated<T>(
	schema: import('zod').ZodSchema<T>,
	path: string,
	init?: RequestInit
): Promise<T> {
	const raw = await httpJson<unknown>(path, init);
	return schema.parse(raw);
}

/**
 * Core IPC utilities shared by all API modules.
 *
 * Provides typed invocation, HTTP fallback, and error handling.
 */

import { invoke } from '@tauri-apps/api/core';
import type { AppError } from '/types';

function isBrowserRuntime(): boolean {
	return typeof window !== 'undefined';
}

export function isTauriRuntime(): boolean {
	if (!isBrowserRuntime()) return false;
	return '__TAURI_INTERNALS__' in window;
}

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

export async function invokeCompat<T>(command: string, args?: Record<string, unknown>): Promise<T> {
	if (isTauriRuntime()) {
		return invoke<T>(command, args);
	}
	return httpJson<T>(`/api/invoke/${command}`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(args ?? {})
	});
}

/**
 * Invoke a command and validate the response against a Zod schema.
 *
 * Throws ZodError if response does not match, surfacing field mismatches
 * as observable failures instead of silent undefined values.
 */
export async function invokeValidated<T>(
	schema: import('zod').ZodSchema<T>,
	command: string,
	args?: Record<string, unknown>
): Promise<T> {
	const raw = await invokeCompat<unknown>(command, args);
	return schema.parse(raw);
}

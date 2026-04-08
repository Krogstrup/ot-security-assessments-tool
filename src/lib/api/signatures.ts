/**
 * Signature management and signature testing.
 */

import type { SignatureTestResult } from '$lib/types/signatures';
import { httpJson } from './core';
import type { SignatureSummary } from '$lib/types';

export async function getSignatures(): Promise<SignatureSummary> {
	return httpJson<SignatureSummary>('/api/v1/signatures');
}

export async function reloadSignatures(): Promise<number> {
	return httpJson<number>('/api/v1/signatures/reload', { method: 'POST' });
}

export async function testSignature(yaml: string): Promise<SignatureTestResult> {
	return httpJson<SignatureTestResult>('/api/v1/signatures/test', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ yaml })
	});
}

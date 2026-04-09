/**
 * Signature management and signature testing.
 */

import type { SignatureTestResult } from '$lib/types/signatures';
import { z } from 'zod';
import { httpValidated } from './core';
import type { SignatureSummary } from '$lib/types';

export async function getSignatures(): Promise<SignatureSummary> {
	return httpValidated(z.unknown(), '/api/v1/signatures') as Promise<SignatureSummary>;
}

export async function reloadSignatures(): Promise<number> {
	return httpValidated(z.number(), '/api/v1/signatures/reload', { method: 'POST' });
}

export async function testSignature(yaml: string): Promise<SignatureTestResult> {
	return httpValidated(z.unknown(), '/api/v1/signatures/test', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ yaml })
	}) as Promise<SignatureTestResult>;
}

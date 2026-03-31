/**
 * Signature management and signature testing.
 */

import type { SignatureTestResult } from '$lib/types/signatures';
import { invokeCompat } from './core';
import type { SignatureSummary } from '$lib/types';

export async function getSignatures(): Promise<SignatureSummary> {
	return invokeCompat<SignatureSummary>('get_signatures');
}

export async function reloadSignatures(): Promise<number> {
	return invokeCompat<number>('reload_signatures');
}

export async function testSignature(yaml: string): Promise<SignatureTestResult> {
	return invokeCompat<SignatureTestResult>('test_signature', { yaml });
}


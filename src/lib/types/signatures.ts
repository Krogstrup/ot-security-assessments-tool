/** Information about a loaded signature. */
export interface SignatureInfo {
	name: string;
	description: string;
	vendor: string | null;
	product_family: string | null;
	protocol: string | null;
	confidence: number;
	role: string | null;
	device_type: string | null;
	filter_count: number;
}

/** Summary payload for loaded signatures. */
export interface SignatureSummary {
	total_count: number;
	signatures: SignatureInfo[];
}

/** Result of testing a signature against loaded data. */
export interface SignatureTestResult {
	match_count: number;
	matches: SignatureTestMatch[];
}

export interface SignatureTestMatch {
	packet_index: number;
	src_ip: string;
	dst_ip: string;
	src_port: number;
	dst_port: number;
	confidence: number;
}

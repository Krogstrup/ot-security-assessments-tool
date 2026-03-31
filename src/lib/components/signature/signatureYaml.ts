import type { SignatureInfo } from '$lib/types/signatures';

export const defaultSignatureYaml = `name: "my_custom_signature"
description: "Description of what this signature matches"
vendor: "Vendor Name"
product_family: "Product Family"
protocol: modbus
filters:
  - field: tcp.dst_port
    value: 502
  - field: payload
    pattern: "\\\\x00\\\\x00"
confidence: 4
role: slave
device_type: plc
payloads: []
`;

export function signatureToYaml(sig: SignatureInfo): string {
	return `name: "${sig.name}"
description: "${sig.description}"
${sig.vendor ? `vendor: "${sig.vendor}"` : '# vendor: null'}
${sig.product_family ? `product_family: "${sig.product_family}"` : '# product_family: null'}
${sig.protocol ? `protocol: ${sig.protocol}` : '# protocol: null'}
confidence: ${sig.confidence}
${sig.role ? `role: ${sig.role}` : '# role: null'}
${sig.device_type ? `device_type: ${sig.device_type}` : '# device_type: null'}
filters: []
payloads: []
`;
}

export interface DialogFilter {
	name: string;
	extensions: string[];
}

export interface OpenDialogOptions {
	title?: string;
	multiple?: boolean;
	defaultPath?: string;
	filters?: DialogFilter[];
}

export interface SaveDialogOptions {
	title?: string;
	defaultPath?: string;
	filters?: DialogFilter[];
}

function promptLabel(kind: 'open' | 'save', options?: { title?: string; multiple?: boolean }): string {
	const title = options?.title ? `${options.title}: ` : '';
	if (kind === 'save') {
		return `${title}enter server-side output path`;
	}
	if (options?.multiple) {
		return `${title}enter one or more server-side paths (comma-separated)`;
	}
	return `${title}enter server-side path`;
}

export async function openPathDialog(options?: OpenDialogOptions): Promise<string | string[] | null> {
	const raw = window.prompt(promptLabel('open', options));
	if (!raw) return null;
	if (options?.multiple) {
		const paths = raw
			.split(',')
			.map((v) => v.trim())
			.filter(Boolean);
		return paths.length > 0 ? paths : null;
	}
	return raw.trim() || null;
}

export async function savePathDialog(options?: SaveDialogOptions): Promise<string | null> {
	const raw = window.prompt(
		promptLabel('save', options) + (options?.defaultPath ? ` (default: ${options.defaultPath})` : '')
	);
	if (raw === null) return null;
	const value = raw.trim();
	if (value) return value;
	return options?.defaultPath ?? null;
}

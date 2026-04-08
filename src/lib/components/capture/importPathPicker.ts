import type { HeadlessImportKind } from '$lib/api/system';

export interface FileDialogFilter {
	name: string;
	extensions: string[];
}

export interface ImportPathPickerOptions {
	kind: HeadlessImportKind;
	serverTitle: string;
	dialogTitle: string;
	multiple: boolean;
	filters: FileDialogFilter[];
}

export async function pickImportPaths(
	options: ImportPathPickerOptions,
	openServerPicker: (
		kind: HeadlessImportKind,
		title: string,
		multiple: boolean
	) => Promise<string[] | null>
): Promise<string[]> {
	return (await openServerPicker(options.kind, options.serverTitle, options.multiple)) ?? [];
}

import { isTauriRuntime } from '$lib/api/core';
import type { HeadlessImportKind } from '$lib/api/system';
import { openPathDialog } from '$lib/utils/dialog';

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
	if (!isTauriRuntime()) {
		return (await openServerPicker(options.kind, options.serverTitle, options.multiple)) ?? [];
	}

	const selected = await openPathDialog({
		title: options.dialogTitle,
		multiple: options.multiple,
		filters: options.filters
	});
	if (!selected) return [];
	return Array.isArray(selected) ? selected : [selected];
}

export interface ServerPickerFile {
	name: string;
	path: string;
	size_bytes: number;
}

export interface ServerPickerState {
	show: boolean;
	loading: boolean;
	error: string;
	baseDir: string;
	files: ServerPickerFile[];
	selectedPaths: string[];
	listLimit: number;
	truncated: boolean;
	title: string;
	allowMultiple: boolean;
	resolve: ((paths: string[] | null) => void) | null;
}

export function createServerPickerState(): ServerPickerState {
	return {
		show: false,
		loading: false,
		error: '',
		baseDir: '',
		files: [],
		selectedPaths: [],
		listLimit: 0,
		truncated: false,
		title: 'Select Server Files',
		allowMultiple: true,
		resolve: null
	};
}

export function beginServerPicker(
	state: ServerPickerState,
	title: string,
	allowMultiple: boolean
): ServerPickerState {
	return {
		...state,
		show: true,
		loading: true,
		error: '',
		baseDir: '',
		files: [],
		selectedPaths: [],
		listLimit: 0,
		truncated: false,
		title,
		allowMultiple
	};
}

export function withServerPickerListing(
	state: ServerPickerState,
	listing: {
		base_dir: string;
		files: ServerPickerFile[];
		list_limit: number;
		truncated: boolean;
	}
): ServerPickerState {
	return {
		...state,
		baseDir: listing.base_dir,
		files: listing.files,
		listLimit: listing.list_limit,
		truncated: listing.truncated
	};
}

export function toggleServerPickerSelection(
	state: ServerPickerState,
	path: string,
	checked: boolean
): ServerPickerState {
	if (!state.allowMultiple) {
		return {
			...state,
			selectedPaths: checked ? [path] : []
		};
	}

	if (checked) {
		if (state.selectedPaths.includes(path)) return state;
		return {
			...state,
			selectedPaths: [...state.selectedPaths, path]
		};
	}

	return {
		...state,
		selectedPaths: state.selectedPaths.filter((entry) => entry !== path)
	};
}

export function closeServerPickerState(
	state: ServerPickerState,
	result: string[] | null
): ServerPickerState {
	state.resolve?.(result);
	return {
		...state,
		show: false,
		resolve: null
	};
}

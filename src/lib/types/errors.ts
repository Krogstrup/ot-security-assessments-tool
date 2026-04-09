/** Machine-readable error code returned by backend command handlers. */
export type AppErrorCode =
	| 'state_lock'
	| 'no_session'
	| 'no_project'
	| 'invalid_input'
	| 'parse_failure'
	| 'db_error'
	| 'io_error'
	| 'no_capture_running'
	| 'external_process'
	| 'bad_request'
	| 'internal_error'
	| 'http_error';

/** Structured error payload shared by IPC responses. */
export interface AppError {
	code: AppErrorCode;
	message?: string;
}

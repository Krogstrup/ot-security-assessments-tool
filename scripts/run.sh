#!/usr/bin/env bash
set -euo pipefail

APP_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ENV_FILE="${KK_ENV_FILE:-$APP_DIR/config/app.env}"

if [[ -f "$ENV_FILE" ]]; then
  set -a
  # shellcheck disable=SC1090
  . "$ENV_FILE"
  set +a
fi

: "${KK_HOST:=0.0.0.0}"
: "${KK_PORT:=4173}"
: "${RUST_LOG:=info}"

: "${KK_FRONTEND_DIST:=$APP_DIR/frontend}"
: "${KK_SIGNATURES_DIR:=$APP_DIR/resources/signatures}"
: "${KK_DATA_DIR:=$APP_DIR/resources/data}"
: "${KK_STATE_HOME:=$APP_DIR/var/home}"
: "${KK_HEADLESS_IMPORTS_ROOT:=$APP_DIR/var/imports}"
: "${KK_HEADLESS_EXPORT_DIR:=$APP_DIR/var/export}"

BIN="$APP_DIR/bin/kusanaginokajiki_web"
if [[ ! -x "$BIN" ]]; then
  echo "Executable not found: $BIN" >&2
  exit 1
fi

if command -v ldd >/dev/null 2>&1; then
  missing_libs="$(ldd "$BIN" 2>/dev/null | awk '/not found/{print $1}')"
  if [[ -n "$missing_libs" ]]; then
    echo "Missing shared libraries for $BIN:" >&2
    echo "$missing_libs" >&2
    exit 1
  fi
fi

mkdir -p \
  "$KK_FRONTEND_DIST" \
  "$KK_SIGNATURES_DIR" \
  "$KK_DATA_DIR" \
  "$KK_STATE_HOME/.kusanaginokajiki" \
  "$KK_HEADLESS_IMPORTS_ROOT" \
  "$KK_HEADLESS_EXPORT_DIR" \
  "$APP_DIR/logs"

export HOME="$KK_STATE_HOME"
export RUST_LOG
export KK_FRONTEND_DIST
export KK_SIGNATURES_DIR
export KK_DATA_DIR
export KK_HEADLESS_IMPORTS_ROOT
export KK_HEADLESS_EXPORT_DIR

exec "$BIN" --host "$KK_HOST" --port "$KK_PORT" --frontend-dist "$KK_FRONTEND_DIST" "$@"

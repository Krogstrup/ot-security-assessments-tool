#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

APP_NAME="kusanaginokajiki"
BIN_NAME="kusanaginokajiki_web"
VERSION="$(sed -n 's/^version = "\(.*\)"/\1/p' backend/Cargo.toml | head -n 1)"
ARCH="$(uname -m)"
PLATFORM="linux-${ARCH}"
PACKAGE_NAME="${APP_NAME}-${VERSION}-${PLATFORM}"
RELEASE_ROOT="$ROOT_DIR/release"
BUNDLE_DIR="$RELEASE_ROOT/$PACKAGE_NAME"
DIST_DIR="$RELEASE_ROOT/dist"
ARCHIVE_PATH="$DIST_DIR/${PACKAGE_NAME}.tar.gz"

FRONTEND_DIR="$ROOT_DIR/build"
BIN_PATH="$ROOT_DIR/backend/target/release/$BIN_NAME"
SIGNATURES_DIR="$ROOT_DIR/backend/signatures"
DATA_DIR="$ROOT_DIR/backend/data"

echo "==> Building frontend + backend release binary"
npm run build

[[ -f "$FRONTEND_DIR/index.html" ]] || {
  echo "Missing frontend output: $FRONTEND_DIR/index.html" >&2
  exit 1
}
[[ -x "$BIN_PATH" ]] || {
  echo "Missing backend binary: $BIN_PATH" >&2
  exit 1
}
[[ -d "$SIGNATURES_DIR" ]] || {
  echo "Missing signatures directory: $SIGNATURES_DIR" >&2
  exit 1
}
[[ -d "$DATA_DIR" ]] || {
  echo "Missing data directory: $DATA_DIR" >&2
  exit 1
}

echo "==> Preparing bundle layout: $BUNDLE_DIR"
rm -rf "$BUNDLE_DIR"
mkdir -p \
  "$BUNDLE_DIR/bin" \
  "$BUNDLE_DIR/frontend" \
  "$BUNDLE_DIR/resources" \
  "$BUNDLE_DIR/config" \
  "$BUNDLE_DIR/var/imports" \
  "$BUNDLE_DIR/var/export" \
  "$BUNDLE_DIR/var/home" \
  "$BUNDLE_DIR/logs"

cp "$BIN_PATH" "$BUNDLE_DIR/bin/$BIN_NAME"
chmod +x "$BUNDLE_DIR/bin/$BIN_NAME"

cp -a "$FRONTEND_DIR/." "$BUNDLE_DIR/frontend/"
cp -a "$SIGNATURES_DIR" "$BUNDLE_DIR/resources/"
cp -a "$DATA_DIR" "$BUNDLE_DIR/resources/"
cp "$ROOT_DIR/scripts/run.sh" "$BUNDLE_DIR/run.sh"
cp "$ROOT_DIR/scripts/app.env.example" "$BUNDLE_DIR/config/app.env"
chmod +x "$BUNDLE_DIR/run.sh"

cat > "$BUNDLE_DIR/VERSION" <<EOF
$VERSION
EOF

mkdir -p "$DIST_DIR"
echo "==> Creating archive: $ARCHIVE_PATH"
tar -C "$RELEASE_ROOT" -czf "$ARCHIVE_PATH" "$PACKAGE_NAME"
sha256sum "$ARCHIVE_PATH" > "${ARCHIVE_PATH}.sha256"

echo
echo "Release package ready:"
echo "  Archive: $ARCHIVE_PATH"
echo "  SHA256 : ${ARCHIVE_PATH}.sha256"
echo
echo "Extract + run:"
echo "  tar -xzf $(basename "$ARCHIVE_PATH")"
echo "  cd $PACKAGE_NAME"
echo "  ./run.sh"

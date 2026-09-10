#!/usr/bin/env bash
# ==============================================================================
# Builds a valid Debian package (.deb) from a compiled binary using ar & tar
# ==============================================================================
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BIN="${1:-$ROOT/target/x86_64-unknown-linux-gnu/release/oride}"
OUTPUT_DIR="${2:-$ROOT/target/packages}"
VERSION="0.2.0"
ARCH="${3:-amd64}"

if [ ! -f "$BIN" ]; then
    echo "Error: Binary not found at $BIN" >&2
    exit 1
fi

STAGE=$(mktemp -d)
trap 'rm -rf "$STAGE"' EXIT

echo "==> Packaging Debian package for $ARCH..."
mkdir -p "$STAGE/DEBIAN"
mkdir -p "$STAGE/usr/bin"
mkdir -p "$STAGE/usr/share/doc/oride"
mkdir -p "$STAGE/usr/share/licenses/oride"
mkdir -p "$OUTPUT_DIR"

# Copy binary & assets
install -m 755 "$BIN" "$STAGE/usr/bin/oride"
cp "$ROOT/README.md" "$STAGE/usr/share/doc/oride/README.md"
cp "$ROOT/assets/config.example.toml" "$STAGE/usr/share/doc/oride/config.example.toml"
cp "$ROOT/LICENSE" "$STAGE/usr/share/licenses/oride/copyright"

# Control file
sed "s/Architecture: .*/Architecture: $ARCH/" "$ROOT/packaging/debian/control" > "$STAGE/DEBIAN/control"
sed -i "s/Version: .*/Version: $VERSION/" "$STAGE/DEBIAN/control"

# Create debian-binary
echo "2.0" > "$STAGE/debian-binary"

# Create control.tar.gz
(
    cd "$STAGE/DEBIAN"
    tar --numeric-owner --owner=0 --group=0 -czf "$STAGE/control.tar.gz" .
)

# Create data.tar.gz
(
    cd "$STAGE"
    tar --numeric-owner --owner=0 --group=0 -czf "$STAGE/data.tar.gz" ./usr
)

DEB_NAME="oride_${VERSION}_${ARCH}.deb"
# Assemble with ar
ar -rc "$OUTPUT_DIR/$DEB_NAME" "$STAGE/debian-binary" "$STAGE/control.tar.gz" "$STAGE/data.tar.gz"

echo "==> Created Debian package: $OUTPUT_DIR/$DEB_NAME"

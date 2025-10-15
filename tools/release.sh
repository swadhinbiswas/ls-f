#!/usr/bin/env bash
set -euo pipefail

# Release packaging script for ls-f
# - Creates versioned tar.gz and zip archives under dist/
# - Includes lsf, column.py, install.sh, LICENSE, README.md, docs/
# - Generates SHA256 checksums and a manifest

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

VERSION="${1:-}"
if [[ -z "$VERSION" ]]; then
  # Try to read from lsf script
  if [[ -f lsf ]]; then
    VERSION=$(grep -E '^VERSION="[^"]+"' lsf | sed -E 's/.*VERSION="([^"]+)".*/\1/')
  fi
fi

if [[ -z "$VERSION" ]]; then
  echo "Usage: $0 <version>" >&2
  echo "Or ensure VERSION is set in ./lsf" >&2
  exit 1
fi

DIST_DIR="$ROOT_DIR/dist"
mkdir -p "$DIST_DIR"

NAME="ls-f-$VERSION"
STAGE="$DIST_DIR/$NAME"
rm -rf "$STAGE"
mkdir -p "$STAGE"

# Files to include
INCLUDE=(
  "lsf"
  "column.py"
  "install.sh"
  "LICENSE"
  "README.md"
  "INSTALL.md"
  "QUICK_REFERENCE.md"
  "docs"
  "config"
)

# Copy files
for item in "${INCLUDE[@]}"; do
  if [[ -e "$item" ]]; then
    cp -a "$item" "$STAGE/"
  else
    echo "WARN: missing $item" >&2
  fi
done

# Generate MANIFEST
{
  echo "name: $NAME"
  echo "version: $VERSION"
  echo "date: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "files:"
  (cd "$STAGE" && find . -type f | sed 's#^\./##' | sort)
} > "$STAGE/MANIFEST.txt"

# Create archives
( cd "$DIST_DIR" && tar -czf "$NAME.tar.gz" "$NAME" )
( cd "$DIST_DIR" && zip -rq "$NAME.zip" "$NAME" )

# Checksums
( cd "$DIST_DIR" && sha256sum "$NAME.tar.gz" "$NAME.zip" > "$NAME.SHA256SUMS" )

# Summary
cat <<EOF
Release artifacts created in $DIST_DIR:
- $NAME.tar.gz
- $NAME.zip
- $NAME.SHA256SUMS
- $NAME/MANIFEST.txt

Next steps:
- Create a GitHub Release tagged v$VERSION
- Upload the archives and checksums
- Paste release notes from RELEASE_NOTES.md
EOF

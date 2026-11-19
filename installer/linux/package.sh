#!/usr/bin/env bash
# CrawRAD — Linux packages: a tar.gz and an AppImage of the same bundle.
#   python3 tools/bundle/bundle.py
#   VERSION=0.1.0 installer/linux/package.sh
# The AppImage step needs `appimagetool` on PATH; without it only the tar.gz is made.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
VERSION="${VERSION:-0.0.0-dev}"
BUNDLE="${BUNDLE:-$ROOT/target/bundle/CrawRAD}"
OUT="$ROOT/target/installer"
mkdir -p "$OUT"

tar -C "$(dirname "$BUNDLE")" -czf "$OUT/CrawRAD-$VERSION-linux-x64.tar.gz" "$(basename "$BUNDLE")"
echo "made $OUT/CrawRAD-$VERSION-linux-x64.tar.gz"

if ! command -v appimagetool >/dev/null; then
  echo "appimagetool not found: AppImage skipped"
  exit 0
fi

APPDIR="$ROOT/target/installer/CrawRAD.AppDir"
rm -rf "$APPDIR"
mkdir -p "$APPDIR/usr/lib"
cp -a "$BUNDLE" "$APPDIR/usr/lib/crawrad"
cat > "$APPDIR/AppRun" <<'EOF'
#!/bin/sh
HERE="$(dirname "$(readlink -f "$0")")"
exec "$HERE/usr/lib/crawrad/crawrad-ide" "$@"
EOF
chmod +x "$APPDIR/AppRun"
cat > "$APPDIR/crawrad.desktop" <<'EOF'
[Desktop Entry]
Type=Application
Name=CrawRAD
Comment=Visual RAD IDE for Rust
Exec=crawrad-ide
Icon=crawrad
Categories=Development;IDE;
EOF
cp "$ROOT/crates/crawrad-ide/crawrad.png" "$APPDIR/crawrad.png" 2>/dev/null \
  || echo "warning: no crawrad.png icon"
ARCH=x86_64 appimagetool "$APPDIR" "$OUT/CrawRAD-$VERSION-x86_64.AppImage"
echo "made $OUT/CrawRAD-$VERSION-x86_64.AppImage"

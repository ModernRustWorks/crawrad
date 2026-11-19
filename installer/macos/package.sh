#!/usr/bin/env bash
# CrawRAD — macOS: a .app inside a .dmg.
#   python3 tools/bundle/bundle.py
#   VERSION=0.1.0 installer/macos/package.sh
# The bundle goes in Contents/MacOS, so the IDE's folder (its working folder at
# start) still holds resources/, crates/, components/ and vendor/.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
VERSION="${VERSION:-0.0.0-dev}"
BUNDLE="${BUNDLE:-$ROOT/target/bundle/CrawRAD}"
OUT="$ROOT/target/installer"
APP="$OUT/dmg/CrawRAD.app"
rm -rf "$OUT/dmg"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"
cp -a "$BUNDLE/." "$APP/Contents/MacOS/"
cat > "$APP/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
  <key>CFBundleName</key><string>CrawRAD</string>
  <key>CFBundleIdentifier</key><string>dev.crawrad.ide</string>
  <key>CFBundleExecutable</key><string>crawrad-ide</string>
  <key>CFBundleVersion</key><string>$VERSION</string>
  <key>CFBundleShortVersionString</key><string>$VERSION</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>NSHighResolutionCapable</key><true/>
</dict></plist>
EOF
ln -s /Applications "$OUT/dmg/Applications"
hdiutil create -volname CrawRAD -srcfolder "$OUT/dmg" -ov -format UDZO \
  "$OUT/CrawRAD-$VERSION-macos.dmg"
echo "made $OUT/CrawRAD-$VERSION-macos.dmg"

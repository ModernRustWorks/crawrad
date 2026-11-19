#!/usr/bin/env sh
# Puts the rust-analyzer CrawRAD ships beside the IDE's executable (ADR 0050).
# Linux and macOS twin of fetch-rust-analyzer.ps1; same pinned version.
#
#   sh tools/rust-analyzer/fetch-rust-analyzer.sh target/release/rust-analyzer
set -eu

OUT="${1:?usage: fetch-rust-analyzer.sh <IDE folder>/rust-analyzer}"

# Pinned 01/10/2026: rust-analyzer 0.3.3065-standalone (03fcb77246 2026-09-27).
VERSION=2026-09-28

case "$(uname -s)-$(uname -m)" in
    Linux-x86_64)  ASSET=rust-analyzer-x86_64-unknown-linux-gnu.gz
                   SHA=23f711d86b5f826e22886f01d7355dc01e0f4c1357dafa29710a95b903b48c85 ;;
    Darwin-arm64)  ASSET=rust-analyzer-aarch64-apple-darwin.gz
                   SHA=54ec873d8996e2c127d758bf45d4eacb6d3371dae4f6f6d5d3f05cedbae5fd59 ;;
    Darwin-x86_64) ASSET=rust-analyzer-x86_64-apple-darwin.gz
                   SHA=d032c0eb75e4597cc8ffc35ea4cdbd9eecc8341936b6edac6749e679fc3f0682 ;;
    *) echo "No pinned rust-analyzer for $(uname -s)-$(uname -m)." >&2; exit 1 ;;
esac

TMP="$(mktemp)"
trap 'rm -f "$TMP"' EXIT
curl -fsSL "https://github.com/rust-lang/rust-analyzer/releases/download/$VERSION/$ASSET" -o "$TMP"

if command -v sha256sum >/dev/null 2>&1; then GOT="$(sha256sum "$TMP" | cut -d' ' -f1)"
else GOT="$(shasum -a 256 "$TMP" | cut -d' ' -f1)"; fi
if [ "$GOT" != "$SHA" ]; then
    echo "SHA-256 mismatch for $ASSET: expected $SHA, got $GOT." >&2
    exit 1
fi

mkdir -p "$OUT"
gunzip -c "$TMP" > "$OUT/rust-analyzer"
chmod +x "$OUT/rust-analyzer"
"$OUT/rust-analyzer" --version

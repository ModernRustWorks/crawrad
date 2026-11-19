#!/usr/bin/env bash
# Builds the debug adapter CrawRAD ships on Linux and macOS: lldb-dap with
# Python turned off — the Unix twin of build-lldb.ps1 (ADR 0048).
#
# The ready-made LLDBs link Python here too (measured on 01/10/2026: Debian's
# lldb-dap-19 and lldb-dap-22 both need libpython3.13), and the owner's rule is
# a debugger without Python. Same LLVM sources, same switches as on Windows.
#
# Output, in OUT: lldb-dap, lldb-server and liblldb, with the adapter's library
# path pointing at its own folder ($ORIGIN on Linux, @loader_path on macOS), so
# the three files work from the IDE's `lldb/` folder and nothing else is needed.
#
# Usage: tools/lldb/build-lldb.sh <llvm-project-<version>.src> <out> [build dir]
# Needs: cmake, ninja, a C++ compiler, patchelf on Linux, and python3 — for the
# LLVM build scripts only; the adapter built has no Python in it.
set -euo pipefail

SOURCE=${1:?the extracted llvm-project source tree}
OUT=${2:?where the three files are copied}
BUILD=${3:-$(dirname "$SOURCE")/build-$(uname -s | tr '[:upper:]' '[:lower:]')}

cmake -G Ninja -S "$SOURCE/llvm" -B "$BUILD" \
  -DCMAKE_BUILD_TYPE=Release \
  -DLLVM_ENABLE_PROJECTS='clang;lldb' \
  -DLLVM_TARGETS_TO_BUILD='X86;AArch64' \
  -DLLDB_ENABLE_PYTHON=OFF \
  -DLLDB_ENABLE_LUA=OFF \
  -DLLDB_ENABLE_LIBEDIT=OFF \
  -DLLDB_ENABLE_CURSES=OFF \
  -DLLDB_ENABLE_LIBXML2=OFF \
  -DLLDB_ENABLE_LZMA=OFF \
  -DLLDB_BUILD_FRAMEWORK=OFF \
  -DLLVM_ENABLE_ZLIB=OFF \
  -DLLVM_ENABLE_ZSTD=OFF \
  -DLLVM_ENABLE_LIBXML2=OFF \
  -DLLVM_ENABLE_TERMINFO=OFF \
  -DLLVM_ENABLE_ASSERTIONS=OFF \
  -DLLVM_INCLUDE_TESTS=OFF \
  -DLLVM_INCLUDE_BENCHMARKS=OFF \
  -DLLVM_INCLUDE_EXAMPLES=OFF \
  -DLLVM_INCLUDE_DOCS=OFF \
  -DCLANG_INCLUDE_TESTS=OFF \
  -DCLANG_INCLUDE_DOCS=OFF \
  -DLLDB_INCLUDE_TESTS=OFF

# JOBS=4 keeps the memory down: LLVM at full width can exhaust a 32 GB machine.
ninja -C "$BUILD" ${JOBS:+-j "$JOBS"} lldb-dap lldb-server liblldb

mkdir -p "$OUT"
cp "$BUILD/bin/lldb-dap" "$BUILD/bin/lldb-server" "$OUT/"
case "$(uname -s)" in
  Linux)
    # The real file, under the name the adapter asks for (its SONAME).
    lib=$(readlink -f "$BUILD/lib/liblldb.so")
    soname=$(patchelf --print-soname "$lib")
    cp "$lib" "$OUT/$soname"
    patchelf --set-rpath '$ORIGIN' "$OUT/lldb-dap"
    ;;
  Darwin)
    lib=$(readlink -f "$BUILD/lib/liblldb.dylib")
    name=$(otool -D "$lib" | tail -1)
    cp "$lib" "$OUT/$(basename "$name")"
    install_name_tool -add_rpath @loader_path "$OUT/lldb-dap"
    ;;
esac
ls -l "$OUT"

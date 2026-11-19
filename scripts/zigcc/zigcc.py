"""Run `zig cc` for a cross target, dropping the clang `--target=` flag cc-rs adds."""
import subprocess
import sys

target = sys.argv[1]
args = [a for a in sys.argv[2:] if not a.startswith("--target=")]
sys.exit(subprocess.call(["zig", "cc", "-target", target, *args]))

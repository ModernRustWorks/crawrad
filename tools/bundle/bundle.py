#!/usr/bin/env python3
"""Build the CrawRAD install folder (the "bundle") for the current system.

The bundle mirrors the repo root, so a project made by the installed IDE finds
the runtime crates the same way it does in a checkout:

    <bundle>/
        crawrad-ide(.exe)  crawrad-dbprobe(.exe)  crawrad-mcp-bridge(.exe)
        resources/      fonts and icons, read at start (MAKEPAD_PACKAGE_DIR)
        Cargo.toml      a workspace that lists only the shipped crates
        Cargo.lock
        crates/         the runtime crates a generated project depends on
        components/     the component suite
        vendor/         the patched makepad-platform
        LICENSE, LICENSE-MIT, LICENSE-APACHE, ADDITIONAL-PERMISSIONS.md, TRADEMARKS.md
        THIRD-PARTY-NOTICES.md   every bundled package and font, with its licence

Usage:  python tools/bundle/bundle.py [--out DIR] [--skip-build]
"""

from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import third_party  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
EXE = ".exe" if os.name == "nt" else ""
BINARIES = ["crawrad-ide", "crawrad-dbprobe", "crawrad-mcp-bridge"]
# What a generated project names (crawrad-codegen/src/dependencies.rs) plus the
# component suite; their local path dependencies are added below.
SDK_ROOTS = ["crawrad-runtime", "crawrad-runtime-makepad", "crawrad-runtime-egui", "crawrad-binding"]
# Crates whose `resources/` makepad opens at start (`crate://<name>/...`).
FONT_CRATES = [
    "makepad-widgets",
    "makepad-fonts-chinese-bold",
    "makepad-fonts-chinese-bold-2",
    "makepad-fonts-chinese-regular",
    "makepad-fonts-chinese-regular-2",
    "makepad-fonts-emoji",
    "crawrad-runtime-makepad",  # Liberation Mono 2.1.5 (OFL), the code font
]
SKIP = {"target", ".git", ".crawrad-cache"}


def run(cmd: list[str], env: dict[str, str] | None = None) -> None:
    print("+", " ".join(cmd), flush=True)
    subprocess.run(cmd, cwd=ROOT, env=env, check=True)


def metadata() -> dict:
    out = subprocess.run(
        ["cargo", "metadata", "--format-version", "1", "--locked"],
        cwd=ROOT, check=True, capture_output=True, encoding="utf-8",
    ).stdout
    return json.loads(out)


def copy_tree(src: Path, dst: Path) -> None:
    shutil.copytree(src, dst, dirs_exist_ok=True,
                    ignore=lambda _d, names: [n for n in names if n in SKIP])


def local_closure(meta: dict, roots: list[str]) -> list[dict]:
    """The workspace packages `roots` reach through normal path dependencies."""
    by_name = {p["name"]: p for p in meta["packages"] if p["source"] is None}
    seen: dict[str, dict] = {}
    stack = [n for n in roots if n in by_name]
    stack += [p["name"] for p in by_name.values()
              if Path(p["manifest_path"]).parent.parent.name == "components"]
    while stack:
        name = stack.pop()
        if name in seen:
            continue
        seen[name] = by_name[name]
        for dep in by_name[name]["dependencies"]:
            if dep.get("kind") in (None, "build") and dep.get("path") and dep["name"] in by_name:
                stack.append(dep["name"])
    return sorted(seen.values(), key=lambda p: p["manifest_path"])


DEV_SECTION = re.compile(
    r"^\[(dev-dependencies|target\.[^\]]+\.dev-dependencies|\[(?:test|bench|example)\])\]\s*$")


def strip_dev_sections(manifest: Path) -> None:
    """Drop dev-dependencies and test/bench/example targets: cargo loads every
    member's dev-dependencies, and those name crates that do not ship."""
    kept, skipping = [], False
    for line in manifest.read_text(encoding="utf-8").splitlines(keepends=True):
        if line.startswith("["):
            skipping = bool(DEV_SECTION.match(line.strip()))
        if not skipping:
            kept.append(line)
    text = "".join(kept)
    # No auto-discovered test or example target may need a missing dev-dependency.
    if "[package]" in text:
        text = text.replace("[package]\n", "[package]\nautotests = false\nautoexamples = false\nautobenches = false\n", 1)
    manifest.write_text(text, encoding="utf-8")


def write_workspace(out: Path, members: list[str]) -> None:
    """The repo manifest with `members` cut to what ships."""
    text = (ROOT / "Cargo.toml").read_text(encoding="utf-8")
    body = "".join(f'    "{m}",\n' for m in members)
    text, n = re.subn(r"members = \[.*?\n\]", "members = [\n" + body + "]",
                      text, count=1, flags=re.S)
    if n != 1:
        sys.exit("bundle: could not find `members = [...]` in Cargo.toml")
    (out / "Cargo.toml").write_text(text, encoding="utf-8")
    shutil.copy2(ROOT / "Cargo.lock", out / "Cargo.lock")


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default=str(ROOT / "target" / "bundle" / "CrawRAD"))
    ap.add_argument("--skip-build", action="store_true")
    args = ap.parse_args()
    out = Path(args.out).resolve()

    if not args.skip_build:
        env = dict(os.environ, MAKEPAD_PACKAGE_DIR="resources")
        run(["cargo", "build", "--release", "--locked",
             "-p", "crawrad-ide", "-p", "crawrad-dbprobe", "-p", "crawrad-mcp"], env)

    meta = metadata()
    release = Path(meta["target_directory"]) / "release"

    if out.exists():
        shutil.rmtree(out)
    out.mkdir(parents=True)

    for b in BINARIES:
        shutil.copy2(release / f"{b}{EXE}", out / f"{b}{EXE}")

    # Fonts: makepad opens `<package_root>/<crate_snake>/resources/...`.
    resources = out / "resources"
    for pkg in meta["packages"]:
        if pkg["name"] in FONT_CRATES:
            src = Path(pkg["manifest_path"]).parent / "resources"
            if src.is_dir():
                copy_tree(src, resources / pkg["name"].replace("-", "_") / "resources")
    # `crate://self/...` in the IDE's own theme.
    ide_assets = ROOT / "crates" / "crawrad-ide" / "assets"
    copy_tree(ide_assets, resources / "assets")
    copy_tree(ide_assets, resources / "crawrad_ide" / "assets")

    members = []
    for pkg in local_closure(meta, SDK_ROOTS):
        src = Path(pkg["manifest_path"]).parent
        rel = src.relative_to(ROOT).as_posix()
        copy_tree(src, out / rel)
        strip_dev_sections(out / rel / "Cargo.toml")
        members.append(rel)
    copy_tree(ROOT / "vendor", out / "vendor")
    for extra in (
        "components/README.md",
        "LICENSE",
        "LICENSE-MIT",
        "LICENSE-APACHE",
        "ADDITIONAL-PERMISSIONS.md",
        "TRADEMARKS.md",
        "README.md",
    ):
        if (ROOT / extra).is_file():
            (out / extra).parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(ROOT / extra, out / extra)
    write_workspace(out, members)
    (out / "THIRD-PARTY-NOTICES.md").write_text(
        third_party.render(meta), encoding="utf-8", newline="\n"
    )

    print(f"bundle: {out}")
    for m in members:
        print(f"  sdk  {m}")


if __name__ == "__main__":
    main()

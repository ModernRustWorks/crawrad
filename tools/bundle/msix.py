#!/usr/bin/env python3
"""Build the Microsoft Store package (MSIX) for CrawRAD on Windows.

Usage:
    python tools/bundle/msix.py --bundle target/bundle/CrawRAD --version X.Y.Z --out target/installer
"""

from __future__ import annotations

import argparse
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def msix_version(text: str) -> str:
    """Convert a version string into an MSIX four-part numeric version 'X.Y.Z.0'.

    Drops pre-release suffixes (e.g. '0.1.0-beta' -> '0.1.0.0').
    Rejects any version where parts are non-numeric or where a 4th part is non-zero.
    """
    clean = text.strip()
    if clean.startswith(("v", "V")):
        clean = clean[1:]
    # Drop pre-release and build metadata suffix (e.g. -beta, +build)
    clean = re.split(r"[-+]", clean)[0]

    parts = clean.split(".")
    if len(parts) == 3:
        if not all(p.isdigit() for p in parts):
            raise ValueError(f"Invalid version '{text}': all parts must be numeric")
        return f"{parts[0]}.{parts[1]}.{parts[2]}.0"
    elif len(parts) == 4:
        if not all(p.isdigit() for p in parts):
            raise ValueError(f"Invalid version '{text}': all parts must be numeric")
        if parts[3] != "0":
            raise ValueError(f"Invalid version '{text}': last part must be 0 for MSIX")
        return f"{parts[0]}.{parts[1]}.{parts[2]}.0"
    else:
        raise ValueError(f"Invalid version '{text}': expected 3 numeric parts (e.g. 1.2.3)")


def fill_manifest(template: str, values: dict[str, str]) -> str:
    """Replace placeholders @KEY@ with values and ensure no placeholder remains."""
    result = template
    for key, val in values.items():
        result = result.replace(f"@{key}@", str(val))
    leftover = re.findall(r"@[A-Z0-9_]+@", result)
    if leftover:
        raise ValueError(f"Unfilled placeholder(s) in manifest: {', '.join(leftover)}")
    return result


def find_makeappx() -> Path:
    """Find the newest makeappx.exe from installed Windows 10/11 SDKs."""
    base_dirs = [
        Path(os.environ.get("ProgramFiles(x86)", r"C:\Program Files (x86)")) / "Windows Kits" / "10" / "bin",
        Path(r"C:\Program Files (x86)\Windows Kits\10\bin"),
        Path(os.environ.get("ProgramFiles", r"C:\Program Files")) / "Windows Kits" / "10" / "bin",
    ]
    candidates: list[Path] = []
    for base in base_dirs:
        if base.is_dir():
            for p in base.glob("*/x64/makeappx.exe"):
                if p.is_file():
                    candidates.append(p)
    if not candidates:
        raise FileNotFoundError(
            "makeappx.exe not found in Windows Kits (C:/Program Files (x86)/Windows Kits/10/bin/*/x64/makeappx.exe)"
        )

    def version_key(path: Path) -> list[int]:
        ver_str = path.parent.parent.name
        try:
            return [int(x) for x in ver_str.split(".")]
        except ValueError:
            return [0]

    candidates.sort(key=version_key)
    return candidates[-1]


def copy_bundle_to_staging(bundle_dir: Path, staging_dir: Path) -> None:
    """Copy bundle contents to staging folder, ignoring transient files."""
    skip = {"target", ".git"}
    shutil.copytree(
        bundle_dir,
        staging_dir,
        dirs_exist_ok=True,
        ignore=lambda _d, names: [n for n in names if n in skip],
    )


def main() -> None:
    ap = argparse.ArgumentParser(description="Package CrawRAD as an MSIX file for the Microsoft Store.")
    ap.add_argument("--bundle", default=str(ROOT / "target" / "bundle" / "CrawRAD"), help="Path to bundle folder")
    ap.add_argument("--version", required=True, help="Release version (e.g. 0.1.0)")
    ap.add_argument("--out", default=str(ROOT / "target" / "installer"), help="Output directory")
    args = ap.parse_args()

    # Check required environment variables
    identity_name = os.environ.get("STORE_IDENTITY_NAME", "").strip()
    publisher = os.environ.get("STORE_PUBLISHER", "").strip()
    publisher_display = os.environ.get("STORE_PUBLISHER_DISPLAY", "").strip()

    if not identity_name or not publisher or not publisher_display:
        print(
            "MSIX skipped: set STORE_IDENTITY_NAME, STORE_PUBLISHER and STORE_PUBLISHER_DISPLAY "
            "(Partner Center > Product identity)"
        )
        sys.exit(0)

    try:
        ver_msix = msix_version(args.version)
    except ValueError as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(2)

    bundle_dir = Path(args.bundle).resolve()
    if not bundle_dir.is_dir():
        print(f"Error: bundle directory not found: {bundle_dir}", file=sys.stderr)
        sys.exit(2)

    staging_dir = ROOT / "target" / "msix-staging"
    if staging_dir.exists():
        shutil.rmtree(staging_dir)
    staging_dir.mkdir(parents=True, exist_ok=True)

    print(f"Copying bundle from {bundle_dir} to {staging_dir}...")
    copy_bundle_to_staging(bundle_dir, staging_dir)

    template_file = ROOT / "installer" / "windows" / "msix" / "AppxManifest.xml"
    if not template_file.is_file():
        print(f"Error: manifest template not found: {template_file}", file=sys.stderr)
        sys.exit(2)

    manifest_template = template_file.read_text(encoding="utf-8")
    filled_manifest = fill_manifest(
        manifest_template,
        {
            "IDENTITY_NAME": identity_name,
            "PUBLISHER": publisher,
            "PUBLISHER_DISPLAY": publisher_display,
            "VERSION": ver_msix,
        },
    )
    (staging_dir / "AppxManifest.xml").write_text(filled_manifest, encoding="utf-8")

    assets_src = ROOT / "installer" / "windows" / "msix" / "Assets"
    assets_dst = staging_dir / "Assets"
    if not assets_src.is_dir():
        print(f"Error: assets directory not found: {assets_src}", file=sys.stderr)
        sys.exit(2)
    shutil.copytree(assets_src, assets_dst, dirs_exist_ok=True)

    try:
        makeappx = find_makeappx()
    except FileNotFoundError as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(2)

    out_dir = Path(args.out).resolve()
    out_dir.mkdir(parents=True, exist_ok=True)
    msix_filename = f"CrawRAD-{args.version}-windows-x64.msix"
    msix_path = out_dir / msix_filename

    cmd = [
        str(makeappx),
        "pack",
        "/o",
        "/h",
        "SHA256",
        "/d",
        str(staging_dir),
        "/p",
        str(msix_path),
    ]
    print("+", " ".join(cmd), flush=True)
    res = subprocess.run(cmd)
    sys.exit(res.returncode)


if __name__ == "__main__":
    main()

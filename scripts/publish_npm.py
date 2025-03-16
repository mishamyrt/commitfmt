#!/usr/bin/env python3
"""Publish commitfmt npm packages"""
import shutil
import os

from helpers.shell import shell
from project_config import NPM_PACKAGES_DIR, DIST_DIR

BINARIES_MAPPING = {
    "commitfmt-darwin-arm64": "commitfmt-aarch64-apple-darwin/commitfmt",
    "commitfmt-darwin-x64": "commitfmt-x86_64-apple-darwin/commitfmt",
    "commitfmt-linux-arm64": "commitfmt-aarch64-unknown-linux-gnu/commitfmt",
    "commitfmt-linux-x64": "commitfmt-x86_64-unknown-linux-gnu/commitfmt",
    "commitfmt-windows-arm64": "commitfmt-aarch64-pc-windows-msvc/commitfmt.exe",
    "commitfmt-windows-x64": "commitfmt-x86_64-pc-windows-msvc/commitfmt.exe",
}

def copy_binaries():
    """Copy binaries to npm packages"""
    for package in NPM_PACKAGES_DIR.iterdir():
        if package.name not in BINARIES_MAPPING:
            continue

        source = DIST_DIR / BINARIES_MAPPING[package.name]
        extension = ".exe" if source.name.endswith(".exe") else ""
        bin_name = f"commitfmt{extension}"
        target = package / bin_name

        print(f"Copying {source} to {target}")
        shutil.copy2(source, target)

def publish():
    """Publish npm packages"""
    for package in NPM_PACKAGES_DIR.iterdir():
        print(f"  - {package.name}")
        shell("npm publish", cwd=package)

def main():
    """Script entry point"""
    print("Copying binaries...")
    copy_binaries()
    print("Publishing packages...")
    publish()

if __name__ == "__main__":
    main()

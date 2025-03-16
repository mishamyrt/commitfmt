#!/usr/bin/env python3
"""Publish commitfmt npm packages"""
import shutil

from helpers.shell import shell
from project_config import NPM_PACKAGES_DIR, OUT_BINARIES

PACKAGE_BINARIES = {
    "commitfmt-darwin-arm64": OUT_BINARIES[("darwin", "arm64")],
    "commitfmt-darwin-x64": OUT_BINARIES[("darwin", "x64")],
    "commitfmt-linux-arm64": OUT_BINARIES[("linux", "arm64")],
    "commitfmt-linux-x64": OUT_BINARIES[("linux", "x64")],
    "commitfmt-windows-arm64": OUT_BINARIES[("windows", "arm64")],
    "commitfmt-windows-x64": OUT_BINARIES[("windows", "x64")],
}

def copy_binaries():
    """Copy binaries to npm packages"""
    for package in NPM_PACKAGES_DIR.iterdir():
        if package.name.startswith(".") or package.name not in PACKAGE_BINARIES:
            continue

        source = PACKAGE_BINARIES[package.name]
        bin_name = source.name
        destination = package / bin_name

        print(f"{source} → {destination}")
        shutil.copy2(source, destination)

def publish():
    """Publish npm packages"""
    for package in NPM_PACKAGES_DIR.iterdir():
        if package.name.startswith("."):
            continue
        print(f"- {package.name}")
        shell("npm publish", cwd=package)

def main():
    """Script entry point"""
    print("Copying binaries...")
    copy_binaries()
    print("Publishing packages...")
    publish()

if __name__ == "__main__":
    main()

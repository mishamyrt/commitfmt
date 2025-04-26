#!/usr/bin/env python3
"""Publish commitfmt npm packages"""
import shutil
import os
import sys

from helpers.shell import shell
from project_config import PYPI_PACKAGES_DIR, OUT_BINARIES

SUFFIX_ARM64 = "arm64"
SUFFIX_X64   = "amd64"

PACKAGE_BINARIES = {
    "commitfmt_darwin": [
        (OUT_BINARIES[("darwin", "x64")], SUFFIX_X64),
        (OUT_BINARIES[("darwin", "arm64")], SUFFIX_ARM64),
    ],
    "commitfmt_linux": [
        (OUT_BINARIES[("linux", "x64")], SUFFIX_X64),
        (OUT_BINARIES[("linux", "arm64")], SUFFIX_ARM64),
    ],
    "commitfmt_windows": [
        (OUT_BINARIES[("windows", "x64")], SUFFIX_X64),
        (OUT_BINARIES[("windows", "arm64")], SUFFIX_ARM64),
    ],
}

def copy_readme():
    """Copy README.md from root to each PyPI package"""
    readme_path = "README.md"
    for package in PYPI_PACKAGES_DIR.iterdir():
        if package.name.startswith("."):
            continue
        destination = package / "README.md"
        print(f"{readme_path} → {destination}")
        shutil.copy2(readme_path, destination)

def copy_binaries():
    """Copy binaries to npm packages"""
    for package in PYPI_PACKAGES_DIR.iterdir():
        if package.name.startswith(".") or package.name not in PACKAGE_BINARIES:
            continue

        sources = PACKAGE_BINARIES[package.name]
        for source, suffix in sources:
            bin_name = source.name.replace("commitfmt", f"commitfmt_{suffix}")
            destination = package / package.name / bin_name

            print(f"Copying {source} to {destination}")
            shutil.copy2(source, destination)

def publish(token: str):
    """Publish npm packages"""
    for package in PYPI_PACKAGES_DIR.iterdir():
        if package.name.startswith("."):
            continue
        print(f"- {package.name}")
        shell("python -m build", cwd=package)
        shell((
            "python -m twine upload dist/* "
            "--repository pypi "
            f"--password {token} "
            "--non-interactive"
        ), cwd=package)

def main():
    """Script entry point"""
    pypi_token = os.getenv("PYPI_TOKEN")
    if pypi_token is None:
        print("PYPI_TOKEN is not set")
        sys.exit(1)
    print("Copying READMEs...")
    copy_readme()
    print("Copying binaries...")
    copy_binaries()
    print("Publishing packages...")
    publish(pypi_token)

if __name__ == "__main__":
    main()

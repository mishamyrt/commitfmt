#!/usr/bin/env python3
"""Build and publish commitfmt PyPI packages."""
import argparse
import shutil
import os
import sys

from helpers.shell import shell
from project_config import PYPI_PACKAGES_DIR, OUT_BINARIES

PACKAGE_BINARIES = {
    "commitfmt_darwin_arm64": OUT_BINARIES[("darwin", "arm64")],
    "commitfmt_darwin_x64": OUT_BINARIES[("darwin", "x64")],
    "commitfmt_linux_arm64": OUT_BINARIES[("linux", "arm64")],
    "commitfmt_linux_x64": OUT_BINARIES[("linux", "x64")],
    "commitfmt_windows_arm64": OUT_BINARIES[("windows", "arm64")],
    "commitfmt_windows_x64": OUT_BINARIES[("windows", "x64")],
}


def package_directories(platform_only=False):
    """Return package directories in publication order."""
    packages = [
        package
        for package in PYPI_PACKAGES_DIR.iterdir()
        if not package.name.startswith(".")
        and (package / "pyproject.toml").is_file()
        and (not platform_only or package.name in PACKAGE_BINARIES)
    ]
    return sorted(packages, key=lambda package: (package.name == "commitfmt", package.name))


def copy_readme(packages):
    """Copy README.md from root to each PyPI package"""
    readme_path = "README.md"
    for package in packages:
        destination = package / "README.md"
        print(f"{readme_path} → {destination}")
        shutil.copy2(readme_path, destination)


def copy_binaries(packages):
    """Copy one binary to each platform package."""
    for package in packages:
        if package.name not in PACKAGE_BINARIES:
            continue

        source = PACKAGE_BINARIES[package.name]
        destination = package / package.name / source.name

        print(f"{source} → {destination}")
        shutil.copy2(source, destination)


def build(packages):
    """Build and validate every package before publishing anything."""
    for package in packages:
        print(f"- {package.name}")
        shutil.rmtree(package / "dist", ignore_errors=True)
        shell("python -m build", cwd=package)
        shell("python -m twine check dist/*", cwd=package)


def publish(packages, token: str):
    """Publish platform packages first and the commitfmt metapackage last."""
    environment = os.environ.copy()
    environment["TWINE_USERNAME"] = "__token__"
    environment["TWINE_PASSWORD"] = token
    for package in packages:
        print(f"- {package.name}")
        shell(
            "python -m twine upload dist/* --repository pypi --non-interactive",
            cwd=package,
            env=environment,
        )


def parse_args():
    """Parse command-line arguments."""
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--platform-only",
        action="store_true",
        help="publish only architecture-specific packages during the initial bootstrap",
    )
    return parser.parse_args()


def main():
    """Script entry point"""
    args = parse_args()
    pypi_token = os.getenv("PYPI_TOKEN")
    if pypi_token is None:
        print("PYPI_TOKEN is not set")
        sys.exit(1)
    packages = package_directories(platform_only=args.platform_only)
    print("Copying READMEs...")
    copy_readme(packages)
    print("Copying binaries...")
    copy_binaries(packages)
    print("Building packages...")
    build(packages)
    print("Publishing packages...")
    publish(packages, pypi_token)


if __name__ == "__main__":
    main()

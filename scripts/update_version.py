#!/usr/bin/env python3
"""Update version in packages metadata files"""

from pathlib import Path
import re
import sys
import json

CLI_CRATE_MANIFEST = "crates/commitfmt/Cargo.toml"
NPM_PACKAGES_DIR = Path("packaging/npm")

def set_cli_cargo_version(version: str):
    """Update version in CLI Cargo.toml"""
    with open(CLI_CRATE_MANIFEST, "r", encoding="utf-8") as file:
        cargo_toml = file.read()
    version_re = r"version = \"(.*)\""
    cargo_toml = re.sub(version_re, f"version = \"{version}\"", cargo_toml)
    with open(CLI_CRATE_MANIFEST, "w", encoding="utf-8") as file:
        file.write(cargo_toml)

def set_npm_version(version: str):
    """Update version in npm package.json files"""
    for package_dir in NPM_PACKAGES_DIR.iterdir():
        manifest = package_dir / "package.json"
        with manifest.open("r", encoding="utf-8") as file:
            package_json = json.load(file)

        package_json["version"] = version

        if package_dir.name == "commitfmt":
            for key, _ in package_json["optionalDependencies"].items():
                package_json["optionalDependencies"][key] = version

        with manifest.open("w", encoding="utf-8") as file:
            json.dump(package_json, file, indent=2)

if __name__ == "__main__":
    target = sys.argv[1]
    set_cli_cargo_version(target)
    set_npm_version(target)

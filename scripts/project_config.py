"""Project configuration constants"""
from pathlib import Path


CLI_CRATE_MANIFEST = "crates/commitfmt/Cargo.toml"
NPM_PACKAGES_DIR = Path("packaging/npm")
PYPI_PACKAGES_DIR = Path("packaging/pypi")
DIST_DIR = Path("target/distrib")

NIX_BIN_NAME = "commitfmt"
WIN_BIN_NAME = f"{NIX_BIN_NAME}.exe"

OUT_BINARIES = {
    ("darwin", "arm64"): DIST_DIR / "commitfmt-aarch64-apple-darwin" / NIX_BIN_NAME,
    ("darwin", "x64"): DIST_DIR / "commitfmt-x86_64-apple-darwin" / NIX_BIN_NAME,
    ("linux", "arm64"): DIST_DIR / "commitfmt-aarch64-unknown-linux-gnu" / NIX_BIN_NAME,
    ("linux", "x64"): DIST_DIR / "commitfmt-x86_64-unknown-linux-gnu" / NIX_BIN_NAME,
    ("windows", "arm64"): DIST_DIR / "commitfmt-aarch64-pc-windows-msvc" / WIN_BIN_NAME,
    ("windows", "x64"): DIST_DIR / "commitfmt-x86_64-pc-windows-msvc" / WIN_BIN_NAME,
}

#  ("darwin", "x64"): "commitfmt-x86_64-apple-darwin/commitfmt",
#     ("linux", "arm64"): "commitfmt-aarch64-unknown-linux-gnu/commitfmt",
#     ("linux", "x64"): "commitfmt-x86_64-unknown-linux-gnu/commitfmt",
#     ("windows", "arm64"): "commitfmt-aarch64-pc-windows-msvc/commitfmt.exe",
#     ("windows", "x64"): "commitfmt-x86_64-pc-windows-msvc/commitfmt.exe",

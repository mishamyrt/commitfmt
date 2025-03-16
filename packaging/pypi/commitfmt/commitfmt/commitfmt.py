"""commitfmt python wrapper entrypoint."""
from os import path
import platform
import subprocess

ARCH_REPLACEMENT = {
    'aarch64': 'arm64',
}

def binary_path(os_name=None, arch=None):
    """Returns path to commitfmt binary for selected platform.
       If os_name or arch is None, the current platform is used."""
    if os_name is None:
        os_name = platform.system().lower()
    if arch is None:
        arch = platform.machine().lower()
    if arch in ARCH_REPLACEMENT:
        arch = ARCH_REPLACEMENT.get(arch)

    ext = ".exe" if os_name == "windows" else ""
    package = f"commitfmt_{os_name}"
    binary = f"commitfmt_{arch}{ext}"
    return path.join(path.dirname(__file__), "..", package, binary)

def run(args=None):
    """Run commitfmt."""
    binary = binary_path()
    if not path.isfile(binary):
        raise FileNotFoundError
    cmd = [binary]
    if args is not None:
        cmd += args

    result = subprocess.run(cmd, check=False)
    return result.returncode

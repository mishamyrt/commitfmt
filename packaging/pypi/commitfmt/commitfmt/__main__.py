#!/usr/bin/env python
"""commitfmt python wrapper script."""

import sys
from .commitfmt import run

def main():
    """Wrapper entrypoint."""
    try:
        ret = run(sys.argv[1:])
    except FileNotFoundError:
        print("commitfmt binary not found")
        ret = 1

    sys.exit(ret)

if __name__ == "__main__":
    sys.exit(run(sys.argv[1:]))

#!/bin/bash
set -euo pipefail

readonly REPO="mishamyrt/commitfmt"
readonly BINARY_NAME="commitfmt"

# Colors
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly NC='\033[0m'

error() {
    echo -e "${RED}Error: $1${NC}" >&2
    exit 1
}

warn() {
    echo -e "${YELLOW}Warning: $1${NC}" >&2
}

info() {
    echo -e "${GREEN}$1${NC}"
}

show_help() {
    cat << EOF
commitfmt installer

Usage: $0 [OPTIONS]

Options:
  -v, --version VERSION    Install specific version (e.g., v1.0.0)
  -h, --help              Show this help

Examples:
  $0                      Install latest version
  $0 -v v1.0.0           Install version v1.0.0

EOF
}

detect_platform() {
    local os arch

    case "$(uname -s)" in
        Linux*)   os="unknown-linux-gnu" ;;
        Darwin*)  os="apple-darwin" ;;
        CYGWIN*|MINGW*|MSYS*) os="pc-windows-msvc" ;;
        *) error "Unsupported OS: $(uname -s)" ;;
    esac

    case "$(uname -m)" in
        x86_64|amd64) arch="x86_64" ;;
        aarch64|arm64) arch="aarch64" ;;
        i386|i686) arch="i686" ;;
        *) error "Unsupported architecture: $(uname -m)" ;;
    esac

    PLATFORM="${arch}-${os}"

    if [[ "$os" == "pc-windows-msvc" ]]; then
        ARCHIVE_EXT="zip"
        EXTRACT_CMD="unzip -q"
    else
        ARCHIVE_EXT="tar.xz"
        # Try modern tar first, fallback to two-step extraction
        if tar --help | grep -q xz 2>/dev/null; then
            EXTRACT_CMD="tar -xf"
        else
            EXTRACT_CMD="extract_tar_xz"
        fi
    fi
}

get_latest_version() {
    if command -v curl >/dev/null 2>&1; then
        LATEST_VERSION=$(curl -sSf "https://api.github.com/repos/$REPO/releases/latest" |
                        grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    elif command -v wget >/dev/null 2>&1; then
        LATEST_VERSION=$(wget -qO- "https://api.github.com/repos/$REPO/releases/latest" |
                        grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    else
        error "curl or wget is required"
    fi

    [[ -n "$LATEST_VERSION" ]] || error "Failed to get latest version"
}

extract_tar_xz() {
    local filename="$1"
    local tar_file="${filename%.xz}"

    # Step 1: decompress .xz to .tar
    if command -v xz >/dev/null 2>&1; then
        xz -d "$filename" || error "xz decompression failed"
    elif command -v unxz >/dev/null 2>&1; then
        unxz "$filename" || error "unxz decompression failed"
    else
        error "No xz decompressor found (xz or unxz required)"
    fi

    # Step 2: extract .tar
    tar -xf "$tar_file" || error "tar extraction failed"
}

download_binary() {
    local version="$1"
    local filename="${BINARY_NAME}-${PLATFORM}.${ARCHIVE_EXT}"
    local url="https://github.com/$REPO/releases/download/$version/$filename"

    TEMP_DIR=$(mktemp -d)
    trap 'rm -rf "$TEMP_DIR"' EXIT

        info "Downloading $filename..."

    if command -v curl >/dev/null 2>&1; then
        curl -sSfL "$url" -o "$TEMP_DIR/$filename" || error "Download failed"
    else
        wget -q "$url" -O "$TEMP_DIR/$filename" || error "Download failed"
    fi

    # Extract archive
    cd "$TEMP_DIR"

    if [[ "$EXTRACT_CMD" == "extract_tar_xz" ]]; then
        extract_tar_xz "$filename"
    else
        $EXTRACT_CMD "$filename" || error "Extraction failed"
    fi

        ls commitfmt-aarch64-apple-darwin

    # Binary is inside platform-specific directory
    local platform_dir="commitfmt-${PLATFORM}"
    if [[ -f "$platform_dir/$BINARY_NAME" ]]; then
        mv "$platform_dir/$BINARY_NAME" . || error "Failed to move binary"
    elif [[ -f "$BINARY_NAME" ]]; then
        # Binary is in root (fallback)
        :
    else
        error "Binary not found in archive"
    fi
}

install_binary() {
    local install_dir

    # Determine install directory
    if [[ -w "/usr/local/bin" ]]; then
        install_dir="/usr/local/bin"
    elif [[ -w "$HOME/.local/bin" ]]; then
        install_dir="$HOME/.local/bin"
    elif [[ -d "$HOME/.local/bin" ]]; then
        install_dir="$HOME/.local/bin"
    else
        install_dir="$HOME/.local/bin"
        mkdir -p "$install_dir"
    fi

    local target="$install_dir/$BINARY_NAME"

    # Install binary
    if [[ -w "$install_dir" ]]; then
        cp "$BINARY_NAME" "$target"
        chmod +x "$target"
    elif command -v sudo >/dev/null 2>&1; then
        sudo cp "$BINARY_NAME" "$target"
        sudo chmod +x "$target"
    else
        error "Cannot write to $install_dir and sudo not available"
    fi

    info "Installed to $target"

    # Check PATH
    if [[ ":$PATH:" != *":$install_dir:"* ]]; then
        warn "$install_dir is not in PATH"
        echo "Add this to your shell profile:"
        echo "export PATH=\"\$PATH:$install_dir\""
    fi
}

main() {
    local version=""

    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -v|--version)
                [[ -n "${2:-}" ]] || error "Version not specified"
                version="$2"
                shift 2
                ;;
            -h|--help)
                show_help
                exit 0
                ;;
            *)
                error "Unknown option: $1"
                ;;
        esac
    done

    detect_platform

    if [[ -n "$version" ]]; then
        # Add 'v' prefix if missing
        [[ "$version" == v* ]] || version="v$version"
        LATEST_VERSION="$version"
    else
        get_latest_version
    fi

    download_binary "$LATEST_VERSION"
    install_binary

    info "Installation complete! Run 'commitfmt --version' to verify."
}

main "$@"

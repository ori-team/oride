#!/usr/bin/env bash
# ==============================================================================
# Oride — One-Line Universal Installer (Linux & macOS)
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/ori-team/oride/main/scripts/install.sh | bash
#
# Environment variables:
#   VERSION      - Specific version to install (default: latest, e.g. "v0.2.0")
#   INSTALL_DIR  - Directory to install the binary (default: ~/.local/bin)
#   NO_PATH_MOD  - Set to 1 to skip automatic PATH modification
# ==============================================================================
set -euo pipefail

REPO="ori-team/oride"
DEFAULT_VERSION="v0.2.0"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
NO_PATH_MOD="${NO_PATH_MOD:-0}"

# Colors for terminal output
BOLD="\033[1m"
GREEN="\033[32m"
BLUE="\033[34m"
YELLOW="\033[33m"
RED="\033[31m"
RESET="\033[0m"

log_info() {
    printf "${BLUE}==>${RESET} ${BOLD}%s${RESET}\n" "$1"
}

log_success() {
    printf "${GREEN}==>${RESET} ${BOLD}%s${RESET}\n" "$1"
}

log_warn() {
    printf "${YELLOW}==> WARNING:${RESET} %s\n" "$1"
}

log_error() {
    printf "${RED}==> ERROR:${RESET} %s\n" "$1" >&2
}

# 1. Detect Operating System
detect_os() {
    case "$(uname -s)" in
        Linux*)  echo "unknown-linux-gnu" ;;
        Darwin*) echo "apple-darwin" ;;
        *)
            log_error "Unsupported operating system: $(uname -s)."
            log_error "For Windows, use: irm https://raw.githubusercontent.com/ori-team/oride/main/scripts/install.ps1 | iex"
            exit 1
            ;;
    esac
}

# 2. Detect Hardware Architecture
detect_arch() {
    case "$(uname -m)" in
        x86_64|amd64)   echo "x86_64" ;;
        aarch64|arm64)  echo "aarch64" ;;
        *)
            log_error "Unsupported architecture: $(uname -m)."
            exit 1
            ;;
    esac
}

# 3. Resolve Target Tuple
resolve_target() {
    local os="$1"
    local arch="$2"
    echo "${arch}-${os}"
}

# 4. Resolve Target Version
resolve_version() {
    if [ -n "${VERSION:-}" ]; then
        echo "$VERSION"
        return
    fi

    # Query latest release tag from GitHub API, fallback to default
    local latest
    latest=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" 2>/dev/null | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/' || true)
    if [ -n "$latest" ]; then
        echo "$latest"
    else
        echo "$DEFAULT_VERSION"
    fi
}

# 5. Configure Shell PATH
configure_path() {
    local target_dir="$1"
    
    # Check if target_dir is already in PATH
    case ":$PATH:" in
        *":$target_dir:"*) return 0 ;;
    esac

    if [ "$NO_PATH_MOD" = "1" ]; then
        log_warn "PATH modification skipped by request. Please add '$target_dir' to your PATH manually."
        return 0
    fi

    log_info "Adding '$target_dir' to shell PATH..."
    local export_line="export PATH=\"\$HOME/.local/bin:\$PATH\""
    local modified_any=0

    # Bash
    if [ -f "$HOME/.bashrc" ] && ! grep -qs "$export_line" "$HOME/.bashrc"; then
        printf "\n# Oride terminal editor\n%s\n" "$export_line" >> "$HOME/.bashrc"
        modified_any=1
    fi

    # Zsh
    if [ -f "$HOME/.zshrc" ] && ! grep -qs "$export_line" "$HOME/.zshrc"; then
        printf "\n# Oride terminal editor\n%s\n" "$export_line" >> "$HOME/.zshrc"
        modified_any=1
    fi

    # Generic profile
    if [ -f "$HOME/.profile" ] && ! grep -qs "$export_line" "$HOME/.profile"; then
        printf "\n# Oride terminal editor\n%s\n" "$export_line" >> "$HOME/.profile"
        modified_any=1
    fi

    # Fish shell
    if [ -d "$HOME/.config/fish" ]; then
        local fish_config="$HOME/.config/fish/config.fish"
        mkdir -p "$HOME/.config/fish"
        if [ ! -f "$fish_config" ] || ! grep -qs "fish_add_path" "$fish_config"; then
            printf "\n# Oride terminal editor\nfish_add_path ~/.local/bin\n" >> "$fish_config"
            modified_any=1
        fi
    fi

    if [ "$modified_any" = "1" ]; then
        log_success "PATH updated in shell configuration files."
        log_info "Restart your terminal or run: export PATH=\"$target_dir:\$PATH\""
    fi
}

main() {
    printf "\n"
    printf "  ${BOLD}${GREEN}╭────────────────────────────────────────╮${RESET}\n"
    printf "  ${BOLD}${GREEN}│     Oride — Terminal Code Editor       │${RESET}\n"
    printf "  ${BOLD}${GREEN}│  Universal Fast Installer (v0.2.0)   │${RESET}\n"
    printf "  ${BOLD}${GREEN}╰────────────────────────────────────────╯${RESET}\n\n"

    local os arch target version
    os=$(detect_os)
    arch=$(detect_arch)
    target=$(resolve_target "$os" "$arch")
    version=$(resolve_version)

    # Ensure leading 'v'
    if [[ ! "$version" =~ ^v ]]; then
        version="v${version}"
    fi

    log_info "Platform detected: $target"
    log_info "Target version:    $version"

    local tarball="oride-${version}-${target}.tar.gz"
    local url="https://github.com/${REPO}/releases/download/${version}/${tarball}"

    local tmp_dir
    tmp_dir=$(mktemp -d)
    trap 'rm -rf "$tmp_dir"' EXIT

    log_info "Downloading $tarball from GitHub Releases..."
    if curl -fsSL "$url" -o "$tmp_dir/$tarball"; then
        log_info "Extracting archive..."
        tar -xzf "$tmp_dir/$tarball" -C "$tmp_dir"
        
        mkdir -p "$INSTALL_DIR"
        install -m 755 "$tmp_dir/oride" "$INSTALL_DIR/oride"
    else
        log_warn "Precompiled release binary not found at $url"
        
        # Check if cargo is available to build from source
        if command -v cargo >/dev/null 2>&1; then
            log_info "Cargo detected! Building Oride from source via cargo..."
            cargo install --git "https://github.com/${REPO}.git" --tag "$version" --bin oride --root "${INSTALL_DIR%/bin}"
        else
            log_error "Failed to download binary and Cargo is not installed."
            log_error "Please install Rust (https://rustup.rs) or visit https://github.com/${REPO}/releases"
            exit 1
        fi
    fi

    configure_path "$INSTALL_DIR"

    printf "\n"
    log_success "Oride has been successfully installed to: $INSTALL_DIR/oride"

    if command -v oride >/dev/null 2>&1 || [ -x "$INSTALL_DIR/oride" ]; then
        local installed_version
        installed_version=$("$INSTALL_DIR/oride" --version 2>/dev/null || echo "$version")
        printf "  Installed version: ${BOLD}%s${RESET}\n" "$installed_version"
    fi

    printf "\n"
    printf "  ${BOLD}Getting Started:${RESET}\n"
    printf "    oride .             # Open current folder in Oride\n"
    printf "    oride src/main.rs   # Open a specific file\n"
    printf "    oride --help        # View command-line options\n\n"
}

main "$@"

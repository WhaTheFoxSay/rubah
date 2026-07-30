#!/usr/bin/env bash
# ==============================================================================
# 🦊 RUBAH (Ruang Baca Harian) - Instant Lightweight One-Line Installer
# Usage: curl -fsSL https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/install.sh | bash
# ==============================================================================

set -e

RED='\031[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
RESET='\033[0m'

echo -e "${CYAN}${BOLD}"
echo "  🦊 RUBAH - Ruang Baca Harian"
echo "  ============================================"
echo "  Instant Pre-compiled Binary Installer"
echo -e "${RESET}"

INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

# Detect OS
OS_TYPE=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH_TYPE=$(uname -m)

case "$OS_TYPE" in
    darwin) OS="macos" ;;
    linux)  OS="linux" ;;
    msys*|cygwin*|mingw*) OS="windows" ;;
    *) echo -e "${RED}OS tidak didukung: $OS_TYPE${RESET}"; exit 1 ;;
esac

case "$ARCH_TYPE" in
    x86_64|amd64) ARCH="amd64" ;;
    arm64|aarch64) ARCH="arm64" ;;
    *) echo -e "${RED}Arsitektur CPU tidak didukung: $ARCH_TYPE${RESET}"; exit 1 ;;
esac

BINARY_NAME="rubah-${OS}-${ARCH}"
if [ "$OS" = "windows" ]; then
    BINARY_NAME="${BINARY_NAME}.exe"
fi

REPO="WhaTheFoxSay/rubah"
DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${BINARY_NAME}"

echo -e "${YELLOW}--> Mengunduh binary pre-compiled untuk ${OS} (${ARCH})...${RESET}"

TMP_FILE=$(mktemp /tmp/rubah_bin_XXXXXX)
trap 'rm -f "$TMP_FILE"' EXIT

# Try downloading pre-compiled binary
HTTP_CODE=$(curl -sL -w "%{http_code}" -o "$TMP_FILE" "$DOWNLOAD_URL" || echo "000")

if [ "$HTTP_CODE" -eq 200 ]; then
    echo -e "${GREEN}--> Download sukses! Memasang binary 'baca' ke $INSTALL_DIR...${RESET}"
    cp "$TMP_FILE" "$INSTALL_DIR/baca"
    chmod +x "$INSTALL_DIR/baca"
    ln -sf "$INSTALL_DIR/baca" "$INSTALL_DIR/rubah"
else
    echo -e "${YELLOW}--> Release binary belum ditemukan di GitHub Releases. Menggunakan fallback kompilasi lokal via Cargo...${RESET}"
    if ! command -v cargo &> /dev/null; then
        echo -e "${YELLOW}--> Cargo belum terinstall. Memasang Rust...${RESET}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
    TMP_DIR=$(mktemp -d /tmp/rubah_install_XXXXXX)
    git clone --depth 1 https://github.com/${REPO}.git "$TMP_DIR/rubah"
    cd "$TMP_DIR/rubah"
    cargo build --release
    cp target/release/rubah "$INSTALL_DIR/baca"
    chmod +x "$INSTALL_DIR/baca"
    ln -sf "$INSTALL_DIR/baca" "$INSTALL_DIR/rubah"
    rm -rf "$TMP_DIR"
fi

# Ensure ~/.local/bin is in PATH
PATH_ADDED=0
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    SHELL_PROFILE=""
    if [ -n "$ZSH_VERSION" ] || [ -f "$HOME/.zshrc" ]; then
        SHELL_PROFILE="$HOME/.zshrc"
    elif [ -n "$BASH_VERSION" ] || [ -f "$HOME/.bashrc" ]; then
        SHELL_PROFILE="$HOME/.bashrc"
    elif [ -f "$HOME/.profile" ]; then
        SHELL_PROFILE="$HOME/.profile"
    fi

    if [ -n "$SHELL_PROFILE" ]; then
        echo -e "\nexport PATH=\"\$HOME/.local/bin:\$PATH\"" >> "$SHELL_PROFILE"
        PATH_ADDED=1
    fi
fi

echo -e "${GREEN}${BOLD}"
echo "  ==========================================================="
echo "  🎉 Instalasi Rubah (Ruang Baca Harian) Berhasil Dituntaskan!"
echo "  ==========================================================="
echo -e "${RESET}"

if [ $PATH_ADDED -eq 1 ]; then
    echo -e "${CYAN}Silakan restart terminal Anda atau jalankan:${RESET}"
    echo -e "${YELLOW}  source $SHELL_PROFILE${RESET}\n"
fi

echo -e "${BOLD}Jalankan aplikasi cukup dengan mengetik:${RESET}"
echo -e "${GREEN}${BOLD}  baca${RESET}\n"

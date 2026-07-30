#!/usr/bin/env bash
# ==============================================================================
# 🦊 RUBAH (Ruang Baca Harian) - Instant Cross-Platform Installer
# Supported OS: Linux, macOS, Windows (PowerShell/CMD/GitBash), BSD, Haiku OS
# Usage: curl -fsSL https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/install.sh | bash
# ==============================================================================

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
RESET='\033[0m'

echo -e "${CYAN}${BOLD}"
echo "  🦊 RUBAH - Ruang Baca Harian"
echo "  ================================================="
echo "  Universal Installer (Linux, macOS, Windows, BSD, Haiku)"
echo -e "${RESET}"

INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

# Detect OS
OS_TYPE=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH_TYPE=$(uname -m)

case "$OS_TYPE" in
    darwin*)                   OS="macos" ;;
    linux*)                    OS="linux" ;;
    msys*|cygwin*|mingw*|win*) OS="windows" ;;
    freebsd*|openbsd*|netbsd*|dragonfly*) OS="bsd" ;;
    haiku*)                    OS="haiku" ;;
    *) echo -e "${RED}OS '$OS_TYPE' tidak dikenal. Menggunakan fallback mode...${RESET}"; OS="linux" ;;
esac

case "$ARCH_TYPE" in
    x86_64|amd64)   ARCH="amd64" ;;
    arm64|aarch64)  ARCH="arm64" ;;
    i386|i686)      ARCH="x86" ;;
    *)              ARCH="amd64" ;;
esac

BINARY_NAME="rubah-${OS}-${ARCH}"
if [ "$OS" = "windows" ]; then
    BINARY_NAME="${BINARY_NAME}.exe"
fi

REPO="WhaTheFoxSay/rubah"
DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${BINARY_NAME}"

echo -e "${YELLOW}--> Mendeteksi OS: ${BOLD}${OS}${RESET}${YELLOW} (${ARCH})...${RESET}"
echo -e "${YELLOW}--> Mengunduh pre-compiled binary instan...${RESET}"

TMP_FILE=$(mktemp /tmp/rubah_bin_XXXXXX 2>/dev/null || mktemp -t rubah_bin)
trap 'rm -f "$TMP_FILE"' EXIT

# Try downloading pre-compiled binary directly
HTTP_CODE=$(curl -sL -w "%{http_code}" -o "$TMP_FILE" "$DOWNLOAD_URL" || echo "000")

if [ "$HTTP_CODE" -eq 200 ]; then
    echo -e "${GREEN}--> Download sukses! Memasang binary 'baca' ke $INSTALL_DIR...${RESET}"
    cp "$TMP_FILE" "$INSTALL_DIR/baca"
    chmod +x "$INSTALL_DIR/baca"
    ln -sf "$INSTALL_DIR/baca" "$INSTALL_DIR/rubah"
else
    echo -e "${YELLOW}--> Pre-compiled binary rilis sedang diproses. Menjalankan fallback build via Cargo...${RESET}"
    if ! command -v cargo &> /dev/null; then
        echo -e "${YELLOW}--> Memasang Rust compiler...${RESET}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env" 2>/dev/null || true
    fi
    TMP_DIR=$(mktemp -d /tmp/rubah_install_XXXXXX 2>/dev/null || mktemp -d -t rubah_install)
    git clone --depth 1 https://github.${REPO}.git "$TMP_DIR/rubah" || curl -fsSL https://github.com/${REPO}/archive/refs/heads/main.tar.gz | tar -xz -C "$TMP_DIR"
    cd "$TMP_DIR/rubah"*
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
    elif [ -f "$HOME/config/settings/profile" ]; then # Haiku OS profile
        SHELL_PROFILE="$HOME/config/settings/profile"
    fi

    if [ -n "$SHELL_PROFILE" ]; then
        echo -e "\nexport PATH=\"\$HOME/.local/bin:\$PATH\"" >> "$SHELL_PROFILE"
        PATH_ADDED=1
    fi
fi

echo -e "${GREEN}${BOLD}"
echo "  ==========================================================="
echo "  🎉 Instalasi Rubah (Ruang Baca Harian) Selesai!"
echo "  ==========================================================="
echo -e "${RESET}"

if [ $PATH_ADDED -eq 1 ]; then
    echo -e "${CYAN}Silakan restart terminal atau jalankan:${RESET}"
    echo -e "${YELLOW}  source $SHELL_PROFILE${RESET}\n"
fi

echo -e "${BOLD}Jalankan aplikasi cukup dengan mengetik:${RESET}"
echo -e "${GREEN}${BOLD}  baca${RESET}\n"

#!/usr/bin/env bash
# ==============================================================================
# 🦊 Rubah (Ruang Baca Harian) - Cross-Platform Installer
# Supported OS: Linux, macOS, Windows, BSD, Haiku OS
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
echo "  🦊 Rubah - Ruang Baca Harian"
echo "  ================================================="
echo "  Installing Rubah RSS Feed Reader TUI..."
echo -e "${RESET}"

INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

# Detect OS & CPU Arch
OS_TYPE=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH_TYPE=$(uname -m)

case "$OS_TYPE" in
    darwin*)                   OS="macos" ;;
    linux*)                    OS="linux" ;;
    msys*|cygwin*|mingw*|win*) OS="windows" ;;
    *) OS="linux" ;;
esac

case "$ARCH_TYPE" in
    x86_64|amd64)   ARCH="amd64" ;;
    arm64|aarch64)  ARCH="arm64" ;;
    *)              ARCH="amd64" ;;
esac

BINARY_NAME="rubah-${OS}-${ARCH}"
if [ "$OS" = "windows" ]; then
    BINARY_NAME="${BINARY_NAME}.exe"
fi

REPO="WhaTheFoxSay/rubah"
RELEASE_URL="https://github.com/${REPO}/releases/download/v0.2.2/${BINARY_NAME}"
ARM64_FALLBACK_URL="https://github.com/${REPO}/releases/download/v0.2.2/rubah-${OS}-arm64"
LATEST_URL="https://github.com/${REPO}/releases/latest/download/${BINARY_NAME}"

echo -e "${YELLOW}--> OS: ${BOLD}${OS}${RESET}${YELLOW} (${ARCH})${RESET}"
echo -e "${YELLOW}--> Mengunduh binary rilis...${RESET}"

TMP_FILE=$(mktemp /tmp/rubah_bin_XXXXXX 2>/dev/null || mktemp -t rubah_bin)
trap 'rm -f "$TMP_FILE"' EXIT

DOWNLOAD_SUCCESS=0
USER_AGENT="Mozilla/5.0 (compatible; RubahInstaller/1.0)"

# Try release URL
HTTP_CODE=$(curl -sL -A "$USER_AGENT" -w "%{http_code}" -o "$TMP_FILE" "$RELEASE_URL" || echo "000")
if [ "$HTTP_CODE" -eq 200 ]; then
    DOWNLOAD_SUCCESS=1
fi

if [ $DOWNLOAD_SUCCESS -eq 0 ]; then
    HTTP_CODE=$(curl -sL -A "$USER_AGENT" -w "%{http_code}" -o "$TMP_FILE" "$ARM64_FALLBACK_URL" || echo "000")
    if [ "$HTTP_CODE" -eq 200 ]; then
        DOWNLOAD_SUCCESS=1
    fi
fi

if [ $DOWNLOAD_SUCCESS -eq 0 ]; then
    HTTP_CODE=$(curl -sL -A "$USER_AGENT" -w "%{http_code}" -o "$TMP_FILE" "$LATEST_URL" || echo "000")
    if [ "$HTTP_CODE" -eq 200 ]; then
        DOWNLOAD_SUCCESS=1
    fi
fi

if [ $DOWNLOAD_SUCCESS -eq 1 ]; then
    echo -e "${GREEN}--> Download berhasil. Memasang 'baca' ke $INSTALL_DIR...${RESET}"
    cp "$TMP_FILE" "$INSTALL_DIR/baca"
    chmod +x "$INSTALL_DIR/baca"
    ln -sf "$INSTALL_DIR/baca" "$INSTALL_DIR/rubah"
else
    echo -e "${RED}--> Gagal mengunduh binary rilis dari GitHub. Silakan periksa koneksi internet Anda.${RESET}"
    exit 1
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
    elif [ -f "$HOME/config/settings/profile" ]; then
        SHELL_PROFILE="$HOME/config/settings/profile"
    fi

    if [ -n "$SHELL_PROFILE" ]; then
        echo -e "\nexport PATH=\"\$HOME/.local/bin:\$PATH\"" >> "$SHELL_PROFILE"
        PATH_ADDED=1
    fi
fi

echo -e "${GREEN}${BOLD}"
echo "  ==========================================================="
echo "  🎉 Instalasi Rubah selesai!"
echo "  ==========================================================="
echo -e "${RESET}"

if [ $PATH_ADDED -eq 1 ]; then
    echo -e "${CYAN}Silakan restart terminal Anda atau jalankan:${RESET}"
    echo -e "${YELLOW}  source $SHELL_PROFILE${RESET}\n"
fi

echo -e "${BOLD}Jalankan aplikasi dengan mengetik:${RESET}"
echo -e "${GREEN}${BOLD}  baca${RESET}\n"

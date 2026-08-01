#!/usr/bin/env bash
# ==============================================================================
# 🦊 Rubah [Ruang Baca Harian] - Installer
# ==============================================================================

set -e

CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
WHITE='\033[1;37m'
GRAY='\033[0;90m'
RED='\033[0;31m'
BOLD='\033[1m'
RESET='\033[0m'

echo -e "${CYAN}${BOLD}--> 🦊 Rubah [Ruang Baca Harian]${RESET}"

INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

# Detect OS & CPU Architecture
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

if [ "$OS" = "macos" ]; then
    IS_ARM=$(sysctl -n hw.optional.arm64 2>/dev/null || echo "0")
    if [ "$ARCH_TYPE" = "x86_64" ]; then
        ARCH="amd64"
    elif [ "$IS_ARM" = "1" ] || [ "$ARCH_TYPE" = "arm64" ]; then
        ARCH="arm64"
    else
        ARCH="amd64"
    fi
fi

BINARY_NAME="rubah-${OS}-${ARCH}"
REPO="WhaTheFoxSay/rubah"
RELEASE_URL="https://github.com/${REPO}/releases/download/v0.8.0/${BINARY_NAME}"
LATEST_URL="https://github.com/${REPO}/releases/latest/download/${BINARY_NAME}"

echo -e "${GRAY}--> OS: ${OS} (${ARCH})${RESET}"
echo -e "${YELLOW}--> Mengunduh binary 'baca'...${RESET}"

TMP_FILE=$(mktemp /tmp/rubah_bin_XXXXXX 2>/dev/null || mktemp -t rubah_bin)
trap 'rm -f "$TMP_FILE"' EXIT

USER_AGENT="Mozilla/5.0 (compatible; RubahInstaller/1.0)"

HTTP_CODE=$(curl -sL -A "$USER_AGENT" -w "%{http_code}" -o "$TMP_FILE" "$RELEASE_URL" || echo "000")

# Check if downloaded binary is valid (at least 3MB)
FILE_SIZE=$(wc -c < "$TMP_FILE" 2>/dev/null || echo "0")
if [ "$HTTP_CODE" -ne 200 ] || [ "$FILE_SIZE" -lt 3000000 ]; then
    # Direct GitHub API stream fallback (bypasses GitHub release CDN BlobNotFound propagation delay)
    API_URL="https://api.github.com/repos/${REPO}/releases/tags/v0.8.0"
    ASSET_URL=$(curl -sL -A "$USER_AGENT" "$API_URL" | grep -B 2 -A 8 "\"name\": \"${BINARY_NAME}\"" | grep '"url":' | head -n 1 | cut -d '"' -f 4)
    if [ -n "$ASSET_URL" ]; then
        curl -sL -H "Accept: application/octet-stream" -A "$USER_AGENT" -o "$TMP_FILE" "$ASSET_URL" || true
        FILE_SIZE=$(wc -c < "$TMP_FILE" 2>/dev/null || echo "0")
    fi
fi

if [ "$FILE_SIZE" -lt 3000000 ]; then
    HTTP_CODE=$(curl -sL -A "$USER_AGENT" -w "%{http_code}" -o "$TMP_FILE" "$LATEST_URL" || echo "000")
    FILE_SIZE=$(wc -c < "$TMP_FILE" 2>/dev/null || echo "0")
fi

if [ "$FILE_SIZE" -lt 3000000 ]; then
    echo -e "${RED}Error: Gagal mengunduh binary 'baca' dari GitHub (File tidak lengkap atau rusak).${RESET}"
    exit 1
fi

cp "$TMP_FILE" "$INSTALL_DIR/baca"
chmod +x "$INSTALL_DIR/baca"
ln -sf "$INSTALL_DIR/baca" "$INSTALL_DIR/rubah"

# Clear shell binary location cache (bash / zsh / sh)
hash -r 2>/dev/null || true
rehash 2>/dev/null || true

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

echo -e "${GREEN}--> Instalasi selesai!${RESET}"

if [ $PATH_ADDED -eq 1 ]; then
    echo -e "${CYAN}Silakan restart terminal atau jalankan:${RESET} ${YELLOW}source $SHELL_PROFILE${RESET}"
fi

echo -e "${WHITE}Jalankan aplikasi dengan mengetik:${RESET} ${GREEN}${BOLD}baca${RESET}\n"

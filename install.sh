#!/usr/bin/env bash
# ==============================================================================
# 🦊 Rubah (Ruang Baca Harian) - Official Setup Wizard
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

clear
echo -e "${CYAN}${BOLD}"
echo "  ┌────────────────────────────────────────────────────────┐"
echo "  │ 🦊  RUBAH RSS READER - SETUP WIZARD                   │"
echo "  │     Retro Terminal User Interface Reader               │"
echo "  └────────────────────────────────────────────────────────┘"
echo -e "${RESET}"

INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

# Step 1: Detect System Architecture
echo -e "${YELLOW}[1/4] 🔍 Detecting operating system architecture...${RESET}"
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
echo -e "${GRAY}      --> Target Platform: ${BOLD}${OS} (${ARCH})${RESET}\n"

# Step 2: Establish Connection
echo -e "${YELLOW}[2/4] 🌐 Establishing secure connection to GitHub Releases...${RESET}"
REPO="WhaTheFoxSay/rubah"
RELEASE_URL="https://github.com/${REPO}/releases/download/v0.3.5/${BINARY_NAME}"
LATEST_URL="https://github.com/${REPO}/releases/latest/download/${BINARY_NAME}"
echo -e "${GRAY}      --> Source: github.com/${REPO}${RESET}\n"

# Step 3: Fast Download Pre-Compiled Binary
echo -e "${YELLOW}[3/4] 💾 Downloading pre-compiled binary package (~9.7 MB)...${RESET}"
TMP_FILE=$(mktemp /tmp/rubah_bin_XXXXXX 2>/dev/null || mktemp -t rubah_bin)
trap 'rm -f "$TMP_FILE"' EXIT

USER_AGENT="Mozilla/5.0 (compatible; RubahInstaller/1.0)"

HTTP_CODE=$(curl -sL -A "$USER_AGENT" -w "%{http_code}" -o "$TMP_FILE" "$RELEASE_URL" || echo "000")
if [ "$HTTP_CODE" -ne 200 ]; then
    HTTP_CODE=$(curl -sL -A "$USER_AGENT" -w "%{http_code}" -o "$TMP_FILE" "$LATEST_URL" || echo "000")
fi

if [ "$HTTP_CODE" -ne 200 ] || [ ! -s "$TMP_FILE" ]; then
    echo -e "${RED}❌ Error: Failed to download binary package. Please check your network connection.${RESET}"
    exit 1
fi
echo -e "${GREEN}      [████████████████████████████████████████] 100% Verified!${RESET}\n"

# Step 4: Installation & Symlinking
echo -e "${YELLOW}[4/4] ⚙️  Installing executable to $INSTALL_DIR/baca...${RESET}"
cp "$TMP_FILE" "$INSTALL_DIR/baca"
chmod +x "$INSTALL_DIR/baca"
ln -sf "$INSTALL_DIR/baca" "$INSTALL_DIR/rubah"

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

echo -e "\n${GREEN}${BOLD} ════════════════════════════════════════════════════════════${RESET}"
echo -e "${GREEN}${BOLD}  🎉 INSTALLATION COMPLETED SUCCESSFULLY!${RESET}"
echo -e "${GREEN}${BOLD} ════════════════════════════════════════════════════════════${RESET}\n"

if [ $PATH_ADDED -eq 1 ]; then
    echo -e "${CYAN}Please restart your terminal or run:${RESET}"
    echo -e "${YELLOW}  source $SHELL_PROFILE${RESET}\n"
fi

echo -e "${WHITE}${BOLD}Launch the application by typing:${RESET}"
echo -e "${GREEN}${BOLD}  baca${RESET}\n"

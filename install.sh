#!/usr/bin/env bash
# ==============================================================================
# 🦊 Rubah [Ruang Baca Harian] - Installer
# ==============================================================================

set -e

ESC=$(printf '\033')
CYAN="${ESC}[0;36m"
GREEN="${ESC}[0;32m"
YELLOW="${ESC}[1;33m"
WHITE="${ESC}[1;37m"
GRAY="${ESC}[0;90m"
ORANGE="${ESC}[38;2;235;115;0m"
BOLD="${ESC}[1m"
RESET="${ESC}[0m"

step() {
    local label="$1"
    local detail="$2"
    printf "  ${GREEN}✔${RESET} %-25s ${GRAY}%s${RESET}\n" "$label" "$detail"
}

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

USER_AGENT="Mozilla/5.0 (compatible; RubahInstaller/1.0)"

# Fetch latest release version tag dynamically from GitHub API
LATEST_TAG=$(curl -sL -A "$USER_AGENT" "https://api.github.com/repos/${REPO}/releases/latest" | grep -o '"tag_name": *"[^"]*"' | head -n 1 | cut -d '"' -f 4)
if [ -n "$LATEST_TAG" ]; then
    VERSION="${LATEST_TAG#v}"
else
    VERSION="1.7.0"
fi

echo ""
echo -e "  ${ORANGE}${BOLD}🦊 RUBAH${RESET} ${WHITE}${BOLD}[Ruang Baca Harian]${RESET} ${GRAY}v${VERSION}${RESET}"
echo -e "  ${GRAY}High-Performance RSS Feed Reader TUI${RESET}\n"

step "System environment" "${OS} (${ARCH})"

RELEASE_URL="https://github.com/${REPO}/releases/download/v${VERSION}/${BINARY_NAME}"
LATEST_URL="https://github.com/${REPO}/releases/latest/download/${BINARY_NAME}"

TMP_FILE=$(mktemp /tmp/rubah_bin_XXXXXX 2>/dev/null || mktemp -t rubah_bin)
trap 'rm -f "$TMP_FILE"' EXIT

HTTP_CODE=$(curl -sL -A "$USER_AGENT" -w "%{http_code}" -o "$TMP_FILE" "$RELEASE_URL" || echo "000")

# Check if downloaded binary is valid (at least 3MB)
FILE_SIZE=$(wc -c < "$TMP_FILE" 2>/dev/null || echo "0")
if [ "$HTTP_CODE" -ne 200 ] || [ "$FILE_SIZE" -lt 3000000 ]; then
    API_URL="https://api.github.com/repos/${REPO}/releases/tags/v${VERSION}"
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
    echo -e "\n  ${ESC}[0;31mError: Failed to download 'baca' binary from GitHub.${RESET}\n"
    exit 1
fi

SIZE_MB=$(awk "BEGIN {printf \"%.1f MB\", $FILE_SIZE/1048576}")
step "Download executable" "v${VERSION} (${SIZE_MB})"

cp "$TMP_FILE" "$INSTALL_DIR/baca"
chmod +x "$INSTALL_DIR/baca"
ln -sf "$INSTALL_DIR/baca" "$INSTALL_DIR/rubah"

step "Install binary & symlink" "~/.local/bin/baca"

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

step "Shell lookup reset" "Hash memory cleared"

echo ""
echo -e "  ${GREEN}${BOLD}✔ Rubah v${VERSION} successfully installed!${RESET}"

if [ $PATH_ADDED -eq 1 ]; then
    echo -e "  ${CYAN}Please restart terminal or run:${RESET} ${YELLOW}source $SHELL_PROFILE${RESET}"
fi

echo -e "  ${WHITE}Run the application by typing:${RESET} ${ORANGE}${BOLD}baca${RESET}\n"

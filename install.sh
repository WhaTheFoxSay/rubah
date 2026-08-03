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
    if [ "$IS_ARM" = "1" ] || [ "$ARCH_TYPE" = "arm64" ] || [ "$ARCH_TYPE" = "aarch64" ]; then
        ARCH="arm64"
    else
        ARCH="amd64"
    fi
fi

BINARY_NAME="rubah-${OS}-${ARCH}"
REPO="WhaTheFoxSay/rubah"

USER_AGENT="Mozilla/5.0 (compatible; RubahInstaller/1.0)"

# Fetch latest release version tag dynamically from GitHub API
LATEST_TAG=$(curl -sL --connect-timeout 5 -A "$USER_AGENT" "https://api.github.com/repos/${REPO}/releases/latest" 2>/dev/null | grep -o '"tag_name": *"[^"]*"' | head -n 1 | cut -d '"' -f 4 || echo "")
if [ -n "$LATEST_TAG" ]; then
    VERSION="${LATEST_TAG#v}"
else
    VERSION="1.7.8"
fi

echo ""
echo -e "  ${ORANGE}${BOLD}🦊 RUBAH${RESET} ${WHITE}${BOLD}[Ruang Baca Harian]${RESET} ${GRAY}v${VERSION}${RESET}"
echo -e "  ${GRAY}High-Performance RSS Feed Reader TUI${RESET}\n"

step "System environment" "${OS} (${ARCH})"

RELEASE_URL="https://github.com/${REPO}/releases/download/v${VERSION}/${BINARY_NAME}"
LATEST_URL="https://github.com/${REPO}/releases/latest/download/${BINARY_NAME}"

TMP_FILE=$(mktemp /tmp/rubah_bin_XXXXXX 2>/dev/null || mktemp -t rubah_bin)
trap 'rm -f "$TMP_FILE"' EXIT

try_download() {
    local url="$1"
    [ -z "$url" ] && return 1
    for try in 1 2 3; do
        rm -f "$TMP_FILE"
        curl -sL --connect-timeout 15 --max-time 120 -A "$USER_AGENT" -o "$TMP_FILE" "$url" 2>/dev/null || true
        FILE_SIZE=$(wc -c < "$TMP_FILE" 2>/dev/null | tr -d ' \t\r\n' || echo "0")
        FILE_SIZE=${FILE_SIZE:-0}
        if [ "$FILE_SIZE" -ge 3000000 ]; then
            return 0
        fi
        sleep 2
    done
    return 1
}

if ! try_download "$RELEASE_URL"; then
    if ! try_download "$LATEST_URL"; then
        API_URL="https://api.github.com/repos/${REPO}/releases/tags/v${VERSION}"
        ASSET_URL=$(curl -sL --connect-timeout 5 -A "$USER_AGENT" "$API_URL" 2>/dev/null | grep -B 2 -A 8 "\"name\": \"${BINARY_NAME}\"" | grep '"url":' | head -n 1 | cut -d '"' -f 4 || echo "")
        if [ -n "$ASSET_URL" ]; then
            try_download "$ASSET_URL" || true
        fi
    fi
fi

FILE_SIZE=$(wc -c < "$TMP_FILE" 2>/dev/null || echo "0")
if [ "$FILE_SIZE" -lt 3000000 ]; then
    echo -e "\n  ${ESC}[0;31mError: Failed to download 'baca' binary from GitHub.${RESET}"
    echo -e "  ${GRAY}GitHub Actions may still be compiling the latest release (~1-2 mins). Please try again shortly!${RESET}\n"
    exit 1
fi

SIZE_MB=$(awk "BEGIN {printf \"%.1f MB\", $FILE_SIZE/1048576}")
step "Download executable" "v${VERSION} (${SIZE_MB})"

cp "$TMP_FILE" "$INSTALL_DIR/baca"
chmod +x "$INSTALL_DIR/baca"

if [ "$OS" = "macos" ]; then
    xattr -c "$INSTALL_DIR/baca" 2>/dev/null || true
    codesign -f -s - "$INSTALL_DIR/baca" 2>/dev/null || true
fi

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

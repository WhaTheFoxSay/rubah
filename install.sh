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
VERSION="0.9.1"
RELEASE_URL="https://github.com/${REPO}/releases/download/v${VERSION}/${BINARY_NAME}"
LATEST_URL="https://github.com/${REPO}/releases/latest/download/${BINARY_NAME}"

draw_table_installer() {
    local percent=$1
    local s1=$2
    local s2=$3
    local s3=$4
    local s4=$5

    local width=30
    local filled=$(( percent * width / 100 ))
    local empty=$(( width - filled ))
    local bar_filled=""
    local bar_empty=""
    local i
    for ((i=0; i<filled; i++)); do bar_filled="${bar_filled}█"; done
    for ((i=0; i<empty; i++)); do bar_empty="${bar_empty}░"; done

    printf "\033[10A\033[J"
    echo -e "${CYAN}┌──────────────────────────────┬─────────────────────────────┐${RESET}"
    printf "${CYAN}│${RESET} ${BOLD}🦊 RUBAH v%-18s${RESET} ${CYAN}│${RESET} ${GRAY}%-27s${RESET} ${CYAN}│${RESET}\n" "$VERSION" "System: ${OS}/${ARCH}"
    echo -e "${CYAN}├──────────────────────────────┼─────────────────────────────┤${RESET}"
    printf "${CYAN}│${RESET} ${BOLD}%-28s${RESET} ${CYAN}│${RESET} ${BOLD}%-27s${RESET} ${CYAN}│${RESET}\n" "Komponen / Tahap" "Status / Detail"
    echo -e "${CYAN}├──────────────────────────────┼─────────────────────────────┤${RESET}"
    printf "${CYAN}│${RESET} %-28s ${CYAN}│${RESET} %-27s ${CYAN}│${RESET}\n" "1. System Environment" "$s1"
    printf "${CYAN}│${RESET} %-28s ${CYAN}│${RESET} %-27s ${CYAN}│${RESET}\n" "2. Download Executable" "$s2"
    printf "${CYAN}│${RESET} %-28s ${CYAN}│${RESET} %-27s ${CYAN}│${RESET}\n" "3. Permissions & Symlinks" "$s3"
    printf "${CYAN}│${RESET} %-28s ${CYAN}│${RESET} %-27s ${CYAN}│${RESET}\n" "4. Shell Lookup Reset" "$s4"
    echo -e "${CYAN}├──────────────────────────────┴─────────────────────────────┤${RESET}"
    printf "${CYAN}│${RESET} Progress: [${CYAN}%s%s${RESET}] ${BOLD}%3d%%${RESET} %-8s ${CYAN}│${RESET}\n" "$bar_filled" "$bar_empty" "$percent" ""
    echo -e "${CYAN}└────────────────────────────────────────────────────────────┘${RESET}"
}

# Print 10 blank lines initially for table row replacement
for i in {1..10}; do echo ""; done

s1="${YELLOW}Memproses...${RESET}"
s2="${GRAY}Menunggu...${RESET}"
s3="${GRAY}Menunggu...${RESET}"
s4="${GRAY}Menunggu...${RESET}"

draw_table_installer 10 "$s1" "$s2" "$s3" "$s4"
sleep 0.1
s1="${GREEN}[✔] OK (~/.local/bin)${RESET}"

draw_table_installer 30 "$s1" "${YELLOW}Mengunduh v${VERSION}...${RESET}" "$s3" "$s4"
TMP_FILE=$(mktemp /tmp/rubah_bin_XXXXXX 2>/dev/null || mktemp -t rubah_bin)
trap 'rm -f "$TMP_FILE"' EXIT

USER_AGENT="Mozilla/5.0 (compatible; RubahInstaller/1.0)"

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
    echo -e "\n${RED}Error: Gagal mengunduh binary 'baca' dari GitHub (File tidak lengkap atau rusak).${RESET}"
    exit 1
fi

SIZE_MB=$(awk "BEGIN {printf \"%.1f MB\", $FILE_SIZE/1048576}")
s2="${GREEN}[✔] v${VERSION} (${SIZE_MB})${RESET}"

draw_table_installer 70 "$s1" "$s2" "${YELLOW}Memasang...${RESET}" "$s4"
cp "$TMP_FILE" "$INSTALL_DIR/baca"
chmod +x "$INSTALL_DIR/baca"
ln -sf "$INSTALL_DIR/baca" "$INSTALL_DIR/rubah"
s3="${GREEN}[✔] ~/.local/bin/baca${RESET}"

draw_table_installer 90 "$s1" "$s2" "$s3" "${YELLOW}Reset cache...${RESET}"
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

s4="${GREEN}[✔] Shell Hash Reset${RESET}"
draw_table_installer 100 "$s1" "$s2" "$s3" "$s4"

echo -e "\n${GREEN}${BOLD}[✔] Rubah v${VERSION} berhasil terinstall di sistem Anda!${RESET}"

if [ $PATH_ADDED -eq 1 ]; then
    echo -e "${CYAN}Silakan restart terminal atau jalankan:${RESET} ${YELLOW}source $SHELL_PROFILE${RESET}"
fi

echo -e "${WHITE}Jalankan aplikasi dengan mengetik:${RESET} ${GREEN}${BOLD}baca${RESET}\n"

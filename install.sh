#!/usr/bin/env bash
# ==============================================================================
# 🦊 RUBAH (Ruang Baca Harian) - One-Line Installer Script
# Usage: curl -fsSL https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/install.sh | bash
# ==============================================================================

set -e

RED='\030[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
RESET='\033[0m'

echo -e "${CYAN}${BOLD}"
echo "  🦊 RUBAH - Ruang Baca Harian"
echo "  ============================================"
echo "  High-Performance Cross-Platform RSS TUI"
echo -e "${RESET}"

INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

TMP_DIR=$(mktemp -d /tmp/rubah_install_XXXXXX)
trap 'rm -rf "$TMP_DIR"' EXIT

echo -e "${YELLOW}--> Memeriksa dependensi sistem...${RESET}"

# Check for Rust / Cargo
if ! command -v cargo &> /dev/null; then
    echo -e "${YELLOW}--> Rust/Cargo belum terinstall. Mengunduh installer Rust (rustup)...${RESET}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

echo -e "${YELLOW}--> Mengunduh source code terbaru Rubah...${RESET}"
if command -v git &> /dev/null; then
    git clone --depth 1 https://github.com/WhaTheFoxSay/rubah.git "$TMP_DIR/rubah"
else
    curl -fsSL https://github.com/WhaTheFoxSay/rubah/archive/refs/heads/main.tar.gz | tar -xz -C "$TMP_DIR"
    mv "$TMP_DIR"/rubah-main "$TMP_DIR/rubah"
fi

echo -e "${YELLOW}--> Mengompilasi binary rilis Rubah (optimasi tinggi)...${RESET}"
cd "$TMP_DIR/rubah"
cargo build --release

echo -e "${YELLOW}--> Memasang binary 'baca' ke $INSTALL_DIR...${RESET}"
cp target/release/rubah "$INSTALL_DIR/baca"
chmod +x "$INSTALL_DIR/baca"
ln -sf "$INSTALL_DIR/baca" "$INSTALL_DIR/rubah"

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

echo -e "${BOLD}Sekarang Anda bisa langsung membaca berita cukup dengan mengetik:${RESET}"
echo -e "${GREEN}${BOLD}  baca${RESET}"
echo -e "  (atau ${GREEN}rubah${RESET})\n"

#!/usr/bin/env bash
# ==============================================================================
# 🦊 Rubah [Ruang Baca Harian] - Uninstaller
# ==============================================================================

set -e

CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
WHITE='\033[1;37m'
GRAY='\033[0;90m'
BOLD='\033[1m'
RESET='\033[0m'

echo -e "${CYAN}${BOLD}--> 🦊 Rubah [Ruang Baca Harian] Uninstaller${RESET}"

echo -e "${YELLOW}--> Menghapus binary executable 'baca'...${RESET}"
rm -f "$HOME/.local/bin/baca" "$HOME/.local/bin/rubah"

echo -e "${YELLOW}--> Menghapus data konfigurasi & database...${RESET}"
rm -rf "$HOME/.config/rubah"

echo -e "${YELLOW}--> Menghapus cache & file sementara...${RESET}"
rm -rf "$HOME/.cache/rubah" 2>/dev/null || true

# Clear shell binary location cache (bash / zsh / sh)
hash -r 2>/dev/null || true
rehash 2>/dev/null || true

echo -e "${GREEN}--> Uninstall berhasil selesai.${RESET}"
echo -e "${WHITE}Terima kasih telah menggunakan Rubah [Ruang Baca Harian].${RESET}"
echo -e "${CYAN}Sampai jumpa kembali! 🦊${RESET}\n"

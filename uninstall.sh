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

draw_progress() {
    local percent=$1
    local step_name=$2
    local width=24
    local filled=$(( percent * width / 100 ))
    local empty=$(( width - filled ))
    local bar_filled=$(printf '%*s' "$filled" '' | tr ' ' '█')
    local bar_empty=$(printf '%*s' "$empty" '' | tr ' ' '░')
    printf "\r\033[K  ${CYAN}[${bar_filled}${bar_empty}]${RESET} ${BOLD}%3d%%${RESET} | ${YELLOW}%s${RESET}" "$percent" "$step_name"
}

show_step() {
    local step_name=$1
    echo -e "  ${GREEN}[✔]${RESET} ${WHITE}${step_name}${RESET}"
}

draw_progress 25 "Menghapus binary executable 'baca' & shortcuts..."
rm -f "$HOME/.local/bin/baca" "$HOME/.local/bin/rubah"
sleep 0.1
show_step "Menghapus binary executable 'baca' & shortcuts..."

draw_progress 60 "Menghapus data konfigurasi & database..."
rm -rf "$HOME/.config/rubah"
rm -rf "$HOME/.cache/rubah" 2>/dev/null || true
sleep 0.1
show_step "Menghapus data konfigurasi, cache & database..."

draw_progress 90 "Membersihkan memori cache shell (hash -r)..."
hash -r 2>/dev/null || true
rehash 2>/dev/null || true

draw_progress 100 "Uninstall selesai!"
sleep 0.1
show_step "Membersihkan memori cache shell (hash -r)..."
echo -e ""

echo -e "${GREEN}${BOLD}[✔] Aplikasi Rubah berhasil di-uninstall dari sistem Anda.${RESET}"
echo -e "${WHITE}Terima kasih telah menggunakan Rubah [Ruang Baca Harian].${RESET}"
echo -e "${CYAN}Sampai jumpa kembali! 🦊${RESET}\n"

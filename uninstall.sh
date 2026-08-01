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

draw_table_uninstaller() {
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
    printf "${CYAN}│${RESET} ${BOLD}🦊 RUBAH UNINSTALLER${RESET}          ${CYAN}│${RESET} ${GRAY}%-27s${RESET} ${CYAN}│${RESET}\n" "System: Clean & Removal"
    echo -e "${CYAN}├──────────────────────────────┼─────────────────────────────┤${RESET}"
    printf "${CYAN}│${RESET} ${BOLD}%-28s${RESET} ${CYAN}│${RESET} ${BOLD}%-27s${RESET} ${CYAN}│${RESET}\n" "Komponen / Tahap" "Status / Detail"
    echo -e "${CYAN}├──────────────────────────────┼─────────────────────────────┤${RESET}"
    printf "${CYAN}│${RESET} %-28s ${CYAN}│${RESET} %-27s ${CYAN}│${RESET}\n" "1. Binary Executable" "$s1"
    printf "${CYAN}│${RESET} %-28s ${CYAN}│${RESET} %-27s ${CYAN}│${RESET}\n" "2. Config & Database" "$s2"
    printf "${CYAN}│${RESET} %-28s ${CYAN}│${RESET} %-27s ${CYAN}│${RESET}\n" "3. Temporary Files & Cache" "$s3"
    printf "${CYAN}│${RESET} %-28s ${CYAN}│${RESET} %-27s ${CYAN}│${RESET}\n" "4. Shell Hash Lookup Reset" "$s4"
    echo -e "${CYAN}├──────────────────────────────┴─────────────────────────────┤${RESET}"
    printf "${CYAN}│${RESET} Progress: [${CYAN}%s%s${RESET}] ${BOLD}%3d%%${RESET} %-8s ${CYAN}│${RESET}\n" "$bar_filled" "$bar_empty" "$percent" ""
    echo -e "${CYAN}└────────────────────────────────────────────────────────────┘${RESET}"
}

for i in {1..10}; do echo ""; done

s1="${YELLOW}Menghapus...${RESET}"
s2="${GRAY}Menunggu...${RESET}"
s3="${GRAY}Menunggu...${RESET}"
s4="${GRAY}Menunggu...${RESET}"

draw_table_uninstaller 25 "$s1" "$s2" "$s3" "$s4"
rm -f "$HOME/.local/bin/baca" "$HOME/.local/bin/rubah"
sleep 0.1
s1="${GREEN}[✔] Terhapus (~/.local/bin)${RESET}"

draw_table_uninstaller 60 "$s1" "${YELLOW}Menghapus...${RESET}" "$s3" "$s4"
rm -rf "$HOME/.config/rubah"
s2="${GREEN}[✔] Terhapus (~/.config)${RESET}"

draw_table_uninstaller 80 "$s1" "$s2" "${YELLOW}Menghapus...${RESET}" "$s4"
rm -rf "$HOME/.cache/rubah" 2>/dev/null || true
s3="${GREEN}[✔] Terhapus (~/.cache)${RESET}"

draw_table_uninstaller 95 "$s1" "$s2" "$s3" "${YELLOW}Reset cache...${RESET}"
hash -r 2>/dev/null || true
rehash 2>/dev/null || true
s4="${GREEN}[✔] Shell Hash Reset${RESET}"

draw_table_uninstaller 100 "$s1" "$s2" "$s3" "$s4"

echo -e "\n${GREEN}${BOLD}[✔] Aplikasi Rubah berhasil di-uninstall dari sistem Anda.${RESET}"
echo -e "${WHITE}Terima kasih telah menggunakan Rubah [Ruang Baca Harian].${RESET}"
echo -e "${CYAN}Sampai jumpa kembali! 🦊${RESET}\n"

#!/usr/bin/env bash
# ==============================================================================
# 🦊 Rubah [Ruang Baca Harian] - Uninstaller
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

echo ""
echo -e "  ${ORANGE}${BOLD}🦊 RUBAH${RESET} ${WHITE}${BOLD}[Ruang Baca Harian] Uninstaller${RESET}"
echo -e "  ${GRAY}High-Performance RSS Feed Reader TUI${RESET}\n"

step() {
    local label="$1"
    local detail="$2"
    printf "  ${GREEN}✔${RESET} %-25s ${GRAY}%s${RESET}\n" "$label" "$detail"
}

rm -f "$HOME/.local/bin/baca" "$HOME/.local/bin/rubah"
step "Binary & symlink" "~/.local/bin/baca deleted"

rm -rf "$HOME/.config/rubah"
step "Config & database" "~/.config/rubah deleted"

rm -rf "$HOME/.cache/rubah" 2>/dev/null || true
step "Cache & temp files" "~/.cache/rubah deleted"

hash -r 2>/dev/null || true
rehash 2>/dev/null || true
step "Shell lookup reset" "Hash memory cleared"

echo ""
echo -e "  ${GREEN}${BOLD}✔ Rubah application successfully uninstalled from your system.${RESET}"
echo -e "  ${WHITE}Thank you for using Rubah [Ruang Baca Harian].${RESET}"
echo -e "  ${ORANGE}See you again! 🦊${RESET}\n"

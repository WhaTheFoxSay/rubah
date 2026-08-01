#!/usr/bin/env bash
# ==============================================================================
# 🦊 Rubah - Uninstaller (English Default & Indonesian Support)
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

# Language detection (Default: English 'en', Indonesian 'id' if specified)
LANG_ENV=$(echo "${LANG:-}${LANGUAGE:-}${LC_ALL:-}" | tr '[:upper:]' '[:lower:]')
LANG_CHOICE="en"
if [ "$1" = "id" ] || [[ "$LANG_ENV" == id* ]]; then
    LANG_CHOICE="id"
fi

if [ "$LANG_CHOICE" = "id" ]; then
    SUBTITLE="Ruang Baca Harian"
    BIN_DETAIL="~/.local/bin/baca terhapus"
    CFG_DETAIL="~/.config/rubah terhapus"
    CACHE_DETAIL="~/.cache/rubah terhapus"
    HASH_LABEL="Reset lookup shell"
    HASH_DETAIL="Memori hash dibersihkan"
    DONE_MSG="Aplikasi Rubah berhasil di-uninstall dari sistem Anda."
    THANKS_MSG="Terima kasih telah menggunakan Rubah [Ruang Baca Harian]."
    BYE_MSG="Sampai jumpa kembali! 🦊"
else
    SUBTITLE="Daily Reading Space"
    BIN_DETAIL="~/.local/bin/baca deleted"
    CFG_DETAIL="~/.config/rubah deleted"
    CACHE_DETAIL="~/.cache/rubah deleted"
    HASH_LABEL="Shell lookup reset"
    HASH_DETAIL="Hash memory cleared"
    DONE_MSG="Rubah application successfully uninstalled from your system."
    THANKS_MSG="Thank you for using Rubah [Daily Reading Space]."
    BYE_MSG="See you again! 🦊"
fi

echo ""
echo -e "  ${ORANGE}${BOLD}🦊 RUBAH${RESET} ${WHITE}${BOLD}[${SUBTITLE}] Uninstaller${RESET}"
echo -e "  ${GRAY}High-Performance RSS Feed Reader TUI${RESET}\n"

step() {
    local label="$1"
    local detail="$2"
    printf "  ${GREEN}✔${RESET} %-25s ${GRAY}%s${RESET}\n" "$label" "$detail"
}

rm -f "$HOME/.local/bin/baca" "$HOME/.local/bin/rubah"
step "Binary & symlink" "$BIN_DETAIL"

rm -rf "$HOME/.config/rubah"
step "Config & database" "$CFG_DETAIL"

rm -rf "$HOME/.cache/rubah" 2>/dev/null || true
step "Cache & temp files" "$CACHE_DETAIL"

hash -r 2>/dev/null || true
rehash 2>/dev/null || true
step "$HASH_LABEL" "$HASH_DETAIL"

echo ""
echo -e "  ${GREEN}${BOLD}✔ ${DONE_MSG}${RESET}"
echo -e "  ${WHITE}${THANKS_MSG}${RESET}"
echo -e "  ${ORANGE}${BYE_MSG}${RESET}\n"

#!/usr/bin/env bash
# ==============================================================================
# 🦊 Rubah RSS Reader - Official Uninstall Wizard
# ==============================================================================

set -e

CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
WHITE='\033[1;37m'
GRAY='\033[0;90m'
BOLD='\033[1m'
RESET='\033[0m'

clear
echo -e "${CYAN}${BOLD}"
echo "  ┌────────────────────────────────────────────────────────┐"
echo "  │ 🦊  RUBAH RSS READER - UNINSTALL WIZARD               │"
echo "  └────────────────────────────────────────────────────────┘"
echo -e "${RESET}"

echo -e "${YELLOW}[1/3] 🗑️  Removing binary executable 'baca' & aliases...${RESET}"
rm -f "$HOME/.local/bin/baca" "$HOME/.local/bin/rubah"
echo -e "${GRAY}      --> Executables removed.${RESET}\n"

echo -e "${YELLOW}[2/3] 📂 Cleaning local configuration & database storage...${RESET}"
rm -rf "$HOME/.config/rubah"
echo -e "${GRAY}      --> Database & settings purged.${RESET}\n"

echo -e "${YELLOW}[3/3] 🧹 Purging cache & temporary files...${RESET}"
rm -rf "$HOME/.cache/rubah" 2>/dev/null || true
echo -e "${GRAY}      --> Cache purged.${RESET}\n"

echo -e "${GREEN}${BOLD} ════════════════════════════════════════════════════════════${RESET}"
echo -e "${GREEN}${BOLD}  👋 UNINSTALL COMPLETED SUCCESSFULLY!${RESET}"
echo -e "${GREEN}${BOLD} ════════════════════════════════════════════════════════════${RESET}\n"

echo -e "${WHITE}${BOLD}Thank you for trying Rubah RSS Reader!${RESET}"
echo -e "${CYAN}We hope to see you again soon. 🦊✨${RESET}\n"

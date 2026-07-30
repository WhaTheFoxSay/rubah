#!/usr/bin/env bash
# ==============================================================================
# 🦊 RUBAH (Ruang Baca Harian) - Uninstall Script
# Usage: curl -fsSL https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/uninstall.sh | bash
# ==============================================================================

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
RESET='\033[0m'

echo -e "${YELLOW}${BOLD}"
echo "  🗑️  Menghapus Rubah (Ruang Baca Harian)..."
echo -e "${RESET}"

# Remove binary files
rm -f "$HOME/.local/bin/baca"
rm -f "$HOME/.local/bin/rubah"

# Optionally remove user database and config
if [ -d "$HOME/.config/rubah" ]; then
    rm -rf "$HOME/.config/rubah"
fi

echo -e "${GREEN}${BOLD}"
echo "  ========================================================"
echo "  ✅ Rubah dan konfigurasi berhasil dihapus dari sistem."
echo "  ========================================================"
echo -e "${RESET}"

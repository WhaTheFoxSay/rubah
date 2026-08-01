<div align="center">

# 🦊 Rubah
### Ruang Baca Harian

A standards-compliant, local-first RSS/Atom feed reader for the terminal.

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows%20%7C%20BSD%20%7C%20Haiku-brightgreen.svg)]()

[Installation](#installation) • [Getting Started](#getting-started) • [Key Features](#key-features) • [Keyboard Shortcuts](#keyboard-shortcuts) • [CLI Commands](#cli-commands) • [License & Privacy](#license--privacy)

</div>

---

**Rubah** (Ruang Baca Harian) is a lightweight, fast, cross-platform RSS & Atom Feed reader powered by a modern Terminal User Interface (TUI). It runs natively on **Linux**, **macOS**, **Windows**, **BSD**, and **Haiku OS**. Built using **Rust**, **Ratatui**, and **Tokio**.

> [!NOTE]
> **Local-First Architecture**: Rubah fetches news feeds directly from publishers to your local device without using intermediary servers or telemetry tracking.

---

## Installation

### Linux / macOS / BSD / Haiku OS:
Run the following command in your terminal:
```bash
curl -fsSL https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/install.sh | bash
```

### Windows (PowerShell / CMD):

**Method 1: One-Line PowerShell Command**
```powershell
irm https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/install.ps1 | iex
```

**Method 2: Direct Executable Download (.exe)**
1. Download the official binary: **[rubah-windows-amd64.exe](https://github.com/WhaTheFoxSay/rubah/releases/latest/download/rubah-windows-amd64.exe)**
2. Rename the downloaded file to `baca.exe`.
3. Launch via CMD or PowerShell by typing `.\baca.exe`.

---

## Getting Started

Once installed, launch the application in your terminal by typing:
```bash
baca
```

*(Alternatively using `rubah`)*

---

## Key Features

- **3-Pane TUI Layout**: Dedicated panes for Categories/Channels, Article List, and Content Reader.
- **Multi-Language Support**: Switch between 6 languages (English, Indonesian, Japanese, Dutch, Spanish, Arabic).
- **Article & Image Reader**: Read full articles and render news photos directly inside the terminal.
- **Realtime Search**: Instantly filter articles by keyword.
- **Bookmarks & Categories**: Bookmark favorite articles and organize feeds by category.

---

## Keyboard Shortcuts

| Shortcut | Action |
| :--- | :--- |
| <kbd>Tab</kbd> / <kbd>Shift</kbd> + <kbd>Tab</kbd> | Move focus between panes (**Channels** ➔ **Articles** ➔ **Reader**) |
| <kbd>j</kbd> / <kbd>k</kbd> or <kbd>↓</kbd> / <kbd>↑</kbd> | Navigate list items up / down |
| <kbd>Enter</kbd> / <kbd>Space</kbd> | Expand Category / Read selected article |
| <kbd>f</kbd> | Toggle Fullscreen Reader Mode |
| <kbd>l</kbd> | Switch Interface Language (EN / ID / JA / NL / ES / AR) |
| <kbd>i</kbd> | Toggle Show / Hide article images |
| <kbd>b</kbd> | Save to / Remove from **Bookmarks** (`★`) |
| <kbd>o</kbd> | Open article link in external Web Browser |
| <kbd>m</kbd> | Move RSS channel to another category |
| <kbd>Shift</kbd> + <kbd>C</kbd> | Delete selected Category |
| <kbd>Shift</kbd> + <kbd>D</kbd> | Delete selected RSS Feed channel |
| <kbd>a</kbd> | Open **Add New RSS Feed** dialog |
| <kbd>r</kbd> | Refresh / reload all RSS feeds |
| <kbd>/</kbd> | Open realtime search bar |
| <kbd>Esc</kbd> | Back to article list / Reset search query |
| <kbd>1</kbd> / <kbd>2</kbd> | Switch Tab: **All Feeds** (1) vs **Bookmarks** (2) |
| <kbd>Shift</kbd> + <kbd>U</kbd> | Open **Uninstall** modal dialog |
| <kbd>?</kbd> | Display Keyboard Shortcuts help modal |
| <kbd>q</kbd> | Quit application |

---

## CLI Commands

In addition to the interactive TUI, you can manage feeds directly from the command line:

```bash
# List all saved RSS channels
baca list

# Add a new RSS Feed channel
baca add --url "https://rss.kompas.com/" --title "Kompas News" --category "Top News"

# Uninstall Rubah from CLI
baca uninstall
```

---

## Uninstall

### Linux / macOS / BSD / Haiku OS:
```bash
curl -fsSL https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/uninstall.sh | bash
```

### Windows (PowerShell):
```powershell
irm https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/uninstall.ps1 | iex
```

---

## License & Privacy

- **License**: Released under the **GNU General Public License v3.0 (GPL-3.0)**. See [LICENSE](LICENSE) for details.
- **Privacy & Policy**: All feed fetching is conducted directly from the user's device to publishers without passing through intermediary servers or cloud telemetry tracking. See **[LEGAL.md](LEGAL.md)** for usage terms and publisher copyright information.

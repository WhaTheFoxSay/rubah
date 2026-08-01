# 🦊 Rubah (Ruang Baca Harian)

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows%20%7C%20BSD%20%7C%20Haiku-brightgreen.svg)]()

> **Rubah [Ruang Baca Harian] is a high-performance, local-first RSS/Atom Feed Reader for the terminal.**

**Rubah** (Ruang Baca Harian) adalah aplikasi pembaca RSS & Atom Feed berbasis Terminal User Interface (TUI) yang ringan, cepat, dan bekerja secara *cross-platform* di **Linux**, **macOS**, **Windows**, **BSD**, dan **Haiku OS**. Aplikasi ini mengambil feed berita langsung dari penerbit tanpa lalu lintas server perantara (*zero cloud*), menjaga privasi 100% lokal di perangkat Anda.

---

## 💻 Instalasi

### 🐧 Linux / 🍎 macOS:
```bash
curl -fsSL https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/install.sh | bash
```

### 🪟 Windows (PowerShell):
```powershell
irm https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/install.ps1 | iex
```

*Direct Download Windows:* **[rubah-windows-amd64.exe](https://github.com/WhaTheFoxSay/rubah/releases/latest/download/rubah-windows-amd64.exe)**

---

## 🚀 Cara Menjalankan

Setelah instalasi selesai, jalankan aplikasi di terminal dengan mengetik:
```bash
baca
```
*(Atau menggunakan perintah `rubah`)*

---

## 📋 Fitur Utama

- **TUI 3-Panel Dashboard**: Layout intuitif yang memisahkan Daftar Channel, Daftar Berita, dan Pembaca Konten.
- **Multi-Language Support**: Dukungan 6 bahasa internasional (Inggris, Indonesia, Jepang, Belanda, Spanyol, Arab) yang dapat diganti secara interaktif (`l`).
- **In-App Auto Update**: Kemampuan memeriksa dan memperbarui aplikasi secara langsung dari dalam TUI (`u`).
- **HD Terminal Image Renderer**: Menampilkan foto utama berita dalam terminal TUI (`i`).
- **Fullscreen Reader Mode**: Tampilan baca layar penuh untuk kenyamanan membaca tanpa gangguan (`f`).
- **Realtime Search & Filter**: Fitur pencarian kata kunci artikel berita secara instan (`/`).
- **Bookmarks & Categorization**: Simpan artikel favorit (`b`) dan kelola kategori feed secara bebas.
- **Local-First & SQLite Storage**: Seluruh data dan pengaturan tersimpan aman di database lokal tanpa pengumpulan telemetry.

---

## ⌨️ Shortcut Keyboard

| Shortcut | Fungsi |
| :--- | :--- |
| `Tab` / `Shift+Tab` | Pindah fokus antar panel (**Channel** ➔ **Berita** ➔ **Reader**) |
| `j` / `k` atau `↓` / `↑` | Navigasi baris item ke bawah / atas |
| `Enter` / `Space` | Ekspansi Kategori / Baca artikel penuh |
| `f` | Toggle **Fullscreen Reader Mode** |
| `l` | Toggle **Bahasa Antarmuka** (EN / ID / JA / NL / ES / AR) |
| `u` | Periksa & proses **In-App Update** |
| `i` | Toggle foto berita dalam terminal |
| `b` | Simpan / Hapus artikel dari **Bookmarks** (`★`) |
| `o` | Membuka link berita di Web Browser eksternal |
| `m` | Pindahkan channel RSS ke kategori lain |
| `Shift + C` | Hapus Kategori terpilih |
| `Shift + D` | Hapus channel RSS terpilih |
| `a` | Tambah channel RSS Feed baru |
| `r` | Refresh / muat ulang seluruh RSS Feed |
| `/` | Buka pencarian realtime |
| `Esc` | Kembali / reset pencarian |
| `1` / `2` | Switch Tab: **All Feeds** (1) vs **Bookmarks** (2) |
| `Shift + U` | Menu modal **Uninstall** |
| `?` | Modal bantuan shortcut keyboard |
| `q` | Keluar dari aplikasi |

---

## 🛠️ Perintah CLI

```bash
# Menampilkan daftar channel tersimpan
baca list

# Menambahkan channel RSS Feed baru
baca add --url "https://rss.kompas.com/" --title "Kompas News" --category "Berita Utama"

# Uninstall aplikasi dari CLI
baca uninstall
```

---

## 🗑️ Uninstall

### 🐧 Linux / 🍎 macOS:
```bash
curl -fsSL https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/uninstall.sh | bash
```

### 🪟 Windows (PowerShell):
```powershell
irm https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/uninstall.ps1 | iex
```

---

## 📄 Lisensi & Privasi

- **Lisensi**: Released under **GNU General Public License v3.0 (GPL-3.0)**.
- **Privasi**: **Zero Telemetry**. Seluruh konfigurasi dan cache berada di perangkat lokal pengguna. Seluruh hak cipta konten milik masing-masing penerbit. Lihat [LEGAL.md](LEGAL.md) untuk detail kebijakan.

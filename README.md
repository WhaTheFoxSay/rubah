# 🦊 Rubah (Ruang Baca Harian)

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows%20%7C%20BSD%20%7C%20Haiku-brightgreen.svg)]()

> **Rubah is a standards-compliant, local-first RSS/Atom reader for the terminal. It retrieves feeds directly from publishers without using intermediary servers or cloud services.**

**Rubah** (Ruang Baca Harian) adalah aplikasi pembaca RSS Feed berbasis *Terminal User Interface* (TUI) yang ringan, cepat, dan bekerja secara *cross-platform* di **Linux**, **macOS**, **Windows**, **BSD**, dan **Haiku OS**. Dibuat menggunakan **Rust** dengan framework **Ratatui** dan **Tokio**.

---

## 💻 Instalasi

### 🐧 Linux / 🍎 macOS:
Jalankan perintah berikut di terminal Anda:
```bash
curl -fsSL https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/install.sh | bash
```

### 🪟 Windows (PowerShell / CMD):

**Metode 1: Perintah 1 Baris PowerShell (Otomatis)**
```powershell
irm https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/install.ps1 | iex
```

**Metode 2: Unduh Direct Executable (.exe)**
1. Unduh binary resmi: **[rubah-windows-amd64.exe](https://github.com/WhaTheFoxSay/rubah/releases/latest/download/rubah-windows-amd64.exe)**
2. Rename file tersebut menjadi `baca.exe`.
3. Jalankan file di CMD atau PowerShell dengan mengetik `.\baca.exe`.

---

## 🚀 Cara Menjalankan

Setelah instalasi selesai, jalankan aplikasi di terminal / CMD / PowerShell dengan mengetik:
```bash
baca
```

*(Atau menggunakan perintah `rubah`)*

---

## 📋 Fitur Utama

- **TUI 3 Panel**: Tampilan terpisah untuk Kategori/Channel, Daftar Berita, dan Reader.
- **Dukungan Multibahasa**: Pilihan 6 bahasa (Inggris, Indonesia, Jepang, Belanda, Spanyol, Arab).
- **Pembaca Konten & Foto**: Membaca artikel penuh dan menampilkan gambar berita di terminal.
- **Pencarian Realtime**: Filter artikel berdasarkan kata kunci.
- **Bookmark & Kategori**: Menyimpan artikel favorit dan mengelompokkan channel feed.

---

## ⌨️ Shortcut Keyboard

| Shortcut | Fungsi |
| :--- | :--- |
| `Tab` / `Shift+Tab` | Pindah fokus antar panel (**Channel** ➔ **Berita** ➔ **Reader**) |
| `j` / `k` atau `↓` / `↑` | Navigasi item ke bawah / atas |
| `Enter` / `Space` | Buka dan baca berita penuh di dalam terminal |
| `f` | Mode Layar Penuh (Reader) |
| `l` | Ganti Bahasa Antarmuka (EN / ID / JA / NL / ES / AR) |
| `i` | Toggle Tampilkan / Sembunyikan foto berita |
| `b` | Simpan / hapus artikel dari **Bookmarks** (`★`) |
| `o` | Membuka link berita di web browser external |
| `m` | Pindahkan channel RSS ke kategori lain |
| `Shift + C` | Hapus Kategori terpilih |
| `Shift + D` | Hapus channel RSS Feed terpilih |
| `a` | Buka dialog **Tambah RSS Feed Baru** |
| `r` | Refresh / muat ulang seluruh RSS Feed |
| `/` | Buka mode pencarian realtime |
| `Esc` | Kembali dari reader ke daftar berita / reset pencarian |
| `1` / `2` | Switch Tab: **All Feeds** (1) vs **Bookmarks** (2) |
| `Shift + U` | Buka menu modal **Uninstall** |
| `?` | Tampilkan modal bantuan shortcut |
| `q` | Keluar dari aplikasi |

---

## 🛠️ Perintah CLI

Selain antarmuka TUI, Anda juga dapat mengelola RSS Feed dari command line:

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

- **Lisensi**: Proyek ini dirilis di bawah lisensi **GNU General Public License v3.0 (GPL-3.0)**. Lihat file [LICENSE](LICENSE) mengenai ketentuan lisensi.
- **Privasi & Kebijakan**: Seluruh proses pengambilan data dilakukan secara langsung dari perangkat pengguna ke penerbit tanpa melalui server perantara (*zero cloud/telemetry*). Lihat **[LEGAL.md](LEGAL.md)** untuk rincian kebijakan penggunaan dan hak cipta.

<div align="center">

# 🦊 Rubah
### Ruang Baca Harian

A standards-compliant, local-first RSS/Atom feed reader for the terminal.

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows%20%7C%20BSD%20%7C%20Haiku-brightgreen.svg)]()

[Instalasi](#instalasi) • [Cara Menjalankan](#cara-menjalankan) • [Fitur Utama](#fitur-utama) • [Shortcut Keyboard](#shortcut-keyboard) • [Perintah CLI](#perintah-cli) • [Lisensi](#lisensi--privasi)

</div>

---

**Rubah** (Ruang Baca Harian) adalah aplikasi pembaca RSS & Atom Feed berbasis *Terminal User Interface* (TUI) yang ringan, cepat, dan bekerja secara *cross-platform* di Linux, macOS, Windows, BSD, dan Haiku OS. Dibuat menggunakan Rust dengan framework Ratatui dan Tokio.

> [!NOTE]
> **Local-First Architecture**: Rubah mengambil feed berita secara langsung dari penerbit ke perangkat Anda tanpa server perantara atau pengumpulan telemetry.

---

## Instalasi

### Linux / macOS / BSD / Haiku OS:
Jalankan perintah berikut di terminal Anda:
```bash
curl -fsSL https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/install.sh | bash
```

### Windows (PowerShell / CMD):

**Metode 1: Perintah 1 Baris PowerShell**
```powershell
irm https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/install.ps1 | iex
```

**Metode 2: Unduh Direct Executable (.exe)**
1. Unduh binary resmi: **[rubah-windows-amd64.exe](https://github.com/WhaTheFoxSay/rubah/releases/latest/download/rubah-windows-amd64.exe)**
2. Rename file tersebut menjadi `baca.exe`.
3. Jalankan file di CMD atau PowerShell dengan mengetik `.\baca.exe`.

---

## Cara Menjalankan

Setelah instalasi selesai, jalankan aplikasi di terminal dengan mengetik:
```bash
baca
```

*(Atau menggunakan perintah `rubah`)*

---

## Fitur Utama

- **TUI 3 Panel**: Tampilan terpisah untuk Kategori/Channel, Daftar Berita, dan Reader.
- **Dukungan Multibahasa**: Pilihan 6 bahasa (Inggris, Indonesia, Jepang, Belanda, Spanyol, Arab).
- **Pembaca Konten & Foto**: Membaca artikel penuh dan menampilkan gambar berita di terminal.
- **Pencarian Realtime**: Filter artikel berdasarkan kata kunci.
- **Bookmark & Kategori**: Menyimpan artikel favorit dan mengelompokkan channel feed.

---

## Shortcut Keyboard

| Shortcut | Fungsi |
| :--- | :--- |
| <kbd>Tab</kbd> / <kbd>Shift</kbd> + <kbd>Tab</kbd> | Pindah fokus antar panel (**Channel** ➔ **Berita** ➔ **Reader**) |
| <kbd>j</kbd> / <kbd>k</kbd> atau <kbd>↓</kbd> / <kbd>↑</kbd> | Navigasi item ke bawah / atas |
| <kbd>Enter</kbd> / <kbd>Space</kbd> | Buka dan baca berita penuh di dalam terminal |
| <kbd>f</kbd> | Mode Layar Penuh (Reader) |
| <kbd>l</kbd> | Ganti Bahasa Antarmuka (EN / ID / JA / NL / ES / AR) |
| <kbd>i</kbd> | Toggle Tampilkan / Sembunyikan foto berita |
| <kbd>b</kbd> | Simpan / hapus artikel dari **Bookmarks** (`★`) |
| <kbd>o</kbd> | Membuka link berita di web browser external |
| <kbd>m</kbd> | Pindahkan channel RSS ke kategori lain |
| <kbd>Shift</kbd> + <kbd>C</kbd> | Hapus Kategori terpilih |
| <kbd>Shift</kbd> + <kbd>D</kbd> | Hapus channel RSS Feed terpilih |
| <kbd>a</kbd> | Buka dialog **Tambah RSS Feed Baru** |
| <kbd>r</kbd> | Refresh / muat ulang seluruh RSS Feed |
| <kbd>/</kbd> | Buka mode pencarian realtime |
| <kbd>Esc</kbd> | Kembali dari reader ke daftar berita / reset pencarian |
| <kbd>1</kbd> / <kbd>2</kbd> | Switch Tab: **All Feeds** (1) vs **Bookmarks** (2) |
| <kbd>Shift</kbd> + <kbd>U</kbd> | Buka menu modal **Uninstall** |
| <kbd>?</kbd> | Tampilkan modal bantuan shortcut |
| <kbd>q</kbd> | Keluar dari aplikasi |

---

## Perintah CLI

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

## Lisensi & Privasi

- **Lisensi**: Proyek ini dirilis di bawah lisensi **GNU General Public License v3.0 (GPL-3.0)**. Lihat file [LICENSE](LICENSE) mengenai ketentuan lisensi.
- **Privasi & Kebijakan**: Seluruh proses pengambilan data dilakukan secara langsung dari perangkat pengguna ke penerbit tanpa melalui server perantara (*zero cloud/telemetry*). Lihat **[LEGAL.md](LEGAL.md)** untuk rincian kebijakan penggunaan dan hak cipta.

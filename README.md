# 🦊 RUBAH - Ruang Baca Harian

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows%20%7C%20BSD%20%7C%20Haiku-brightgreen.svg)]()

> **Rubah (Ruang Baca Harian)** adalah aplikasi pembaca berita RSS Feed berbasis Terminal User Interface (TUI) yang sangat cepat, hemat sumber daya, dan **True Cross-Platform** (mendukung **Linux**, **macOS**, **Windows PowerShell/CMD**, **BSD**, dan **Haiku OS**). Dibuat menggunakan bahasa **Rust** dengan framework **Ratatui** dan **Tokio**.

---

## ⚡ Instalasi Instan 1 Baris (1-2 Detik Tanpa Kompilasi)

Cukup salin dan tempel perintah berikut di terminal Anda untuk menginstall **Rubah** secara instan:

```bash
curl -fsSL https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/install.sh | bash
```

Setelah instalasi selesai dalam 1 detik, buka aplikasi cukup dengan mengetik:

```bash
baca
```

*(Catatan: Anda juga bisa mengetikkan perintah `rubah`)*

---

## ✨ Fitur Utama

- 🚀 **Kecepatan Kilat & Hemat RAM**: Pemuatan puluhan RSS Feed secara paralel dalam waktu kurang dari 1 detik. Penggunaan memori sangat minim (~5-15MB RAM).
- 🐧 **Zero-GLIBC Static Linux Binary**: Menggunakan target **Static MUSL Binary** yang kompatibel 100% dengan **semua distro Linux** (RHEL 7/8/9, CentOS, AlmaLinux, Rocky Linux, Ubuntu, Debian, Alpine, Fedora, Arch) tanpa error glibc.
- 📺 **Dashboard TUI 3 Panel**:
  - **Panel Left (Channel)**: Daftar RSS Feed yang dikelompokkan berdasarkan kategori.
  - **Panel Middle (Artikel)**: Daftar berita dengan indikator dibaca (`●`/`○`), tanggal, dan bookmark (`★`).
  - **Panel Right (Reader)**: Pembaca berita penuh di dalam terminal dengan dukungan scroll keyboard (`j`/`k`, `d`/`u`).
- 💾 **Penyimpanan Database SQLite**: Menyimpan RSS feed kustom, artikel favorit (bookmarks), dan riwayat berita yang sudah dibaca.
- 🔍 **Pencarian Realtime**: Filter judul dan konten berita secara langsung di terminal (`/`).
- 🌐 **Pre-configured Feed**: Langsung dibekali portal berita populer Indonesia (CNN Indonesia, Antara, Tempo, Detikcom) dan Internasional (Hacker News, TechCrunch, BBC News).

---

## ⌨️ Bantuan Navigasi & Shortcut Keyboard

| Tombol | Fungsi |
| :--- | :--- |
| `Tab` / `Shift+Tab` | Pindah fokus antar panel (**Channel** ➔ **Berita** ➔ **Reader**) |
| `j` / `k` atau `↓` / `↑` | Navigasi item ke bawah / atas |
| `Enter` atau `Space` | Masuk ke mode pembaca penuh berita di terminal |
| `Esc` | Kembali dari mode reader ke daftar berita |
| `o` | Membuka link berita asli di Web Browser external |
| `b` | Simpan / hapus artikel dari **Bookmarks** (`★`) |
| `r` | Refresh / muat ulang seluruh RSS Feed |
| `a` | Buka dialog **Tambah Channel RSS Feed Baru** |
| `D` (Shift+D) | Hapus channel RSS Feed terpilih |
| `/` | Cari berita secara realtime |
| `1` / `2` | Switch Tab: **All Feeds** (1) vs **Bookmarks** (2) |
| `Shift + U` | Buka menu modal **Uninstall Aplikasi Rubah** |
| `?` | Tampilkan modal bantuan shortcut |
| `q` | Keluar dari aplikasi |

---

## 🛠️ Perintah CLI (Command Line)

Selain antarmuka TUI, Anda dapat mengelola RSS Feed langsung dari CLI:

```bash
# Menampilkan daftar channel yang tersimpan
baca list

# Menambahkan channel RSS Feed baru
baca add --url "https://rss.kompas.com/" --title "Kompas News" --category "Berita Utama"

# Uninstall aplikasi Rubah dari CLI
baca uninstall
```

---

## 🗑️ Cara Uninstall

Anda dapat meng-uninstall aplikasi **Rubah** langsung dari menu aplikasi (tombol `Shift + U`), melalui CLI (`baca uninstall`), atau melalui script perintah 1 baris:

```bash
curl -fsSL https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/uninstall.sh | bash
```

---

## 📄 Lisensi

Proyek ini dilesensikan di bawah **GNU General Public License v3.0 (GPL-3.0)**. Lihat file [LICENSE](LICENSE) untuk informasi lebih detail.

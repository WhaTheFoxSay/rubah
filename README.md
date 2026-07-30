# 🦊 RUBAH - Ruang Baca Harian

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows%20%7C%20BSD%20%7C%20Haiku-brightgreen.svg)]()

> **Rubah (Ruang Baca Harian)** adalah aplikasi pembaca berita RSS Feed berbasis Terminal User Interface (TUI) yang sangat cepat, hemat sumber daya, dan **True Cross-Platform** (mendukung **Linux**, **macOS**, **Windows PowerShell/CMD**, **BSD**, dan **Haiku OS**). Dibuat menggunakan bahasa **Rust** dengan framework **Ratatui** dan **Tokio**.

---

## ⚡ Instalasi 1 Perintah (One-Line Setup)

Cukup salin dan tempel perintah berikut di terminal Anda untuk menginstall **Rubah**:

```bash
curl -fsSL https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/install.sh | bash
```

Setelah proses penginstalan selesai, Anda cukup mengetikkan **`baca`** di terminal untuk langsung membuka aplikasi:

```bash
baca
```

*(Catatan: Anda juga bisa mengetikkan perintah `rubah`)*

---

## ✨ Fitur Utama

- 🚀 **Kecepatan Kilat & Hemat RAM**: Pemuatan puluhan RSS Feed secara paralel dalam waktu kurang dari 1 detik. Penggunaan memori sangat minim (~5-15MB RAM).
- 📺 **Dashboard TUI 3 Panel**:
  - **Panel Left (Channel)**: Daftar RSS Feed yang dikelompokkan berdasarkan kategori.
  - **Panel Middle (Artikel)**: Daftar berita dengan indikator dibaca (`●`/`○`), tanggal, dan bookmark (`★`).
  - **Panel Right (Reader)**: Preview isi berita dalam tampilan teks teratur dengan kemampuan scroll.
- 💾 **Penyimpanan Database SQLite**: Menyimpan RSS feed kustom, artikel favorit (bookmarks), dan riwayat berita yang sudah dibaca.
- 🔍 **Pencarian Realtime**: Filter judul dan konten berita secara langsung di terminal (`/`).
- 🌐 **Pre-configured Feed**: Langsung dibekali portal berita populer Indonesia (CNN Indonesia, Antara, Tempo, Detikcom) dan Internasional (Hacker News, TechCrunch, BBC News).

---

## ⌨️ Bantuan Navigasi & Shortcut Keyboard

| Tombol | Fungsi |
| :--- | :--- |
| `Tab` / `Shift+Tab` | Pindah fokus antar panel (**Channel** ➔ **Berita** ➔ **Reader**) |
| `j` / `k` atau `↓` / `↑` | Navigasi item ke bawah / atas |
| `Enter` atau `o` | Membuka link berita asli di Web Browser default |
| `b` | Simpan / hapus artikel dari **Bookmarks** (`★`) |
| `r` | Refresh / muat ulang seluruh RSS Feed |
| `a` | Buka dialog **Tambah Channel RSS Feed Baru** |
| `d` | Hapus channel RSS Feed terpilih |
| `/` | Cari berita secara realtime |
| `1` / `2` | Switch Tab: **All Feeds** (1) vs **Bookmarks** (2) |
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
```

---

## 🗑️ Cara Uninstall

Jika ingin menghapus aplikasi **Rubah** beserta seluruh konfigurasinya dari terminal:

```bash
curl -fsSL https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/uninstall.sh | bash
```

Atau bisa dilakukan secara manual dengan 1 baris perintah:

```bash
rm -f ~/.local/bin/baca ~/.local/bin/rubah && rm -rf ~/.config/rubah
```

---

## 📄 Lisensi

Proyek ini dilesensikan di bawah **GNU General Public License v3.0 (GPL-3.0)**. Lihat file [LICENSE](LICENSE) untuk informasi lebih detail.

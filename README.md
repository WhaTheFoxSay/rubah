# 🦊 Rubah (Ruang Baca Harian)

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows%20%7C%20BSD%20%7C%20Haiku-brightgreen.svg)]()

**Rubah** (Ruang Baca Harian) adalah aplikasi pembaca RSS Feed berbasis *Terminal User Interface* (TUI) yang ringan, cepat, dan bekerja secara *cross-platform* di **Linux**, **macOS**, **Windows**, **BSD**, dan **Haiku OS**. Dibuat menggunakan **Rust** dengan framework **Ratatui** dan **Tokio**.

---

## 💻 Instalasi

### 🐧 Linux / 🍎 macOS:
```bash
curl -fsSL https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/install.sh | bash
```

### 🪟 Windows (PowerShell):
Buka PowerShell lalu jalankan perintah 1 baris berikut:
```powershell
irm https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/install.ps1 | iex
```

*(Atau unduh langsung file `rubah-windows-amd64.exe` dari [Releases](https://github.com/WhaTheFoxSay/rubah/releases) lalu jalankan di CMD/PowerShell)*

Setelah instalasi selesai, buka aplikasi dengan perintah:
```bash
baca
```

*(Atau menggunakan perintah `rubah`)*

---

## 📋 Fitur Utama

- **TUI 3-Panel Dashboard**:
  - **Left**: Daftar channel RSS Feed berdasarkan kategori.
  - **Middle**: Daftar berita dengan status dibaca (`●`/`○`), tanggal, dan bookmark (`★`).
  - **Right**: Pembaca konten berita langsung di dalam terminal dengan scroll keyboard.
- **Ringan & Portabel**: Menggunakan binary statis (MUSL untuk Linux) yang kompatibel dengan berbagai distro Linux (RHEL, CentOS, Ubuntu, Debian, Alpine, dll.) tanpa ketergantungan library luar.
- **Penyimpanan Lokal SQLite**: Menyimpan RSS feed, riwayat baca, dan artikel favorit di database lokal (`~/.config/rubah/rubah.db`).
- **Pencarian Realtime**: Filter judul dan konten berita secara langsung (`/`).
- **Pre-configured Feeds**: Dilengkapi feed berita default populer (CNN Indonesia, Antara, Tempo, Detikcom, Hacker News, TechCrunch, BBC News).

---

## ⌨️ Shortcut Keyboard

| Shortcut | Fungsi |
| :--- | :--- |
| `Tab` / `Shift+Tab` | Pindah fokus antar panel (**Channel** ➔ **Berita** ➔ **Reader**) |
| `j` / `k` atau `↓` / `↑` | Navigasi item ke bawah / atas |
| `Enter` / `Space` | Buka dan baca berita penuh di dalam terminal |
| `Esc` | Kembali dari reader ke daftar berita / reset pencarian |
| `o` | Membuka link berita di web browser external |
| `b` | Simpan / hapus artikel dari **Bookmarks** (`★`) |
| `r` | Refresh / muat ulang seluruh RSS Feed |
| `a` | Buka dialog **Tambah RSS Feed Baru** |
| `Shift + D` | Hapus channel RSS Feed terpilih |
| `/` | Buka mode pencarian realtime |
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

Untuk menghapus aplikasi Rubah dan data konfigurasinya:

```bash
curl -fsSL https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/uninstall.sh | bash
```

Atau secara manual:

```bash
rm -f ~/.local/bin/baca ~/.local/bin/rubah && rm -rf ~/.config/rubah
```

---

## 📄 Lisensi

Proyek ini dirilis di bawah lisensi **GNU General Public License v3.0 (GPL-3.0)**. Lihat file [LICENSE](LICENSE) untuk informasi lebih lanjut.

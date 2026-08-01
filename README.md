# 🦊 Rubah (Ruang Baca Harian) v1.5.0

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows%20%7C%20BSD%20%7C%20Haiku-brightgreen.svg)]()
[![Release](https://img.shields.io/badge/Release-v1.5.0-brightgreen.svg)](https://github.com/WhaTheFoxSay/rubah/releases/tag/v1.5.0)

> **Rubah [Ruang Baca Harian] is a high-performance, local-first, zero-cloud RSS/Atom Feed Reader TUI (Terminal User Interface) built with Rust, Ratatui, and Tokio.**

**Rubah** (Ruang Baca Harian) adalah aplikasi pembaca RSS & Atom Feed berbasis Terminal User Interface (TUI) modern yang ringan, ultra-cepat, dan bekerja secara *cross-platform* di **Linux**, **macOS**, **Windows**, **BSD**, dan **Haiku OS**. Aplikasi ini mengambil feed berita langsung dari penerbit tanpa lalu lintas server cloud perantara, menjaga privasi 100% lokal di perangkat Anda.

---

## 💻 Instalasi Otomatis (1 Baris Perintah)

### 🐧 Linux / 🍎 macOS:
Buka terminal Anda, lalu *copy-paste* dan jalankan perintah berikut:
```bash
curl -fsSL https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/install.sh | bash
```

### 🪟 Windows (PowerShell):
Buka **PowerShell**, lalu jalankan perintah berikut:
```powershell
irm https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/install.ps1 | iex
```

*Atau unduh executable langsung:* **[rubah-windows-amd64.exe](https://github.com/WhaTheFoxSay/rubah/releases/latest/download/rubah-windows-amd64.exe)**

---

## 🚀 Cara Menjalankan Aplikasi

Setelah instalasi selesai, jalankan aplikasi langsung dari terminal dengan mengetik:
```bash
baca
```
*(Atau menggunakan perintah `rubah`)*

---

## 🌟 Fitur Lengkap Aplikasi

### 🖼️ 1. Modern 3-Panel Dashboard Layout (Ratatui TUI)
- **Panel Kiri (Channels & Categories)**: Struktur hirarki pohon kategori dan channel RSS Feed dilengkapi indikator expandable (`[+]`/`[-]`) dan jumlah channel (`(N)`).
- **Panel Tengah (News Articles)**: Daftar artikel berita dengan penanda status dibaca (`●` Unread / `○` Read), bookmark favorit (`★`), tanggal penerbitan, dan nama author.
- **Panel Kanan (Article Reader)**: Tampilan penuh pembaca artikel berita dilengkapi header metadata (Judul, Sumber, Tanggal, Link Web) dan isi konten lengkap.

### 🌐 2. Multi-Language Support (Dukungan 6 Bahasa Internasional)
Dukungan penerjemahan antarmuka secara dinamis untuk 6 bahasa internasional (dapat diganti kapan saja via shortcut `l`):
1. 🇬🇧 **English (`EN`)** — *Default*
2. 🇮🇩 **Bahasa Indonesia (`ID`)**
3. 🇯🇵 **Jepang (`JA`)** — 日本語
4. 🇳🇱 **Belanda (`NL`)** — Nederlands
5. 🇪🇸 **Spanyol (`ES`)** — Español
6. 🇸🇦 **Arab (`AR`)** — العربية
*Format nama hari dan bulan pada jam header disesuaikan secara otomotais dengan bahasa yang aktif.*

### 🔄 3. In-App Auto Update Engine
- Memeriksa pembaruan rilis terbaru dari server GitHub secara langsung (`u`).
- Dialog modal pembaruan dilengkapi animasi spinner (`⠋⠙⠹...`), persentase progress bar, kalkulasi ukuran file patch (MB/MB), dan instalasi biner rilis otomatis dari dalam aplikasi.

### 🖼️ 4. 24-bit TrueColor HD Terminal Image Renderer
- Mengambil foto/gambar utama artikel berita dan merendernya dalam grafik 24-bit sharpened RGB Ansi Art langsung di dalam panel pembaca terminal (*toggle* `i`).

### 📖 5. Fullscreen Reader Mode
- Memperluas tampilan panel pembaca artikel ke layar penuh (*fullscreen*) tanpa gangguan panel navigasi (*toggle* `f`).

### 🔍 6. Pencarian Realtime & Filter Kata Kunci
- Filter berita secara instan berdasarkan judul atau kata kunci konten secara *live* di seluruh channel (`/`).
- Reset pencarian cepat dengan tombol `Esc`.

### ⚡ 7. Indikator Latensi Jaringan Real-Time
- Mengukur latensi jaringan ke Cloudflare DNS 1.1.1.1 secara langsung dalam milidetik (`[<120ms]` hijau, `[<300ms]` oranye, `[>300ms]` kuning).

### ⭐️ 8. Sistem Bookmark & Tab Kategori
- Menyimpan artikel berita favorit (`★`) dengan tombol `b`.
- Navigasi cepat antar Tab: **All Feeds** (`1`) vs **Bookmarks** (`2`).

### 📁 9. Manajemen Kategori & Feed Interaktif
- **Tambah Feed Baru (`a`)**: Modal interaktif dengan auto-completion nama kategori.
- **Pindah Kategori Feed (`m`)**: Memindahkan channel RSS Feed ke kategori lain atau kategori baru.
- **Hapus Kategori (`Shift+C`)**: Konfirmasi penghapusan kategori beserta seluruh feed di dalamnya.
- **Hapus Feed (`Shift+D`)**: Menghapus channel feed terpilih.

### 🏛️ 10. Arsitektur Local-First & Penyimpanan SQLite
- Data feed, artikel, bookmark, dan pengaturan tersimpan aman di database lokal SQLite (`rubah.db`) tanpa ketergantungan server cloud.
- Lokasi DB: `~/.config/rubah/rubah.db` (Linux/macOS) atau `%LOCALAPPDATA%\Rubah\rubah.db` (Windows).

---

## ⌨️ Daftar Shortcut Keyboard Lengkap

| Shortcut | Fungsi Utama |
| :--- | :--- |
| `Tab` / `Shift+Tab` | Pindah fokus antar panel (**Channel** ➔ **Berita** ➔ **Reader**) |
| `j` / `k` atau `↓` / `↑` | Navigasi baris item ke bawah / atas |
| `Enter` / `Space` | Ekspansi Kategori / Buka & baca artikel penuh |
| `f` / `F` | Toggle **Fullscreen Reader Mode** (tampilan artikel layar penuh) |
| `l` / `L` | Toggle **Bahasa Antarmuka** (EN / ID / JA / NL / ES / AR) |
| `u` / `U` | Periksa & jalankan **In-App Auto Update** ke versi rilis terbaru |
| `i` | Toggle Tampilkan / Sembunyikan foto berita dalam terminal |
| `b` | Simpan artikel ke / Hapus dari **Bookmarks** (`★`) |
| `o` | Membuka link artikel berita di Web Browser eksternal |
| `m` | Pindahkan channel RSS Feed ke kategori lain |
| `Shift + C` | Hapus Kategori terpilih beserta seluruh channel feed di dalamnya |
| `Shift + D` | Hapus channel RSS Feed terpilih |
| `a` | Buka modal **Tambah Channel RSS Feed Baru** |
| `r` | Refresh / muat ulang seluruh RSS Feed secara bersamaan |
| `/` | Buka mode pencarian berita realtime |
| `Esc` | Kembali dari reader ke daftar berita / bersihkan filter pencarian |
| `1` / `2` | Switch Tab: **All Feeds** (1) vs **Bookmarks** (2) |
| `Shift + U` | Buka modal konfirmasi **Uninstall** dari dalam aplikasi |
| `?` | Tampilkan modal bantuan shortcut keyboard lengkap |
| `q` | Keluar dari aplikasi |

---

## 🛠️ Perintah CLI (Command Line Interface)

Selain antarmuka TUI interaktif, Rubah menyediakan perintah CLI untuk otomasi dan pengelolaan tanpa grafis TUI:

```bash
# 1. Menampilkan daftar seluruh channel RSS Feed tersimpan
baca list

# 2. Menambahkan channel RSS Feed baru langsung dari terminal
baca add --url "https://rss.kompas.com/" --title "Kompas News" --category "Berita Utama"

# 3. Menghapus/uninstall aplikasi Rubah dari sistem
baca uninstall

# 4. Menampilkan versi aplikasi saat ini
baca --version
```

---

## 🗑️ Prosedur Uninstall Clean

### 🐧 Linux / 🍎 macOS:
```bash
curl -fsSL https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/uninstall.sh | bash
```

### 🪟 Windows (PowerShell):
```powershell
irm https://raw.githubusercontent.com/WhaTheFoxSay/rubah/main/uninstall.ps1 | iex
```

*Skrip uninstaller akan menghapus executable, symlink (`~/.local/bin/baca`), database SQLite (`~/.config/rubah`), serta cache sementara secara bersih.*

---

## 📄 Lisensi Perangkat Lunak

Proyek ini dirilis di bawah lisensi **GNU General Public License v3.0 (GPL-3.0)**. Lihat dokumen [LICENSE](LICENSE) untuk informasi lebih lanjut.

---

## 📜 Kebijakan Penggunaan & Privasi (Terms & Privacy)

- 🌐 **Standar Terbuka**: Rubah memproses standar terbuka RSS 2.0 & Atom Feed yang dipublikasikan oleh masing-masing penerbit.
- 🛡️ **Zero Telemetry & Tracking**: Rubah tidak mengumpulkan, menyimpan, atau mengirimkan data pengguna ke server mana pun. Seluruh konfigurasi dan cache berada di perangkat lokal pengguna.
- ✍️ **Hak Cipta Content**: Seluruh hak cipta materi berita tetap milik penerbit feed masing-masing.

Lihat dokumen **[LEGAL.md](LEGAL.md)** untuk rincian kebijakan penggunaan dan hak cipta.

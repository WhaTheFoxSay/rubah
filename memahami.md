# 🦊 Panduan Memahami Proyek Rubah (Ruang Baca Harian) v1.5.0 untuk AI Agentic

Dokumen ini berisi panduan lengkap mengenai arsitektur, aturan kode, desain UI, fitur sistem, dan alur rilis/maintenance proyek **Rubah (Ruang Baca Harian)**. Ketika AI Agentic membaca file ini, AI harus mematuhi seluruh standar dan arsitektur yang dijelaskan di sini.

---

## 📌 1. Ringkasan & Arsitektur Proyek

- **Nama Aplikasi**: Rubah (Ruang Baca Harian)
- **Fungsi**: Cross-platform RSS/Atom Feed Reader berbasis Terminal User Interface (TUI), *local-first*, langsung mengunduh dari penerbit tanpa server perantara (*no cloud/proxy/telemetry*).
- **Bahasa & Stack Utama**:
  - **Core**: Rust 2021
  - **TUI Framework**: `ratatui` (v0.29) + `crossterm` (v0.28)
  - **Async Runtime**: `tokio` (v1.40)
  - **HTTP Client**: `reqwest` (v0.12, `rustls-tls`)
  - **Parser RSS/Atom**: `feed-rs` (v2.1)
  - **HTML to Text**: `html2text` (v0.13)
  - **Database Lokal**: `rusqlite` (v0.32, `bundled SQLite`)
  - **CLI Parsing**: `clap` (v4.5)
  - **Image Rendering**: `image` (v0.25)

---

## 📁 2. Peta File & Tanggung Jawab Modul (`src/`)

- [src/main.rs](file:///Users/inan/rubah/src/main.rs)
  - Entry point aplikasi.
  - Penanganan argument CLI (`baca add`, `baca list`, `baca uninstall`).
  - Menginisialisasi raw mode terminal, Crossterm backend, panic hook (restorasi terminal saat crash), dan loop event input keyboard.
- [src/app.rs](file:///Users/inan/rubah/src/app.rs)
  - State management utama (`App`, `ActivePane`, `ActiveTab`, `InputMode`, `ChannelTreeItem`).
  - Mengelola daftar feeds, artikel, cache full-content (`article_cache`), status bar, running text marquee tick, latensi DNS Cloudflare (1.1.1.1), In-App Auto Update progress channel, serta aksi pindah/hapus kategori.
- [src/models.rs](file:///Users/inan/rubah/src/models.rs)
  - Data struktur: `FeedSource` (id, title, url, category) dan `Article` (id, link, title, published, author, summary, content, is_read, is_bookmarked).
  - Mengatur `default_feeds()` bawaan.
- [src/storage.rs](file:///Users/inan/rubah/src/storage.rs)
  - Manajemen database SQLite (`~/.config/rubah/rubah.db` / `%LOCALAPPDATA%\Rubah\rubah.db`).
  - Tabel: `feeds`, `articles`, `settings`, `read_articles`, `bookmarks`.
  - Method: `add_feed`, `delete_feed`, `update_feed_category`, `delete_category`, `mark_article_read`, `toggle_bookmark`, `get_bookmarks`.
- [src/network.rs](file:///Users/inan/rubah/src/network.rs)
  - Pengambil data RSS/Atom (`reqwest` & `feed-rs`).
  - Pembersih HTML (`clean_html`), pemilah teks berita utama (`extract_article_paragraphs`), dan ekstraksi URL gambar dari meta tag `og:image` / `<img src>`.
  - Engine pengunduh patch pembaruan aplikasi langsung dari GitHub API (`download_and_install_update`) dengan MPSC progress stream channel.
- [src/image_render.rs](file:///Users/inan/rubah/src/image_render.rs)
  - Engine penampil gambar berita di terminal TUI menggunakan karakter half-block (`▀`) 24-bit RGB dual-pixel dengan resampling *Lanczos3*.
- [src/i18n.rs](file:///Users/inan/rubah/src/i18n.rs)
  - Engine multi-bahasa internasional 6 bahasa (`English`, `Indonesian`, `Japanese`, `Dutch`, `Spanish`, `Arabic`).
  - Pemetaan translasi teks UI, dialog modal, pesan status, format nama hari & bulan pada jam header, dan translasi nama kategori (`translate_category`).
- [src/ui.rs](file:///Users/inan/rubah/src/ui.rs)
  - Komponen UI & Layout Ratatui: Header Banner, Panel Channel & Kategori, Panel Berita, Panel Reader Mode, Fullscreen Reader Mode, Search bar, Status bar, Footer shortcuts, serta Modal Dialogs (`Help`, `Uninstall`, `Add Feed`, `Move Category`, `Delete Category`, `Check Update`, `In-App Update Progress`).
- [src/cli.rs](file:///Users/inan/rubah/src/cli.rs)
  - Parsing argument CLI menggunakan `clap` (`#[command(version)]` dinamis dari `Cargo.toml`).

---

## 🎨 3. Aturan Desain & Tampilan UI (Mandatori)

### ⚠️ Aturan Strict Emoji / Emoticon
> **DILARANG** menggunakan emoji/emoticon dekoratif di UI maupun log CLI (seperti `●`, `○`, `★`, `📡`, `🗑️`, `✅`, dll).
> **HANYA** emoticon `🦊` Rubah yang diperbolehkan khusus untuk logo / nama Rubah.

### 🟢 Aturan Penanda Dot Artikel (`draw_articles_pane`):
1. **Artikel Terpilih (Active selection)**: `● ` (Dot padat berwarna Hijau, `THEME.success`).
2. **Artikel Belum Terpilih & Belum Dibaca (Unread)**: `● ` (Dot padat berwarna Orange, `THEME.accent`).
3. **Artikel Belum Terpilih & Sudah Dibaca (Read)**: `○ ` (Dot berlubang/hollow berwarna Orange, `THEME.accent`).
4. **Navigasi Kursor `j`/`k`**: Navigasi kursor naik/turun di daftar berita **TIDAK BOLEH** memanggil `mark_current_read()`. Artikel **HANYA** ditandai dibaca ketika pengguna secara eksplisit membukanya (menekan `Enter`, `Space`, `o`, atau `f`).

### 🌲 Structure Tree Panel Channel (`draw_feeds_pane`):
- Kategori Header: `[-] ` (Expanded) atau `[+] ` (Collapsed) + Nama Kategori + `(jumlah feed)`.
- Feed Item di bawah Kategori: Inden 2 spasi `  └─ ` + Judul Feed.
- Menekan `Enter` / `Space` pada Kategori akan memicu expand/collapse (`toggle_selected_category_expand()`).

### 🔍 Fitur Pencarian Realtime (`/`):
- Menekan `/` akan mengaktifkan `InputMode::Search`.
- **Pencarian Global**: Mencari kata kunci secara otomatis di seluruh channel RSS pada judul, ringkasan, penulis, konten, dan nama feed.
- **Navigasi Langsung**: Saat mengetik kata kunci, pengguna dapat langsung menekan tombol `Down` / `Up` / `Tab` untuk memilih hasil berita.
- **Bersihkan Pencarian**: Menekan `Esc` dari mode pencarian atau mode normal langsung membersihkan query (`clear_search()`).

---

## 🛠️ 4. Spesifikasi Fitur Utama v1.5.0

1. **Multi-Language Support (6 Bahasa)**:
   - Dukungan bahasa antarmuka: **Inggris (`en`)**, **Indonesia (`id`)**, **Jepang (`ja`)**, **Belanda (`nl`)**, **Spanyol (`es`)**, dan **Arab (`ar`)**.
   - Rotasi bahasa interaktif via shortcut keyboard `[l]`. Bahasa tersimpan permanen di SQLite.
2. **In-App Auto Update Engine**:
   - Memeriksa pembaruan rilis terbaru via GitHub API (`[u]`).
   - Mengunduh patch biner secara asinkron menggunakan MPSC progress stream dengan kalkulasi persentase dan ukuran download (MB).
3. **24-bit TrueColor Terminal Image Renderer**:
   - Ekstraksi gambar utama artikel (`og:image` / `<img src>`) dan render Ansi RGB dual-pixel (`[i]`).
4. **Fullscreen Reader Mode**:
   - Memperluas panel pembaca ke layar penuh tanpa hambatan panel navigasi (`[f]`).
5. **Monitoring Latensi Jaringan Real-Time**:
   - Pengukuran latensi ping jaringan ke Cloudflare DNS 1.1.1.1 (`[<120ms]` / `[<300ms]` / `[>300ms]`).
6. **Manajemen Kategori & Feed**:
   - Tambah feed (`a`), pindah kategori (`m`), hapus kategori (`Shift+C`), dan hapus feed (`Shift+D`).

---

## 🚀 5. Panduan Alur Maintenance, Bump Version & Release

Ketika diminta memperbarui fitur atau melakukan rilis versi baru (misalnya dari `v1.5.0` ke `v1.6.0`), AI **WAJIB** mengikuti langkah-langkah berikut secara berurutan:

### Langkah 1: Update Versi di 4 File Utama
1. [Cargo.toml](file:///Users/inan/rubah/Cargo.toml):
   ```toml
   version = "1.6.0"
   ```
2. [src/cli.rs](file:///Users/inan/rubah/src/cli.rs): Pastikan menggunakan `#[command(version)]` dinamis.
3. [install.sh](file:///Users/inan/rubah/install.sh):
   ```bash
   RELEASE_URL="https://github.com/${REPO}/releases/download/v1.6.0/${BINARY_NAME}"
   ```
4. [install.ps1](file:///Users/inan/rubah/install.ps1):
   ```powershell
   $PrimaryUrl = "https://github.com/WhaTheFoxSay/rubah/releases/download/v1.6.0/rubah-windows-amd64.exe"
   ```

### Langkah 2: Build & Perbarui Executable Lokal
Jalankan perintah kompilasi lokal untuk memperbarui biner di folder `bin/` dan sistem lokal:
```bash
cargo build --release && cp target/release/rubah ~/.local/bin/baca && cp target/release/rubah bin/rubah-macos-arm64
```

### Langkah 3: Konfirmasi CI/CD GitHub Actions (`release.yml`)
Pastikan [.github/workflows/release.yml](file:///Users/inan/rubah/.github/workflows/release.yml) memiliki opsi `overwrite: true` pada `softprops/action-gh-release@v2`.

### Langkah 4: Commit, Buat Tag, dan Force Push ke Remote
```bash
git add .
git commit -m "feat: [deskripsi fitur] & bump to v1.6.0"
git tag -f v1.6.0
git push origin main --tags -f
```

---

## 🌐 6. Aturan Integritas Nama Brand & Bahasa (Product Branding & i18n)

1. **Integritas Nama Produk/Brand**:
   - `RUBAH` adalah nama produk/aplikasi dan merupakan akronim resmi dari **`[Ruang Baca Harian]`**.
   - **DILARANG MENGUBAH ATOMIK BRANDING**: String `RUBAH [Ruang Baca Harian]` **TIDAK BOLEH** diterjemahkan ke bahasa asing (seperti `Daily Reading Space`) di mana pun (TUI Banner, Skrip Installer CLI, Uninstaller, Log Terminal, maupun Dokumentasi).
2. **Bahasa Skrip Installer & Uninstaller**:
   - Skrip instalasi (`install.sh`, `install.ps1`) dan uninstall (`uninstall.sh`, `uninstall.ps1`) **SELALU** menggunakan **Bahasa Inggris** bawaan (default).
3. **Pilihan Bahasa Dalam Aplikasi (In-App Menu)**:
   - Pilihan bahasa antarmuka TUI mendukung 6 bahasa internasional: **Inggris (`en`)**, **Indonesia (`id`)**, **Jepang (`ja`)**, **Belanda (`nl`)**, **Spanyol (`es`)**, dan **Arab (`ar`)**.
   - Pengguna dapat mengganti bahasa antarmuka secara interaktif dengan menekan tombol **`[l]`** / **`[L]`**.

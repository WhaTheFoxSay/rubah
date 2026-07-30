# 📜 Kebijakan Penggunaan, Hak Cipta, & Privasi (Terms of Use, Copyright & Privacy)

Dokumen ini menjelaskan prinsip operasional, penanganan data, privasi, dan atribusi hak cipta pada aplikasi **Rubah (Ruang Baca Harian)**.

---

## 1. Prinsip Operasional & Protokol Sindikasi

**Rubah** adalah aplikasi *Terminal User Interface (TUI)* berbasis *client-side* yang berfungsi sebagai alat pembaca (*reader client*) untuk format sindikasi terbuka (**RSS 2.0 / Atom Feed**).

- Aplikasi memproses umpan data (*feeds*) yang dipublikasikan secara terbuka oleh masing-masing penerbit media berita.
- Pengambilan data dilakukan secara langsung dari perangkat lokal pengguna (*client-side request*) tanpa melalui server perantara terpusat (*middleman proxy server*).

---

## 2. Hak Cipta & Atribusi Konten

1. **Kepemilikan Konten**: Seluruh artikel, naskah, judul, dan gambar berita yang ditampilkan melalui aplikasi ini sepenuhnya merupakan hak cipta milik masing-masing penerbit media atau penulis asli. Rubah tidak mengklaim kepemilikan atas materi tersebut.
2. **Atribusi Sumber**: Aplikasi secara konsisten menyajikan atribusi informasi secara jelas pada setiap artikel, mencakup:
   - ✍️ **Nama Penulis** (*Author*)
   - 📰 **Nama Penerbit / Kanal** (*Publisher Channel*)
   - 📅 **Waktu Publikasi** (*Publication Timestamp*)
   - 🔗 **Tautan Web Asli** (*Original Source URL*)
3. **Navigasi ke Sumber Asli**: Pengguna menyediakan pintasan navigasi (`[o]`) untuk membuka artikel asli langsung di peramban web (*web browser*) resmi penyedia konten.

---

## 3. Batasan Penggunaan & Pembacaan Lokal

- **Penggunaan Pribadi**: Aplikasi ini dirancang sebagai antarmuka pembacaan pribadi (*personal RSS reader client*).
- **Tanpa Monetisasi Konten**: Aplikasi tidak menjual ulang konten berita, tidak menyisipkan iklan pihak ketiga, dan tidak memungut biaya atas akses konten sindikasi.
- **Kesetaraan Fungsi**: Cara kerja antarmuka ini setara dengan peramban web (*web browser*) atau alat pembaca RSS *open-source* pada umumnya.

---

## 4. Kebijakan Privasi & Keamanan Data

- **Penyimpanan Lokal (*Local-First Storage*)**: Seluruh konfigurasi kanal, daftar penanda (*bookmarks*), dan riwayat bacaan disimpan secara eksklusif di dalam basis data lokal di perangkat pengguna (`~/.config/rubah/rubah.db`).
- **Tanpa Telemetri (*Zero Telemetry*)**: Aplikasi tidak mengumpulkan, menyimpan, atau mentransmisikan identitas pengguna, alamat IP, maupun riwayat aktivitas pembacaan ke server eksternal mana pun.

---

## 5. Penolakan Tanggung Jawab (*Disclaimer*)

Aplikasi ini disediakan secara *as-is* (apa adanya). Pengembang aplikasi Rubah tidak bertanggung jawab atas kebenaran, keakuratan, maupun perubahan isi konten yang dipublikasikan oleh penyedia umpan RSS pihak ketiga.

---

## 📄 Lisensi Kode Sumber

Kode sumber aplikasi dirilis di bawah lisensi **GNU General Public License v3.0 (GPL-3.0)**.

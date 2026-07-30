# 📜 Kebijakan Penggunaan, Hak Cipta, & Privasi (Terms of Use, Copyright & Privacy)

Dokumen ini menjelaskan prinsip operasional, penanganan data, arsitektur teknis, dan atribusi hak cipta dari aplikasi **Rubah (Ruang Baca Harian)**.

---

## 1. Prinsip Operasional & Deskripsi Aplikasi

Aplikasi **Rubah** adalah perangkat lunak *client-side Terminal User Interface (TUI)* untuk membaca RSS/Atom Feed yang dikembangkan menggunakan teknologi terbuka (*Open Source*). 

Rubah dirancang untuk memanfaatkan RSS/Atom Feed yang dipublikasikan secara terbuka oleh masing-masing penerbit sesuai standar sindikasi web.

- **Dukungan Standar Feed**: Rubah mendukung standar RSS 2.0 dan Atom Feed sebagaimana dipublikasikan oleh masing-masing penyedia konten.

---

## 2. Arsitektur Terbuka & Pembacaan Lokal (*Local-First Architecture*)

Arsitektur aplikasi Rubah beroperasi secara **100% lokal (*Local-First*)** tanpa melalui server perantara:

```
[ Publisher RSS / Atom ] ──── (Koneksi Langsung) ────> [ Rubah TUI Client ] ────> [ SQLite Lokal ]
```

- **Tanpa Server Perantara**: Rubah tidak memiliki server *cloud*, *proxy*, *mirror*, atau API terpusat yang mengumpulkan, menyimpan, atau mendistribusikan ulang berita milik penerbit.
- **Client-Side Processing**: Seluruh proses pengambilan (*fetching*), penafsiran (*parsing*), dan tampilan dilakukan langsung pada perangkat lokal komputer pengguna.
- **Penyimpanan Eksklusif Lokal**: Konfigurasi kanal, daftar penanda (*bookmarks*), dan riwayat bacaan disimpan di perangkat lokal pengguna (`~/.config/rubah/rubah.db`).

---

## 3. Tujuan Penggunaan & Cara Kerja Aplikasi

Rubah dikembangkan sebagai aplikasi pembaca RSS/Atom untuk penggunaan pribadi maupun umum dengan prinsip teknis berikut:

- **Tidak Menghosting ataupun Mendistribusikan Ulang Konten**: Rubah tidak menyimpan, mengindeks, atau mendistribusikan ulang basis data artikel penerbit secara terpusat.
- **Integritas Konten**: Rubah menampilkan metadata maupun isi RSS/Atom Feed sebagaimana dipublikasikan oleh penyedia feed. Rubah tidak secara sengaja mengubah atribusi, nama penulis, tanggal publikasi, atau tautan ke sumber asli.
- **Integritas Hak Cipta**: Rubah tidak menghapus watermark, atribusi, maupun informasi hak cipta yang disediakan dalam RSS Feed apabila tersedia.
- **Navigasi ke Sumber Asli**: Pengguna menyediakan pintasan navigasi (`[o]`) untuk membuka artikel asli langsung di peramban web (*web browser*) resmi penyedia konten.
- **Kesepadanan Fungsi**: Secara umum Rubah memiliki fungsi yang serupa dengan berbagai aplikasi pembaca RSS/Atom seperti *Newsboat, Liferea, Feedly, Inoreader, NetNewsWire*, maupun pembaca RSS lainnya.

---

## 4. Kepemilikan Konten & Hak Cipta

- **Tanpa Hak Kepemilikan**: Rubah tidak memperoleh hak kepemilikan ataupun lisensi atas konten yang ditampilkan. Seluruh hak tetap berada pada penerbit atau pemegang hak cipta masing-masing.
- **Daftar Feed Bawaan**: Feed bawaan yang disertakan saat pertama kali diinstal hanya disediakan sebagai contoh dan kemudahan bagi pengguna baru. Pengguna bebas menambah, menghapus, atau mengganti feed sesuai kebutuhan. Daftar feed bawaan dapat berubah sewaktu-waktu mengikuti perubahan kebijakan penerbit maupun perkembangan proyek Rubah.

---

## 5. Tanggung Jawab Pengguna (*User Responsibility*)

Pengguna bertanggung jawab atas daftar RSS/Atom Feed yang mereka tambahkan sendiri ke dalam Rubah. Rubah tidak memverifikasi legalitas maupun lisensi dari setiap feed yang dipilih oleh pengguna.

---

## 6. Hak Penerbit (*Publisher Rights*)

Seluruh hak cipta artikel, gambar, dan materi lain tetap menjadi milik penerbit atau pemegang hak cipta masing-masing.

Rubah menghormati hak penerbit. Apabila suatu penerbit meminta agar RSS Feed miliknya tidak lagi disertakan dalam daftar feed bawaan Rubah, permintaan tersebut akan ditinjau dan dihormati sesuai kebijakan proyek.

---

## 7. Permintaan Penghapusan Feed (*Feed Removal Request*)

Apabila Anda merupakan pemilik atau perwakilan resmi suatu penerbit dan ingin agar RSS Feed Anda tidak lagi disertakan dalam daftar feed bawaan Rubah, silakan buat Issue di GitHub atau hubungi maintainer proyek.

Permintaan yang sah akan ditinjau dan diproses dengan itikad baik.

---

## 8. Kebijakan Privasi (*Zero Telemetry*)

Aplikasi Rubah tidak mengumpulkan, menyimpan, atau mentransmisikan identitas pengguna, alamat IP, maupun riwayat aktivitas pembacaan ke server mana pun.

---

## 9. Penolakan Tanggung Jawab (*Disclaimer*)

Aplikasi ini disediakan secara *as-is* (apa adanya). Pengembang aplikasi Rubah tidak bertanggung jawab atas kebenaran, keakuratan, maupun perubahan isi konten yang dipublikasikan oleh penyedia umpan RSS/Atom pihak ketiga.

---

## 📄 Lisensi Kode Sumber

Kode sumber aplikasi dirilis di bawah lisensi **GNU General Public License v3.0 (GPL-3.0)**.

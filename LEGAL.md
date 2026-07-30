# 📜 Kebijakan Penggunaan, Hak Cipta, & Privasi (Terms of Use, Copyright & Privacy)

Dokumen ini menjelaskan prinsip operasional, penanganan data, arsitektur teknis, dan atribusi hak cipta dari aplikasi **Rubah (Ruang Baca Harian)**.

---

## 1. Prinsip Operasional & Deskripsi Aplikasi

Aplikasi **Rubah** adalah perangkat lunak *client-side Terminal User Interface (TUI)* untuk membaca RSS/Atom Feed yang dikembangkan menggunakan teknologi terbuka (*Open Source*). 

Rubah dirancang untuk memanfaatkan RSS/Atom Feed yang dipublikasikan secara terbuka oleh masing-masing penerbit sesuai standar sindikasi web.

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

- **Tidak Menghosting Konten**: Rubah tidak menyimpan atau mengindeks basis data artikel penerbit secara terpusat.
- **Tidak Mengubah Konten**: Rubah menampilkan informasi sebagaimana disediakan oleh RSS Feed publik, termasuk atribusi, nama penulis, tanggal publikasi, dan tautan ke artikel asli.
- **Ketersediaan Pintasan Navigasi**: Pengguna dapat langsung menuju ke halaman situs web resmi penyedia konten dengan menekan tombol pintasan `[o]` (*Open in Browser*).
- **Kesepadanan Fungsi**: Secara umum Rubah memiliki fungsi yang serupa dengan berbagai aplikasi pembaca RSS/Atom seperti *Newsboat, Liferea, Feedly, Inoreader, NetNewsWire*, maupun pembaca RSS lainnya.

---

## 4. Kepemilikan Konten & Hak Cipta

- **Tanpa Hak Kepemilikan**: Rubah tidak memperoleh hak kepemilikan ataupun lisensi atas konten yang ditampilkan. Seluruh hak tetap berada pada penerbit atau pemegang hak cipta masing-masing.
- **Daftar Feed Bawaan**: Feed bawaan yang disertakan saat pertama kali diinstal hanya disediakan sebagai contoh dan kemudahan bagi pengguna baru. Pengguna bebas menambah, menghapus, atau mengganti feed sesuai kebutuhan.

---

## 5. Hak Penerbit (*Publisher Rights*)

Seluruh hak cipta artikel, gambar, dan materi lain tetap menjadi milik penerbit atau pemegang hak cipta masing-masing.

Rubah menghormati hak penerbit. Apabila suatu penerbit meminta agar RSS Feed miliknya tidak lagi disertakan dalam daftar feed bawaan Rubah, permintaan tersebut akan ditinjau dan dihormati sesuai kebijakan proyek.

---

## 6. Kebijakan Privasi (*Zero Telemetry*)

Aplikasi Rubah tidak mengumpulkan, menyimpan, atau mentransmisikan identitas pengguna, alamat IP, maupun riwayat aktivitas pembacaan ke server mana pun.

---

## 7. Penolakan Tanggung Jawab (*Disclaimer*)

Aplikasi ini disediakan secara *as-is* (apa adanya). Pengembang aplikasi Rubah tidak bertanggung jawab atas kebenaran, keakuratan, maupun perubahan isi konten yang dipublikasikan oleh penyedia umpan RSS/Atom pihak ketiga.

---

## 📄 Lisensi Kode Sumber

Kode sumber aplikasi dirilis di bawah lisensi **GNU General Public License v3.0 (GPL-3.0)**.

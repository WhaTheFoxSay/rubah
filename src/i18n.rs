use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    English,
    Indonesian,
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Indonesian => "id",
        }
    }

    #[allow(dead_code)]
    pub fn display_name(&self) -> &'static str {
        match self {
            Language::English => "English (EN)",
            Language::Indonesian => "Bahasa Indonesia (ID)",
        }
    }

    pub fn toggle(&self) -> Self {
        match self {
            Language::English => Language::Indonesian,
            Language::Indonesian => Language::English,
        }
    }

    pub fn from_code(code: &str) -> Self {
        match code {
            "id" => Language::Indonesian,
            _ => Language::English,
        }
    }
}

pub fn t<'a>(lang: Language, key: &'a str) -> &'a str {
    match (lang, key) {
        // App Title & Header
        (Language::English, "sub_title") => "Daily Reading Space",
        (Language::Indonesian, "sub_title") => "Ruang Baca Harian",

        // Pane Headers
        (Language::English, "pane_channels") => " Channels & Categories ",
        (Language::Indonesian, "pane_channels") => " Channel & Kategori ",

        (Language::English, "pane_articles") => " News Articles ",
        (Language::Indonesian, "pane_articles") => " Berita ",

        (Language::English, "pane_reader") => " Article Reader ",
        (Language::Indonesian, "pane_reader") => " Pembaca Berita ",

        // Tabs
        (Language::English, "tab_all_feeds") => "All Feeds",
        (Language::Indonesian, "tab_all_feeds") => "Semua Feed",

        (Language::English, "tab_bookmarks") => "Bookmarks",
        (Language::Indonesian, "tab_bookmarks") => "Bookmark",

        // Search
        (Language::English, "search_title") => " News Search Mode ",
        (Language::Indonesian, "search_title") => " Mode Pencarian Berita ",

        (Language::English, "search_keyword") => " Keywords: ",
        (Language::Indonesian, "search_keyword") => " Kata Kunci: ",

        (Language::English, "search_placeholder") => "[Type search keywords here...]",
        (Language::Indonesian, "search_placeholder") => "[Ketik kata kunci di sini...]",

        (Language::English, "search_hints") => "   [Enter] Open | [Down/Up] Select | [Esc] Reset",
        (Language::Indonesian, "search_hints") => "   [Enter] Buka | [Down/Up] Pilih | [Esc] Reset",

        (Language::English, "search_filter_active") => " Active Search Filter: ",
        (Language::Indonesian, "search_filter_active") => " Filter Cari Aktif: ",

        (Language::English, "search_filter_hint") => " (Press [Esc] to clear search filter)",
        (Language::Indonesian, "search_filter_hint") => " (Tekan [Esc] untuk bersihkan filter pencarian)",

        // Default Status Tip
        (Language::English, "status_tip_prefix") => " Tip: ",
        (Language::Indonesian, "status_tip_prefix") => " Tip: ",

        (Language::English, "default_status") => "Press [?] for help | [l] Language | [/] Search",
        (Language::Indonesian, "default_status") => "Tekan [?] untuk bantuan | [l] Bahasa | [/] Cari",

        // Footer Keys
        (Language::English, "footer_nav") => "Nav",
        (Language::Indonesian, "footer_nav") => "Navigasi",

        (Language::English, "footer_select") => "Select",
        (Language::Indonesian, "footer_select") => "Pilih",

        (Language::English, "footer_open") => "Open",
        (Language::Indonesian, "footer_open") => "Buka",

        (Language::English, "footer_fullscreen") => "Fullscreen",
        (Language::Indonesian, "footer_fullscreen") => "Fullscreen",

        (Language::English, "footer_move_cat") => "Move Cat",
        (Language::Indonesian, "footer_move_cat") => "Pindah Kat",

        (Language::English, "footer_del_cat") => "Delete Cat",
        (Language::Indonesian, "footer_del_cat") => "Hapus Kat",

        (Language::English, "footer_add_feed") => "Add Feed",
        (Language::Indonesian, "footer_add_feed") => "Tambah Feed",

        (Language::English, "footer_del_feed") => "Delete Feed",
        (Language::Indonesian, "footer_del_feed") => "Hapus Feed",

        (Language::English, "footer_update") => "Update",
        (Language::Indonesian, "footer_update") => "Update",

        (Language::English, "footer_lang") => "Lang: EN",
        (Language::Indonesian, "footer_lang") => "Bahasa: ID",

        (Language::English, "footer_help") => "Help",
        (Language::Indonesian, "footer_help") => "Bantuan",

        (Language::English, "footer_quit") => "Quit",
        (Language::Indonesian, "footer_quit") => "Keluar",

        // Help Modal
        (Language::English, "help_title") => " Help ",
        (Language::Indonesian, "help_title") => " Bantuan ",

        (Language::English, "help_heading") => "🦊 Rubah [Daily Reading Space] - Keyboard Shortcuts",
        (Language::Indonesian, "help_heading") => "🦊 Rubah [Ruang Baca Harian] - Bantuan Shortcut Keyboard",

        (Language::English, "help_tab") => ": Switch pane focus",
        (Language::Indonesian, "help_tab") => ": Pindah antar panel",

        (Language::English, "help_jk") => ": Navigate items",
        (Language::Indonesian, "help_jk") => ": Navigasi item",

        (Language::English, "help_enter") => ": Expand Category / Open Article",
        (Language::Indonesian, "help_enter") => ": Toggle Kategori / Buka Artikel",

        (Language::English, "help_f") => ": Toggle Fullscreen Reader Mode",
        (Language::Indonesian, "help_f") => ": Toggle Fullscreen Reader Mode",

        (Language::English, "help_l") => ": Toggle Language (English / Bahasa Indonesia)",
        (Language::Indonesian, "help_l") => ": Ganti Bahasa (English / Bahasa Indonesia)",

        (Language::English, "help_u") => ": Check for Latest Release Update",
        (Language::Indonesian, "help_u") => ": Periksa Pembaruan Rilis Terbaru",

        (Language::English, "help_m") => ": Move Feed to another Category",
        (Language::Indonesian, "help_m") => ": Pindahkan Feed ke Kategori lain",

        (Language::English, "help_shift_c") => ": Delete Category and all feeds inside",
        (Language::Indonesian, "help_shift_c") => ": Hapus Kategori beserta seluruh feed",

        (Language::English, "help_shift_d") => ": Delete selected Feed",
        (Language::Indonesian, "help_shift_d") => ": Hapus Feed terpilih",

        (Language::English, "help_esc") => ": Back to list / Clear search filter",
        (Language::Indonesian, "help_esc") => ": Kembali ke daftar / reset cari",

        (Language::English, "help_i") => ": Toggle Image display ON/OFF",
        (Language::Indonesian, "help_i") => ": Toggle Gambar ON/OFF",

        (Language::English, "help_b") => ": Save / Remove Bookmark",
        (Language::Indonesian, "help_b") => ": Simpan / hapus Bookmark",

        (Language::English, "help_o") => ": Open article in Web Browser",
        (Language::Indonesian, "help_o") => ": Buka artikel di Web Browser",

        (Language::English, "help_r") => ": Refresh / Reload all RSS feeds",
        (Language::Indonesian, "help_r") => ": Refresh / reload seluruh feed",

        (Language::English, "help_a") => ": Add new RSS Feed source",
        (Language::Indonesian, "help_a") => ": Tambah channel RSS Feed baru",

        (Language::English, "help_search") => ": Search news in real-time",
        (Language::Indonesian, "help_search") => ": Cari berita realtime",

        (Language::English, "help_tabs") => ": Switch Tab (All Feeds / Bookmarks)",
        (Language::Indonesian, "help_tabs") => ": Switch Tab (Semua Feed / Bookmark)",

        (Language::English, "help_q") => ": Exit application",
        (Language::Indonesian, "help_q") => ": Keluar dari aplikasi",

        (Language::English, "help_close") => "Press Esc or [?] to close help modal",
        (Language::Indonesian, "help_close") => "Tekan Esc atau [?] untuk menutup bantuan ini",

        // Add Feed Modal
        (Language::English, "add_modal_title") => " Add New RSS Feed ",
        (Language::Indonesian, "add_modal_title") => " Tambah RSS Feed Baru ",

        (Language::English, "add_modal_heading") => "➕ Add New RSS Feed Channel",
        (Language::Indonesian, "add_modal_heading") => "➕ Tambah Channel RSS Feed Baru",

        (Language::English, "add_field_title") => "Feed Name / Title: ",
        (Language::Indonesian, "add_field_title") => "Nama Feed / Title: ",

        (Language::English, "add_field_url") => "RSS Feed URL   : ",
        (Language::Indonesian, "add_field_url") => "URL Feed RSS   : ",

        (Language::English, "add_field_cat") => "Category       : ",
        (Language::Indonesian, "add_field_cat") => "Kategori       : ",

        (Language::English, "add_hints") => "  [Enter] Next Field | [Esc] Cancel",
        (Language::Indonesian, "add_hints") => "  [Enter] Lanjut Field | [Esc] Batal",

        // Move Category Modal
        (Language::English, "move_modal_title") => " Move Feed Category ",
        (Language::Indonesian, "move_modal_title") => " Pindah Kategori Feed ",

        (Language::English, "move_modal_prompt") => "Enter target category name for: ",
        (Language::Indonesian, "move_modal_prompt") => "Masukkan nama kategori baru untuk: ",

        (Language::English, "move_hints") => "  [Enter] Save Category | [Esc] Cancel",
        (Language::Indonesian, "move_hints") => "  [Enter] Simpan Kategori | [Esc] Batal",

        // Delete Category Modal
        (Language::English, "del_cat_title") => " Delete Category Confirmation ",
        (Language::Indonesian, "del_cat_title") => " Konfirmasi Hapus Kategori ",

        (Language::English, "del_cat_warning") => "Are you sure you want to delete category: ",
        (Language::Indonesian, "del_cat_warning") => "Apakah Anda yakin ingin menghapus kategori: ",

        (Language::English, "del_cat_sub") => "(All feeds in this category will be removed)",
        (Language::Indonesian, "del_cat_sub") => "(Seluruh feed di kategori ini akan ikut terhapus)",

        (Language::English, "del_cat_hints") => "  [Y] Confirm Delete | [N / Esc] Cancel",
        (Language::Indonesian, "del_cat_hints") => "  [Y] Ya, Hapus | [N / Esc] Batal",

        // Update Modal
        (Language::English, "update_title_new") => " Update Available ",
        (Language::Indonesian, "update_title_new") => " Pembaruan Tersedia ",

        (Language::English, "update_title_latest") => " Latest Version ",
        (Language::Indonesian, "update_title_latest") => " Versi Terupdate ",

        (Language::English, "update_msg_new") => "🦊 Rubah Application Update Available!",
        (Language::Indonesian, "update_msg_new") => "🦊 Pembaruan Aplikasi Rubah Tersedia!",

        (Language::English, "update_msg_latest") => "🦊 Rubah [Daily Reading Space]",
        (Language::Indonesian, "update_msg_latest") => "🦊 Rubah [Ruang Baca Harian]",

        (Language::English, "update_curr_ver") => "Installed Version: ",
        (Language::Indonesian, "update_curr_ver") => "Versi Terpasang : ",

        (Language::English, "update_latest_ver") => "Latest Version   : ",
        (Language::Indonesian, "update_latest_ver") => "Versi Terbaru   : ",

        (Language::English, "update_notes_label") => "Release Notes:",
        (Language::Indonesian, "update_notes_label") => "Catatan Pembaruan Rilis:",

        (Language::English, "update_run_installer") => "To update to the latest version, run installer:",
        (Language::Indonesian, "update_run_installer") => "Untuk meng-update ke versi terbaru, jalankan installer:",

        (Language::English, "update_status_up_to_date") => "Already using the latest version.",
        (Language::Indonesian, "update_status_up_to_date") => "Sudah menggunakan versi terbaru.",

        (Language::English, "update_msg_up_to_date") => "Your application is fully updated and secure.",
        (Language::Indonesian, "update_msg_up_to_date") => "Aplikasi Anda sudah di versi paling terupdate dan aman.",

        (Language::English, "update_close_hint") => "Press Esc or [Enter] to close this modal",
        (Language::Indonesian, "update_close_hint") => "Tekan Esc atau [Enter] untuk menutup modal ini",

        // Reader details
        (Language::English, "reader_author") => "Author: ",
        (Language::Indonesian, "reader_author") => "Penulis: ",

        (Language::English, "reader_source") => "Source: ",
        (Language::Indonesian, "reader_source") => "Sumber: ",

        (Language::English, "reader_published") => "Published: ",
        (Language::Indonesian, "reader_published") => "Diterbitkan: ",

        (Language::English, "reader_press_o") => "Press [o] to open full article in web browser",
        (Language::Indonesian, "reader_press_o") => "Tekan [o] untuk membuka artikel lengkap di web browser",

        (Language::English, "reader_summary_label") => "ARTICLE SUMMARY:",
        (Language::Indonesian, "reader_summary_label") => "RINGKASAN ARTIKEL:",

        (Language::English, "reader_full_label") => "FULL ARTICLE CONTENT:",
        (Language::Indonesian, "reader_full_label") => "ISI LENGKAP ARTIKEL:",

        (Language::English, "reader_select_prompt") => "Select an article from the list to start reading",
        (Language::Indonesian, "reader_select_prompt") => "Pilih artikel di daftar sebelah kiri untuk mulai membaca",

        // Uninstall Modal & CLI Messages
        (Language::English, "uninstall_title") => " Uninstall Confirmation ",
        (Language::Indonesian, "uninstall_title") => " Konfirmasi Uninstall ",

        (Language::English, "uninstall_heading") => "Rubah Application Uninstall Confirmation",
        (Language::Indonesian, "uninstall_heading") => "Konfirmasi Uninstall Rubah",

        (Language::English, "uninstall_body_1") => "Are you sure you want to uninstall Rubah",
        (Language::Indonesian, "uninstall_body_1") => "Apakah Anda yakin ingin menghapus Rubah",

        (Language::English, "uninstall_body_2") => "and remove all configuration & data from your system?",
        (Language::Indonesian, "uninstall_body_2") => "dan seluruh data konfigurasinya dari sistem?",

        (Language::English, "uninstall_press_y") => "Press ",
        (Language::Indonesian, "uninstall_press_y") => "Tekan ",

        (Language::English, "uninstall_y_label") => " for Yes, or ",
        (Language::Indonesian, "uninstall_y_label") => " untuk Ya, atau ",

        (Language::English, "uninstall_n_label") => " for Cancel",
        (Language::Indonesian, "uninstall_n_label") => " untuk Batal",

        (Language::English, "uninstall_bin_deleted") => "Binary & symlink         ~/.local/bin/baca deleted",
        (Language::Indonesian, "uninstall_bin_deleted") => "Binary & symlink         ~/.local/bin/baca terhapus",

        (Language::English, "uninstall_config_deleted") => "Config & database        ~/.config/rubah deleted",
        (Language::Indonesian, "uninstall_config_deleted") => "Config & database        ~/.config/rubah terhapus",

        (Language::English, "uninstall_cache_deleted") => "Cache & temp files       ~/.cache/rubah deleted",
        (Language::Indonesian, "uninstall_cache_deleted") => "Cache & temp files       ~/.cache/rubah terhapus",

        (Language::English, "uninstall_done_msg") => "✔ Rubah application successfully uninstalled from your system.",
        (Language::Indonesian, "uninstall_done_msg") => "✔ Aplikasi Rubah berhasil di-uninstall dari sistem Anda.",

        (Language::English, "uninstall_thanks_msg") => "Thank you for using Rubah [Daily Reading Space].",
        (Language::Indonesian, "uninstall_thanks_msg") => "Terima kasih telah menggunakan Rubah [Ruang Baca Harian].",

        (Language::English, "uninstall_goodbye_msg") => "See you again! 🦊",
        (Language::Indonesian, "uninstall_goodbye_msg") => "Sampai jumpa kembali! 🦊",

        _ => key,
    }
}

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rubah")]
#[command(author = "Rubah Team")]
#[command(version = "0.1.0")]
#[command(about = "Ruang Baca Harian - High-performance RSS Feed Reader TUI for Terminal", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Tambah RSS Feed link baru ke Rubah
    Add {
        /// URL feed RSS/Atom
        #[arg(short, long)]
        url: String,

        /// Judul channel RSS (opsional)
        #[arg(short, long)]
        title: Option<String>,

        /// Kategori channel (opsional, default: Umum)
        #[arg(short, long, default_value = "Umum")]
        category: String,
    },

    /// Tampilkan daftar seluruh RSS Feed yang tersimpan
    List,

    /// Hapus/uninstall aplikasi Rubah & konfigurasinya dari sistem
    Uninstall,
}

use crate::models::{Article, FeedSource};
use crate::network::Fetcher;
use crate::storage::Storage;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePane {
    Feeds,
    Articles,
    Reader,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveTab {
    AllFeeds,
    Bookmarks,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Search,
    AddFeedTitle,
    AddFeedUrl,
    AddFeedCategory,
}

pub struct App {
    pub storage: Storage,
    pub fetcher: Fetcher,
    pub feeds: Vec<FeedSource>,
    pub articles_by_feed: HashMap<String, Vec<Article>>,
    pub read_articles: HashSet<String>,
    pub active_pane: ActivePane,
    pub active_tab: ActiveTab,
    pub selected_feed_idx: usize,
    pub selected_article_idx: usize,
    pub reader_scroll: u16,
    pub is_loading: bool,
    pub status_message: String,
    pub show_help: bool,
    pub show_uninstall_confirm: bool,
    pub current_image_url: Option<String>,

    // Search & Filter
    pub input_mode: InputMode,
    pub search_query: String,

    // Add Feed Form
    pub new_feed_title: String,
    pub new_feed_url: String,
    pub new_feed_category: String,
}

impl App {
    pub fn new() -> Self {
        let storage = Storage::new();
        let feeds = storage.get_feeds().unwrap_or_default();
        let read_articles = storage.get_read_article_ids();
        let fetcher = Fetcher::new();

        Self {
            storage,
            fetcher,
            feeds,
            articles_by_feed: HashMap::new(),
            read_articles,
            active_pane: ActivePane::Feeds,
            active_tab: ActiveTab::AllFeeds,
            selected_feed_idx: 0,
            selected_article_idx: 0,
            reader_scroll: 0,
            is_loading: false,
            status_message: "Tekan [?] Bantuan | [Enter] Baca Penuh | [v] Lihat Foto Asli HD | [r] Refresh | [/] Cari".to_string(),
            show_help: false,
            show_uninstall_confirm: false,
            current_image_url: None,
            input_mode: InputMode::Normal,
            search_query: String::new(),
            new_feed_title: String::new(),
            new_feed_url: String::new(),
            new_feed_category: "Umum".to_string(),
        }
    }

    pub async fn refresh_all_feeds(&mut self) {
        self.is_loading = true;
        self.status_message = "Memuat ulang seluruh RSS feed...".to_string();

        let results = self.fetcher.fetch_all_feeds(&self.feeds).await;
        let mut count = 0;

        for (feed_id, res) in results {
            if let Ok(articles) = res {
                count += articles.len();
                self.articles_by_feed.insert(feed_id, articles);
            }
        }

        self.is_loading = false;
        self.status_message = format!("Selesai! Dimuat {} berita dari {} channel.", count, self.feeds.len());
    }

    pub async fn fetch_full_content_for_selected(&mut self) {
        let (article_id, article_link, article_title) = match self.current_article() {
            Some(art) => (art.id, art.link, art.title),
            None => return,
        };

        if article_link.is_empty() {
            return;
        }

        self.status_message = format!("📥 Mengunduh isi artikel: '{}'...", article_title);
        self.current_image_url = None;

        match self.fetcher.fetch_full_article_body(&article_link).await {
            Ok(res) => {
                let full_text = res.body_text;
                self.current_image_url = res.image_url;

                if !full_text.trim().is_empty() {
                    // Update in articles_by_feed
                    if !self.feeds.is_empty() && self.selected_feed_idx < self.feeds.len() {
                        let feed_id = &self.feeds[self.selected_feed_idx].id;
                        if let Some(articles) = self.articles_by_feed.get_mut(feed_id) {
                            for art in articles.iter_mut() {
                                if art.id == article_id {
                                    art.content = full_text.clone();
                                }
                            }
                        }
                    }
                }

                if self.current_image_url.is_some() {
                    self.status_message = "✅ Artikel dimuat! Tekan [v] untuk melihat Foto Asli HD.".to_string();
                } else {
                    self.status_message = "✅ Artikel penuh berhasil dimuat!".to_string();
                }
            }
            Err(e) => {
                self.status_message = format!("Gagal memuat artikel: {}", e);
            }
        }
    }

    pub fn view_real_image(&mut self) {
        if let Some(art) = self.current_article() {
            if let Some(img_url) = self.current_image_url.clone() {
                let title = art.title.clone();
                self.status_message = format!("📸 Membuka foto asli HD: '{}'...", title);
                tokio::spawn(async move {
                    if let Ok(response) = reqwest::get(&img_url).await {
                        if let Ok(bytes) = response.bytes().await {
                            let cache_dir = dirs::cache_dir()
                                .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
                                .join("rubah");
                            let _ = std::fs::create_dir_all(&cache_dir);
                            let img_path = cache_dir.join("photo.jpg");
                            if std::fs::write(&img_path, &bytes).is_ok() {
                                let _ = open::that(&img_path);
                            }
                        }
                    }
                });
            } else {
                self.status_message = "Tidak ada foto untuk artikel ini.".to_string();
            }
        }
    }

    pub fn current_articles(&self) -> Vec<Article> {
        if self.active_tab == ActiveTab::Bookmarks {
            let mut bookmarks = self.storage.get_bookmarks();
            for art in &mut bookmarks {
                art.is_bookmarked = true;
                art.is_read = self.read_articles.contains(&art.id);
            }
            return self.filter_articles(bookmarks);
        }

        if self.feeds.is_empty() || self.selected_feed_idx >= self.feeds.len() {
            return Vec::new();
        }

        let feed_id = &self.feeds[self.selected_feed_idx].id;
        let articles = self.articles_by_feed.get(feed_id).cloned().unwrap_or_default();

        let mut processed = Vec::new();
        let bookmarked_ids: HashSet<String> = self.storage.get_bookmarks().iter().map(|b| b.id.clone()).collect();

        for mut art in articles {
            art.is_read = self.read_articles.contains(&art.id);
            art.is_bookmarked = bookmarked_ids.contains(&art.id);
            processed.push(art);
        }

        self.filter_articles(processed)
    }

    fn filter_articles(&self, articles: Vec<Article>) -> Vec<Article> {
        if self.search_query.is_empty() {
            return articles;
        }

        let query = self.search_query.to_lowercase();
        articles
            .into_iter()
            .filter(|a| {
                a.title.to_lowercase().contains(&query)
                    || a.summary.to_lowercase().contains(&query)
                    || a.author.to_lowercase().contains(&query)
            })
            .collect()
    }

    pub fn current_article(&self) -> Option<Article> {
        let articles = self.current_articles();
        if articles.is_empty() || self.selected_article_idx >= articles.len() {
            None
        } else {
            Some(articles[self.selected_article_idx].clone())
        }
    }

    pub fn next_pane(&mut self) {
        self.active_pane = match self.active_pane {
            ActivePane::Feeds => ActivePane::Articles,
            ActivePane::Articles => ActivePane::Reader,
            ActivePane::Reader => ActivePane::Feeds,
        };
    }

    pub fn prev_pane(&mut self) {
        self.active_pane = match self.active_pane {
            ActivePane::Feeds => ActivePane::Reader,
            ActivePane::Articles => ActivePane::Feeds,
            ActivePane::Reader => ActivePane::Articles,
        };
    }

    pub fn next_item(&mut self) {
        match self.active_pane {
            ActivePane::Feeds => {
                if !self.feeds.is_empty() {
                    self.selected_feed_idx = (self.selected_feed_idx + 1) % self.feeds.len();
                    self.selected_article_idx = 0;
                    self.reader_scroll = 0;
                    self.current_image_url = None;
                }
            }
            ActivePane::Articles => {
                let len = self.current_articles().len();
                if len > 0 {
                    self.selected_article_idx = (self.selected_article_idx + 1) % len;
                    self.reader_scroll = 0;
                    self.current_image_url = None;
                    self.mark_current_read();
                }
            }
            ActivePane::Reader => {
                self.reader_scroll = self.reader_scroll.saturating_add(1);
            }
        }
    }

    pub fn scroll_reader_down(&mut self) {
        self.reader_scroll = self.reader_scroll.saturating_add(5);
    }

    pub fn scroll_reader_up(&mut self) {
        self.reader_scroll = self.reader_scroll.saturating_sub(5);
    }

    pub fn prev_item(&mut self) {
        match self.active_pane {
            ActivePane::Feeds => {
                if !self.feeds.is_empty() {
                    if self.selected_feed_idx == 0 {
                        self.selected_feed_idx = self.feeds.len() - 1;
                    } else {
                        self.selected_feed_idx -= 1;
                    }
                    self.selected_article_idx = 0;
                    self.reader_scroll = 0;
                    self.current_image_url = None;
                }
            }
            ActivePane::Articles => {
                let len = self.current_articles().len();
                if len > 0 {
                    if self.selected_article_idx == 0 {
                        self.selected_article_idx = len - 1;
                    } else {
                        self.selected_article_idx -= 1;
                    }
                    self.reader_scroll = 0;
                    self.current_image_url = None;
                    self.mark_current_read();
                }
            }
            ActivePane::Reader => {
                self.reader_scroll = self.reader_scroll.saturating_sub(2);
            }
        }
    }

    pub fn mark_current_read(&mut self) {
        if let Some(art) = self.current_article() {
            if !self.read_articles.contains(&art.id) {
                self.read_articles.insert(art.id.clone());
                let _ = self.storage.mark_article_read(&art.id);
            }
        }
    }

    pub fn toggle_current_bookmark(&mut self) {
        if let Some(art) = self.current_article() {
            if let Ok(added) = self.storage.toggle_bookmark(&art) {
                if added {
                    self.status_message = format!("Disimpan ke Bookmark: '{}'", art.title);
                } else {
                    self.status_message = format!("Dihapus dari Bookmark: '{}'", art.title);
                }
            }
        }
    }

    pub fn open_current_in_browser(&mut self) {
        if let Some(art) = self.current_article() {
            if !art.link.is_empty() {
                if open::that(&art.link).is_ok() {
                    self.status_message = format!("Membuka browser: {}", art.link);
                } else {
                    self.status_message = format!("Gagal membuka link: {}", art.link);
                }
            }
        }
    }

    pub fn delete_selected_feed(&mut self) {
        if !self.feeds.is_empty() && self.selected_feed_idx < self.feeds.len() {
            let feed = self.feeds.remove(self.selected_feed_idx);
            let _ = self.storage.delete_feed(&feed.id);
            self.status_message = format!("Feed '{}' berhasil dihapus.", feed.title);
            if self.selected_feed_idx >= self.feeds.len() && !self.feeds.is_empty() {
                self.selected_feed_idx = self.feeds.len() - 1;
            }
        }
    }

    pub fn submit_new_feed(&mut self) {
        if !self.new_feed_title.is_empty() && !self.new_feed_url.is_empty() {
            let feed = FeedSource::new(&self.new_feed_title, &self.new_feed_url, &self.new_feed_category);
            let _ = self.storage.add_feed(&feed);
            self.status_message = format!("Feed baru '{}' ditambahkan!", feed.title);
            self.feeds.push(feed);
            self.new_feed_title.clear();
            self.new_feed_url.clear();
            self.new_feed_category = "Umum".to_string();
            self.input_mode = InputMode::Normal;
        } else {
            self.status_message = "Judul dan URL feed tidak boleh kosong!".to_string();
        }
    }

    pub fn perform_uninstall() -> Result<(), Box<dyn std::error::Error>> {
        if let Some(home) = dirs::home_dir() {
            let _ = std::fs::remove_file(home.join(".local").join("bin").join("baca"));
            let _ = std::fs::remove_file(home.join(".local").join("bin").join("rubah"));
        }
        if let Some(config_dir) = dirs::config_dir() {
            let _ = std::fs::remove_dir_all(config_dir.join("rubah"));
        }
        Ok(())
    }
}

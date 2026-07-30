use crate::image_render::render_image_to_lines;
use crate::models::{Article, FeedSource};
use crate::network::Fetcher;
use crate::storage::Storage;
use ratatui::text::Line;
use std::collections::{HashMap, HashSet};

use ratatui::widgets::ListState;

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
    pub feed_list_state: ListState,
    pub article_list_state: ListState,
    pub reader_scroll: u16,
    pub is_loading: bool,
    pub status_message: String,
    pub show_help: bool,
    pub show_uninstall_confirm: bool,
    pub show_image: bool,
    pub current_image_lines: Option<Vec<Line<'static>>>,
    pub latency_ms: Option<u128>,
    pub article_cache: HashMap<String, (String, Option<Vec<Line<'static>>>)>,

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

        let mut feed_list_state = ListState::default();
        feed_list_state.select(Some(0));
        let mut article_list_state = ListState::default();
        article_list_state.select(Some(0));

        Self {
            storage,
            fetcher,
            feeds,
            articles_by_feed: HashMap::new(),
            article_cache: HashMap::new(),
            read_articles,
            active_pane: ActivePane::Feeds,
            active_tab: ActiveTab::AllFeeds,
            selected_feed_idx: 0,
            selected_article_idx: 0,
            feed_list_state,
            article_list_state,
            reader_scroll: 0,
            is_loading: false,
            status_message: "Tekan [?] Bantuan | [j/k] Pilih | [Enter] Baca Penuh | [i] Gambar | [/] Cari".to_string(),
            show_help: false,
            show_uninstall_confirm: false,
            show_image: true,
            current_image_lines: None,
            latency_ms: None,
            input_mode: InputMode::Normal,
            search_query: String::new(),
            new_feed_title: String::new(),
            new_feed_url: String::new(),
            new_feed_category: "Umum".to_string(),
        }
    }

    pub async fn update_latency(&mut self) {
        let start = std::time::Instant::now();
        let client = self.fetcher.clone();
        tokio::spawn(async move {
            let _ = client;
        });

        // Fast network latency measurement to Cloudflare DNS 1.1.1.1
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(2))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        if client.get("https://1.1.1.1").send().await.is_ok() {
            self.latency_ms = Some(start.elapsed().as_millis());
        } else {
            self.latency_ms = None;
        }
    }

    pub fn toggle_image_display(&mut self) {
        self.show_image = !self.show_image;
        if self.show_image {
            self.status_message = "Gambar [ON]".to_string();
        } else {
            self.status_message = "Gambar [OFF]".to_string();
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

        // Fast Cache Hit (0ms)
        if let Some((cached_text, cached_img)) = self.article_cache.get(&article_id) {
            let full_text = cached_text.clone();
            self.current_image_lines = cached_img.clone();
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
            self.status_message = "Tekan [?] Bantuan | [j/k] Pilih | [Enter] Baca Penuh | [i] Gambar | [/] Cari".to_string();
            return;
        }

        self.status_message = format!("Memuat: '{}'...", article_title);
        self.current_image_lines = None;

        match self.fetcher.fetch_full_article_body(&article_link).await {
            Ok(res) => {
                let full_text = res.body_text;

                if !full_text.trim().is_empty() {
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

                let mut rendered_img = None;
                if let Some(img_url) = res.image_url {
                    if let Some(bytes) = self.fetcher.fetch_image_bytes(&img_url).await {
                        if let Some(lines) = render_image_to_lines(&bytes, 44, 14) {
                            rendered_img = Some(lines.clone());
                            self.current_image_lines = Some(lines);
                        }
                    }
                }

                self.article_cache.insert(article_id, (full_text, rendered_img));
                self.status_message = "Tekan [?] Bantuan | [j/k] Pilih | [Enter] Baca Penuh | [i] Gambar | [/] Cari".to_string();
            }
            Err(e) => {
                self.status_message = format!("Gagal memuat artikel: {}", e);
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
                    self.feed_list_state.select(Some(self.selected_feed_idx));
                    self.article_list_state.select(Some(0));
                    self.reader_scroll = 0;
                    self.current_image_lines = None;
                }
            }
            ActivePane::Articles => {
                let len = self.current_articles().len();
                if len > 0 {
                    self.selected_article_idx = (self.selected_article_idx + 1) % len;
                    self.article_list_state.select(Some(self.selected_article_idx));
                    self.reader_scroll = 0;
                    self.current_image_lines = None;
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
                    self.feed_list_state.select(Some(self.selected_feed_idx));
                    self.article_list_state.select(Some(0));
                    self.reader_scroll = 0;
                    self.current_image_lines = None;
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
                    self.article_list_state.select(Some(self.selected_article_idx));
                    self.reader_scroll = 0;
                    self.current_image_lines = None;
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

        if let Some(data_dir) = dirs::data_local_dir() {
            let _ = std::fs::remove_dir_all(data_dir.join("rubah"));
            let _ = std::fs::remove_dir_all(data_dir.join("Programs").join("Rubah"));
        }

        // On Windows, running .exe cannot be deleted synchronously while active.
        // We spawn a 1-second delayed background CMD process to clean up baca.exe and its folder upon exit.
        #[cfg(target_os = "windows")]
        {
            if let Ok(exe_path) = std::env::current_exe() {
                if let Some(parent_dir) = exe_path.parent() {
                    let parent_str = parent_dir.to_string_lossy();
                    let _ = std::process::Command::new("powershell")
                        .args(&[
                            "-NoProfile",
                            "-WindowStyle",
                            "Hidden",
                            "-Command",
                            &format!("Start-Sleep -Seconds 1; Remove-Item -Path '{}' -Recurse -Force", parent_str),
                        ])
                        .spawn();
                }
            }
        }

        Ok(())
    }
}

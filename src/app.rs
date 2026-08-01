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
    MoveFeedCategory,
    DeleteCategoryConfirm,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChannelTreeItem {
    CategoryHeader {
        name: String,
        is_expanded: bool,
        count: usize,
    },
    FeedItem {
        feed: FeedSource,
        category: String,
    },
}

pub struct App {
    pub storage: Storage,
    pub fetcher: Fetcher,
    pub feeds: Vec<FeedSource>,
    pub articles_by_feed: HashMap<String, Vec<Article>>,
    pub read_articles: HashSet<String>,
    pub active_pane: ActivePane,
    pub active_tab: ActiveTab,
    pub selected_tree_idx: usize,
    pub selected_article_idx: usize,
    pub feed_list_state: ListState,
    pub article_list_state: ListState,
    pub expanded_categories: HashSet<String>,
    pub marquee_tick: usize,
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

    // Move Feed Form & Delete Category
    pub move_feed_category_input: String,
    pub target_category_to_delete: Option<String>,

    // Fullscreen Reader Mode
    pub is_fullscreen_reader: bool,

    // Check Update Modal
    pub is_checking_update: bool,
    pub update_info: Option<crate::network::UpdateInfo>,
    pub show_update_modal: bool,

    // Internationalization (i18n)
    pub language: crate::i18n::Language,

    // In-App Self-Update Progress State
    pub is_updating_in_app: bool,
    pub update_percentage: f32,
    pub update_downloaded_bytes: u64,
    pub update_total_bytes: u64,
    pub update_stage_status: String,
    pub update_completed: bool,
    pub update_failed: Option<String>,
}

impl App {
    pub fn new() -> Self {
        let storage = Storage::new();
        let feeds = storage.get_feeds().unwrap_or_default();
        let read_articles = storage.get_read_article_ids();
        let fetcher = Fetcher::new();

        let language = storage
            .get_setting("language")
            .map(|code| crate::i18n::Language::from_code(&code))
            .unwrap_or_default();

        let mut expanded_categories = HashSet::new();
        for feed in &feeds {
            expanded_categories.insert(feed.category.clone());
        }
        if expanded_categories.is_empty() {
            expanded_categories.insert("Umum".to_string());
        }

        let mut feed_list_state = ListState::default();
        if !feeds.is_empty() {
            feed_list_state.select(Some(0));
        }
        let article_list_state = ListState::default();

        let default_status = crate::i18n::t(language, "default_status").to_string();

        Self {
            storage,
            fetcher,
            feeds,
            articles_by_feed: HashMap::new(),
            article_cache: HashMap::new(),
            read_articles,
            active_pane: ActivePane::Feeds,
            active_tab: ActiveTab::AllFeeds,
            selected_tree_idx: 0,
            selected_article_idx: 0,
            feed_list_state,
            article_list_state,
            expanded_categories,
            marquee_tick: 0,
            reader_scroll: 0,
            is_loading: false,
            status_message: default_status,
            show_help: false,
            show_uninstall_confirm: false,
            show_image: true,
            current_image_lines: None,
            latency_ms: None,
            input_mode: InputMode::Normal,
            search_query: String::new(),
            new_feed_title: String::new(),
            new_feed_url: String::new(),
            new_feed_category: String::new(),
            move_feed_category_input: String::new(),
            target_category_to_delete: None,
            is_fullscreen_reader: false,
            is_checking_update: false,
            update_info: None,
            show_update_modal: false,
            language,
            is_updating_in_app: false,
            update_percentage: 0.0,
            update_downloaded_bytes: 0,
            update_total_bytes: 0,
            update_stage_status: String::new(),
            update_completed: false,
            update_failed: None,
        }
    }

    pub fn start_in_app_update(&mut self, tx: tokio::sync::mpsc::UnboundedSender<crate::network::UpdateProgress>) {
        if let Some(ref info) = self.update_info {
            let latest_version = info.latest_version.clone();
            self.is_updating_in_app = true;
            self.update_percentage = 0.0;
            self.update_stage_status = "Connecting to GitHub Release Assets...".to_string();
            self.update_completed = false;
            self.update_failed = None;

            let fetcher = self.fetcher.clone();
            tokio::spawn(async move {
                if let Err(e) = fetcher.download_and_install_update(&latest_version, tx.clone()).await {
                    let _ = tx.send(crate::network::UpdateProgress::Failed(e));
                }
            });
        }
    }

    pub fn toggle_language(&mut self) {
        self.language = self.language.toggle();
        let _ = self.storage.set_setting("language", self.language.code());
        let msg = match self.language {
            crate::i18n::Language::English => "Language changed to English (EN)",
            crate::i18n::Language::Indonesian => "Bahasa diubah ke Bahasa Indonesia (ID)",
            crate::i18n::Language::Japanese => "言語を日本語 (JA) に変更しました",
            crate::i18n::Language::Dutch => "Taal gewijzigd naar Nederlands (NL)",
            crate::i18n::Language::Spanish => "Idioma cambiado a Español (ES)",
            crate::i18n::Language::Arabic => "تم تغيير اللغة إلى العربية (AR)",
        };
        self.status_message = format!("[OK] {}", msg);
    }

    pub async fn check_for_update_async(&mut self) {
        self.is_checking_update = true;
        self.status_message = match self.language {
            crate::i18n::Language::Indonesian => "🔄 Memeriksa pembaruan rilis terbaru dari GitHub...".to_string(),
            crate::i18n::Language::Japanese => "🔄 GitHubから最新リリースをチェック中...".to_string(),
            crate::i18n::Language::Dutch => "🔄 Controleren op nieuwste release van GitHub...".to_string(),
            crate::i18n::Language::Spanish => "🔄 Comprobando la última versión de GitHub...".to_string(),
            crate::i18n::Language::Arabic => "🔄 جاري التحقق من أحدث إصدار من GitHub...".to_string(),
            _ => "🔄 Checking for latest release update from GitHub...".to_string(),
        };

        let fetcher = self.fetcher.clone();
        match fetcher.check_for_update().await {
            Ok(info) => {
                self.is_checking_update = false;
                if info.has_update {
                    self.status_message = match self.language {
                        crate::i18n::Language::Indonesian => format!("✨ Pembaruan tersedia: v{}! (Versi saat ini: v{}).", info.latest_version, info.current_version),
                        crate::i18n::Language::Japanese => format!("✨ 更新が利用可能: v{}! (現在のバージョン: v{}).", info.latest_version, info.current_version),
                        crate::i18n::Language::Dutch => format!("✨ Update beschikbaar: v{}! (Huidige versie: v{}).", info.latest_version, info.current_version),
                        crate::i18n::Language::Spanish => format!("✨ Actualización disponible: v{}! (Versión actual: v{}).", info.latest_version, info.current_version),
                        crate::i18n::Language::Arabic => format!("✨ التحديث متاح: v{}! (الإصدار الحالي: v{}).", info.latest_version, info.current_version),
                        _ => format!("✨ Update available: v{}! (Current version: v{}).", info.latest_version, info.current_version),
                    };
                    self.update_info = Some(info);
                    self.show_update_modal = true;
                } else {
                    self.status_message = match self.language {
                        crate::i18n::Language::Indonesian => format!("[OK] Rubah sudah di versi terbaru (v{}). Tidak ada pembaruan.", info.current_version),
                        crate::i18n::Language::Japanese => format!("[OK] Rubahは最新バージョン (v{}) です。更新は必要ありません。", info.current_version),
                        crate::i18n::Language::Dutch => format!("[OK] Rubah is op de nieuwste versie (v{}). Geen updates.", info.current_version),
                        crate::i18n::Language::Spanish => format!("[OK] Rubah está en la versión más reciente (v{}). No hay actualizaciones.", info.current_version),
                        crate::i18n::Language::Arabic => format!("[OK] Rubah في أحدث إصدار (v{}). لا توجد تحديثات.", info.current_version),
                        _ => format!("[OK] Rubah is already on the latest version (v{}). No update needed.", info.current_version),
                    };
                    self.update_info = Some(info);
                    self.show_update_modal = true;
                }
            }
            Err(err) => {
                self.is_checking_update = false;
                self.status_message = match self.language {
                    crate::i18n::Language::Indonesian => format!("⚠️ Gagal periksa update: {}", err),
                    crate::i18n::Language::Japanese => format!("⚠️ 更新の確認に失敗しました: {}", err),
                    crate::i18n::Language::Dutch => format!("⚠️ Controleren op update mislukt: {}", err),
                    crate::i18n::Language::Spanish => format!("⚠️ Error al comprobar actualización: {}", err),
                    crate::i18n::Language::Arabic => format!("⚠️ فشل التحقق من التحديث: {}", err),
                    _ => format!("⚠️ Failed to check for update: {}", err),
                };
            }
        }
    }

    pub fn get_existing_categories(&self) -> Vec<String> {
        let mut categories = Vec::new();
        for feed in &self.feeds {
            if !categories.contains(&feed.category) {
                categories.push(feed.category.clone());
            }
        }
        if categories.is_empty() {
            categories.push("Umum".to_string());
        }
        categories
    }

    pub fn visible_channel_items(&self) -> Vec<ChannelTreeItem> {
        let mut items = Vec::new();
        let categories = self.get_existing_categories();

        for cat in categories {
            let cat_feeds: Vec<&FeedSource> = self.feeds.iter().filter(|f| f.category == cat).collect();
            let count = cat_feeds.len();
            let is_expanded = self.expanded_categories.contains(&cat);

            items.push(ChannelTreeItem::CategoryHeader {
                name: cat.clone(),
                is_expanded,
                count,
            });

            if is_expanded {
                for feed in cat_feeds {
                    items.push(ChannelTreeItem::FeedItem {
                        feed: feed.clone(),
                        category: cat.clone(),
                    });
                }
            }
        }
        items
    }

    pub fn current_selected_channel_item(&self) -> Option<ChannelTreeItem> {
        let items = self.visible_channel_items();
        if items.is_empty() || self.selected_tree_idx >= items.len() {
            None
        } else {
            Some(items[self.selected_tree_idx].clone())
        }
    }

    pub fn toggle_selected_category_expand(&mut self) {
        if let Some(ChannelTreeItem::CategoryHeader { name, is_expanded, .. }) = self.current_selected_channel_item() {
            if is_expanded {
                self.expanded_categories.remove(&name);
            } else {
                self.expanded_categories.insert(name);
            }
        }
    }

    pub fn cycle_category_suggestion(&mut self) {
        let categories = self.get_existing_categories();
        if categories.is_empty() {
            return;
        }
        if let Some(pos) = categories.iter().position(|c| c == &self.new_feed_category) {
            let next_pos = (pos + 1) % categories.len();
            self.new_feed_category = categories[next_pos].clone();
        } else {
            self.new_feed_category = categories[0].clone();
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
        self.status_message = match (self.language, self.show_image) {
            (crate::i18n::Language::Indonesian, true) => "Gambar [ON]".to_string(),
            (crate::i18n::Language::Indonesian, false) => "Gambar [OFF]".to_string(),
            (crate::i18n::Language::Japanese, true) => "画像 [ON]".to_string(),
            (crate::i18n::Language::Japanese, false) => "画像 [OFF]".to_string(),
            (crate::i18n::Language::Dutch, true) => "Afbeeldingen [AAN]".to_string(),
            (crate::i18n::Language::Dutch, false) => "Afbeeldingen [UIT]".to_string(),
            (crate::i18n::Language::Spanish, true) => "Imágenes [ON]".to_string(),
            (crate::i18n::Language::Spanish, false) => "Imágenes [OFF]".to_string(),
            (crate::i18n::Language::Arabic, true) => "الصور [مفعل]".to_string(),
            (crate::i18n::Language::Arabic, false) => "الصور [معطل]".to_string(),
            (_, true) => "Images [ON]".to_string(),
            (_, false) => "Images [OFF]".to_string(),
        };
    }

    pub async fn toggle_fullscreen_reader(&mut self) {
        self.is_fullscreen_reader = !self.is_fullscreen_reader;
        if self.is_fullscreen_reader {
            self.active_pane = ActivePane::Reader;
            self.mark_current_read();
            self.fetch_full_content_for_selected().await;
            self.status_message = match self.language {
                crate::i18n::Language::Indonesian => "Fullscreen Reader Mode [ON] (Tekan [f] atau [Esc] untuk keluar)".to_string(),
                crate::i18n::Language::Japanese => "全画面リーダーモード [ON] ([f] または [Esc] で終了)".to_string(),
                crate::i18n::Language::Dutch => "Volledig Scherm Lezersmodus [AAN] (Druk [f] of [Esc] om te sluiten)".to_string(),
                crate::i18n::Language::Spanish => "Modo Lector Pantalla Completa [ON] (Presione [f] o [Esc] para salir)".to_string(),
                crate::i18n::Language::Arabic => "وضع القراءة ملء الشاشة [مفعل] (اضغط [f] أو [Esc] للخروج)".to_string(),
                _ => "Fullscreen Reader Mode [ON] (Press [f] or [Esc] to exit)".to_string(),
            };
        } else {
            self.status_message = match self.language {
                crate::i18n::Language::Indonesian => "Keluar dari Fullscreen Reader Mode".to_string(),
                crate::i18n::Language::Japanese => "全画面リーダーモードを終了しました".to_string(),
                crate::i18n::Language::Dutch => "Volledig Scherm Lezersmodus verlaten".to_string(),
                crate::i18n::Language::Spanish => "Salió del Modo Lector de Pantalla Completa".to_string(),
                crate::i18n::Language::Arabic => "تم الخروج من وضع القراءة ملء الشاشة".to_string(),
                _ => "Exited Fullscreen Reader Mode".to_string(),
            };
        }
    }

    pub async fn refresh_all_feeds(&mut self) {
        self.is_loading = true;
        self.status_message = match self.language {
            crate::i18n::Language::Indonesian => "Memuat ulang seluruh RSS feed...".to_string(),
            crate::i18n::Language::Japanese => "すべてのRSSフィードを再読み込み中...".to_string(),
            crate::i18n::Language::Dutch => "Alle RSS-feeds vernieuwen...".to_string(),
            crate::i18n::Language::Spanish => "Actualizando todas las fuentes RSS...".to_string(),
            crate::i18n::Language::Arabic => "جاري إعادة تحميل جميع خلاصات RSS...".to_string(),
            _ => "Reloading all RSS feeds...".to_string(),
        };

        let results = self.fetcher.fetch_all_feeds(&self.feeds).await;
        let mut count = 0;

        for (feed_id, res) in results {
            if let Ok(articles) = res {
                count += articles.len();
                self.articles_by_feed.insert(feed_id, articles);
            }
        }

        self.is_loading = false;
        self.status_message = match self.language {
            crate::i18n::Language::Indonesian => format!("Selesai! Dimuat {} berita dari {} channel.", count, self.feeds.len()),
            crate::i18n::Language::Japanese => format!("完了！{} チャンネルから {} 件の記事を読み込みました。", self.feeds.len(), count),
            crate::i18n::Language::Dutch => format!("Klaar! {} artikelen geladen van {} kanalen.", count, self.feeds.len()),
            crate::i18n::Language::Spanish => format!("¡Hecho! {} artículos cargados de {} canales.", count, self.feeds.len()),
            crate::i18n::Language::Arabic => format!("تم! تم تحميل {} مقالاً من {} قناة.", count, self.feeds.len()),
            _ => format!("Done! Loaded {} articles from {} channels.", count, self.feeds.len()),
        };
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
            for articles in self.articles_by_feed.values_mut() {
                for art in articles.iter_mut() {
                    if art.id == article_id {
                        art.content = full_text.clone();
                    }
                }
            }
            self.status_message = match self.language {
                crate::i18n::Language::Indonesian => "Tekan [?] Bantuan | [j/k] Pilih | [Enter] Baca Penuh | [i] Gambar | [/] Cari".to_string(),
                crate::i18n::Language::Japanese => "[?] ヘルプ | [j/k] 選択 | [Enter] 全文表示 | [i] 画像 | [/] 検索".to_string(),
                crate::i18n::Language::Dutch => "Druk [?] Hulp | [j/k] Selecteer | [Enter] Lees Volledig | [i] Afbeelding | [/] Zoeken".to_string(),
                crate::i18n::Language::Spanish => "Presione [?] Ayuda | [j/k] Seleccionar | [Enter] Leer Completo | [i] Imagen | [/] Buscar".to_string(),
                crate::i18n::Language::Arabic => "اضغط [?] مساعدة | [j/k] تحديد | [Enter] قراءة كاملة | [i] صورة | [/] بحث".to_string(),
                _ => "Press [?] Help | [j/k] Select | [Enter] Read Full | [i] Image | [/] Search".to_string(),
            };
            return;
        }

        self.status_message = match self.language {
            crate::i18n::Language::Indonesian => format!("Memuat: '{}'...", article_title),
            crate::i18n::Language::Japanese => format!("読み込み中: '{}'...", article_title),
            crate::i18n::Language::Dutch => format!("Laden: '{}'...", article_title),
            crate::i18n::Language::Spanish => format!("Cargando: '{}'...", article_title),
            crate::i18n::Language::Arabic => format!("جاري التحميل: '{}'...", article_title),
            _ => format!("Loading: '{}'...", article_title),
        };
        self.current_image_lines = None;

        match self.fetcher.fetch_full_article_body(&article_link).await {
            Ok(res) => {
                let full_text = res.body_text;

                if !full_text.trim().is_empty() {
                    for articles in self.articles_by_feed.values_mut() {
                        for art in articles.iter_mut() {
                            if art.id == article_id {
                                art.content = full_text.clone();
                            }
                        }
                    }
                }

                let mut rendered_img = None;
                if let Some(img_url) = res.image_url {
                    if let Some(bytes) = self.fetcher.fetch_image_bytes(&img_url).await {
                        if let Some(lines) = render_image_to_lines(&bytes, 48, 16) {
                            rendered_img = Some(lines.clone());
                            self.current_image_lines = Some(lines);
                        }
                    }
                }

                self.article_cache.insert(article_id, (full_text, rendered_img));
                self.status_message = match self.language {
                    crate::i18n::Language::Indonesian => "Tekan [?] Bantuan | [j/k] Pilih | [Enter] Baca Penuh | [i] Gambar | [/] Cari".to_string(),
                    crate::i18n::Language::Japanese => "[?] ヘルプ | [j/k] 選択 | [Enter] 全文表示 | [i] 画像 | [/] 検索".to_string(),
                    crate::i18n::Language::Dutch => "Druk [?] Hulp | [j/k] Selecteer | [Enter] Lees Volledig | [i] Afbeelding | [/] Zoeken".to_string(),
                    crate::i18n::Language::Spanish => "Presione [?] Ayuda | [j/k] Seleccionar | [Enter] Leer Completo | [i] Imagen | [/] Buscar".to_string(),
                    crate::i18n::Language::Arabic => "اضغط [?] مساعدة | [j/k] تحديد | [Enter] قراءة كاملة | [i] صورة | [/] بحث".to_string(),
                    _ => "Press [?] Help | [j/k] Select | [Enter] Read Full | [i] Image | [/] Search".to_string(),
                };
            }
            Err(e) => {
                self.status_message = match self.language {
                    crate::i18n::Language::Indonesian => format!("Gagal memuat artikel: {}", e),
                    crate::i18n::Language::Japanese => format!("記事の読み込みに失敗しました: {}", e),
                    crate::i18n::Language::Dutch => format!("Artikel laden mislukt: {}", e),
                    crate::i18n::Language::Spanish => format!("Error al cargar el artículo: {}", e),
                    crate::i18n::Language::Arabic => format!("فشل في تحميل المقال: {}", e),
                    _ => format!("Failed to load article: {}", e),
                };
            }
        }
    }

    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.input_mode = InputMode::Normal;
        self.selected_article_idx = 0;
        self.status_message = match self.language {
            crate::i18n::Language::Indonesian => "Pencarian dibersihkan.".to_string(),
            crate::i18n::Language::Japanese => "検索をクリアしました。".to_string(),
            crate::i18n::Language::Dutch => "Zoekopdracht gewist.".to_string(),
            crate::i18n::Language::Spanish => "Búsqueda borrada.".to_string(),
            crate::i18n::Language::Arabic => "تم مسح البحث.".to_string(),
            _ => "Search cleared.".to_string(),
        };
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

        // Global search across all feeds when search query is present
        if !self.search_query.trim().is_empty() {
            let mut all_global = Vec::new();
            let bookmarked_ids: HashSet<String> = self.storage.get_bookmarks().iter().map(|b| b.id.clone()).collect();
            for feed in &self.feeds {
                if let Some(articles) = self.articles_by_feed.get(&feed.id) {
                    for art in articles {
                        let mut processed = art.clone();
                        processed.is_read = self.read_articles.contains(&art.id);
                        processed.is_bookmarked = bookmarked_ids.contains(&art.id);
                        all_global.push(processed);
                    }
                }
            }
            return self.filter_articles(all_global);
        }

        let current_item = match self.current_selected_channel_item() {
            Some(item) => item,
            None => return Vec::new(),
        };

        let target_feeds: Vec<FeedSource> = match current_item {
            ChannelTreeItem::CategoryHeader { name, .. } => {
                self.feeds.iter().filter(|f| f.category == name).cloned().collect()
            }
            ChannelTreeItem::FeedItem { feed, .. } => vec![feed],
        };

        let mut all_articles = Vec::new();
        let bookmarked_ids: HashSet<String> = self.storage.get_bookmarks().iter().map(|b| b.id.clone()).collect();

        for feed in target_feeds {
            if let Some(articles) = self.articles_by_feed.get(&feed.id) {
                for art in articles {
                    let mut processed = art.clone();
                    processed.is_read = self.read_articles.contains(&art.id);
                    processed.is_bookmarked = bookmarked_ids.contains(&art.id);
                    all_articles.push(processed);
                }
            }
        }

        self.filter_articles(all_articles)
    }

    fn filter_articles(&self, articles: Vec<Article>) -> Vec<Article> {
        let query = self.search_query.trim().to_lowercase();
        if query.is_empty() {
            return articles;
        }

        articles
            .into_iter()
            .filter(|a| {
                a.title.to_lowercase().contains(&query)
                    || a.summary.to_lowercase().contains(&query)
                    || a.author.to_lowercase().contains(&query)
                    || a.content.to_lowercase().contains(&query)
                    || a.feed_title.to_lowercase().contains(&query)
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
        self.marquee_tick = 0;
        match self.active_pane {
            ActivePane::Feeds => {
                let len = self.visible_channel_items().len();
                if len > 0 {
                    self.selected_tree_idx = (self.selected_tree_idx + 1) % len;
                    self.feed_list_state.select(Some(self.selected_tree_idx));
                    self.selected_article_idx = 0;
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
        self.marquee_tick = 0;
        match self.active_pane {
            ActivePane::Feeds => {
                let len = self.visible_channel_items().len();
                if len > 0 {
                    if self.selected_tree_idx == 0 {
                        self.selected_tree_idx = len - 1;
                    } else {
                        self.selected_tree_idx -= 1;
                    }
                    self.feed_list_state.select(Some(self.selected_tree_idx));
                    self.selected_article_idx = 0;
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
                self.status_message = match (self.language, added) {
                    (crate::i18n::Language::Indonesian, true) => format!("Disimpan ke Bookmark: '{}'", art.title),
                    (crate::i18n::Language::Indonesian, false) => format!("Dihapus dari Bookmark: '{}'", art.title),
                    (crate::i18n::Language::Japanese, true) => format!("ブックマークに保存しました: '{}'", art.title),
                    (crate::i18n::Language::Japanese, false) => format!("ブックマークから削除しました: '{}'", art.title),
                    (crate::i18n::Language::Dutch, true) => format!("Opgeslagen in Bladwijzers: '{}'", art.title),
                    (crate::i18n::Language::Dutch, false) => format!("Verwijderd uit Bladwijzers: '{}'", art.title),
                    (crate::i18n::Language::Spanish, true) => format!("Guardado en Marcadores: '{}'", art.title),
                    (crate::i18n::Language::Spanish, false) => format!("Eliminado de Marcadores: '{}'", art.title),
                    (crate::i18n::Language::Arabic, true) => format!("تم الحفظ في الإشارات المرجعية: '{}'", art.title),
                    (crate::i18n::Language::Arabic, false) => format!("تم الحذف من الإشارات المرجعية: '{}'", art.title),
                    (_, true) => format!("Saved to Bookmarks: '{}'", art.title),
                    (_, false) => format!("Removed from Bookmarks: '{}'", art.title),
                };
            }
        }
    }

    pub fn open_current_in_browser(&mut self) {
        if let Some(art) = self.current_article() {
            if !art.link.is_empty() {
                if open::that(&art.link).is_ok() {
                    self.status_message = match self.language {
                        crate::i18n::Language::Indonesian => format!("Membuka browser: {}", art.link),
                        crate::i18n::Language::Japanese => format!("ブラウザで開きます: {}", art.link),
                        crate::i18n::Language::Dutch => format!("Browser openen: {}", art.link),
                        crate::i18n::Language::Spanish => format!("Abriendo navegador: {}", art.link),
                        crate::i18n::Language::Arabic => format!("جاري فتح المتصفح: {}", art.link),
                        _ => format!("Opening browser: {}", art.link),
                    };
                } else {
                    self.status_message = match self.language {
                        crate::i18n::Language::Indonesian => format!("Gagal membuka link: {}", art.link),
                        crate::i18n::Language::Japanese => format!("リンクを開くのに失敗しました: {}", art.link),
                        crate::i18n::Language::Dutch => format!("Link openen mislukt: {}", art.link),
                        crate::i18n::Language::Spanish => format!("Error al abrir enlace: {}", art.link),
                        crate::i18n::Language::Arabic => format!("فشل فتح الرابط: {}", art.link),
                        _ => format!("Failed to open link: {}", art.link),
                    };
                }
            }
        }
    }

    pub fn start_move_feed_category(&mut self) {
        if let Some(ChannelTreeItem::FeedItem { feed, .. }) = self.current_selected_channel_item() {
            self.move_feed_category_input = feed.category.clone();
            self.input_mode = InputMode::MoveFeedCategory;
        } else {
            self.status_message = match self.language {
                crate::i18n::Language::Indonesian => "Pilih Feed di bawah kategori untuk memindahkan ke kategori lain.".to_string(),
                _ => "Select a Feed under a category to move to another category.".to_string(),
            };
        }
    }

    pub fn submit_move_feed_category(&mut self) {
        let new_cat = self.move_feed_category_input.trim().to_string();
        if new_cat.is_empty() {
            self.status_message = match self.language {
                crate::i18n::Language::Indonesian => "Nama kategori baru tidak boleh kosong!".to_string(),
                _ => "New category name cannot be empty!".to_string(),
            };
            return;
        }

        if let Some(ChannelTreeItem::FeedItem { feed, .. }) = self.current_selected_channel_item() {
            let feed_id = feed.id.clone();
            let feed_title = feed.title.clone();
            let _ = self.storage.update_feed_category(&feed_id, &new_cat);
            if let Some(f) = self.feeds.iter_mut().find(|f| f.id == feed_id) {
                f.category = new_cat.clone();
            }
            self.expanded_categories.insert(new_cat.clone());
            self.status_message = match self.language {
                crate::i18n::Language::Indonesian => format!("Feed '{}' berhasil dipindahkan ke kategori '{}'.", feed_title, new_cat),
                _ => format!("Feed '{}' moved to category '{}'.", feed_title, new_cat),
            };
            self.input_mode = InputMode::Normal;
            self.move_feed_category_input.clear();
        }
    }

    pub fn start_delete_category(&mut self) {
        if let Some(item) = self.current_selected_channel_item() {
            let cat_name = match item {
                ChannelTreeItem::CategoryHeader { name, .. } => name,
                ChannelTreeItem::FeedItem { category, .. } => category,
            };
            self.target_category_to_delete = Some(cat_name);
            self.input_mode = InputMode::DeleteCategoryConfirm;
        }
    }

    pub fn confirm_delete_category(&mut self) {
        if let Some(cat) = self.target_category_to_delete.take() {
            let _ = self.storage.delete_category(&cat);
            self.feeds.retain(|f| f.category != cat);
            self.expanded_categories.remove(&cat);
            self.status_message = match self.language {
                crate::i18n::Language::Indonesian => format!("Kategori '{}' dan seluruh feed di dalamnya berhasil dihapus.", cat),
                _ => format!("Category '{}' and all feeds inside were deleted.", cat),
            };
            let len = self.visible_channel_items().len();
            if self.selected_tree_idx >= len && len > 0 {
                self.selected_tree_idx = len - 1;
            } else if len == 0 {
                self.selected_tree_idx = 0;
            }
            self.input_mode = InputMode::Normal;
        }
    }

    pub fn delete_selected_feed(&mut self) {
        if let Some(ChannelTreeItem::FeedItem { feed, .. }) = self.current_selected_channel_item() {
            let feed_id = feed.id.clone();
            let title = feed.title.clone();
            let _ = self.storage.delete_feed(&feed_id);
            self.feeds.retain(|f| f.id != feed_id);
            self.status_message = match self.language {
                crate::i18n::Language::Indonesian => format!("Feed '{}' berhasil dihapus.", title),
                _ => format!("Feed '{}' deleted.", title),
            };
            let len = self.visible_channel_items().len();
            if self.selected_tree_idx >= len && len > 0 {
                self.selected_tree_idx = len - 1;
            }
        } else if let Some(ChannelTreeItem::CategoryHeader { name, .. }) = self.current_selected_channel_item() {
            self.target_category_to_delete = Some(name);
            self.input_mode = InputMode::DeleteCategoryConfirm;
        }
    }

    pub fn submit_new_feed(&mut self) {
        if !self.new_feed_title.is_empty() && !self.new_feed_url.is_empty() {
            let category = if self.new_feed_category.trim().is_empty() {
                "Umum".to_string()
            } else {
                self.new_feed_category.trim().to_string()
            };

            let feed = FeedSource::new(&self.new_feed_title, &self.new_feed_url, &category);
            let _ = self.storage.add_feed(&feed);
            self.expanded_categories.insert(category);
            self.status_message = match self.language {
                crate::i18n::Language::Indonesian => format!("Feed baru '{}' ditambahkan!", feed.title),
                _ => format!("New feed '{}' added!", feed.title),
            };
            self.feeds.push(feed);
            self.new_feed_title.clear();
            self.new_feed_url.clear();
            self.new_feed_category = "Umum".to_string();
            self.input_mode = InputMode::Normal;
        } else {
            self.status_message = match self.language {
                crate::i18n::Language::Indonesian => "Judul dan URL feed tidak boleh kosong!".to_string(),
                _ => "Feed title and URL cannot be empty!".to_string(),
            };
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

        #[cfg(unix)]
        {
            let _ = std::process::Command::new("sh")
                .arg("-c")
                .arg("hash -r 2>/dev/null || rehash 2>/dev/null || true")
                .status();
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

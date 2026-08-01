use crate::models::{Article, FeedSource};
use feed_rs::parser;
use regex::Regex;
use reqwest::Client;
use std::time::Duration;

#[derive(Clone)]
pub struct Fetcher {
    client: Client,
}

pub struct FullArticleResult {
    pub body_text: String,
    pub image_url: Option<String>,
}

impl Fetcher {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .build()
            .unwrap_or_else(|_| Client::new());

        Self { client }
    }

    pub async fn fetch_feed(&self, feed: &FeedSource) -> Result<Vec<Article>, String> {
        if !feed.url.starts_with("http://") && !feed.url.starts_with("https://") {
            return Err("URL feed tidak valid. Harus diawali http:// atau https://".to_string());
        }

        let response = self
            .client
            .get(&feed.url)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("Read error: {}", e))?;

        let parsed = parser::parse(&bytes[..])
            .map_err(|e| format!("RSS parsing error: {}", e))?;

        let feed_title = feed.title.clone();
        let mut articles = Vec::new();

        for entry in parsed.entries {
            let title = entry.title.map(|t| t.content).unwrap_or_else(|| "Tanpa Judul".to_string());
            let link = entry.links.first().map(|l| l.href.clone()).unwrap_or_default();

            let published = entry
                .published
                .or(entry.updated)
                .map(|dt| dt.format("%d %b %Y, %H:%M").to_string())
                .unwrap_or_else(|| "Terkini".to_string());

            let author = entry
                .authors
                .first()
                .map(|a| a.name.clone())
                .unwrap_or_else(|| feed_title.clone());

            let raw_summary = entry.summary.map(|s| s.content).unwrap_or_default();
            let raw_content = entry
                .content
                .and_then(|c| c.body)
                .unwrap_or_else(|| raw_summary.clone());

            let summary = clean_html(&raw_summary);
            let content = if raw_content.is_empty() {
                summary.clone()
            } else {
                clean_html(&raw_content)
            };

            articles.push(Article::new(
                &feed.url,
                &feed_title,
                title,
                link,
                published,
                author,
                summary,
                content,
            ));
        }

        Ok(articles)
    }

    pub async fn fetch_all_feeds(&self, feeds: &[FeedSource]) -> Vec<(String, Result<Vec<Article>, String>)> {
        let mut tasks = Vec::new();
        for feed in feeds {
            let feed_clone = feed.clone();
            let client = self.client.clone();
            tasks.push(tokio::spawn(async move {
                let fetcher = Fetcher { client };
                (feed_clone.id.clone(), fetcher.fetch_feed(&feed_clone).await)
            }));
        }

        let mut results = Vec::new();
        for task in tasks {
            if let Ok(res) = task.await {
                results.push(res);
            }
        }
        results
    }

    pub async fn fetch_full_article_body(&self, url: &str) -> Result<FullArticleResult, String> {
        if url.is_empty() || (!url.starts_with("http://") && !url.starts_with("https://")) {
            return Err("URL artikel tidak valid".to_string());
        }

        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| format!("Gagal mengunduh artikel: {}", e))?;

        let html_text = response
            .text()
            .await
            .map_err(|e| format!("Gagal membaca HTML: {}", e))?;

        let image_url = extract_first_image_url(&html_text);
        let extracted = extract_article_paragraphs(&html_text);

        let body_text = if extracted.trim().is_empty() {
            clean_html(&html_text)
        } else {
            extracted
        };

        Ok(FullArticleResult { body_text, image_url })
    }

    pub async fn fetch_image_bytes(&self, url: &str) -> Option<Vec<u8>> {
        if url.is_empty() || (!url.starts_with("http://") && !url.starts_with("https://")) {
            return None;
        }
        let response = self.client.get(url).send().await.ok()?;
        let bytes = response.bytes().await.ok()?;
        Some(bytes.to_vec())
    }

    pub async fn check_for_update(&self) -> Result<UpdateInfo, String> {
        let current_version = env!("CARGO_PKG_VERSION").to_string();
        let api_url = "https://api.github.com/repos/WhaTheFoxSay/rubah/releases/latest";

        let response = self
            .client
            .get(api_url)
            .header("User-Agent", "RubahInstaller/1.0")
            .send()
            .await
            .map_err(|e| format!("Gagal menghubungi GitHub API: {}", e))?;

        let json_text = response
            .text()
            .await
            .map_err(|e| format!("Gagal membaca data rilis: {}", e))?;

        let re_tag = Regex::new(r#""tag_name"\s*:\s*"v?([^"]+)""#).map_err(|e| e.to_string())?;
        let latest_version = if let Some(cap) = re_tag.captures(&json_text) {
            cap[1].to_string()
        } else {
            return Err("Format versi tidak ditemukan dari GitHub API".to_string());
        };

        let re_body = Regex::new(r#""body"\s*:\s*"([^"]*)""#).ok();
        let release_notes = re_body
            .and_then(|re| re.captures(&json_text))
            .map(|cap| cap[1].replace("\\n", "\n").replace("\\r", ""))
            .unwrap_or_else(|| "Latest release notes and changelog are available on GitHub.".to_string());

        let has_update = is_newer_version(&current_version, &latest_version);

        Ok(UpdateInfo {
            current_version,
            latest_version,
            has_update,
            release_notes,
        })
    }

    pub async fn download_and_install_update(
        &self,
        latest_version: &str,
        tx: tokio::sync::mpsc::UnboundedSender<UpdateProgress>,
    ) -> Result<(), String> {
        let asset_name = get_target_asset_name();
        let download_url = format!(
            "https://github.com/WhaTheFoxSay/rubah/releases/download/v{}/{}",
            latest_version.trim_start_matches('v'),
            asset_name
        );

        let response = self
            .client
            .get(&download_url)
            .header("User-Agent", "RubahAutoUpdater/1.0")
            .send()
            .await
            .map_err(|e| format!("Gagal mengunduh biner rilis: {}", e))?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err("RELEASE_BUILDING".to_string());
        }

        if !response.status().is_success() {
            return Err(format!(
                "Server mengembalikan status HTTP {}",
                response.status()
            ));
        }

        let total_size = response.content_length().unwrap_or(0);
        let mut response = response;
        let mut downloaded_bytes = Vec::new();

        while let Ok(Some(chunk)) = response.chunk().await {
            downloaded_bytes.extend_from_slice(&chunk);
            let downloaded = downloaded_bytes.len() as u64;
            let percentage = if total_size > 0 {
                (downloaded as f32 / total_size as f32) * 100.0
            } else {
                0.0
            };

            let _ = tx.send(UpdateProgress::Downloading {
                downloaded,
                total: total_size,
                percentage,
            });
        }

        let _ = tx.send(UpdateProgress::Installing);

        // Determine target executable path
        let target_path = if let Ok(exe) = std::env::current_exe() {
            exe
        } else if let Some(home) = dirs::home_dir() {
            #[cfg(target_os = "windows")]
            {
                home.join(".local").join("bin").join("baca.exe")
            }
            #[cfg(not(target_os = "windows"))]
            {
                home.join(".local").join("bin").join("baca")
            }
        } else {
            return Err("Gagal menemukan lokasi executable aplikasi".to_string());
        };

        let temp_path = target_path.with_extension("tmp_download");

        // Write downloaded bytes to temp file
        std::fs::write(&temp_path, &downloaded_bytes)
            .map_err(|e| format!("Gagal menulis file temporer biner: {}", e))?;

        // Set executable permissions on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = std::fs::metadata(&temp_path) {
                let mut perms = metadata.permissions();
                perms.set_mode(0o755);
                let _ = std::fs::set_permissions(&temp_path, perms);
            }
        }

        // Perform self-replacement / atomic swap
        #[cfg(not(target_os = "windows"))]
        {
            std::fs::rename(&temp_path, &target_path)
                .map_err(|e| format!("Gagal mengganti biner aplikasi: {}", e))?;
        }

        #[cfg(target_os = "windows")]
        {
            let old_path = target_path.with_extension("old.exe");
            let _ = std::fs::remove_file(&old_path);
            let _ = std::fs::rename(&target_path, &old_path);
            std::fs::rename(&temp_path, &target_path)
                .map_err(|e| format!("Gagal mengganti biner executable Windows: {}", e))?;
        }

        let _ = tx.send(UpdateProgress::Completed(latest_version.to_string()));
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum UpdateProgress {
    Downloading { downloaded: u64, total: u64, percentage: f32 },
    Installing,
    Completed(String),
    Failed(String),
}

pub fn get_target_asset_name() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        "rubah-windows-amd64.exe"
    }
    #[cfg(target_os = "macos")]
    {
        if cfg!(target_arch = "aarch64") {
            "rubah-macos-arm64"
        } else {
            "rubah-macos-amd64"
        }
    }
    #[cfg(target_os = "linux")]
    {
        "rubah-linux-amd64"
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        "rubah-linux-amd64"
    }
}

#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub current_version: String,
    pub latest_version: String,
    pub has_update: bool,
    pub release_notes: String,
}

fn is_newer_version(current: &str, latest: &str) -> bool {
    let parse_ver = |v: &str| -> (u32, u32, u32) {
        let parts: Vec<u32> = v
            .trim_start_matches('v')
            .split('.')
            .filter_map(|p| p.parse().ok())
            .collect();
        (
            *parts.get(0).unwrap_or(&0),
            *parts.get(1).unwrap_or(&0),
            *parts.get(2).unwrap_or(&0),
        )
    };
    parse_ver(latest) > parse_ver(current)
}

pub fn extract_first_image_url(html: &str) -> Option<String> {
    // 1. Try og:image / twitter:image meta tags
    let re_og = Regex::new(r#"(?i)<meta[^>]+(?:property|name)=["'](?:og:image|twitter:image|image)["'][^>]+content=["']([^"']+)["']"#).ok()?;
    if let Some(cap) = re_og.captures(html) {
        let src = &cap[1];
        if src.starts_with("http://") || src.starts_with("https://") {
            return Some(src.to_string());
        }
    }

    let re_og_rev = Regex::new(r#"(?i)<meta[^>]+content=["']([^"']+)["'][^>]+(?:property|name)=["'](?:og:image|twitter:image|image)["']"#).ok()?;
    if let Some(cap) = re_og_rev.captures(html) {
        let src = &cap[1];
        if src.starts_with("http://") || src.starts_with("https://") {
            return Some(src.to_string());
        }
    }

    // 2. Try <img src="..."> or data-src="..."
    let re_img = Regex::new(r#"(?i)<img[^>]+(?:src|data-src|data-original)=["']([^"']+)["']"#).ok()?;
    for cap in re_img.captures_iter(html) {
        let src = &cap[1];
        if src.starts_with("http://") || src.starts_with("https://") {
            if !src.contains("icon") && !src.contains("logo") && !src.contains("avatar") && !src.contains("tracking") {
                return Some(src.to_string());
            }
        }
    }

    None
}

fn clean_html(html: &str) -> String {
    if html.is_empty() {
        return String::new();
    }
    let text = html2text::from_read(html.as_bytes(), 80).unwrap_or_default();
    let mut cleaned_lines = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        let lower = trimmed.to_lowercase();
        if !lower.contains("tercopy")
            && !lower.contains("copy url")
            && !lower.contains("link tercopy")
            && !lower.contains("dengarkan artikel")
            && !lower.contains("tempo circle")
            && !lower.contains("pengumuman tender")
        {
            cleaned_lines.push(line);
        }
    }
    cleaned_lines.join("\n").trim().to_string()
}

fn extract_article_paragraphs(html: &str) -> String {
    let re_script = Regex::new(r"(?is)<script[^>]*?>.*?</script>").unwrap();
    let re_style = Regex::new(r"(?is)<style[^>]*?>.*?</style>").unwrap();
    let re_nav = Regex::new(r"(?is)<nav[^>]*?>.*?</nav>").unwrap();
    let re_header = Regex::new(r"(?is)<header[^>]*?>.*?</header>").unwrap();
    let re_footer = Regex::new(r"(?is)<footer[^>]*?>.*?</footer>").unwrap();
    let re_aside = Regex::new(r"(?is)<aside[^>]*?>.*?</aside>").unwrap();
    let re_form = Regex::new(r"(?is)<form[^>]*?>.*?</form>").unwrap();
    let re_iframe = Regex::new(r"(?is)<iframe[^>]*?>.*?</iframe>").unwrap();

    let cleaned = re_script.replace_all(html, "");
    let cleaned = re_style.replace_all(&cleaned, "");
    let cleaned = re_nav.replace_all(&cleaned, "");
    let cleaned = re_header.replace_all(&cleaned, "");
    let cleaned = re_footer.replace_all(&cleaned, "");
    let cleaned = re_aside.replace_all(&cleaned, "");
    let cleaned = re_form.replace_all(&cleaned, "");
    let cleaned = re_iframe.replace_all(&cleaned, "");

    // Mozilla Readability Heuristic Container Extraction (<article> or main container)
    let target_html = if let Some(re_article) = Regex::new(r"(?is)<article[^>]*?>(.*?)</article>").ok() {
        if let Some(cap) = re_article.captures(&cleaned) {
            cap[1].to_string()
        } else {
            cleaned.to_string()
        }
    } else {
        cleaned.to_string()
    };

    let re_p = Regex::new(r"(?is)<p[^>]*?>(.*?)</p>").unwrap();
    let re_tags = Regex::new(r"<[^>]*>").unwrap();

    let mut paragraphs = Vec::new();
    for cap in re_p.captures_iter(&target_html) {
        let raw_p = &cap[1];
        let text_p = re_tags.replace_all(raw_p, "");
        let clean_p = text_p.trim();
        let clean_lower = clean_p.to_lowercase();

        // Readability heuristic density score filter: word count > 5, text len > 25
        let word_count = clean_p.split_whitespace().count();

        if clean_p.len() >= 25
            && word_count >= 5
            && !clean_lower.contains("tercopy")
            && !clean_lower.contains("copy url")
            && !clean_lower.contains("link tercopy")
            && !clean_lower.contains("copyright")
            && !clean_lower.contains("advertisement")
            && !clean_lower.contains("scroll to continue")
            && !clean_lower.contains("foto:")
            && !clean_lower.contains("googletag")
            && !clean_lower.contains("dengarkan artikel")
            && !clean_lower.contains("tempo circle")
            && !clean_lower.contains("pengumuman tender")
            && !clean_lower.contains("pilihan editor")
            && !clean_lower.contains("berita terkait")
            && !clean_lower.contains("simak video")
            && !clean_lower.contains("tonton juga")
            && !clean_lower.starts_with("baca juga")
            && !clean_lower.starts_with("iklan")
            && !clean_lower.ends_with("iklan")
        {
            paragraphs.push(clean_p.to_string());
        }
    }

    paragraphs.join("\n\n")
}

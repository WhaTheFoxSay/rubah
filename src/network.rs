use crate::models::{Article, FeedSource};
use feed_rs::parser;
use regex::Regex;
use reqwest::Client;
use std::time::Duration;

pub struct Fetcher {
    client: Client,
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

    pub async fn fetch_full_article_body(&self, url: &str) -> Result<String, String> {
        if url.is_empty() {
            return Err("URL kosong".to_string());
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

        let extracted = extract_article_paragraphs(&html_text);
        if extracted.trim().is_empty() {
            Ok(clean_html(&html_text))
        } else {
            Ok(extracted)
        }
    }
}

fn clean_html(html: &str) -> String {
    if html.is_empty() {
        return String::new();
    }
    let text = html2text::from_read(html.as_bytes(), 80).unwrap_or_default();
    text.trim().to_string()
}

fn extract_article_paragraphs(html: &str) -> String {
    let re_script = Regex::new(r"(?is)<script[^>]*?>.*?</script>").unwrap();
    let re_style = Regex::new(r"(?is)<style[^>]*?>.*?</style>").unwrap();
    let re_nav = Regex::new(r"(?is)<nav[^>]*?>.*?</nav>").unwrap();
    let re_header = Regex::new(r"(?is)<header[^>]*?>.*?</header>").unwrap();
    let re_footer = Regex::new(r"(?is)<footer[^>]*?>.*?</footer>").unwrap();

    let cleaned = re_script.replace_all(html, "");
    let cleaned = re_style.replace_all(&cleaned, "");
    let cleaned = re_nav.replace_all(&cleaned, "");
    let cleaned = re_header.replace_all(&cleaned, "");
    let cleaned = re_footer.replace_all(&cleaned, "");

    let re_p = Regex::new(r"(?is)<p[^>]*?>(.*?)</p>").unwrap();
    let re_tags = Regex::new(r"<[^>]*>").unwrap();

    let mut paragraphs = Vec::new();
    for cap in re_p.captures_iter(&cleaned) {
        let raw_p = &cap[1];
        let text_p = re_tags.replace_all(raw_p, "");
        let clean_p = text_p.trim();

        if clean_p.len() > 25
            && !clean_p.starts_with("Copyright")
            && !clean_p.starts_with("Foto:")
            && !clean_p.starts_with("ADVERTISEMENT")
            && !clean_p.starts_with("SCROLL TO CONTINUE")
            && !clean_p.contains("googletag")
        {
            paragraphs.push(clean_p.to_string());
        }
    }

    paragraphs.join("\n\n")
}

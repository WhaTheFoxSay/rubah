use crate::models::{Article, FeedSource};
use feed_rs::parser;
use reqwest::Client;
use std::time::Duration;

pub struct Fetcher {
    client: Client,
}

impl Fetcher {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .user_agent("Mozilla/5.0 (compatible; Rubah/0.1; +https://github.com/rubah/rubah)")
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
}

fn clean_html(html: &str) -> String {
    if html.is_empty() {
        return String::new();
    }
    // Use html2text to parse clean plain-text suitable for terminal view
    let text = html2text::from_read(html.as_bytes(), 80).unwrap_or_default();
    text.trim().to_string()
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FeedSource {
    pub id: String,
    pub title: String,
    pub url: String,
    pub category: String,
}

impl FeedSource {
    pub fn new(title: impl Into<String>, url: impl Into<String>, category: impl Into<String>) -> Self {
        let url_str = url.into();
        let id = format!("{:x}", md5_hash(&url_str));
        Self {
            id,
            title: title.into(),
            url: url_str,
            category: category.into(),
        }
    }
}

fn md5_hash(input: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: String,
    pub feed_url: String,
    pub feed_title: String,
    pub title: String,
    pub link: String,
    pub published: String,
    pub author: String,
    pub summary: String,
    pub content: String,
    pub is_read: bool,
    pub is_bookmarked: bool,
}

impl Article {
    pub fn new(
        feed_url: impl Into<String>,
        feed_title: impl Into<String>,
        title: impl Into<String>,
        link: impl Into<String>,
        published: impl Into<String>,
        author: impl Into<String>,
        summary: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        let link_str = link.into();
        let id = format!("{:x}", md5_hash(&link_str));
        Self {
            id,
            feed_url: feed_url.into(),
            feed_title: feed_title.into(),
            title: title.into(),
            link: link_str,
            published: published.into(),
            author: author.into(),
            summary: summary.into(),
            content: content.into(),
            is_read: false,
            is_bookmarked: false,
        }
    }
}

pub fn default_feeds() -> Vec<FeedSource> {
    vec![
        FeedSource::new("CNN Indonesia - Nasional", "https://www.cnnindonesia.com/nasional/rss", "Berita Utama"),
        FeedSource::new("Antara News - Terkini", "https://www.antaranews.com/rss/terkini.xml", "Berita Utama"),
        FeedSource::new("Tempo - Nasional", "https://rss.tempo.co/nasional", "Berita Utama"),
        FeedSource::new("Hacker News", "https://news.ycombinator.com/rss", "Teknologi"),
        FeedSource::new("TechCrunch", "https://techcrunch.com/feed/", "Teknologi"),
        FeedSource::new("BBC News - World", "http://feeds.bbci.co.uk/news/world/rss.xml", "Internasional"),
    ]
}

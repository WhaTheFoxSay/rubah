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
        // Nasional
        FeedSource::new("CNN Indonesia - Nasional", "https://www.cnnindonesia.com/nasional/rss", "Nasional"),
        FeedSource::new("Antara News - Terkini", "https://www.antaranews.com/rss/terkini.xml", "Nasional"),
        FeedSource::new("Tempo - Nasional", "https://rss.tempo.co/nasional", "Nasional"),
        FeedSource::new("Jawa Pos", "https://jawapos.com/rss.xml", "Nasional"),
        FeedSource::new("Suara Surabaya", "https://www.suarasurabaya.net/feed/", "Nasional"),
        FeedSource::new("Kabar Trenggalek", "https://kabartrenggalek.com/rss.xml", "Nasional"),
        FeedSource::new("Kabar Blitar", "https://kabarblitar.com/rss.xml", "Nasional"),
        FeedSource::new("Radar Jatim", "https://radarjatim.id/feed/", "Nasional"),
        FeedSource::new("Surya Pos", "https://surya-pos.com/feed/", "Nasional"),
        FeedSource::new("EKSPOSKALTIM", "https://eksposkaltim.com/rss.xml", "Nasional"),
        FeedSource::new("Headline Indonesia", "https://www.headline.co.id/feed/", "Nasional"),
        FeedSource::new("Liputan6.com", "https://feed.liputan6.com/rss/news", "Nasional"),
        FeedSource::new("Kompas.com", "https://kompas.id/feed/", "Nasional"),
        FeedSource::new("Viva.co.id", "https://www.viva.co.id/get/all", "Nasional"),

        // Hiburan
        FeedSource::new("KapanLagi.com | Musik", "https://www.kapanlagi.com/newsfeed/kategori-musik/", "Hiburan"),
        FeedSource::new("KapanLagi.com | Film", "https://www.kapanlagi.com/newsfeed/kategori-film/", "Hiburan"),
        FeedSource::new("Cumicumi.com", "https://stock.cumicumi.com/feeds/latest", "Hiburan"),

        // Olahraga
        FeedSource::new("Bola.com | Indonesia", "https://feed.bola.com/rss/indonesia", "Olahraga"),
        FeedSource::new("Bola.com | Dunia", "https://feed.bola.com/rss/dunia", "Olahraga"),
        FeedSource::new("Bola.com | Inggris", "https://feed.bola.com/rss/inggris", "Olahraga"),
        FeedSource::new("Bola.com | Spanyol", "https://feed.bola.com/rss/spanyol", "Olahraga"),
        FeedSource::new("Bola.com | MotoGP", "https://feed.bola.com/rss/moto-gp", "Olahraga"),

        // Teknologi
        FeedSource::new("Hacker News", "https://news.ycombinator.com/rss", "Teknologi"),
        FeedSource::new("TechCrunch", "https://techcrunch.com/feed/", "Teknologi"),
        FeedSource::new("Linux.org", "https://www.linux.org/articles/index.rss", "Teknologi"),
        FeedSource::new("DistroWatch.com", "https://distrowatch.com/news/dw.xml", "Teknologi"),
        FeedSource::new("LWN.net", "https://lwn.net/headlines/newrss", "Teknologi"),
        FeedSource::new("9to5 Linux", "https://9to5linux.com/feed/atom", "Teknologi"),
        FeedSource::new("Network World", "https://www.networkworld.com/feed/", "Teknologi"),
        FeedSource::new("Linux Commands", "https://www.tecmint.com/category/linux-commands/feed/", "Teknologi"),
        FeedSource::new("How To Forge", "https://www.howtoforge.com/feed.rss", "Teknologi"),
        FeedSource::new("Linux Journal", "https://www.linuxjournal.com/node/feed", "Teknologi"),
        FeedSource::new("Network Admin Stuff", "https://ciscoiseasy.blogspot.com/feeds/posts/default", "Teknologi"),
        FeedSource::new("Reddit.com | GNU Linux", "https://www.reddit.com/r/linux/.rss", "Teknologi"),
        FeedSource::new("Planet GNU", "https://planet.gnu.org/atom.xml", "Teknologi"),

        // Internasional
        FeedSource::new("BBC News - World", "http://feeds.bbci.co.uk/news/world/rss.xml", "Internasional"),
    ]
}

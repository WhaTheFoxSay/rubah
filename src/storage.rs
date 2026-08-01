use crate::models::{default_feeds, Article, FeedSource};
use rusqlite::{params, Connection, Result};
use std::collections::HashSet;
use std::path::PathBuf;

pub struct Storage {
    conn: Connection,
}

impl Storage {
    pub fn new() -> Self {
        let db_path = Self::get_db_path();
        if let Some(parent) = db_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let conn = Self::open_or_fallback(&db_path);

        let storage = Self { conn };
        let _ = storage.init_tables();

        // Pre-populate default feeds if empty
        if storage.get_feeds().map(|f| f.is_empty()).unwrap_or(true) {
            for feed in default_feeds() {
                let _ = storage.add_feed(&feed);
            }
        }

        storage
    }

    fn open_or_fallback(db_path: &PathBuf) -> Connection {
        // Try opening target DB path
        if let Ok(conn) = Connection::open(db_path) {
            if conn.execute("PRAGMA journal_mode=DELETE;", []).is_ok() {
                return conn;
            }
        }

        // If corrupted or permission failed, try recreating file
        if db_path.exists() {
            let _ = std::fs::remove_file(db_path);
            let _ = std::fs::remove_file(db_path.with_extension("db-journal"));
            let _ = std::fs::remove_file(db_path.with_extension("db-wal"));
            let _ = std::fs::remove_file(db_path.with_extension("db-shm"));
            if let Ok(conn) = Connection::open(db_path) {
                let _ = conn.execute("PRAGMA journal_mode=DELETE;", []);
                return conn;
            }
        }

        // Fallback to home dir ~/.rubah.db
        if let Some(home) = dirs::home_dir() {
            let fallback_path = home.join(".rubah.db");
            if let Ok(conn) = Connection::open(&fallback_path) {
                let _ = conn.execute("PRAGMA journal_mode=DELETE;", []);
                return conn;
            }
        }

        // Guaranteed in-memory fallback
        Connection::open_in_memory().unwrap_or_else(|_| Connection::open(":memory:").unwrap())
    }

    fn get_db_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rubah")
            .join("rubah.db")
    }

    fn init_tables(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS feeds (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                url TEXT NOT NULL UNIQUE,
                category TEXT NOT NULL
            );",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS read_articles (
                article_id TEXT PRIMARY KEY
            );",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS bookmarks (
                article_id TEXT PRIMARY KEY,
                data TEXT NOT NULL
            );",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );",
            [],
        )?;

        Ok(())
    }

    pub fn get_setting(&self, key: &str) -> Option<String> {
        let mut stmt = self.conn.prepare("SELECT value FROM settings WHERE key = ?1").ok()?;
        stmt.query_row(params![key], |row| row.get(0)).ok()
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn get_feeds(&self) -> Result<Vec<FeedSource>> {
        let mut stmt = self.conn.prepare("SELECT id, title, url, category FROM feeds")?;
        let rows = stmt.query_map([], |row| {
            Ok(FeedSource {
                id: row.get(0)?,
                title: row.get(1)?,
                url: row.get(2)?,
                category: row.get(3)?,
            })
        })?;

        let mut feeds = Vec::new();
        for feed in rows {
            if let Ok(f) = feed {
                feeds.push(f);
            }
        }
        Ok(feeds)
    }

    pub fn add_feed(&self, feed: &FeedSource) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO feeds (id, title, url, category) VALUES (?1, ?2, ?3, ?4)",
            params![feed.id, feed.title, feed.url, feed.category],
        )?;
        Ok(())
    }

    pub fn delete_feed(&self, feed_id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM feeds WHERE id = ?1", params![feed_id])?;
        Ok(())
    }

    pub fn update_feed_category(&self, feed_id: &str, new_category: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE feeds SET category = ?1 WHERE id = ?2",
            params![new_category, feed_id],
        )?;
        Ok(())
    }

    pub fn delete_category(&self, category: &str) -> Result<()> {
        self.conn.execute("DELETE FROM feeds WHERE category = ?1", params![category])?;
        Ok(())
    }

    pub fn get_read_article_ids(&self) -> HashSet<String> {
        let mut stmt = match self.conn.prepare("SELECT article_id FROM read_articles") {
            Ok(s) => s,
            Err(_) => return HashSet::new(),
        };

        let rows = stmt.query_map([], |row| row.get::<_, String>(0));
        let mut ids = HashSet::new();
        if let Ok(rows) = rows {
            for row in rows {
                if let Ok(id) = row {
                    ids.insert(id);
                }
            }
        }
        ids
    }

    pub fn mark_article_read(&self, article_id: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO read_articles (article_id) VALUES (?1)",
            params![article_id],
        )?;
        Ok(())
    }

    pub fn toggle_bookmark(&self, article: &Article) -> Result<bool> {
        let mut stmt = self.conn.prepare("SELECT 1 FROM bookmarks WHERE article_id = ?1")?;
        let exists = stmt.exists(params![article.id])?;

        if exists {
            self.conn.execute("DELETE FROM bookmarks WHERE article_id = ?1", params![article.id])?;
            Ok(false)
        } else {
            let json = serde_json::to_string(article).unwrap_or_default();
            self.conn.execute(
                "INSERT INTO bookmarks (article_id, data) VALUES (?1, ?2)",
                params![article.id, json],
            )?;
            Ok(true)
        }
    }

    pub fn get_bookmarks(&self) -> Vec<Article> {
        let mut stmt = match self.conn.prepare("SELECT data FROM bookmarks") {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };

        let rows = stmt.query_map([], |row| row.get::<_, String>(0));
        let mut articles = Vec::new();
        if let Ok(rows) = rows {
            for row in rows {
                if let Ok(data) = row {
                    if let Ok(art) = serde_json::from_str::<Article>(&data) {
                        articles.push(art);
                    }
                }
            }
        }
        articles
    }
}

use anyhow::{Context, Result};
use chrono::Utc;
use rusqlite::{params, Connection};
use std::path::Path;

use crate::models::{Asset, AssetInsert, MemoVersion, Quote, QuoteInsert};

pub struct Db {
    pub conn: Connection,
}

impl Db {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let conn = Connection::open(path).context("Failed to open SQLite database")?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        let db = Db { conn };
        db.migrate()?;
        Ok(db)
    }

    fn migrate(&self) -> Result<()> {
        self.conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS quotes (
                id            TEXT PRIMARY KEY,
                quote         TEXT NOT NULL,
                source_title  TEXT,
                source_author TEXT,
                source_url    TEXT,
                tags          TEXT NOT NULL DEFAULT '[]',
                embedding     BLOB,
                created_at    TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS memo_versions (
                id         INTEGER PRIMARY KEY AUTOINCREMENT,
                quote_id   TEXT NOT NULL REFERENCES quotes(id),
                version    INTEGER NOT NULL,
                memo       TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS assets (
                hash       TEXT PRIMARY KEY,
                ext        TEXT NOT NULL,
                note       TEXT,
                created_at TEXT NOT NULL
            );
            ",
        )?;
        Ok(())
    }

    // ── Quotes ────────────────────────────────────────────────────────────────

    pub fn insert_quote(&self, input: &QuoteInsert, embedding: Option<Vec<f32>>) -> Result<String> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let tags_json = serde_json::to_string(&input.tags)?;
        let emb_blob = embedding.map(embedding_to_blob);

        self.conn.execute(
            "INSERT INTO quotes (id, quote, source_title, source_author, source_url, tags, embedding, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                id,
                input.quote,
                input.source_title,
                input.source_author,
                input.source_url,
                tags_json,
                emb_blob,
                now,
            ],
        )?;

        // Insert initial memo version (version=1, even if empty)
        let memo = input.memo.clone().unwrap_or_default();
        self.conn.execute(
            "INSERT INTO memo_versions (quote_id, version, memo, created_at) VALUES (?1, 1, ?2, ?3)",
            params![id, memo, now],
        )?;

        Ok(id)
    }

    pub fn get_quote(&self, id: &str) -> Result<Option<Quote>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, quote, source_title, source_author, source_url, tags, created_at FROM quotes WHERE id = ?1",
        )?;
        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row_to_quote(row)?))
        } else {
            Ok(None)
        }
    }

    pub fn list_quotes(&self) -> Result<Vec<Quote>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, quote, source_title, source_author, source_url, tags, created_at FROM quotes ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map([], |row| row_to_quote(row))?;
        rows.map(|r| r.map_err(anyhow::Error::from)).collect()
    }

    pub fn list_quotes_by_tag(&self, tag: &str) -> Result<Vec<Quote>> {
        // tags is a JSON array; use LIKE for simple matching
        let pattern = format!("%\"{}%", tag);
        let mut stmt = self.conn.prepare(
            "SELECT id, quote, source_title, source_author, source_url, tags, created_at FROM quotes WHERE tags LIKE ?1 ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map(params![pattern], |row| row_to_quote(row))?;
        rows.map(|r| r.map_err(anyhow::Error::from)).collect()
    }

    pub fn all_embeddings(&self) -> Result<Vec<(String, Vec<f32>)>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, embedding FROM quotes WHERE embedding IS NOT NULL")?;
        let rows = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let blob: Vec<u8> = row.get(1)?;
            Ok((id, blob))
        })?;
        rows.map(|r| {
            r.map_err(anyhow::Error::from)
                .map(|(id, blob)| (id, blob_to_embedding(&blob)))
        })
        .collect()
    }

    // ── Memo versions ────────────────────────────────────────────────────────

    pub fn add_memo_version(&self, quote_id: &str, memo: &str) -> Result<i64> {
        let now = Utc::now().to_rfc3339();
        let next_version: i64 = self.conn.query_row(
            "SELECT COALESCE(MAX(version), 0) + 1 FROM memo_versions WHERE quote_id = ?1",
            params![quote_id],
            |row| row.get(0),
        )?;
        self.conn.execute(
            "INSERT INTO memo_versions (quote_id, version, memo, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![quote_id, next_version, memo, now],
        )?;
        Ok(next_version)
    }

    pub fn get_memo_versions(&self, quote_id: &str) -> Result<Vec<MemoVersion>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, quote_id, version, memo, created_at FROM memo_versions WHERE quote_id = ?1 ORDER BY version ASC",
        )?;
        let rows = stmt.query_map(params![quote_id], |row| {
            Ok(MemoVersion {
                id: row.get(0)?,
                quote_id: row.get(1)?,
                version: row.get(2)?,
                memo: row.get(3)?,
                created_at: {
                    let s: String = row.get(4)?;
                    s.parse::<chrono::DateTime<chrono::Utc>>()
                        .unwrap_or_default()
                },
            })
        })?;
        rows.map(|r| r.map_err(anyhow::Error::from)).collect()
    }

    pub fn latest_memo(&self, quote_id: &str) -> Result<Option<MemoVersion>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, quote_id, version, memo, created_at FROM memo_versions WHERE quote_id = ?1 ORDER BY version DESC LIMIT 1",
        )?;
        let mut rows = stmt.query(params![quote_id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(MemoVersion {
                id: row.get(0)?,
                quote_id: row.get(1)?,
                version: row.get(2)?,
                memo: row.get(3)?,
                created_at: {
                    let s: String = row.get(4)?;
                    s.parse::<chrono::DateTime<chrono::Utc>>()
                        .unwrap_or_default()
                },
            }))
        } else {
            Ok(None)
        }
    }

    // ── Assets ───────────────────────────────────────────────────────────────

    pub fn insert_asset(&self, input: &AssetInsert) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT OR IGNORE INTO assets (hash, ext, note, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![input.hash, input.ext, input.note, now],
        )?;
        Ok(())
    }

    pub fn get_asset(&self, hash: &str) -> Result<Option<Asset>> {
        let mut stmt = self
            .conn
            .prepare("SELECT hash, ext, note, created_at FROM assets WHERE hash = ?1")?;
        let mut rows = stmt.query(params![hash])?;
        if let Some(row) = rows.next()? {
            Ok(Some(Asset {
                hash: row.get(0)?,
                ext: row.get(1)?,
                note: row.get(2)?,
                created_at: {
                    let s: String = row.get(3)?;
                    s.parse::<chrono::DateTime<chrono::Utc>>()
                        .unwrap_or_default()
                },
            }))
        } else {
            Ok(None)
        }
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn row_to_quote(row: &rusqlite::Row<'_>) -> Result<Quote, rusqlite::Error> {
    let tags_json: String = row.get(5)?;
    let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
    let created_at_str: String = row.get(6)?;
    Ok(Quote {
        id: row.get(0)?,
        quote: row.get(1)?,
        source_title: row.get(2)?,
        source_author: row.get(3)?,
        source_url: row.get(4)?,
        tags,
        created_at: created_at_str
            .parse::<chrono::DateTime<chrono::Utc>>()
            .unwrap_or_default(),
    })
}

pub fn embedding_to_blob(v: Vec<f32>) -> Vec<u8> {
    v.iter().flat_map(|f| f.to_le_bytes()).collect()
}

pub fn blob_to_embedding(blob: &[u8]) -> Vec<f32> {
    blob.chunks_exact(4)
        .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        .collect()
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub id: String,
    pub quote: String,
    /// Free-text source/reference (author, title, URL, etc.)
    pub source: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoVersion {
    pub id: i64,
    pub quote_id: String,
    pub version: i64,
    /// memo text; may contain {{img:hash}} / {{vid:hash}} markers
    pub memo: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub hash: String,
    pub ext: String,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
}

// --- Insert DTOs ---

#[derive(Debug)]
pub struct QuoteInsert {
    pub quote: String,
    /// Free-text source/reference
    pub source: Option<String>,
    pub memo: Option<String>,
}

#[derive(Debug)]
pub struct AssetInsert {
    pub hash: String,
    pub ext: String,
    pub note: Option<String>,
}

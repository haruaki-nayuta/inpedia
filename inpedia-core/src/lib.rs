pub mod db;
pub mod embedding;
pub mod models;
pub mod search;

pub use db::Db;
pub use embedding::Embedder;
pub use models::*;
pub use search::{search, SearchResult};

use anyhow::Result;
use std::path::PathBuf;

/// Returns ~/.inpedia directory, creating it if needed.
pub fn data_dir() -> Result<PathBuf> {
    let dir = dirs_next::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Cannot determine home directory"))?
        .join(".inpedia");
    std::fs::create_dir_all(&dir)?;
    std::fs::create_dir_all(dir.join("assets"))?;
    Ok(dir)
}

/// Open the default database at ~/.inpedia/inpedia.db.
pub fn open_db() -> Result<Db> {
    let dir = data_dir()?;
    Db::open(dir.join("inpedia.db"))
}

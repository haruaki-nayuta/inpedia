use crate::{db::Db, embedding::Embedder, models::Quote};
use anyhow::Result;

#[derive(Debug)]
pub struct SearchResult {
    pub quote: Quote,
    pub score: f32,
}

/// Search quotes by semantic similarity.
/// Loads all embeddings from DB into memory, computes cosine similarity against query embedding.
pub fn search(db: &Db, embedder: &mut Embedder, query: &str, top_n: usize) -> Result<Vec<SearchResult>> {
    let query_vec = embedder.embed(query)?;
    let all = db.all_embeddings()?;

    let mut scored: Vec<(String, f32)> = all
        .into_iter()
        .map(|(id, emb)| {
            let score = cosine_similarity(&query_vec, &emb);
            (id, score)
        })
        .collect();

    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    scored.truncate(top_n);

    let mut results = Vec::new();
    for (id, score) in scored {
        if let Some(quote) = db.get_quote(&id)? {
            results.push(SearchResult { quote, score });
        }
    }

    Ok(results)
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    dot / (norm_a * norm_b)
}

use anyhow::{Context, Result};
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};

pub struct Embedder {
    model: TextEmbedding,
}

impl Embedder {
    pub fn new() -> Result<Self> {
        let model = TextEmbedding::try_new(
            InitOptions::new(EmbeddingModel::MultilingualE5Small).with_show_download_progress(true),
        )
        .context("Failed to initialize embedding model")?;
        Ok(Self { model })
    }

    /// Generate embedding for a single text.
    pub fn embed(&mut self, text: &str) -> Result<Vec<f32>> {
        let results = self
            .model
            .embed(vec![text.to_string()], None)
            .context("Embedding generation failed")?;
        results
            .into_iter()
            .next()
            .context("No embedding returned")
    }

    /// Generate embeddings for multiple texts in a batch.
    pub fn embed_batch(&mut self, texts: Vec<String>) -> Result<Vec<Vec<f32>>> {
        self.model
            .embed(texts, None)
            .context("Batch embedding generation failed")
    }
}

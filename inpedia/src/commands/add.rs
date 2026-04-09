use anyhow::{bail, Context, Result};
use inpedia_core::{open_db, Embedder, QuoteInsert};
use crate::output;

pub async fn run(
    quote: String,
    source: Option<String>,
    memo: Option<String>,
    json: bool,
) -> Result<()> {
    let quote = quote.trim().to_string();
    if quote.is_empty() {
        bail!("--quote が空です。引用テキストを指定してください。");
    }

    output::print_info("embedding を生成中…", json);

    let mut embedder = Embedder::new()
        .context("embedding モデルの初期化に失敗しました（ONNX Runtime / モデルファイルを確認してください）")?;

    let embedding = embedder
        .embed(&quote)
        .context("embedding の生成に失敗しました")?;

    let db = open_db().context("データベースを開けませんでした")?;

    let id = db
        .insert_quote(
            &QuoteInsert { quote, source, memo },
            Some(embedding),
        )
        .context("引用の保存に失敗しました")?;

    output::print_ok(&id, json);
    Ok(())
}

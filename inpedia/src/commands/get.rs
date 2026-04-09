use anyhow::{Context, Result};
use inpedia_core::open_db;
use serde::Serialize;
use crate::output;

#[derive(Serialize)]
struct QuoteOut {
    id: String,
    quote: String,
    source: Option<String>,
    latest_memo: Option<String>,
    memo_version_count: usize,
    created_at: String,
}

pub async fn run(id: &str, json: bool) -> Result<()> {
    let db = open_db().context("データベースを開けませんでした")?;

    let q = db
        .get_quote(id)
        .context("データベースの検索に失敗しました")?
        .ok_or_else(|| anyhow::anyhow!("ID '{}' の引用が見つかりません", id))?;

    let versions = db.get_memo_versions(id)
        .context("メモ履歴の取得に失敗しました")?;
    let latest_memo = versions.last().map(|v| v.memo.clone());

    let out = QuoteOut {
        id: q.id.clone(),
        quote: q.quote.clone(),
        source: q.source.clone(),
        latest_memo,
        memo_version_count: versions.len(),
        created_at: q.created_at.format("%Y-%m-%d").to_string(),
    };

    output::print_data("", &out, json);
    Ok(())
}

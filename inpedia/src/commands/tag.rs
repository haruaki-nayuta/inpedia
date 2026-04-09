use anyhow::{Context, Result};
use inpedia_core::open_db;
use serde::Serialize;
use crate::output;

#[derive(Serialize)]
struct QuoteOut {
    id: String,
    quote: String,
    source_author: Option<String>,
    tags: Vec<String>,
    created_at: String,
}

pub async fn run(tag: &str, json: bool) -> Result<()> {
    if tag.trim().is_empty() {
        anyhow::bail!("タグ名が空です。");
    }
    let db = open_db().context("データベースを開けませんでした")?;
    let quotes = db.list_quotes_by_tag(tag)
        .context(format!("タグ '{}' での絞り込みに失敗しました", tag))?;

    if quotes.is_empty() {
        if json { println!("[]"); } else { eprintln!("{}", colored::Colorize::yellow(format!("タグ '{}' に一致する引用はありません。", tag).as_str())); }
        return Ok(());
    }

    let out: Vec<QuoteOut> = quotes.iter().map(|q| QuoteOut {
        id: q.id.clone(),
        quote: q.quote.clone(),
        source_author: q.source_author.clone(),
        tags: q.tags.clone(),
        created_at: q.created_at.format("%Y-%m-%d").to_string(),
    }).collect();

    output::print_data(&format!("tag='{}' — {} 件", tag, out.len()), &out, json);
    Ok(())
}

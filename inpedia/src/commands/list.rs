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
    created_at: String,
}

pub async fn run(json: bool) -> Result<()> {
    let db = open_db().context("データベースを開けませんでした")?;
    let quotes = db.list_quotes().context("引用一覧の取得に失敗しました")?;

    if quotes.is_empty() {
        if json { println!("[]"); } else { eprintln!("{}", colored::Colorize::yellow("登録された引用はありません。")); }
        return Ok(());
    }

    let out: Vec<QuoteOut> = quotes.iter().map(|q| {
        let memo = db.latest_memo(&q.id).ok().flatten().map(|m| m.memo);
        QuoteOut {
            id: q.id.clone(),
            quote: q.quote.clone(),
            source: q.source.clone(),
            latest_memo: memo,
            created_at: q.created_at.format("%Y-%m-%d").to_string(),
        }
    }).collect();

    output::print_data(&format!("引用一覧 {} 件", out.len()), &out, json);
    Ok(())
}

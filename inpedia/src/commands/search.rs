use anyhow::{Context, Result};
use inpedia_core::{open_db, search, Embedder};
use serde::Serialize;
use crate::output;

#[derive(Serialize)]
struct SearchResultOut {
    id: String,
    score: f32,
    quote: String,
    source_author: Option<String>,
    source_title: Option<String>,
    source_url: Option<String>,
    tags: Vec<String>,
    latest_memo: Option<String>,
    created_at: String,
}

pub async fn run(query: &str, top: usize, json: bool) -> Result<()> {
    if query.trim().is_empty() {
        anyhow::bail!("検索クエリが空です。");
    }

    output::print_info("embedding を生成中…", json);

    let mut embedder = Embedder::new()
        .context("embedding モデルの初期化に失敗しました")?;
    let db = open_db().context("データベースを開けませんでした")?;
    let results = search(&db, &mut embedder, query, top)
        .context("検索に失敗しました")?;

    if results.is_empty() {
        if json {
            println!("[]");
        } else {
            eprintln!("{}", colored::Colorize::yellow("結果なし"));
        }
        return Ok(());
    }

    let out: Vec<SearchResultOut> = results
        .iter()
        .map(|r| {
            let memo = db.latest_memo(&r.quote.id).ok().flatten().map(|m| m.memo);
            SearchResultOut {
                id: r.quote.id.clone(),
                score: r.score,
                quote: r.quote.quote.clone(),
                source_author: r.quote.source_author.clone(),
                source_title: r.quote.source_title.clone(),
                source_url: r.quote.source_url.clone(),
                tags: r.quote.tags.clone(),
                latest_memo: memo,
                created_at: r.quote.created_at.format("%Y-%m-%d").to_string(),
            }
        })
        .collect();

    if json {
        println!("{}", serde_json::to_string(&out)?);
    } else {
        use colored::Colorize;
        println!("{} {} 件", "── 検索結果".cyan(), out.len());
        for (i, r) in out.iter().enumerate() {
            println!("\n{} {}  {}", format!("[{}]", i + 1).bold(), format!("score: {:.3}", r.score).dimmed(), r.id.dimmed());
            println!("  {}", r.quote.white());
            if let Some(a) = &r.source_author { print!("  — {}", a.italic()); }
            if let Some(t) = &r.source_title  { print!("  『{}』", t.italic()); }
            if !r.tags.is_empty() { print!("  {}", r.tags.join(", ").dimmed()); }
            println!();
            if let Some(m) = &r.latest_memo { if !m.is_empty() { println!("  {}", m.trim().dimmed()); } }
        }
    }
    Ok(())
}

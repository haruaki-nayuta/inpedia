use anyhow::{Context, Result};
use inpedia_core::open_db;
use serde::Serialize;
use similar::{ChangeTag, TextDiff};
use crate::output;

#[derive(Serialize)]
struct VersionOut {
    version: i64,
    memo: String,
    created_at: String,
    diff_from_prev: Option<Vec<DiffLine>>,
}

#[derive(Serialize)]
struct DiffLine {
    tag: &'static str, // "equal" | "insert" | "delete"
    content: String,
}

pub async fn run(id: &str, json: bool) -> Result<()> {
    let db = open_db().context("データベースを開けませんでした")?;

    db.get_quote(id)
        .context("データベースの検索に失敗しました")?
        .ok_or_else(|| anyhow::anyhow!("ID '{}' の引用が見つかりません", id))?;

    let versions = db
        .get_memo_versions(id)
        .context("メモ履歴の取得に失敗しました")?;

    if versions.is_empty() {
        if json { println!("[]"); } else { eprintln!("{}", colored::Colorize::yellow("メモ版がありません。")); }
        return Ok(());
    }

    if json {
        let out: Vec<VersionOut> = versions.iter().enumerate().map(|(i, v)| {
            let diff = if i == 0 {
                None
            } else {
                let prev = &versions[i - 1].memo;
                let d = TextDiff::from_lines(prev.as_str(), v.memo.as_str());
                Some(d.iter_all_changes().map(|c| DiffLine {
                    tag: match c.tag() {
                        ChangeTag::Equal  => "equal",
                        ChangeTag::Insert => "insert",
                        ChangeTag::Delete => "delete",
                    },
                    content: c.to_string(),
                }).collect())
            };
            VersionOut {
                version: v.version,
                memo: v.memo.clone(),
                created_at: v.created_at.format("%Y-%m-%d %H:%M").to_string(),
                diff_from_prev: diff,
            }
        }).collect();
        println!("{}", serde_json::to_string(&out)?);
    } else {
        use colored::Colorize;
        println!("{} {} — {} 版", "── history".cyan(), id.bold(), versions.len());
        for (i, v) in versions.iter().enumerate() {
            println!("\n{}", format!("v{}  {}", v.version, v.created_at.format("%Y-%m-%d %H:%M")).bold());
            if i == 0 {
                for line in v.memo.lines() { println!("  {}", line.white()); }
            } else {
                let prev = &versions[i - 1].memo;
                let diff = TextDiff::from_lines(prev.as_str(), v.memo.as_str());
                for change in diff.iter_all_changes() {
                    match change.tag() {
                        ChangeTag::Delete => print!("  {}", format!("- {}", change).red()),
                        ChangeTag::Insert => print!("  {}", format!("+ {}", change).green()),
                        ChangeTag::Equal  => print!("    {}", change.to_string().dimmed()),
                    }
                }
            }
        }
    }
    Ok(())
}

use anyhow::{Context, Result};
use inpedia_core::open_db;
use crate::output;
use serde_json::json;

pub async fn run(id: &str, memo: &str, json: bool) -> Result<()> {
    let db = open_db().context("データベースを開けませんでした")?;

    // Verify the quote exists before updating
    db.get_quote(id)
        .context("データベースの検索に失敗しました")?
        .ok_or_else(|| anyhow::anyhow!("ID '{}' の引用が見つかりません", id))?;

    let version = db
        .add_memo_version(id, memo)
        .context("メモバージョンの保存に失敗しました")?;

    if json {
        println!("{}", json!({"ok": true, "id": id, "version": version}));
    } else {
        use colored::Colorize;
        println!("{} id={} v{}", "✓ メモ更新".green(), id.dimmed(), version);
    }
    Ok(())
}

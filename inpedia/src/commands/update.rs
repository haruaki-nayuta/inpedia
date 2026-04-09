use anyhow::Result;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Editor};
use inpedia_core::open_db;

pub async fn run(id: &str) -> Result<()> {
    let db = open_db()?;
    let quote = db
        .get_quote(id)?
        .ok_or_else(|| anyhow::anyhow!("ID が見つかりません: {}", id))?;

    println!("{}", "── inpedia update ───────────────────".cyan());
    println!("  {}", quote.quote.white());
    println!();

    let current = db
        .latest_memo(id)?
        .map(|mv| mv.memo)
        .unwrap_or_default();

    let new_memo = Editor::new()
        .edit(&current)?
        .ok_or_else(|| anyhow::anyhow!("編集がキャンセルされました"))?;

    if new_memo.trim() == current.trim() {
        println!("{}", "変更なし".yellow());
        return Ok(());
    }

    let version = db.add_memo_version(id, &new_memo)?;
    println!("{} v{}", "✓ メモ更新完了".green(), version);
    Ok(())
}

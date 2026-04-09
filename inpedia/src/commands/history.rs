use anyhow::Result;
use colored::Colorize;
use inpedia_core::open_db;
use similar::{ChangeTag, TextDiff};

pub async fn run(id: &str) -> Result<()> {
    let db = open_db()?;
    let _quote = db
        .get_quote(id)?
        .ok_or_else(|| anyhow::anyhow!("ID が見つかりません: {}", id))?;

    let versions = db.get_memo_versions(id)?;

    if versions.is_empty() {
        println!("{}", "メモ版がありません。".yellow());
        return Ok(());
    }

    println!("{} {} — {} 版", "── history".cyan(), id.bold(), versions.len());

    for (i, mv) in versions.iter().enumerate() {
        println!(
            "\n{} {}",
            format!("v{}  {}", mv.version, mv.created_at.format("%Y-%m-%d %H:%M")).bold(),
            "",
        );

        if i == 0 {
            // First version — print as-is
            for line in mv.memo.lines() {
                println!("  {}", line.white());
            }
        } else {
            // Show diff from previous version
            let prev = &versions[i - 1].memo;
            let diff = TextDiff::from_lines(prev.as_str(), mv.memo.as_str());
            for change in diff.iter_all_changes() {
                match change.tag() {
                    ChangeTag::Delete => print!("  {}", format!("- {}", change).red()),
                    ChangeTag::Insert => print!("  {}", format!("+ {}", change).green()),
                    ChangeTag::Equal => print!("    {}", change.to_string().dimmed()),
                }
            }
        }
    }

    Ok(())
}

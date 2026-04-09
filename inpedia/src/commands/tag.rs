use anyhow::Result;
use colored::Colorize;
use inpedia_core::open_db;

pub async fn run(tag: &str) -> Result<()> {
    let db = open_db()?;
    let quotes = db.list_quotes_by_tag(tag)?;

    if quotes.is_empty() {
        println!("{} '{}' に一致する引用はありません。", "tag:".yellow(), tag);
        return Ok(());
    }

    println!("{} '{}' — {} 件", "── tag".cyan(), tag.bold(), quotes.len());
    for q in &quotes {
        println!(
            "\n{}  {}\n  {}",
            q.id.dimmed(),
            q.quote.white().bold(),
            q.source_author.as_deref().unwrap_or("—").italic(),
        );
    }
    Ok(())
}

use anyhow::Result;
use colored::Colorize;
use inpedia_core::open_db;

pub async fn run() -> Result<()> {
    let db = open_db()?;
    let quotes = db.list_quotes()?;

    if quotes.is_empty() {
        println!("{}", "引用がまだ登録されていません。".yellow());
        return Ok(());
    }

    println!("{} {} 件", "── 引用一覧".cyan(), quotes.len());
    for q in &quotes {
        let author = q
            .source_author
            .as_deref()
            .unwrap_or("—");
        let tags = if q.tags.is_empty() {
            String::new()
        } else {
            format!("  [{}]", q.tags.join(", "))
        };
        println!(
            "\n{}\n  {}  {}{}\n  {}",
            q.id.dimmed(),
            q.quote.white().bold(),
            author.italic(),
            tags.dimmed(),
            q.created_at.format("%Y-%m-%d").to_string().dimmed(),
        );
    }
    Ok(())
}

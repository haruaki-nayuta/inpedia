use anyhow::Result;
use colored::Colorize;
use inpedia_core::{open_db, search, Embedder};

pub async fn run(query: &str, top: usize) -> Result<()> {
    println!("{} {}", "── search:".cyan(), query.bold());
    println!("{}", "embedding を生成中...".dimmed());

    let mut embedder = Embedder::new()?;
    let db = open_db()?;
    let results = search(&db, &mut embedder, query, top)?;

    if results.is_empty() {
        println!("{}", "結果なし".yellow());
        return Ok(());
    }

    for (i, r) in results.iter().enumerate() {
        let q = &r.quote;
        println!(
            "\n{} {}  {}",
            format!("[{}]", i + 1).bold(),
            format!("score: {:.3}", r.score).dimmed(),
            q.id.dimmed(),
        );
        println!("  {}", q.quote.white());
        if let Some(author) = &q.source_author {
            print!("  {} {}", "—".dimmed(), author.italic());
        }
        if let Some(title) = &q.source_title {
            print!("  『{}』", title.italic());
        }
        if !q.tags.is_empty() {
            print!("  {}", q.tags.join(", ").dimmed());
        }
        println!();

        // Show latest memo if any
        if let Ok(Some(mv)) = db.latest_memo(&q.id) {
            if !mv.memo.is_empty() {
                println!("  {}", mv.memo.trim().dimmed());
            }
        }
    }

    Ok(())
}

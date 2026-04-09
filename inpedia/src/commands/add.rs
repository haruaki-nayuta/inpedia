use anyhow::Result;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, Editor, Input};
use inpedia_core::{open_db, Embedder, QuoteInsert};

pub async fn run() -> Result<()> {
    let theme = ColorfulTheme::default();

    println!("{}", "── inpedia add ──────────────────────".cyan());

    let quote: String = Input::with_theme(&theme)
        .with_prompt("引用テキスト")
        .interact_text()?;

    let source_author: String = Input::with_theme(&theme)
        .with_prompt("著者名 (空白でスキップ)")
        .allow_empty(true)
        .interact_text()?;

    let source_title: String = Input::with_theme(&theme)
        .with_prompt("出典タイトル (空白でスキップ)")
        .allow_empty(true)
        .interact_text()?;

    let source_url: String = Input::with_theme(&theme)
        .with_prompt("URL (空白でスキップ)")
        .allow_empty(true)
        .interact_text()?;

    let tags_raw: String = Input::with_theme(&theme)
        .with_prompt("タグ (カンマ区切り, 空白でスキップ)")
        .allow_empty(true)
        .interact_text()?;

    let tags: Vec<String> = tags_raw
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let add_memo = Confirm::with_theme(&theme)
        .with_prompt("メモを追加しますか？")
        .default(false)
        .interact()?;

    let memo = if add_memo {
        Editor::new().edit("# メモを入力してください\n")?
    } else {
        None
    };

    println!("{}", "embedding を生成中...".dimmed());
    let mut embedder = Embedder::new()?;
    let embedding = embedder.embed(&quote)?;

    let db = open_db()?;
    let id = db.insert_quote(
        &QuoteInsert {
            quote: quote.clone(),
            source_author: opt_str(source_author),
            source_title: opt_str(source_title),
            source_url: opt_str(source_url),
            tags,
            memo,
        },
        Some(embedding),
    )?;

    println!("{} {}", "✓ 登録完了 id:".green(), id.bold());
    Ok(())
}

fn opt_str(s: String) -> Option<String> {
    if s.is_empty() { None } else { Some(s) }
}

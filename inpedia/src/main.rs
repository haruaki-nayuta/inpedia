use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(name = "inpedia", about = "引用の電子辞書 + CMS", version)]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// 引用・メモ・出典を登録
    Add,
    /// セマンティック検索
    Search {
        query: String,
        #[arg(short, long, default_value = "5")]
        top: usize,
    },
    /// メモを更新（旧版は自動保持）
    Update { id: String },
    /// 全引用を一覧表示
    List,
    /// タグで絞り込み
    Tag { tag: String },
    /// 版の変遷を表示
    History { id: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Cmd::Add => commands::add::run().await,
        Cmd::Search { query, top } => commands::search::run(&query, top).await,
        Cmd::Update { id } => commands::update::run(&id).await,
        Cmd::List => commands::list::run().await,
        Cmd::Tag { tag } => commands::tag::run(&tag).await,
        Cmd::History { id } => commands::history::run(&id).await,
    }
}

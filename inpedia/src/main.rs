use clap::{Parser, Subcommand};

mod commands;
pub mod output;

#[derive(Parser)]
#[command(
    name = "inpedia",
    about = "引用の電子辞書 + CMS",
    version,
    after_help = "出力は --json フラグで JSON 形式に切り替えられます。\nエラー時は exit code 1 を返します。"
)]
struct Cli {
    /// 出力を JSON 形式にする（LLM・スクリプト連携用）
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// 引用を登録して ID を返す
    ///
    /// Example: inpedia add --quote "存在するとは知覚されることである" --author "バークリー" --tags "哲学,認識論"
    Add {
        /// 引用テキスト（必須）
        #[arg(long, short)]
        quote: String,

        /// 著者名
        #[arg(long, short)]
        author: Option<String>,

        /// 出典タイトル
        #[arg(long, short = 't')]
        title: Option<String>,

        /// 出典 URL
        #[arg(long, short = 'u')]
        url: Option<String>,

        /// タグ（カンマ区切り）
        #[arg(long, short = 'g')]
        tags: Option<String>,

        /// メモ（{{img:hash}} / {{vid:hash}} 記法使用可）
        #[arg(long, short = 'm')]
        memo: Option<String>,
    },

    /// セマンティック検索
    ///
    /// Example: inpedia search "認知の歪み" --top 10
    Search {
        /// 検索クエリ
        query: String,

        /// 返す件数（デフォルト: 5）
        #[arg(short, long, default_value = "5")]
        top: usize,
    },

    /// メモを更新（旧版は自動保持）
    ///
    /// Example: inpedia update abc123 --memo "新しいメモ内容"
    Update {
        /// 引用 ID
        id: String,

        /// 新しいメモ内容（必須）
        #[arg(long, short)]
        memo: String,
    },

    /// 全引用を一覧表示
    List,

    /// タグで絞り込み
    ///
    /// Example: inpedia tag 哲学
    Tag {
        /// タグ名
        tag: String,
    },

    /// メモの版の変遷を表示
    ///
    /// Example: inpedia history abc123
    History {
        /// 引用 ID
        id: String,
    },

    /// 引用を ID で取得
    ///
    /// Example: inpedia get abc123
    Get {
        /// 引用 ID
        id: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let json = cli.json;

    let result = match cli.command {
        Cmd::Add { quote, author, title, url, tags, memo } =>
            commands::add::run(quote, author, title, url, tags, memo, json).await,
        Cmd::Search { query, top } =>
            commands::search::run(&query, top, json).await,
        Cmd::Update { id, memo } =>
            commands::update::run(&id, &memo, json).await,
        Cmd::List =>
            commands::list::run(json).await,
        Cmd::Tag { tag } =>
            commands::tag::run(&tag, json).await,
        Cmd::History { id } =>
            commands::history::run(&id, json).await,
        Cmd::Get { id } =>
            commands::get::run(&id, json).await,
    };

    if let Err(e) = result {
        output::print_error(&e.to_string(), json);
        std::process::exit(1);
    }
}

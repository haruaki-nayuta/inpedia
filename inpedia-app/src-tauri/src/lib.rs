use inpedia_core::Embedder;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

pub mod commands;

pub struct AppState {
    pub embedder: Mutex<Option<Embedder>>,
}

// ── DTOs ─────────────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize)]
pub struct QuoteDto {
    pub id: String,
    pub quote: String,
    pub source: Option<String>,
    pub created_at: String,
    pub latest_memo: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchResultDto {
    pub quote: QuoteDto,
    pub score: f32,
}

#[derive(Serialize, Deserialize)]
pub struct MemoVersionDto {
    pub version: i64,
    pub memo: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddQuoteInput {
    pub quote: String,
    pub source: Option<String>,
    pub memo: Option<String>,
}

pub fn quote_to_dto(q: inpedia_core::Quote, latest_memo: Option<String>) -> QuoteDto {
    QuoteDto {
        id: q.id,
        quote: q.quote,
        source: q.source,
        created_at: q.created_at.format("%Y-%m-%d").to_string(),
        latest_memo,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let embedder = Embedder::new().ok();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            embedder: Mutex::new(embedder),
        })
        .invoke_handler(tauri::generate_handler![
            commands::search_quotes,
            commands::list_quotes,
            commands::add_quote,
            commands::get_history,
            commands::update_memo,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

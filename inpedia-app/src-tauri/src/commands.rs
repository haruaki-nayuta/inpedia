use inpedia_core::{open_db, QuoteInsert};
use tauri::State;
use crate::{AppState, QuoteDto, SearchResultDto, MemoVersionDto, AddQuoteInput, quote_to_dto};
use anyhow::Result;

#[tauri::command]
pub fn search_quotes(
    query: String,
    top: usize,
    state: State<'_, AppState>,
) -> Result<Vec<SearchResultDto>, String> {
    let db = open_db().map_err(|e| e.to_string())?;
    let mut guard = state.embedder.lock().map_err(|_| "mutex poisoned".to_string())?;
    let embedder = guard.as_mut().ok_or("Embedder not initialized")?;
    let results = inpedia_core::search(&db, embedder, &query, top).map_err(|e| e.to_string())?;
    Ok(results.into_iter().map(|r| {
        let memo = db.latest_memo(&r.quote.id).ok().flatten().map(|m| m.memo);
        SearchResultDto { score: r.score, quote: quote_to_dto(r.quote, memo) }
    }).collect())
}

#[tauri::command]
pub fn list_quotes() -> Result<Vec<QuoteDto>, String> {
    let db = open_db().map_err(|e| e.to_string())?;
    let quotes = db.list_quotes().map_err(|e| e.to_string())?;
    Ok(quotes.into_iter().map(|q| {
        let memo = db.latest_memo(&q.id).ok().flatten().map(|m| m.memo);
        quote_to_dto(q, memo)
    }).collect())
}

#[tauri::command]
pub fn add_quote(input: AddQuoteInput, state: State<'_, AppState>) -> Result<String, String> {
    let db = open_db().map_err(|e| e.to_string())?;
    let mut guard = state.embedder.lock().map_err(|_| "mutex poisoned".to_string())?;
    let embedder = guard.as_mut().ok_or("Embedder not initialized")?;
    let embedding = embedder.embed(&input.quote).map_err(|e| e.to_string())?;
    db.insert_quote(&QuoteInsert {
        quote: input.quote,
        source_title: input.source_title,
        source_author: input.source_author,
        source_url: input.source_url,
        tags: input.tags,
        memo: input.memo,
    }, Some(embedding)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_history(quote_id: String) -> Result<Vec<MemoVersionDto>, String> {
    let db = open_db().map_err(|e| e.to_string())?;
    db.get_memo_versions(&quote_id)
        .map_err(|e| e.to_string())
        .map(|vs| vs.into_iter().map(|v| MemoVersionDto {
            version: v.version,
            memo: v.memo,
            created_at: v.created_at.format("%Y-%m-%d %H:%M").to_string(),
        }).collect())
}

#[tauri::command]
pub fn update_memo(quote_id: String, memo: String) -> Result<i64, String> {
    let db = open_db().map_err(|e| e.to_string())?;
    db.add_memo_version(&quote_id, &memo).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_by_tag(tag: String) -> Result<Vec<QuoteDto>, String> {
    let db = open_db().map_err(|e| e.to_string())?;
    let quotes = db.list_quotes_by_tag(&tag).map_err(|e| e.to_string())?;
    Ok(quotes.into_iter().map(|q| {
        let memo = db.latest_memo(&q.id).ok().flatten().map(|m| m.memo);
        quote_to_dto(q, memo)
    }).collect())
}

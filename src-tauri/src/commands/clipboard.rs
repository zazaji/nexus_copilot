// src-tauri/src/commands/clipboard.rs
use crate::{
    database::{models, queries},
    error::{AppError, Result},
    state::AppState,
};
use tauri::State;

#[tauri::command]
pub fn get_clipboard_history(state: State<'_, AppState>) -> Result<Vec<models::ClipboardHistoryItem>> {
    let conn = state.db.lock().unwrap();
    match queries::get_clipboard_history(&conn) {
        Ok(history) => Ok(history),
        Err(AppError::Database(e)) if e.contains("no such table") => {
            log::warn!("'clipboard_history' table not found, returning empty history. This may happen on first run after an update.");
            Ok(vec![])
        },
        Err(e) => Err(e),
    }
}

#[tauri::command]
pub fn clear_clipboard_history(state: State<'_, AppState>) -> Result<()> {
    let conn = state.db.lock().unwrap();
    queries::clear_clipboard_history(&conn)
}

#[tauri::command]
pub fn delete_clipboard_item(state: State<'_, AppState>, id: String) -> Result<()> {
    let conn = state.db.lock().unwrap();
    queries::delete_clipboard_item(&conn, &id)
}

#[tauri::command]
pub fn delete_clipboard_items(state: State<'_, AppState>, ids: Vec<String>) -> Result<()> {
    let mut conn = state.db.lock().unwrap();
    queries::delete_clipboard_items(&mut conn, ids)
}

#[tauri::command]
pub fn toggle_clipboard_item_pinned(state: State<'_, AppState>, id: String, is_pinned: bool) -> Result<()> {
    let conn = state.db.lock().unwrap();
    queries::toggle_pinned_status(&conn, &id, is_pinned)
}
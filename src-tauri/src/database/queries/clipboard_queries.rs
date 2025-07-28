// src-tauri/src/database/queries/clipboard_queries.rs
use crate::database::models::*;
use crate::error::Result;
use rusqlite::{params, Connection};

pub fn add_clipboard_item(conn: &Connection, item: &ClipboardHistoryItem) -> Result<()> {
    conn.execute(
        "INSERT INTO clipboard_history (id, type, content, timestamp, is_pinned) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![&item.id, &item.item_type, &item.content, &item.timestamp, &item.is_pinned],
    )?;
    Ok(())
}

pub fn get_clipboard_history(conn: &Connection) -> Result<Vec<ClipboardHistoryItem>> {
    let mut stmt = conn.prepare("SELECT id, type, content, timestamp, is_pinned FROM clipboard_history ORDER BY is_pinned DESC, timestamp DESC LIMIT 200")?;
    let item_iter = stmt.query_map([], |row| {
        Ok(ClipboardHistoryItem {
            id: row.get(0)?,
            item_type: row.get(1)?,
            content: row.get(2)?,
            timestamp: row.get(3)?,
            is_pinned: row.get(4)?,
        })
    })?;
    item_iter.collect::<rusqlite::Result<Vec<_>>>().map_err(Into::into)
}

pub fn clear_clipboard_history(conn: &Connection) -> Result<()> {
    conn.execute("DELETE FROM clipboard_history WHERE is_pinned = FALSE", [])?;
    Ok(())
}

pub fn delete_clipboard_item(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM clipboard_history WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn delete_clipboard_items(conn: &mut Connection, ids: Vec<String>) -> Result<()> {
    let tx = conn.transaction()?;
    {
        let mut stmt = tx.prepare("DELETE FROM clipboard_history WHERE id = ?1")?;
        for id in ids {
            stmt.execute(params![id])?;
        }
    }
    tx.commit()?;
    Ok(())
}

pub fn toggle_pinned_status(conn: &Connection, id: &str, is_pinned: bool) -> Result<()> {
    conn.execute(
        "UPDATE clipboard_history SET is_pinned = ?2 WHERE id = ?1",
        params![id, is_pinned],
    )?;
    Ok(())
}
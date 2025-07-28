// src-tauri/src/database/schema.rs
use crate::error::Result;
use rusqlite::Connection;

/// This function defines the latest database schema.
/// It should only be called for a completely new database.
pub fn initialize_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "BEGIN;
        CREATE TABLE settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        CREATE TABLE conversations (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            created_at INTEGER NOT NULL
        );
        CREATE TABLE messages (
            id TEXT PRIMARY KEY,
            conversation_id TEXT NOT NULL,
            role TEXT NOT NULL,
            content TEXT NOT NULL,
            timestamp INTEGER NOT NULL,
            suggestions TEXT,
            model TEXT,
            knowledge_base_selection TEXT,
            sources TEXT,
            error TEXT,
            FOREIGN KEY (conversation_id) REFERENCES conversations (id) ON DELETE CASCADE
        );
        CREATE TABLE notes (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );
        CREATE TABLE note_links (
            source_id TEXT NOT NULL,
            target_id TEXT NOT NULL,
            PRIMARY KEY (source_id, target_id),
            FOREIGN KEY (source_id) REFERENCES notes (id) ON DELETE CASCADE,
            FOREIGN KEY (target_id) REFERENCES notes (id) ON DELETE CASCADE
        );
        CREATE TABLE configured_tools (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            script_path TEXT NOT NULL,
            parameters TEXT NOT NULL,
            show_in_copilot BOOLEAN NOT NULL DEFAULT FALSE
        );
        COMMIT;"
    )?;
    Ok(())
}
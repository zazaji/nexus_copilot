// src-tauri/src/database/migrations.rs
use crate::error::Result;
use rusqlite::Connection;

const LATEST_VERSION: u32 = 21;

fn table_exists(conn: &Connection, table_name: &str) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?1")?;
    let exists = stmt.query_row([table_name], |_| Ok(true)).ok().is_some();
    Ok(exists)
}

fn column_exists(conn: &Connection, table_name: &str, column_name: &str) -> Result<bool> {
    let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table_name))?;
    let column_names = stmt.query_map([], |row| row.get::<_, String>(1))?.collect::<std::result::Result<Vec<String>, _>>()?;
    Ok(column_names.iter().any(|name| name.eq_ignore_ascii_case(column_name)))
}

fn run_initial_schema(conn: &mut Connection) -> Result<()> {
    log::info!("Running initial schema setup...");
    conn.execute_batch(
        "BEGIN;
        CREATE TABLE settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        CREATE TABLE conversations (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            session_type TEXT NOT NULL DEFAULT 'chat'
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
            agent_task_id TEXT,
            FOREIGN KEY (conversation_id) REFERENCES conversations (id) ON DELETE CASCADE
        );
        CREATE TABLE configured_tools (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            script_path TEXT,
            webhook_url TEXT,
            webhook_method TEXT,
            webhook_headers TEXT,
            webhook_body_template TEXT,
            input_schema TEXT,
            runtime TEXT NOT NULL DEFAULT 'python',
            parameters TEXT NOT NULL,
            show_in_copilot BOOLEAN NOT NULL DEFAULT FALSE,
            is_favorite BOOLEAN NOT NULL DEFAULT FALSE,
            input_source TEXT NOT NULL DEFAULT 'user_input',
            requires_ai_pre_processing BOOLEAN NOT NULL DEFAULT FALSE,
            pre_processing_prompt TEXT NOT NULL DEFAULT '',
            output_handling TEXT NOT NULL DEFAULT 'raw_text',
            requires_ai_post_processing BOOLEAN NOT NULL DEFAULT FALSE,
            post_processing_prompt TEXT NOT NULL DEFAULT ''
        );
        CREATE TABLE clipboard_history (
            id TEXT PRIMARY KEY,
            type TEXT NOT NULL,
            content TEXT NOT NULL,
            timestamp INTEGER NOT NULL,
            is_pinned BOOLEAN NOT NULL DEFAULT FALSE
        );
        CREATE TABLE online_knowledge_bases (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            url TEXT NOT NULL,
            token TEXT NOT NULL
        );
        CREATE TABLE creation_artifacts (
            id TEXT PRIMARY KEY,
            conversation_id TEXT NOT NULL,
            artifact_type TEXT NOT NULL,
            prompt TEXT NOT NULL,
            model_used TEXT NOT NULL,
            file_path TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            metadata TEXT,
            FOREIGN KEY (conversation_id) REFERENCES conversations (id) ON DELETE CASCADE
        );
        COMMIT;"
    )?;
    conn.execute(&format!("PRAGMA user_version = {}", LATEST_VERSION), [])?;
    log::info!("Initialized fresh database schema at version {}", LATEST_VERSION);
    Ok(())
}

pub fn run(conn: &mut Connection) -> Result<()> {
    conn.execute("PRAGMA foreign_keys = ON", [])?;

    let user_version: u32 = conn.query_row("PRAGMA user_version", [], |row| row.get(0))?;

    if user_version == 0 && !table_exists(conn, "settings")? {
        return run_initial_schema(conn);
    }

    log::info!("Database version check: current={}, latest={}", user_version, LATEST_VERSION);

    if user_version >= LATEST_VERSION {
        log::info!("Database is up to date.");
        return Ok(());
    }

    log::info!("Current database version {} is older than latest version {}. Running migrations...", user_version, LATEST_VERSION);

    if user_version < 20 {
        // Migrations up to 20...
        if user_version < 18 {
             if !column_exists(conn, "configured_tools", "input_schema")? {
                conn.execute("ALTER TABLE configured_tools ADD COLUMN input_schema TEXT;", [])?;
            }
        }
        if user_version < 19 {
            conn.execute_batch(
                "BEGIN;
                CREATE TABLE IF NOT EXISTS creation_sessions (
                    id TEXT PRIMARY KEY,
                    title TEXT NOT NULL,
                    created_at INTEGER NOT NULL
                );
                CREATE TABLE IF NOT EXISTS creation_artifacts (
                    id TEXT PRIMARY KEY,
                    session_id TEXT NOT NULL,
                    artifact_type TEXT NOT NULL,
                    prompt TEXT NOT NULL,
                    model_used TEXT NOT NULL,
                    file_path TEXT NOT NULL,
                    created_at INTEGER NOT NULL,
                    metadata TEXT,
                    FOREIGN KEY (session_id) REFERENCES creation_sessions (id) ON DELETE CASCADE
                );
                COMMIT;"
            )?;
        }
        if user_version < 20 {
            log::info!("Migrating from version {} to 20...", user_version);
            conn.execute_batch("PRAGMA foreign_keys=OFF;")?;
            conn.execute_batch(
                "BEGIN;
                ALTER TABLE conversations ADD COLUMN session_type TEXT NOT NULL DEFAULT 'chat';
                CREATE TABLE creation_artifacts_new (
                    id TEXT PRIMARY KEY,
                    conversation_id TEXT NOT NULL,
                    artifact_type TEXT NOT NULL,
                    prompt TEXT NOT NULL,
                    model_used TEXT NOT NULL,
                    file_path TEXT NOT NULL,
                    created_at INTEGER NOT NULL,
                    metadata TEXT,
                    FOREIGN KEY (conversation_id) REFERENCES conversations (id) ON DELETE CASCADE
                );
                INSERT INTO creation_artifacts_new (id, conversation_id, artifact_type, prompt, model_used, file_path, created_at, metadata)
                SELECT id, session_id, artifact_type, prompt, model_used, file_path, created_at, metadata FROM creation_artifacts;
                DROP TABLE creation_artifacts;
                ALTER TABLE creation_artifacts_new RENAME TO creation_artifacts;
                DROP TABLE IF EXISTS creation_sessions;
                COMMIT;"
            )?;
            conn.execute_batch("PRAGMA foreign_keys=ON;")?;
        }
    }
    
    if user_version < 21 {
        log::info!("Migrating from version {} to 21...", user_version);
        // Ensure the clipboard_history table exists before trying to alter it.
        if table_exists(conn, "clipboard_history")? {
            if !column_exists(conn, "clipboard_history", "is_pinned")? {
                conn.execute("ALTER TABLE clipboard_history ADD COLUMN is_pinned BOOLEAN NOT NULL DEFAULT FALSE;", [])?;
                log::info!("Added 'is_pinned' column to 'clipboard_history' table.");
            }
        } else {
            // If the table doesn't exist for some reason, create it with the full new schema.
            conn.execute(
                "CREATE TABLE clipboard_history (
                    id TEXT PRIMARY KEY,
                    type TEXT NOT NULL,
                    content TEXT NOT NULL,
                    timestamp INTEGER NOT NULL,
                    is_pinned BOOLEAN NOT NULL DEFAULT FALSE
                );",
                [],
            )?;
            log::info!("Created 'clipboard_history' table as it was missing.");
        }
        log::info!("Migration to version 21 successful.");
    }

    conn.execute(&format!("PRAGMA user_version = {}", LATEST_VERSION), [])?;
    log::info!("All migrations applied. Database is now at version {}", LATEST_VERSION);

    Ok(())
}
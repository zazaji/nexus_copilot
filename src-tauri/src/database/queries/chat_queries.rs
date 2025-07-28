// src-tauri/src/database/queries/chat_queries.rs
use crate::database::models::*;
use crate::error::Result;
use rusqlite::{params, Connection, OptionalExtension};

pub fn save_message(conn: &Connection, msg: &ChatMessage) -> Result<()> {
    let content_json = serde_json::to_string(&msg.content)?;
    let sources_json = serde_json::to_string(&msg.sources)?;
    let suggestions_json = serde_json::to_string(&msg.suggestions)?;
    let error_json = serde_json::to_string(&msg.error)?;

    conn.execute(
        "INSERT OR REPLACE INTO messages (id, conversation_id, role, content, timestamp, suggestions, model, knowledge_base_selection, sources, error, agent_task_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            msg.id,
            msg.conversation_id,
            msg.role,
            content_json,
            msg.timestamp,
            suggestions_json,
            msg.model,
            msg.knowledge_base_selection,
            sources_json,
            error_json,
            msg.agent_task_id,
        ],
    )?;
    Ok(())
}

pub fn delete_message(conn: &Connection, message_id: &str) -> Result<()> {
    let affected = conn.execute("DELETE FROM messages WHERE id = ?1", params![message_id])?;
    if affected == 0 {
        log::warn!("Attempted to delete a non-existent message with ID: {}", message_id);
    }
    Ok(())
}

pub fn update_message_content(conn: &Connection, message_id: &str, new_content_json: &str) -> Result<()> {
    conn.execute(
        "UPDATE messages SET content = ?2 WHERE id = ?1",
        params![message_id, new_content_json],
    )?;
    Ok(())
}

pub fn get_conversation_history(conn: &Connection, conversation_id: &str) -> Result<Vec<ChatMessage>> {
    let mut stmt = conn.prepare("SELECT id, conversation_id, role, content, timestamp, suggestions, model, knowledge_base_selection, sources, error, agent_task_id FROM messages WHERE conversation_id = ?1 ORDER BY timestamp ASC")?;
    let msg_iter = stmt.query_map(params![conversation_id], |row| {
        let agent_task_id: Option<String> = row.get(10)?;
        
        let agent_task = if agent_task_id.is_some() {
            Some(AgentTask::default())
        } else {
            None
        };

        Ok(ChatMessage {
            id: row.get(0)?,
            conversation_id: row.get(1)?,
            role: row.get(2)?,
            content: serde_json::from_str(&row.get::<_, String>(3)?).unwrap_or_default(),
            timestamp: row.get(4)?,
            suggestions: row.get::<_, Option<String>>(5)?.and_then(|s| serde_json::from_str(&s).ok()).flatten(),
            model: row.get(6)?,
            knowledge_base_selection: row.get(7)?,
            sources: row.get::<_, Option<String>>(8)?.and_then(|s| serde_json::from_str(&s).ok()).flatten(),
            error: row.get::<_, Option<String>>(9)?.and_then(|s| serde_json::from_str(&s).ok()).flatten(),
            agent_task_id: row.get(10)?,
            agent_task,
            is_executing: None,
            execution_output: None,
        })
    })?;
    msg_iter.collect::<rusqlite::Result<Vec<_>>>().map_err(Into::into)
}

pub fn list_conversations(conn: &Connection) -> Result<Vec<Conversation>> {
    let mut stmt = conn.prepare("SELECT id, title, created_at, session_type FROM conversations ORDER BY created_at DESC")?;
    let convo_iter = stmt.query_map([], |row| {
        Ok(Conversation {
            id: row.get(0)?,
            title: row.get(1)?,
            created_at: row.get(2)?,
            session_type: row.get(3)?,
            artifacts: vec![],
        })
    })?;
    convo_iter.collect::<rusqlite::Result<Vec<_>>>().map_err(Into::into)
}

pub fn get_conversation_by_id(conn: &Connection, id: &str) -> Result<Option<Conversation>> {
    conn.query_row(
        "SELECT id, title, created_at, session_type FROM conversations WHERE id = ?1",
        [id],
        |row| {
            Ok(Conversation {
                id: row.get(0)?,
                title: row.get(1)?,
                created_at: row.get(2)?,
                session_type: row.get(3)?,
                artifacts: vec![],
            })
        },
    ).optional().map_err(Into::into)
}

pub fn delete_conversation(conn: &mut Connection, conversation_id: &str) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute("DELETE FROM messages WHERE conversation_id = ?1", params![conversation_id])?;
    tx.execute("DELETE FROM creation_artifacts WHERE conversation_id = ?1", params![conversation_id])?;
    tx.execute("DELETE FROM conversations WHERE id = ?1", params![conversation_id])?;
    tx.commit()?;
    Ok(())
}

pub fn ensure_conversation_exists(conn: &Connection, id: &str, title: &str, session_type: &str) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO conversations (id, title, created_at, session_type) VALUES (?1, ?2, ?3, ?4)",
        params![id, title, chrono::Utc::now().timestamp_millis(), session_type],
    )?;
    Ok(())
}

pub fn update_conversation_title(conn: &Connection, id: &str, new_title: &str) -> Result<()> {
    conn.execute("UPDATE conversations SET title = ?2 WHERE id = ?1", params![id, new_title])?;
    Ok(())
}

pub fn delete_empty_conversations(conn: &Connection) -> Result<()> {
    let count = conn.execute(
        "DELETE FROM conversations WHERE title = 'New Chat...' AND id NOT IN (SELECT DISTINCT conversation_id FROM messages)",
        [],
    )?;
    if count > 0 {
        log::info!("Cleaned up {} empty, abandoned conversations.", count);
    }
    Ok(())
}

pub fn create_creation_artifact(conn: &Connection, artifact: &CreationArtifact) -> Result<()> {
    conn.execute(
        "INSERT INTO creation_artifacts (id, conversation_id, artifact_type, prompt, model_used, file_path, created_at, metadata) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            artifact.id,
            artifact.conversation_id,
            artifact.artifact_type,
            artifact.prompt,
            artifact.model_used,
            artifact.file_path,
            artifact.created_at,
            artifact.metadata,
        ],
    )?;
    Ok(())
}

pub fn get_artifacts_for_conversation(conn: &Connection, conversation_id: &str) -> Result<Vec<CreationArtifact>> {
    let mut stmt = conn.prepare("SELECT id, conversation_id, artifact_type, prompt, model_used, file_path, created_at, metadata FROM creation_artifacts WHERE conversation_id = ?1 ORDER BY created_at DESC")?;
    let artifact_iter = stmt.query_map(params![conversation_id], |row| {
        Ok(CreationArtifact {
            id: row.get(0)?,
            conversation_id: row.get(1)?,
            artifact_type: row.get(2)?,
            prompt: row.get(3)?,
            model_used: row.get(4)?,
            file_path: row.get(5)?,
            created_at: row.get(6)?,
            metadata: row.get(7)?,
        })
    })?;
    artifact_iter.collect::<rusqlite::Result<Vec<_>>>().map_err(Into::into)
}
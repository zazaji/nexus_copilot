// src-tauri/src/database/queries/agent_queries.rs
use crate::error::Result;
use rusqlite::{params, Connection};

pub fn link_agent_task_to_message(conn: &Connection, message_id: &str, agent_task_id: &str) -> Result<()> {
    conn.execute(
        "UPDATE messages SET agent_task_id = ?2 WHERE id = ?1",
        params![message_id, agent_task_id],
    )?;
    Ok(())
}
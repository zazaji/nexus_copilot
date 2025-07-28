use crate::database::models::*;
use crate::error::Result;
use rusqlite::{params, Connection, OptionalExtension};

pub fn list_online_kbs(conn: &Connection) -> Result<Vec<OnlineKnowledgeBase>> {
    let mut stmt = conn.prepare("SELECT id, name, url, token FROM online_knowledge_bases")?;
    let kb_iter = stmt.query_map([], |row| {
        Ok(OnlineKnowledgeBase {
            id: row.get(0)?,
            name: row.get(1)?,
            url: row.get(2)?,
            token: row.get(3)?,
        })
    })?;
    kb_iter.collect::<rusqlite::Result<Vec<_>>>().map_err(Into::into)
}

pub fn add_online_kb(conn: &Connection, kb: &OnlineKnowledgeBase) -> Result<()> {
    conn.execute(
        "INSERT INTO online_knowledge_bases (id, name, url, token) VALUES (?1, ?2, ?3, ?4)",
        params![&kb.id, &kb.name, &kb.url, &kb.token],
    )?;
    Ok(())
}

pub fn update_online_kb(conn: &Connection, kb: &OnlineKnowledgeBase) -> Result<()> {
    conn.execute(
        "UPDATE online_knowledge_bases SET name = ?2, url = ?3, token = ?4 WHERE id = ?1",
        params![&kb.id, &kb.name, &kb.url, &kb.token],
    )?;
    Ok(())
}

pub fn delete_online_kb(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM online_knowledge_bases WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn get_online_kb_by_id(conn: &Connection, id: &str) -> Result<Option<OnlineKnowledgeBase>> {
    conn.query_row(
        "SELECT id, name, url, token FROM online_knowledge_bases WHERE id = ?1",
        [id],
        |row| {
            Ok(OnlineKnowledgeBase {
                id: row.get(0)?,
                name: row.get(1)?,
                url: row.get(2)?,
                token: row.get(3)?,
            })
        },
    ).optional().map_err(Into::into)
}
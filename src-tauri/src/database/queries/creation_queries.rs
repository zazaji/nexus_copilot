// src-tauri/src/database/queries/creation_queries.rs
use crate::database::models::{CreationArtifact, CreationSession};
use crate::error::Result;
use rusqlite::{params, Connection, OptionalExtension};

pub fn create_creation_session(conn: &Connection, session: &CreationSession) -> Result<()> {
    conn.execute(
        "INSERT INTO creation_sessions (id, title, created_at) VALUES (?1, ?2, ?3)",
        params![session.id, session.title, session.created_at],
    )?;
    Ok(())
}

pub fn list_creation_sessions(conn: &Connection) -> Result<Vec<CreationSession>> {
    let mut stmt = conn.prepare("SELECT id, title, created_at FROM creation_sessions ORDER BY created_at DESC")?;
    let session_iter = stmt.query_map([], |row| {
        Ok(CreationSession {
            id: row.get(0)?,
            title: row.get(1)?,
            created_at: row.get(2)?,
            artifacts: vec![], // Artifacts are loaded separately
        })
    })?;
    session_iter.collect::<rusqlite::Result<Vec<_>>>().map_err(Into::into)
}

pub fn get_creation_session(conn: &Connection, session_id: &str) -> Result<Option<CreationSession>> {
    conn.query_row(
        "SELECT id, title, created_at FROM creation_sessions WHERE id = ?1",
        [session_id],
        |row| {
            Ok(CreationSession {
                id: row.get(0)?,
                title: row.get(1)?,
                created_at: row.get(2)?,
                artifacts: vec![], // Artifacts are loaded separately
            })
        },
    ).optional().map_err(Into::into)
}

pub fn create_creation_artifact(conn: &Connection, artifact: &CreationArtifact) -> Result<()> {
    conn.execute(
        "INSERT INTO creation_artifacts (id, session_id, artifact_type, prompt, model_used, file_path, created_at, metadata) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            artifact.id,
            artifact.session_id,
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

pub fn get_artifacts_for_session(conn: &Connection, session_id: &str) -> Result<Vec<CreationArtifact>> {
    let mut stmt = conn.prepare("SELECT id, session_id, artifact_type, prompt, model_used, file_path, created_at, metadata FROM creation_artifacts WHERE session_id = ?1 ORDER BY created_at DESC")?;
    let artifact_iter = stmt.query_map(params![session_id], |row| {
        Ok(CreationArtifact {
            id: row.get(0)?,
            session_id: row.get(1)?,
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
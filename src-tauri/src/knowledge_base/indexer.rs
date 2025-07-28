// src-tauri/src/knowledge_base/indexer.rs
use super::parser;
use crate::{
    database::queries,
    error::{AppError, Result},
    services::vector_client,
    state::AppState,
};
use serde_json::json;
use std::path::Path;
use tauri::{AppHandle, Manager};
use walkdir::WalkDir;

async fn process_file(state: &AppState, path: &Path) -> Result<()> {
    let content = parser::parse_file(path)?;
    if content.trim().is_empty() {
        return Ok(());
    }

    let (api_config, backend_url) = {
        let conn = state.db.lock().unwrap();
        let settings = queries::get_settings(&conn)?;
        (settings.api_config, settings.execution.backend_url)
    };

    let url = format!("{}/api/v1/knowledge_base/process-file", backend_url);
    let payload = json!({
        "file_path": path.to_string_lossy(),
        "content": content,
        "api_config": api_config,
    });

    let response = state.http_client.post(url).json(&payload).send().await?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown backend error".into());
        log::error!(
            "Backend failed to process file {}: {}",
            path.display(),
            error_text
        );
        return Err(AppError::Internal(format!(
            "Backend failed to process file: {}",
            error_text
        )));
    }

    Ok(())
}

pub async fn reindex_file(state: AppState, path_str: String) -> Result<()> {
    log::info!("Re-indexing single file: {}", path_str);
    let path = Path::new(&path_str);

    delete_documents_for_path(&state, path.to_str().unwrap()).await?;
    process_file(&state, &path).await?;

    log::info!("Finished re-indexing file: {}", path.display());
    Ok(())
}

pub async fn index_directory(app: AppHandle, state: AppState, path: String) -> Result<()> {
    log::info!("Starting indexing for path: {}", path);

    let root_path = Path::new(&path);
    let files_to_process: Vec<_> = WalkDir::new(root_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .collect();

    let total_files = files_to_process.len();
    for (i, file_path) in files_to_process.into_iter().enumerate() {
        let progress = ((i + 1) as f32 / total_files as f32) * 100.0;
        app.emit_all(
            "indexing-progress",
            serde_json::json!({
                "file": file_path.to_string_lossy(),
                "progress": progress,
            }),
        )?;

        if let Err(e) = process_file(&state, &file_path).await {
            log::error!("Failed to process file {}: {}", file_path.display(), e);
        }
    }

    log::info!("Finished indexing for path: {}", path);
    Ok(())
}

pub async fn delete_documents_for_path(state: &AppState, path: &str) -> Result<()> {
    log::info!("Deleting documents for path: {}", path);
    let backend_url = {
        let conn = state.db.lock().unwrap();
        queries::get_settings(&conn)?.execution.backend_url
    };

    let payload = vector_client::DeletePayload {
        base: vector_client::VectorBase { database: "nexus_db", collection: "knowledge_base" },
        where_metadata: json!({ "file_path": path }),
    };

    vector_client::delete(state, &backend_url, &payload).await?;
    log::info!("Successfully deleted documents for path: {}", path);
    Ok(())
}

pub async fn update_path_in_vector_db(state: &AppState, old_path: &str, new_path: &str) -> Result<()> {
    log::info!("Updating file path in vector DB from {} to {}", old_path, new_path);
    let backend_url = {
        let conn = state.db.lock().unwrap();
        queries::get_settings(&conn)?.execution.backend_url
    };
    let payload = vector_client::UpdateMetadataPayload {
        base: vector_client::VectorBase { database: "nexus_db", collection: "knowledge_base" },
        where_metadata: json!({ "file_path": old_path }),
        new_metadata: json!({ "file_path": new_path }),
    };
    vector_client::update_metadata(state, &backend_url, &payload).await
}

pub async fn clear_collection(state: &AppState) -> Result<()> {
    log::info!("Clearing entire knowledge base collection.");
    let backend_url = {
        let conn = state.db.lock().unwrap();
        queries::get_settings(&conn)?.execution.backend_url
    };
    vector_client::clear_collection(state, &backend_url).await
}
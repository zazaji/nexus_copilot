// src-tauri/src/commands/knowledge_base.rs
use crate::{database::{self, models, queries}, error::{AppError, Result}, knowledge_base, state::AppState};
use futures_util::StreamExt;
use reqwest_eventsource::{Event, EventSource};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tauri::{AppHandle, Manager, State};
use walkdir::WalkDir;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileNode {
    key: String,
    title: String,
    is_leaf: bool,
    children: Option<Vec<FileNode>>,
    file_type: Option<String>,
}

#[derive(Serialize, Debug)]
struct NoteForGraph {
    file_path: String,
    content: String,
    title: String,
}


#[tauri::command]
pub async fn find_file_in_kb(state: State<'_, AppState>, query: String) -> Result<Vec<String>> {
    let settings = queries::get_settings(&state.db.lock().unwrap())?;
    let indexed_dirs = settings.knowledge_base.indexed_directories;
    let mut found_files = Vec::new();
    let lower_query = query.to_lowercase();

    if lower_query.is_empty() {
        return Ok(found_files);
    }

    for dir in indexed_dirs {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.to_lowercase().contains(&lower_query) {
                        found_files.push(entry.path().to_string_lossy().to_string());
                    }
                }
            }
        }
    }
    found_files.truncate(20);
    Ok(found_files)
}


#[tauri::command]
pub async fn list_files_in_directory(path: String) -> Result<Vec<FileNode>> {
    fn build_tree(dir: &Path) -> Result<Vec<FileNode>> {
        let mut entries = Vec::new();
        if let Ok(read_dir) = fs::read_dir(dir) {
            for entry in read_dir.filter_map(|e| e.ok()) {
                let path = entry.path();
                let title = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                let key = path.to_string_lossy().to_string();

                if path.is_dir() {
                    let children = build_tree(&path)?;
                    entries.push(FileNode {
                        key,
                        title,
                        is_leaf: false,
                        children: Some(children),
                        file_type: None,
                    });
                } else if path.is_file() {
                    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
                    let file_type = match extension.to_lowercase().as_str() {
                        "md" | "txt" | "rs" | "py" | "html" | "css" | "json" | "toml" => Some("text".to_string()),
                        "pdf" => Some("pdf".to_string()),
                        "doc" | "docx" => Some("doc".to_string()),
                        "ppt" | "pptx" => Some("ppt".to_string()),
                        _ => None,
                    };

                    if file_type.is_some() {
                        entries.push(FileNode {
                            key,
                            title,
                            is_leaf: true,
                            children: None,
                            file_type,
                        });
                    }
                }
            }
        }
        // Sort directories first, then files, all alphabetically
        entries.sort_by(|a, b| {
            if a.is_leaf != b.is_leaf {
                a.is_leaf.cmp(&b.is_leaf)
            } else {
                a.title.cmp(&b.title)
            }
        });
        Ok(entries)
    }

    build_tree(Path::new(&path))
}

#[tauri::command]
pub async fn create_unique_file(dir_path: String) -> Result<String> {
    let base_name = "Untitled";
    let extension = ".md";
    let mut counter = 0;
    let mut new_path;

    loop {
        let file_name = if counter == 0 {
            format!("{}{}", base_name, extension)
        } else {
            format!("{}-{}{}", base_name, counter, extension)
        };
        new_path = Path::new(&dir_path).join(file_name);
        if !new_path.exists() {
            break;
        }
        counter += 1;
    }

    fs::File::create(&new_path)?;
    Ok(new_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn rename_file(state: State<'_, AppState>, old_path: String, new_path: String) -> Result<()> {
    if Path::new(&new_path).exists() {
        return Err(AppError::Internal(format!("File '{}' already exists.", new_path)));
    }

    fs::rename(&old_path, &new_path).map_err(|e| {
        AppError::Io(format!("Failed to rename file on disk: {}", e))
    })?;

    knowledge_base::indexer::update_path_in_vector_db(&state, &old_path, &new_path).await?;
    Ok(())
}

#[tauri::command]
pub async fn move_file(state: State<'_, AppState>, old_path: String, new_parent_dir: String) -> Result<()> {
    let old_path_obj = PathBuf::from(&old_path);
    let file_name = old_path_obj.file_name().ok_or_else(|| AppError::Internal("Could not get file name from old path".to_string()))?;

    let new_path = Path::new(&new_parent_dir).join(file_name);
    let new_path_str = new_path.to_string_lossy().to_string();

    if new_path.exists() {
        return Err(AppError::Internal(format!("A file with the same name already exists in the destination directory.")));
    }

    log::info!("Moving file from {} to {}", old_path, new_path_str);
    fs::rename(&old_path, &new_path)?;

    knowledge_base::indexer::update_path_in_vector_db(&state, &old_path, &new_path_str).await?;
    log::info!("Successfully moved file and updated vector index.");
    Ok(())
}


#[tauri::command]
pub async fn save_file_content(state: State<'_, AppState>, path: String, content: String) -> Result<()> {
    fs::write(&path, &content).map_err(|e| AppError::Io(e.to_string()))?;

    let state_clone = state.inner().clone();
    let path_clone = path.clone();
    let content_clone = content.clone();

    tokio::spawn(async move {
        if let Err(e) = knowledge_base::indexer::reindex_file(state_clone.clone(), path_clone.clone()).await {
            log::error!("Error during background re-indexing for {}: {}", path_clone, e);
        }

        let backend_url = match queries::get_settings(&state_clone.db.lock().unwrap()) {
            Ok(settings) => settings.execution.backend_url,
            Err(e) => {
                log::error!("Could not get backend URL to process note links: {}", e);
                return;
            }
        };

        let url = format!("{}/api/v1/knowledge_base/process_note_links", backend_url);
        
        let title = Path::new(&path_clone)
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let note_payload = NoteForGraph {
            file_path: path_clone.clone(),
            content: content_clone,
            title,
        };
        
        let payload = serde_json::json!({ "note": note_payload });

        match state_clone.http_client.post(url).json(&payload).timeout(Duration::from_secs(120)).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    log::info!("Successfully requested link processing for {}", path_clone);
                } else {
                    let status = response.status();
                    let text = response.text().await.unwrap_or_default();
                    log::error!("Failed to process note links for {}. Status: {}. Body: {}", path_clone, status, text);
                }
            }
            Err(e) => {
                log::error!("HTTP error when calling link processor for {}: {}", path_clone, e);
            }
        }
    });

    Ok(())
}

#[tauri::command]
pub async fn save_note_to_kb(state: State<'_, AppState>, dir_path: String, base_filename: String, content: String) -> Result<String> {
    let dir = Path::new(&dir_path);
    let stem = Path::new(&base_filename).file_stem().unwrap_or_default().to_string_lossy();
    let extension = Path::new(&base_filename).extension().unwrap_or_default().to_string_lossy();
    
    let mut counter = 0;
    let mut final_path;

    loop {
        let new_filename = if counter == 0 {
            base_filename.clone()
        } else {
            format!("{}-{}.{}", stem, counter, extension)
        };
        final_path = dir.join(&new_filename);
        if !final_path.exists() {
            break;
        }
        counter += 1;
    }

    // Reuse the logic from save_file_content
    save_file_content(state, final_path.to_string_lossy().to_string(), content).await?;

    Ok(final_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn read_file_content(path: String) -> Result<String> {
    log::info!("Attempting to read file content for path: {}", path);
    fs::read_to_string(path).map_err(|e| AppError::Io(e.to_string()))
}

#[tauri::command]
pub async fn start_indexing(app: AppHandle, state: State<'_, AppState>, path: String) -> Result<()> {
    let state_clone = state.inner().clone();
    tokio::spawn(async move {
        if let Err(e) = knowledge_base::indexer::index_directory(app, state_clone, path).await {
            log::error!("Error during indexing: {:?}", e);
        }
    });
    Ok(())
}

#[tauri::command]
pub async fn search_local_kb(
    state: State<'_, AppState>,
    query: String,
    top_k: u32,
    score_threshold: f32,
) -> Result<Vec<knowledge_base::models::KnowledgeSource>> {
    knowledge_base::searcher::search(&state, query, None, top_k, score_threshold).await
}

#[tauri::command]
pub async fn search_online_kb(
    state: State<'_, AppState>,
    kb_id: String,
    query: String,
    top_k: u32,
    score_threshold: f32,
) -> Result<Vec<knowledge_base::models::KnowledgeSource>> {
    knowledge_base::searcher::search_online(&state, &kb_id, &query, top_k, score_threshold).await
}

#[tauri::command]
pub async fn remove_indexed_directory(state: State<'_, AppState>, path: String) -> Result<()> {
    knowledge_base::indexer::delete_documents_for_path(&state, &path).await?;

    let conn = state.db.lock().unwrap();
    let mut settings = queries::get_settings(&conn)?;
    settings.knowledge_base.indexed_directories.retain(|p| p != &path);
    queries::save_settings(&conn, &settings)?;

    Ok(())
}

#[tauri::command]
pub async fn delete_file(state: State<'_, AppState>, path: String) -> Result<()> {
    log::info!("Attempting to delete file: {}", path);
    fs::remove_file(&path)?;
    knowledge_base::indexer::delete_documents_for_path(&state, &path).await?;
    log::info!("Successfully deleted file and its index: {}", path);
    Ok(())
}

#[tauri::command]
pub async fn clear_knowledge_base(state: State<'_, AppState>) -> Result<()> {
    knowledge_base::indexer::clear_collection(&state).await
}

#[tauri::command]
pub async fn rebuild_index(app: AppHandle, state: State<'_, AppState>, path: String) -> Result<()> {
    knowledge_base::indexer::delete_documents_for_path(&state, &path).await?;
    start_indexing(app, state, path).await
}

#[tauri::command]
pub async fn get_note_details(state: State<'_, AppState>, note_id: String) -> Result<Option<database::models::KnowledgeNote>> {
    let backend_url = queries::get_settings(&state.db.lock().unwrap())?.execution.backend_url;
    let url = format!("{}/api/v1/knowledge_base/notes/{}", backend_url, note_id);
    let response = state.http_client.get(url).send().await?.error_for_status()?;
    let note = response.json::<Option<database::models::KnowledgeNote>>().await?;
    Ok(note)
}

#[tauri::command]
pub async fn get_knowledge_graph_data(state: State<'_, AppState>) -> Result<database::models::KnowledgeGraphData> {
    let backend_url = queries::get_settings(&state.db.lock().unwrap())?.execution.backend_url;
    let url = format!("{}/api/v1/knowledge_base/graph-data", backend_url);
    let response = state.http_client.get(url).send().await?.error_for_status()?;
    let graph_data = response.json::<database::models::KnowledgeGraphData>().await?;
    Ok(graph_data)
}

#[tauri::command]
pub async fn rebuild_knowledge_graph(state: State<'_, AppState>) -> Result<()> {
    let settings = queries::get_settings(&state.db.lock().unwrap())?;
    let indexed_dirs = settings.knowledge_base.indexed_directories;
    let mut notes_for_graph = Vec::new();

    for dir in indexed_dirs {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                if let Ok(content) = fs::read_to_string(path) {
                    let title = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
                    notes_for_graph.push(NoteForGraph {
                        file_path: path.to_string_lossy().to_string(),
                        content,
                        title,
                    });
                }
            }
        }
    }

    let backend_url = settings.execution.backend_url;
    let url = format!("{}/api/v1/knowledge_base/process_all_note_links", backend_url);
    let payload = serde_json::json!({ "notes": notes_for_graph });

    let response = state.http_client.post(url).json(&payload).send().await?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown backend error".into());
        return Err(AppError::Internal(format!("Backend graph rebuild failed: {}", error_text)));
    }

    Ok(())
}

#[tauri::command]
pub async fn batch_convert_to_markdown(app_handle: AppHandle, state: State<'_, AppState>, input_dir: String, output_dir: String) -> Result<()> {
    let state_clone = state.inner().clone();
    tokio::spawn(async move {
        let backend_url = match queries::get_settings(&state_clone.db.lock().unwrap()) {
            Ok(settings) => settings.execution.backend_url,
            Err(e) => {
                log::error!("Could not get backend URL from settings: {}", e);
                let _ = app_handle.emit_all("conversion-progress", serde_json::json!({"progress": 100, "message": "Could not get backend URL from settings.", "error": true}));
                return;
            }
        };

        let url = format!("{}/api/v1/convert/to-markdown", backend_url);
        let request_body = serde_json::json!({
            "input_dir": input_dir,
            "output_dir": output_dir,
        });

        let request_builder = state_clone.http_client.post(url).json(&request_body);
        let mut es = match EventSource::new(request_builder) {
            Ok(es) => es,
            Err(e) => {
                log::error!("Failed to create EventSource for conversion: {}", e);
                let _ = app_handle.emit_all("conversion-progress", serde_json::json!({"progress": 100, "message": "Failed to connect to backend for conversion.", "error": true}));
                return;
            }
        };

        while let Some(event) = es.next().await {
            match event {
                Ok(Event::Message(message)) => {
                    if let Ok(payload) = serde_json::from_str::<serde_json::Value>(&message.data) {
                        if app_handle.emit_all("conversion-progress", payload).is_err() {
                            log::error!("Failed to emit conversion progress event.");
                            break;
                        }
                    }
                }
                Err(e) => {
                    log::error!("Error during conversion stream: {}", e);
                    let _ = app_handle.emit_all("conversion-progress", serde_json::json!({"progress": 100, "message": format!("Stream error: {}", e), "error": true}));
                    es.close();
                    break;
                }
                _ => {}
            }
        }
    });
    Ok(())
}

#[tauri::command]
pub async fn list_online_kbs(state: State<'_, AppState>) -> Result<Vec<models::OnlineKnowledgeBase>> {
    let conn = state.db.lock().unwrap();
    queries::list_online_kbs(&conn)
}

#[tauri::command]
pub async fn add_online_kb(state: State<'_, AppState>, kb: models::OnlineKnowledgeBase) -> Result<()> {
    let conn = state.db.lock().unwrap();
    queries::add_online_kb(&conn, &kb)
}

#[tauri::command]
pub async fn update_online_kb(state: State<'_, AppState>, kb: models::OnlineKnowledgeBase) -> Result<()> {
    let conn = state.db.lock().unwrap();
    queries::update_online_kb(&conn, &kb)
}

#[tauri::command]
pub async fn delete_online_kb(state: State<'_, AppState>, id: String) -> Result<()> {
    let conn = state.db.lock().unwrap();
    queries::delete_online_kb(&conn, &id)
}
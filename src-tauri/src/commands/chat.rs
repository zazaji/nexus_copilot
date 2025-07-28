// src-tauri/src/commands/chat.rs
use crate::{
    database::{models, queries},
    error::{AppError, Result},
    services,
    state::AppState,
};
use futures_util::StreamExt;
use serde_json::Value;
use std::fs;
use tauri::{AppHandle, Manager, State};
use uuid::Uuid;

fn get_extension_from_content_type(content_type: Option<&str>) -> &str {
    match content_type {
        Some(ct) if ct.contains("video/mp4") => "mp4",
        Some(ct) if ct.contains("video/webm") => "webm",
        Some(ct) if ct.contains("image/jpeg") => "jpg",
        Some(ct) if ct.contains("image/gif") => "gif",
        Some(ct) if ct.contains("audio/mpeg") => "mp3",
        Some(ct) if ct.contains("audio/wav") => "wav",
        Some(ct) if ct.starts_with("image/") => "png",
        Some(ct) if ct.starts_with("audio/") => "mp3",
        _ => "bin",
    }
}

#[tauri::command]
pub fn create_conversation(
    state: State<'_, AppState>,
    session_type: String,
    title: Option<String>,
) -> Result<models::Conversation> {
    let conn = state.db.lock().unwrap();
    let new_convo = models::Conversation {
        id: Uuid::new_v4().to_string(),
        title: title.unwrap_or_else(|| 
            if session_type == "creation" { "New Creation".to_string() } else { "New Chat...".to_string() }
        ),
        created_at: chrono::Utc::now().timestamp_millis(),
        session_type: session_type.clone(),
        artifacts: vec![],
    };

    if session_type == "creation" {
        let session_dir = state.context.app_data_dir.join("creations").join(&new_convo.id);
        fs::create_dir_all(&session_dir)?;
    }

    queries::ensure_conversation_exists(&conn, &new_convo.id, &new_convo.title, &new_convo.session_type)?;

    log::info!(
        "Successfully created new {} session with ID: {}",
        session_type, new_convo.id
    );
    Ok(new_convo)
}

#[tauri::command]
pub async fn generate_artifact(
    app_handle: AppHandle,
    state: State<'_, AppState>,
    conversation_id: String,
    creation_type: String,
    prompt: String,
    model_name: String,
    params: Value,
    provider: models::ApiProvider,
) -> Result<models::CreationArtifact> {
    let settings = queries::get_settings(&state.db.lock().unwrap())?;
    let backend_url = &settings.execution.backend_url;
    let url = format!("{}/api/v1/creations/generate", backend_url);

    let request_body = serde_json::json!({
        "creationType": creation_type,
        "prompt": prompt,
        "modelName": model_name,
        "params": params,
        "provider": provider,
    });

    let response = state.http_client.post(url).json(&request_body).send().await?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown backend error".to_string());
        return Err(AppError::ApiClient(format!("Backend generation failed: {}", error_text)));
    }

    let artifact_id = Uuid::new_v4().to_string();
    let final_file_path;
    let mut final_prompt = prompt.clone();
    let content_type_header;

    let response_content_type = response.headers()
        .get("content-type")
        .and_then(|value| value.to_str().ok());

    if response_content_type.map_or(false, |ct| ct.contains("application/json")) {
        let json_body: Value = response.json().await?;
        final_file_path = json_body["url"].as_str().unwrap_or_default().to_string();
        final_prompt = json_body["prompt"].as_str().unwrap_or(&prompt).to_string();
        content_type_header = json_body["content_type"].as_str().map(|s| s.to_string());
    } else if response_content_type.map_or(false, |ct| ct.contains("text/event-stream")) {
        // Handle SSE for video
        let mut stream = response.bytes_stream();
        let mut final_payload: Option<Value> = None;

        while let Some(item) = stream.next().await {
            let chunk = item?;
            let lines = String::from_utf8_lossy(&chunk);
            for line in lines.lines() {
                if line.starts_with("data:") {
                    let data_str = line.strip_prefix("data:").unwrap().trim();
                    if let Ok(data_json) = serde_json::from_str::<Value>(data_str) {
                        if data_json.get("status").and_then(Value::as_str) == Some("SUCCESS") {
                            final_payload = Some(data_json.clone());
                        }
                        app_handle.emit_all("artifact-progress", data_json)?;
                    }
                }
            }
        }

        let final_data = final_payload.ok_or_else(|| AppError::Internal("Video stream ended without a success payload.".to_string()))?;
        final_file_path = final_data["url"].as_str().unwrap_or_default().to_string();
        final_prompt = final_data["prompt"].as_str().unwrap_or(&prompt).to_string();
        content_type_header = final_data["content_type"].as_str().map(|s| s.to_string());
    } else {
        // Handle direct binary stream (e.g., audio)
        content_type_header = response_content_type.map(|s| s.to_string());
        let content = response.bytes().await?;
        let extension = get_extension_from_content_type(content_type_header.as_deref());
        let file_name = format!("{}.{}", artifact_id, extension);
        let session_dir = state.context.app_data_dir.join("creations").join(&conversation_id);
        let file_path_buf = session_dir.join(&file_name);
        fs::write(&file_path_buf, &content)?;
        final_file_path = file_path_buf.to_string_lossy().to_string();
    }

    let model_used = format!("{}::{}", provider.id, model_name);

    let new_artifact = models::CreationArtifact {
        id: artifact_id,
        conversation_id: conversation_id.clone(),
        artifact_type: creation_type,
        prompt: final_prompt,
        model_used,
        file_path: final_file_path,
        created_at: chrono::Utc::now().timestamp_millis(),
        metadata: content_type_header,
    };

    let conn = state.db.lock().unwrap();
    queries::create_creation_artifact(&conn, &new_artifact)?;

    app_handle.emit_all("artifact-created", &new_artifact)?;

    Ok(new_artifact)
}


#[tauri::command]
pub async fn process_chat_message(
    app: AppHandle,
    state: State<'_, AppState>,
    user_message: models::ChatMessage,
    ai_message_id: String,
    mut api_config: models::ApiConfig,
) -> Result<()> {
    log::info!(
        "Received user message {} for conversation {}, AI placeholder ID {}",
        user_message.id,
        user_message.conversation_id,
        ai_message_id
    );
    
    let online_kbs = queries::list_online_kbs(&state.db.lock().unwrap())?;
    api_config.online_kbs = Some(online_kbs);

    let state_clone = state.inner().clone();
    tokio::spawn(async move {
        if let Err(e) = services::chat::handle_message(app, state_clone, user_message, ai_message_id, api_config).await {
            log::error!("Error handling chat message: {}", e);
        }
    });
    Ok(())
}

#[tauri::command]
pub async fn stop_chat_generation(state: State<'_, AppState>, message_id: String) -> Result<()> {
    if let Some(flag) = state.running_chat_tasks.lock().unwrap().get(&message_id) {
        flag.store(true, std::sync::atomic::Ordering::SeqCst);
        log::info!("Stop flag set for message ID: {}", message_id);
    } else {
        log::warn!("Could not find running task to stop for message ID: {}", message_id);
    }
    Ok(())
}

#[tauri::command]
pub fn save_message(state: State<'_, AppState>, message: models::ChatMessage) -> Result<()> {
    let conn = state.db.lock().unwrap();
    queries::save_message(&conn, &message)
}

#[tauri::command]
pub fn link_agent_task_to_message(state: State<'_, AppState>, message_id: String, agent_task_id: String) -> Result<()> {
    let conn = state.db.lock().unwrap();
    queries::link_agent_task_to_message(&conn, &message_id, &agent_task_id)
}

#[tauri::command]
pub fn delete_message(state: State<'_, AppState>, message_id: String) -> Result<()> {
    let conn = state.db.lock().unwrap();
    queries::delete_message(&conn, &message_id)
}

#[tauri::command]
pub async fn generate_title_for_conversation(
    state: State<'_, AppState>,
    conversation_id: String,
    user_query: String,
    ai_response: String,
) -> Result<models::Conversation> {
    let title =
        crate::services::chat::generate_title_for_conversation(&state, &user_query, &ai_response)
            .await?;

    let convo = {
        let conn = state.db.lock().unwrap();
        queries::update_conversation_title(&conn, &conversation_id, &title)?;
        queries::get_conversation_by_id(&conn, &conversation_id)?
            .ok_or_else(|| AppError::Internal("Failed to fetch updated conversation".to_string()))?
    };

    Ok(convo)
}

#[tauri::command]
pub fn get_conversation_history(
    state: State<'_, AppState>,
    conversation_id: String,
) -> Result<Vec<models::ChatMessage>> {
    let conn = state.db.lock().unwrap();
    queries::get_conversation_history(&conn, &conversation_id)
}

#[tauri::command]
pub fn list_conversations(state: State<'_, AppState>) -> Result<Vec<models::Conversation>> {
    let conn = state.db.lock().unwrap();
    queries::list_conversations(&conn)
}

#[tauri::command]
pub fn get_conversation_with_artifacts(state: State<'_, AppState>, conversation_id: String) -> Result<models::Conversation> {
    let conn = state.db.lock().unwrap();
    let mut convo = queries::get_conversation_by_id(&conn, &conversation_id)?
        .ok_or_else(|| AppError::Database("Conversation not found".to_string()))?;
    
    if convo.session_type == "creation" {
        convo.artifacts = queries::get_artifacts_for_conversation(&conn, &conversation_id)?;
    }
    Ok(convo)
}


#[tauri::command]
pub fn delete_conversation(state: State<'_, AppState>, conversation_id: String) -> Result<()> {
    let mut conn = state.db.lock().unwrap();
    queries::delete_conversation(&mut conn, &conversation_id)
}

#[tauri::command]
pub fn clear_all_conversations(state: State<'_, AppState>) -> Result<()> {
    let mut conn = state.db.lock().unwrap();

    let tx = conn
        .transaction()
        .map_err(|e| AppError::Database(e.to_string()))?;

    tx.execute("DELETE FROM messages", [])?;
    tx.execute("DELETE FROM creation_artifacts", [])?;
    tx.execute("DELETE FROM conversations", [])?;

    tx.commit()
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(())
}

#[tauri::command]
pub fn update_conversation_title(
    app: AppHandle,
    state: State<'_, AppState>,
    conversation_id: String,
    new_title: String,
) -> Result<()> {
    let conn = state.db.lock().unwrap();
    queries::update_conversation_title(&conn, &conversation_id, &new_title)?;

    if let Ok(Some(convo)) = queries::get_conversation_by_id(&conn, &conversation_id) {
        app.emit_all("conversation-title-updated", convo)?;
    }

    Ok(())
}

#[tauri::command]
pub fn update_message_content(
    state: State<'_, AppState>,
    message_id: String,
    new_content: String,
) -> Result<()> {
    let conn = state.db.lock().unwrap();
    let content_parts = vec![models::ChatMessageContentPart::Text { text: new_content }];
    let content_json = serde_json::to_string(&content_parts)?;
    queries::update_message_content(&conn, &message_id, &content_json)
}
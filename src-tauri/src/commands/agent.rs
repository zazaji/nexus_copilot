// src-tauri/src/commands/agent.rs
use crate::{
    database::{models, queries},
    error::{AppError, Result},
    state::AppState,
};
use serde_json::{json, Value};
use tauri::State;
use std::time::Duration;

#[tauri::command]
pub async fn process_agentic_instruction(
    state: State<'_, AppState>,
    conversation_id: String,
    instruction: String,
    mut api_config: models::ApiConfig,
    mode: String,
    knowledge_base_selection: String,
) -> Result<serde_json::Value> {
    let settings = queries::get_settings(&state.db.lock().unwrap())?;
    let backend_url = &settings.execution.backend_url;
    let url = format!("{}/api/v1/agent/start-task", backend_url);

    // Inject online KBs and appearance into the config before sending to backend
    let online_kbs = queries::list_online_kbs(&state.db.lock().unwrap())?;
    api_config.online_kbs = Some(online_kbs);
    api_config.appearance = Some(settings.appearance);

    let request_body = json!({
        "goal": instruction,
        "api_config": api_config,
        "conversation_id": conversation_id,
        "mode": mode,
        "knowledge_base_selection": knowledge_base_selection,
    });

    let response = state.http_client
        .post(url)
        .json(&request_body)
        .timeout(Duration::from_secs(15))
        .send()
        .await
        .map_err(|e| {
            log::error!("Agent start request failed: {:?}", e);
            AppError::ApiClient(format!("Failed to connect to backend to start agent task: {}", e))
        })?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown backend error".to_string());
        log::error!("Backend returned an error on agent start ({}): {}", status, error_text);
        return Err(AppError::ApiClient(format!("Backend returned an error ({}): {}", status, error_text)));
    }

    let response_data = response.json::<serde_json::Value>().await
        .map_err(|e| {
            log::error!("Failed to parse agent start response: {:?}", e);
            AppError::ApiClient(format!("Failed to parse backend response: {}", e))
        })?;

    Ok(response_data)
}

#[tauri::command]
pub async fn stop_agent_task(state: State<'_, AppState>, task_id: String) -> Result<()> {
    let settings = queries::get_settings(&state.db.lock().unwrap())?;
    let backend_url = &settings.execution.backend_url;
    let url = format!("{}/api/v1/agent/stop-task/{}", backend_url, task_id);

    state.http_client
        .post(url)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

#[tauri::command]
pub async fn restart_agent_task(state: State<'_, AppState>, task_id: String) -> Result<()> {
    let settings = queries::get_settings(&state.db.lock().unwrap())?;
    let backend_url = &settings.execution.backend_url;
    let url = format!("{}/api/v1/agent/restart-task", backend_url);

    state.http_client
        .post(url)
        .json(&json!({ "task_id": task_id }))
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

#[tauri::command]
pub async fn resume_write_task(state: State<'_, AppState>, task_id: String, elaboration: Value, plan: Value) -> Result<()> {
    let settings = queries::get_settings(&state.db.lock().unwrap())?;
    let backend_url = &settings.execution.backend_url;
    let url = format!("{}/api/v1/agent/resume-write-task", backend_url);

    state.http_client
        .post(url)
        .json(&json!({ "task_id": task_id, "elaboration": elaboration, "plan": plan }))
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

#[tauri::command]
pub async fn get_task_status(state: State<'_, AppState>, task_id: String) -> Result<String> {
    let settings = queries::get_settings(&state.db.lock().unwrap())?;
    let backend_url = &settings.execution.backend_url;
    let url = format!("{}/api/v1/agent/get-task-status/{}", backend_url, task_id);

    let response = state.http_client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    Ok(response)
}

#[tauri::command]
pub async fn generate_research_node_content(state: State<'_, AppState>, task_id: String, node_id: String) -> Result<()> {
    let settings = queries::get_settings(&state.db.lock().unwrap())?;
    let backend_url = &settings.execution.backend_url;
    let url = format!("{}/api/v1/agent/generate-node-content", backend_url);

    let request_body = json!({
        "task_id": task_id,
        "node_id": node_id,
    });

    state.http_client
        .post(url)
        .json(&request_body)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

#[tauri::command]
pub async fn refine_agent_task_section(state: State<'_, AppState>, task_id: String, node_id: String, prompt: String, model: String, is_manual: bool) -> Result<()> {
    let settings = queries::get_settings(&state.db.lock().unwrap())?;
    let backend_url = &settings.execution.backend_url;
    let url = format!("{}/api/v1/agent/refine-section", backend_url);

    let request_body = json!({
        "task_id": task_id,
        "node_id": node_id,
        "prompt": prompt,
        "model": model,
        "is_manual": is_manual,
    });

    state.http_client
        .post(url)
        .json(&request_body)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}
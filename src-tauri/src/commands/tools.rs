// src-tauri/src/commands/tools.rs
use crate::{
    database::{models, queries},
    error::{AppError, Result},
    services,
    state::AppState,
};
use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::path::Path;
use tauri::{AppHandle, State as TauriState};

#[derive(Debug, Serialize, Clone)]
pub struct DynamicTool {
    pub id: String,
    pub name: String,
    pub description: String,
    pub script_path: String,
    pub runtime: models::ToolRuntime,
}

#[tauri::command]
pub async fn list_tools(state: TauriState<'_, AppState>) -> Result<Vec<DynamicTool>> {
    services::tools::list_dynamic_tools(&state).await
}

#[tauri::command]
pub async fn list_configured_tools(state: TauriState<'_, AppState>) -> Result<Vec<models::ConfiguredTool>> {
    let conn = state.db.lock().unwrap();
    queries::list_configured_tools(&conn)
}

#[tauri::command]
pub async fn save_configured_tool(state: TauriState<'_, AppState>, tool: models::ConfiguredTool) -> Result<()> {
    let conn = state.db.lock().unwrap();
    queries::save_configured_tool(&conn, &tool)
}

#[tauri::command]
pub async fn delete_configured_tool(state: TauriState<'_, AppState>, id: String) -> Result<()> {
    let conn = state.db.lock().unwrap();
    queries::delete_configured_tool(&conn, &id)
}

#[tauri::command]
pub async fn execute_tool(app: AppHandle, state: TauriState<'_, AppState>, payload: Value) -> Result<String> {
    log::info!("[Command] execute_tool called with payload: {:?}", payload);
    services::tools::execute_tool_from_payload(app, &state, payload).await
}

#[tauri::command]
pub async fn execute_python_code(app: AppHandle, state: TauriState<'_, AppState>, execution_id: String, code: String) -> Result<()> {
    services::tools::execute_python_code(app, state.inner().clone(), execution_id, code).await
}

#[tauri::command]
pub async fn save_tool_script(state: TauriState<'_, AppState>, script_name: String, script_content: String) -> Result<String> {
    let settings = queries::get_settings(&state.db.lock().unwrap())?;
    let scripts_dir = settings.knowledge_base.scripts_directories.get(0)
        .ok_or_else(|| AppError::Config("No script directory configured in settings.".to_string()))?;

    let file_path = Path::new(scripts_dir).join(script_name);

    fs::write(&file_path, script_content)?;

    Ok(file_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn execute_shell_command(app: AppHandle, command: String) -> Result<String> {
    services::tools::execute_shell_command_service(app, command).await
}

#[tauri::command]
pub async fn execute_generic_code(
    app: AppHandle,
    state: TauriState<'_, AppState>,
    runtime: models::ToolRuntime,
    code: String,
) -> Result<String> {
    log::info!("[Command] execute_generic_code called for runtime: {:?}", runtime);
    services::tools::execute_generic_code(app, state.inner().clone(), runtime, code).await
}

#[tauri::command]
pub async fn setup_task_workspace(state: TauriState<'_, AppState>, task_id: String) -> Result<String> {
    let tasks_dir = state.context.app_data_dir.join("tasks");
    let task_dir = tasks_dir.join(&task_id);
    fs::create_dir_all(&task_dir)?;
    
    let mut working_dir_guard = state.current_task_working_dir.lock().unwrap();
    *working_dir_guard = Some(task_dir.clone());

    Ok(task_dir.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn write_file_to_task_dir(state: TauriState<'_, AppState>, filename: String, content: String) -> Result<()> {
    let working_dir_guard = state.current_task_working_dir.lock().unwrap();
    let task_dir = working_dir_guard.as_ref()
        .ok_or_else(|| AppError::Internal("Task workspace is not set up.".to_string()))?;

    let file_path = task_dir.join(filename);
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    fs::write(file_path, content)?;
    Ok(())
}
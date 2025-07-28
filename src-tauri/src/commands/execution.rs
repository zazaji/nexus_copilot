// src-tauri/src/commands/execution.rs
use crate::{error::Result, services, state::AppState};
use serde::Serialize;
use tauri::{AppHandle, State};

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BackendStatusResponse {
    pub status: String,
}

#[tauri::command]
pub async fn check_backend_status(
    app_handle: AppHandle,
    state: State<'_, AppState>,
    python_path: String,
) -> Result<BackendStatusResponse> {
    let status = services::execution::check_status(&app_handle, &state, &python_path).await?;
    Ok(BackendStatusResponse { status })
}

#[tauri::command]
pub async fn install_backend_service(
    app_handle: AppHandle,
    state: State<'_, AppState>,
) -> Result<()> {
    let state_clone = state.inner().clone();
    tokio::spawn(async move {
        if let Err(e) = services::execution::install_service(&app_handle, &state_clone).await {
            log::error!("Failed to install backend service: {}", e);
        }
    });
    Ok(())
}

#[tauri::command]
pub async fn start_backend_service(
    app_handle: AppHandle,
    state: State<'_, AppState>,
    python_path: String,
) -> Result<()> {
    services::execution::start_service(&app_handle, &state, &python_path).await
}

#[tauri::command]
pub async fn stop_backend_service(state: State<'_, AppState>) -> Result<()> {
    services::execution::stop_service(&state).await
}
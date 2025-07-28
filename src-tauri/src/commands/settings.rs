use crate::{database::{models, queries}, error::Result, services, state::AppState};
use tauri::{AppHandle, State};

#[tauri::command]
pub async fn get_user_settings(state: State<'_, AppState>) -> Result<models::Settings> {
    let conn = state.db.lock().map_err(|e| {
        crate::error::AppError::Internal(format!("Failed to acquire database lock: {}", e))
    })?;

    queries::get_settings(&conn)
}

#[tauri::command]
pub async fn update_user_settings(
    app_handle: AppHandle,
    state: State<'_, AppState>,
    settings: models::Settings
) -> Result<()> {
    let conn = state.db.lock().map_err(|e| {
        crate::error::AppError::Internal(format!("Failed to acquire database lock: {}", e))
    })?;

    queries::save_settings(&conn, &settings)?;

    // After saving, immediately update the global shortcuts
    services::shortcuts::update_global_shortcuts(&app_handle, &settings)
        .map_err(|e| crate::error::AppError::Internal(format!("Failed to update shortcuts: {}", e)))?;

    Ok(())
}
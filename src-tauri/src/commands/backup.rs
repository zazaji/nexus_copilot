use crate::{
    database::queries,
    error::Result,
    state::AppState,
};
use std::{fs, io::Write};
use tauri::State;

#[tauri::command]
pub async fn export_data(state: State<'_, AppState>, file_path: String) -> Result<()> {
    log::info!("Requesting data export from Python backend...");

    let backend_url = {
        let conn = state.db.lock().unwrap();
        queries::get_settings(&conn)?.execution.backend_url
    };
    let export_url = format!("{}/api/v1/backup/export", backend_url);

    let response = state.http_client.get(export_url).send().await?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown backend error".into());
        return Err(crate::error::AppError::Internal(format!("Backend export failed: {}", error_text)));
    }

    let backup_data = response.bytes().await?;

    let mut file = fs::File::create(file_path)?;
    file.write_all(&backup_data)?;

    log::info!("Data export completed successfully.");
    Ok(())
}

#[tauri::command]
pub async fn import_data(state: State<'_, AppState>, file_path: String) -> Result<()> {
    log::info!("Sending data import request to Python backend...");

    let backup_data = fs::read(file_path)?;

    let backend_url = {
        let conn = state.db.lock().unwrap();
        queries::get_settings(&conn)?.execution.backend_url
    };
    let import_url = format!("{}/api/v1/backup/import", backend_url);

    let response = state.http_client
        .post(import_url)
        .header("Content-Type", "application/json")
        .body(backup_data)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown backend error".into());
        return Err(crate::error::AppError::Internal(format!("Backend import failed: {}", error_text)));
    }

    log::info!("Data import request processed by backend successfully.");
    Ok(())
}
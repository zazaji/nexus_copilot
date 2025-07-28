// src-tauri/src/commands/assets.rs
use crate::error::{AppError, Result};
use base64::{engine::general_purpose, Engine as _};
use std::fs;
use tauri::{api::path::app_data_dir, AppHandle};

#[tauri::command]
pub async fn save_temp_asset_with_hash(app_handle: AppHandle, base64_data: String, hash: String, extension: String) -> Result<String> {
    let data_dir = app_data_dir(&app_handle.config())
        .ok_or_else(|| AppError::Internal("Could not get app data directory".to_string()))?;
    
    let assets_dir = data_dir.join("temp_assets");
    if !assets_dir.exists() {
        fs::create_dir_all(&assets_dir)?;
    }

    let filename = format!("{}.{}", hash, extension);
    let file_path = assets_dir.join(&filename);

    if file_path.exists() {
        log::info!("Asset with hash {} already exists. Skipping save.", hash);
        return Ok(file_path.to_string_lossy().to_string());
    }

    let image_bytes = general_purpose::STANDARD
        .decode(base64_data)
        .map_err(|e| AppError::Internal(format!("Failed to decode base64 image: {}", e)))?;
    
    fs::write(&file_path, image_bytes)?;

    Ok(file_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn read_file_as_base64(path: String) -> Result<String> {
    let file_bytes = fs::read(path)?;
    Ok(general_purpose::STANDARD.encode(&file_bytes))
}
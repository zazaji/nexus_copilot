// src-tauri/src/commands/app_info.rs
use crate::error::{AppError, Result};
use serde::Serialize;
use std::fs;
use tauri::PackageInfo;

#[derive(Serialize)]
pub struct AppInfo {
    name: String,
    version: String,
}

#[tauri::command]
pub async fn get_app_info(package_info: tauri::State<'_, PackageInfo>) -> Result<AppInfo> {
    Ok(AppInfo {
        name: package_info.name.clone(),
        version: package_info.version.to_string(),
    })
}

#[tauri::command]
pub async fn get_readme_content() -> Result<String> {
    // Assuming README.md is in the project root, relative to the executable in dev and bundled in prod.
    // For robustness, we check a few locations.
    let possible_paths = ["./README.md", "../README.md", "../../README.md"];
    for path in possible_paths {
        if let Ok(content) = fs::read_to_string(path) {
            return Ok(content);
        }
    }
    Err(AppError::Io(
        "README.md not found. Please ensure it is in the project root.".to_string(),
    ))
}
// src-tauri/src/system/fs.rs
use crate::error::{AppError, Result};
use once_cell::sync::Lazy;
use std::path::Path;
use std::process::Command;
use tokio::sync::Mutex;

static DIALOG_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

pub async fn pick_directory() -> Result<Option<String>> {
    let _guard = DIALOG_MUTEX.lock().await;
    let path = tauri::api::dialog::blocking::FileDialogBuilder::new().pick_folder();
    Ok(path.map(|p| p.to_string_lossy().to_string()))
}

pub fn show_in_folder(path_str: String) -> Result<()> {
    let path = Path::new(&path_str);

    #[cfg(target_os = "windows")]
    {
        if path.is_dir() {
            Command::new("explorer")
                .arg(&path_str)
                .spawn()
                .map_err(|e| AppError::Io(e.to_string()))?;
        } else {
            Command::new("explorer")
                .args(["/select,", &path_str]) // The comma after select is not a typo.
                .spawn()
                .map_err(|e| AppError::Io(e.to_string()))?;
        }
    }

    #[cfg(target_os = "macos")]
    {
        if path.is_dir() {
            Command::new("open")
                .arg(&path_str)
                .spawn()
                .map_err(|e| AppError::Io(e.to_string()))?;
        } else {
            Command::new("open")
                .args(["-R", &path_str])
                .spawn()
                .map_err(|e| AppError::Io(e.to_string()))?;
        }
    }

    #[cfg(target_os = "linux")]
    {
        if path.is_dir() {
            Command::new("xdg-open")
                .arg(&path_str)
                .spawn()
                .map_err(|e| AppError::Io(e.to_string()))?;
        } else {
            if let Some(parent) = path.parent() {
                Command::new("xdg-open")
                    .arg(parent)
                    .spawn()
                    .map_err(|e| AppError::Io(e.to_string()))?;
            } else {
                return Err(AppError::Internal(format!("Could not get parent directory of {}", path_str)));
            }
        }
    }

    Ok(())
}
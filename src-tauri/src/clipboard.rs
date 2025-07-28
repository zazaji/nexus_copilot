// src-tauri/src/system/clipboard.rs
use crate::commands::system::ClipboardPayload;
use crate::error::{AppError, Result};
use arboard::Clipboard;
use std::{fs, path::PathBuf};
use tauri::{api::path::cache_dir, Manager};
use uuid::Uuid;

pub async fn read_clipboard() -> Result<Option<ClipboardPayload>> {
    let mut clipboard = Clipboard::new().map_err(|e| AppError::Internal(e.to_string()))?;
    
    if let Ok(text) = clipboard.get_text() {
        if !text.trim().is_empty() {
            return Ok(Some(ClipboardPayload {
                payload_type: "text".to_string(),
                content: text,
            }));
        }
    }

    if let Ok(image_data) = clipboard.get_image() {
        let cache_path = cache_dir().ok_or_else(|| AppError::Internal("Could not get cache directory".to_string()))?;
        if !cache_path.exists() {
            fs::create_dir_all(&cache_path)?;
        }
        let temp_file_name = format!("{}.png", Uuid::new_v4());
        let temp_file_path: PathBuf = cache_path.join(temp_file_name);

        image::save_buffer(
            &temp_file_path,
            &image_data.bytes,
            image_data.width as u32,
            image_data.height as u32,
            image::ColorType::Rgba8,
        ).map_err(|e| AppError::Internal(format!("Failed to save clipboard image: {}", e)))?;

        // Correctly map the error from `from_file_path`
        let file_url = tauri::Url::from_file_path(&temp_file_path)
            .map_err(|_| AppError::Internal("Could not convert path to URL".to_string()))?
            .to_string();

        return Ok(Some(ClipboardPayload {
            payload_type: "image".to_string(),
            content: file_url,
        }));
    }
    
    Ok(None)
}

pub async fn start_clipboard_monitor(app: tauri::AppHandle) {
    tokio::spawn(async move {
        let mut clipboard = match Clipboard::new() {
            Ok(cb) => cb,
            Err(e) => {
                log::error!("Failed to initialize clipboard: {}", e);
                return;
            }
        };
        
        let mut last_text = clipboard.get_text().unwrap_or_default();

        loop {
            if let Ok(current_text) = clipboard.get_text() {
                if !current_text.is_empty() && current_text != last_text {
                    log::info!("System clipboard updated with new text. Broadcasting 'clipboard-updated' event globally.");
                    
                    let payload = ClipboardPayload {
                        payload_type: "text".to_string(),
                        content: current_text.clone(),
                    };

                    if let Err(e) = app.emit_all("clipboard-updated", payload) {
                        log::error!("Failed to emit clipboard-updated event: {}", e);
                    }
                    
                    last_text = current_text;
                }
            }
            
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    });
}
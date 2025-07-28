// src-tauri/src/system/clipboard.rs
use crate::{
    commands::system::ClipboardPayload,
    database::{models, queries},
    error::{AppError, Result},
    state::AppState,
};
use arboard::{Clipboard, ImageData};
use base64::{engine::general_purpose, Engine as _};
use image::codecs::png::PngEncoder;
use image::ImageEncoder;
use serde_json::json;
use std::{
    fs,
    io::Cursor,
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};
use tauri::{AppHandle, Manager};
use tokio::time::Instant;
use uuid::Uuid;

fn image_to_base64_uri(image_data: &ImageData) -> Result<String> {
    let mut image_buffer: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut image_buffer);

    PngEncoder::new(&mut cursor)
        .write_image(
            &image_data.bytes,
            image_data.width as u32,
            image_data.height as u32,
            image::ColorType::Rgba8.into(),
        )
        .map_err(|e| AppError::Internal(format!("Failed to encode image to PNG: {}", e)))?;

    let base64_str = general_purpose::STANDARD.encode(&image_buffer);
    Ok(format!("data:image/png;base64,{}", base64_str))
}

pub async fn read_clipboard(_app_handle: &AppHandle) -> Result<Option<ClipboardPayload>> {
    let mut clipboard = Clipboard::new().map_err(|e| AppError::Internal(e.to_string()))?;

    if let Ok(text) = clipboard.get_text() {
        if !text.trim().is_empty() {
            let lines: Vec<&str> = text.lines().map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
            
            // Check for single image file path
            if lines.len() == 1 {
                let path = Path::new(lines[0]);
                if path.is_file() {
                    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
                    if ["png", "jpg", "jpeg", "webp", "gif"].contains(&extension.as_str()) {
                        let image_bytes = fs::read(path)?;
                        let base64_str = general_purpose::STANDARD.encode(&image_bytes);
                        let data_url = format!("data:image/{};base64,{}", extension, base64_str);
                        return Ok(Some(ClipboardPayload {
                            content_type: "image".to_string(),
                            content: json!(data_url),
                        }));
                    }
                }
            }

            // Check for list of files (potentially images)
            let mut file_paths = Vec::new();
            let is_potential_file_list = !lines.is_empty() && lines.iter().any(|line| Path::new(line).exists());
            if is_potential_file_list {
                 for line in &lines {
                    let path = Path::new(line);
                    if path.is_file() {
                        file_paths.push(line.to_string());
                    } else {
                        // If any line is not a file, treat the whole thing as text
                        file_paths.clear();
                        break;
                    }
                }
                if !file_paths.is_empty() {
                     return Ok(Some(ClipboardPayload {
                        content_type: "files".to_string(),
                        content: json!(file_paths),
                    }));
                }
            }
            
            // Fallback to plain text
            return Ok(Some(ClipboardPayload {
                content_type: "text".to_string(),
                content: json!(text),
            }));
        }
    }

    if let Ok(image_data) = clipboard.get_image() {
        let base64_uri = image_to_base64_uri(&image_data)?;
        return Ok(Some(ClipboardPayload {
            content_type: "image".to_string(),
            content: json!(base64_uri),
        }));
    }

    Ok(None)
}

pub async fn start_clipboard_monitor(app: AppHandle) {
    tokio::spawn(async move {
        let mut last_content_json = json!(null);
        let last_update_time = Arc::new(Mutex::new(Instant::now()));

        loop {
            tokio::time::sleep(Duration::from_millis(500)).await;

            if let Ok(Some(payload)) = read_clipboard(&app).await {
                if payload.content != last_content_json {
                    last_content_json = payload.content.clone();
                    handle_clipboard_update(&app, payload, &last_update_time).await;
                }
            } else {
                last_content_json = json!(null);
            }
        }
    });
}

async fn handle_clipboard_update(app: &AppHandle, payload: ClipboardPayload, last_update_time: &Arc<Mutex<Instant>>) {
    let app_clone = app.clone();
    let last_update_time_clone = last_update_time.clone();

    let app_state: tauri::State<AppState> = app.state();
    let new_item = models::ClipboardHistoryItem {
        id: Uuid::new_v4().to_string(),
        item_type: payload.content_type.clone(),
        content: payload.content.to_string(),
        timestamp: chrono::Utc::now().timestamp_millis(),
        is_pinned: false,
    };
    if let Ok(conn) = app_state.db.lock() {
        if let Err(e) = queries::add_clipboard_item(&conn, &new_item) {
            log::error!("Failed to save clipboard item to DB: {}", e);
        }
    }

    if let Err(e) = app.emit_all("clipboard-item-added", new_item) {
        log::error!("Failed to emit clipboard-item-added event: {}", e);
    }

    tokio::spawn(async move {
        let trigger_time = Instant::now();
        {
            let mut last_update = last_update_time_clone.lock().unwrap();
            *last_update = trigger_time;
        }

        tokio::time::sleep(Duration::from_millis(350)).await;

        let should_trigger = {
            let last_update = last_update_time_clone.lock().unwrap();
            *last_update == trigger_time
        };

        if should_trigger {
            if let Some(copilot_window) = app_clone.get_window("copilot") {
                if copilot_window.is_focused().unwrap_or(false) {
                    log::info!("Copilot window is focused, skipping self-activation to prevent feedback loop.");
                    return;
                }
            }
            if let Some(main_window) = app_clone.get_window("main") {
                if main_window.is_focused().unwrap_or(false) {
                    log::info!("Main window is focused, skipping Copilot activation.");
                    return;
                }
            }

            log::info!("Clipboard content stable. Showing Copilot window without focus.");
            if let Err(e) = crate::commands::system::show_window("copilot", app_clone, false).await {
                 log::error!("Failed to show copilot window from clipboard monitor: {}", e);
            }
        } else {
            log::info!("Clipboard changed again, skipping activation for this event.");
        }
    });
}